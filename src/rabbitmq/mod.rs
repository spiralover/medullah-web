use futures_util::StreamExt;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, ChannelState, ConnectionState};
use log::{error, info, warn};
use std::future::Future;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub use {
    lapin::message::{Delivery, DeliveryResult},
    lapin::types::ReplyCode,
    lapin::{options::*, ExchangeKind},
};

use crate::prelude::{AppMessage, AppResult, OnceLockHelper};
pub use crate::rabbitmq::message::Message;
use crate::MEDULLAH;

pub mod conn;
mod message;

#[derive(Clone)]
pub struct RabbitMQ {
    conn_pool: deadpool_lapin::Pool,
    publish_channel: Channel,
    consume_channel: Channel,
    /// helps determine if the connection can be reconnected
    can_reconnect: bool,
    /// automatically nack a message if the handler returns an error.
    nack_on_failure: bool,
    /// whether to requeue a message if the handler returns an error.
    requeue_on_failure: bool,
    /// whether the handler should be executed in the background (asynchronously) or not.
    execute_handler_asynchronously: bool,
    /// max reconnection attempts, defaults to 1,000,000
    max_reconnection_attempts: usize,
    /// max reconnection delay, defaults to 1 second
    max_reconnection_delay: Duration,
    /// default publish options
    default_publish_options: BasicPublishOptions,
    /// default publish properties
    default_publish_props: BasicProperties,
    /// default consume options
    default_consume_options: BasicConsumeOptions,
}

#[derive(Default)]
pub struct RabbitMQOptions {
    /// automatically nack a message if the handler returns an error.
    pub nack_on_failure: bool,
    /// whether to requeue a message if the handler returns an error.
    pub requeue_on_failure: bool,
    /// whether the handler should be executed in the background (asynchronously) or not.
    pub execute_handler_asynchronously: bool,
}
impl RabbitMQ {
    const RETRY_DELAY: Duration = Duration::from_secs(2);

    /// Create a new instance and connect to the RabbitMQ server
    pub async fn new(pool: deadpool_lapin::Pool) -> AppResult<Self> {
        Self::new_opt(
            pool,
            RabbitMQOptions {
                nack_on_failure: true,
                requeue_on_failure: true,
                execute_handler_asynchronously: true,
            },
        )
        .await
    }

    /// Create a new instance with connection from medullah static context
    pub async fn new_from_medullah() -> AppResult<Self> {
        Self::new_opt(
            MEDULLAH.rabbitmq_pool(),
            RabbitMQOptions {
                nack_on_failure: true,
                requeue_on_failure: true,
                execute_handler_asynchronously: true,
            },
        )
        .await
    }

    pub async fn new_opt(pool: deadpool_lapin::Pool, opt: RabbitMQOptions) -> AppResult<Self> {
        let connection = pool.get().await?;
        let publish_channel = connection.create_channel().await?;
        let consume_channel = connection.create_channel().await?;

        Ok(Self {
            conn_pool: pool,
            publish_channel,
            consume_channel,
            can_reconnect: true,
            max_reconnection_attempts: 1_000_000,
            max_reconnection_delay: Duration::from_secs(1),
            nack_on_failure: opt.nack_on_failure,
            requeue_on_failure: opt.requeue_on_failure,
            execute_handler_asynchronously: opt.execute_handler_asynchronously,
            default_publish_options: BasicPublishOptions::default(),
            default_publish_props: BasicProperties::default(),
            default_consume_options: BasicConsumeOptions::default(),
        })
    }

    /// Set whether to nack a message if the handler returns an error.
    /// Default value is `true`
    pub fn nack_on_failure(&mut self, state: bool) -> &mut Self {
        self.nack_on_failure = state;
        self
    }

    /// Set whether to requeue a message if the handler returns an error.
    /// Default value is `true`
    pub fn requeue_on_failure(&mut self, state: bool) -> &mut Self {
        self.requeue_on_failure = state;
        self
    }

    /// Set whether the handler should be executed in the background (asynchronously) or not.
    /// Default value is `true`
    pub fn execute_handler_asynchronously(&mut self, state: bool) -> &mut Self {
        self.execute_handler_asynchronously = state;
        self
    }

    pub async fn declare_exchange(&mut self, exchange: &str, kind: ExchangeKind) -> AppResult<()> {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .exchange_declare(
                exchange,
                kind.clone(),
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn declare_queue(
        &mut self,
        queue: &str,
        options: QueueDeclareOptions,
        args: FieldTable,
    ) -> AppResult<()> {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .queue_declare(queue, options, args)
            .await?;

        Ok(())
    }

    pub async fn bind_queue<R: ToString>(
        &mut self,
        queue: &str,
        exchange: &str,
        routing_key: R,
        options: QueueBindOptions,
        args: FieldTable,
    ) -> AppResult<()> {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .queue_bind(queue, exchange, &routing_key.to_string(), options, args)
            .await?;

        Ok(())
    }

    pub async fn publish<E, R>(
        &mut self,
        exchange: E,
        routing_key: R,
        payload: &[u8],
    ) -> AppResult<()>
    where
        E: ToString,
        R: ToString,
    {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .basic_publish(
                &exchange.to_string(),
                &routing_key.to_string(),
                self.default_publish_options,
                payload,
                self.default_publish_props.clone(),
            )
            .await?;

        Ok(())
    }

    pub async fn consume<F, Fut>(&mut self, queue: &str, tag: &str, func: F) -> AppResult<()>
    where
        F: Fn(Message) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        info!("subscribing to '{}'...", queue);
        loop {
            match self.start_consume(queue, tag, func).await {
                Ok(_) => {
                    info!("Consumer {} stopped normally", tag);
                    break;
                }
                Err(err) => {
                    error!(
                        "Consumer {} encountered an error: {:?}, restarting...",
                        tag, err
                    );
                    sleep(Self::RETRY_DELAY).await;
                }
            }
        }
        Ok(())
    }

    async fn start_consume<F, Fut>(&mut self, queue: &str, tag: &str, func: F) -> AppResult<()>
    where
        F: Fn(Message) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        self.ensure_channel_is_usable(false).await?;

        let mut consumer = self
            .consume_channel
            .basic_consume(
                queue,
                tag,
                self.default_consume_options,
                FieldTable::default(),
            )
            .await?;

        let instance = self.clone();
        while let Some(result) = consumer.next().await {
            if let Ok(delivery) = result {
                let mut instance = instance.clone();
                let consumer_tag = tag.to_owned();

                let handler = async move {
                    let delivery_tag = delivery.delivery_tag;
                    match func(Message::new(delivery)).await {
                        Ok(_) => {}
                        Err(err) => {
                            if instance.nack_on_failure {
                                let _ = instance
                                    .nack(delivery_tag, instance.requeue_on_failure)
                                    .await;
                            }
                            error!(
                                "[consume-executor][{}] returned error: {:?}",
                                consumer_tag, err
                            );
                        }
                    }
                };

                if self.execute_handler_asynchronously {
                    Handle::current().spawn(handler);
                } else {
                    handler.await;
                }
            }
        }

        Ok(())
    }

    /// Consume messages from a specified queue and execute an async function on each message
    /// This method will run in detached mode :)
    pub async fn consume_detached<F, Fut>(
        &self,
        queue: &str,
        tag: &str,
        func: F,
    ) -> JoinHandle<AppResult<()>>
    where
        F: Fn(Message) -> Fut + Copy + Send + Sync + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let tag = tag.to_owned();
        let queue = queue.to_owned();
        let instance = self.clone();
        Handle::current().spawn(async move {
            let mut instance = instance.clone();
            instance.consume(&queue, &tag, func).await
        })
    }

    pub async fn ack(&mut self, delivery_tag: u64) -> AppResult<()> {
        self.ensure_channel_is_usable(false).await?;

        self.consume_channel
            .basic_ack(delivery_tag, BasicAckOptions::default())
            .await?;

        Ok(())
    }

    pub async fn nack(&mut self, delivery_tag: u64, requeue: bool) -> AppResult<()> {
        self.ensure_channel_is_usable(false).await?;

        self.consume_channel
            .basic_nack(
                delivery_tag,
                BasicNackOptions {
                    multiple: false,
                    requeue,
                },
            )
            .await?;

        Ok(())
    }

    /// Request a connection close.
    ///
    /// This method is only successful if the connection is in the connected state,
    /// otherwise an [`InvalidConnectionState`] error is returned.
    ///
    pub async fn close(&mut self, reply_code: ReplyCode, reply_text: &str) -> AppResult<()> {
        let connection = self.conn_pool.get().await?;
        self.can_reconnect = false;
        Ok(connection.close(reply_code, reply_text).await?)
    }

    /// Acquire connection pool in use by this instance
    pub fn connection_pool(&self) -> deadpool_lapin::Pool {
        self.conn_pool.clone()
    }

    async fn ensure_channel_is_usable(&mut self, is_publish_channel: bool) -> AppResult<()> {
        loop {
            let channel = match is_publish_channel {
                true => &self.publish_channel,
                false => &self.consume_channel,
            };

            // Check if the connection is still valid before checking the channel
            let connection = self.conn_pool.get().await;
            if connection.is_err() {
                warn!("Lost connection to RabbitMQ, attempting to reconnect...");
                self.recreate_connection().await?;
                continue;
            }

            let state = channel.status().state();
            match state {
                ChannelState::Closed | ChannelState::Closing | ChannelState::Error => {
                    warn!("Channel is not usable: {state:?}, recreating...");
                    self.recreate_channel(is_publish_channel).await?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    async fn recreate_channel(&mut self, is_publish_channel: bool) -> AppResult<()> {
        info!("Recreating unusable channel...");

        if !self.can_reconnect {
            warn!("Cannot reconnect, channel recreation aborted");
            return Err(AppMessage::RabbitmqError(
                lapin::Error::InvalidConnectionState(ConnectionState::Closed),
            ));
        }

        let connection = self.conn_pool.get().await?;
        let state = connection.status().state();

        if state != ConnectionState::Connected {
            warn!("Connection is not usable: {state:?}, attempting to re-establish...");
            self.recreate_connection().await?;
        }

        sleep(Self::RETRY_DELAY).await;

        info!("Performing channel recreation...");
        let result = match is_publish_channel {
            true => connection.create_channel().await,
            false => connection.create_channel().await,
        };

        if result.is_err() {
            warn!("Failed to recreate channel, attempting to re-establish connection...");
            self.recreate_connection().await?;
        }

        match is_publish_channel {
            true => self.publish_channel = connection.create_channel().await?,
            false => self.consume_channel = connection.create_channel().await?,
        };

        info!("Channel recreation completed ✔");

        Ok(())
    }

    async fn recreate_connection(&mut self) -> AppResult<()> {
        if !self.can_reconnect {
            warn!("Cannot reconnect, re-establishing connection aborted");
            return Err(AppMessage::RabbitmqError(
                lapin::Error::InvalidConnectionState(ConnectionState::Closed),
            ));
        }

        let mut delay = self.max_reconnection_delay;
        for attempt in 1..=self.max_reconnection_attempts {
            info!("Attempting to reconnect to RabbitMQ, attempt {attempt}...");
            match self.conn_pool.get().await {
                Ok(connection) => {
                    self.publish_channel = connection.create_channel().await?;
                    self.consume_channel = connection.create_channel().await?;
                    info!(
                        "Reconnected to RabbitMQ successfully on attempt {}",
                        attempt
                    );
                    return Ok(());
                }
                Err(err) => {
                    warn!(
                        "Failed to reconnect to RabbitMQ (attempt {}): {}",
                        attempt, err
                    );
                    sleep(delay).await;
                    delay = delay.saturating_mul(2); // Exponential backoff
                }
            }
        }

        error!("Max reconnection attempts reached, giving up");
        Err(AppMessage::RabbitmqError(
            lapin::Error::InvalidConnectionState(ConnectionState::Closed),
        ))
    }
}
