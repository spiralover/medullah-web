use futures_util::StreamExt;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, ChannelState};
use log::{error, info, warn};
use std::future::Future;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub use {
    lapin::message::{Delivery, DeliveryResult},
    lapin::{options::*, ExchangeKind},
};

use crate::prelude::{AppResult, OnceLockHelper};
pub use crate::rabbitmq::message::Message;
use crate::MEDULLAH;

pub mod conn;
mod message;

#[derive(Clone)]
pub struct RabbitMQ {
    conn_pool: deadpool_lapin::Pool,
    publish_channel: Channel,
    consume_channel: Channel,
    /// automatically nack a message if the handler returns an error.
    nack_on_failure: bool,
    /// whether to requeue a message if the handler returns an error.
    requeue_on_failure: bool,
    /// whether the handler should be executed in the background (asynchronously) or not.
    execute_handler_asynchronously: bool,
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
            nack_on_failure: opt.nack_on_failure,
            requeue_on_failure: opt.requeue_on_failure,
            execute_handler_asynchronously: opt.execute_handler_asynchronously,
        })
    }

    pub fn nack_on_failure(&mut self, state: bool) {
        self.nack_on_failure = state;
    }

    pub fn requeue_on_failure(&mut self, state: bool) {
        self.requeue_on_failure = state;
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

    pub async fn declare_queue(&mut self, queue: &str) -> AppResult<()> {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;

        Ok(())
    }

    pub async fn bind_queue<R: ToString>(
        &mut self,
        queue: &str,
        exchange: &str,
        routing_key: R,
    ) -> AppResult<()> {
        self.ensure_channel_is_usable(true).await?;

        self.publish_channel
            .queue_bind(
                queue,
                exchange,
                &routing_key.to_string(),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
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
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn consume<F, Fut>(&mut self, queue: &str, tag: &str, func: F) -> AppResult<()>
    where
        F: Fn(Message) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        info!("subscribing to {}...", queue);
        self.ensure_channel_is_usable(false).await?;

        let mut consumer = self
            .consume_channel
            .basic_consume(
                queue,
                tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let instance = self.clone();
        while let Some(result) = consumer.next().await {
            let consumer_tag = tag.to_owned();

            if let Ok(delivery) = result {
                let mut instance = instance.clone();
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

    async fn ensure_channel_is_usable(&mut self, is_publish_channel: bool) -> AppResult<()> {
        loop {
            let channel = match is_publish_channel {
                true => &self.publish_channel,
                false => &self.consume_channel,
            };

            match channel.status().state() {
                ChannelState::Closed => {
                    warn!("channel is closed, reconnecting...");
                    self.recreate_channel(is_publish_channel).await?;
                }
                ChannelState::Closing => {
                    warn!("channel is closing, reconnecting...");
                    self.recreate_channel(is_publish_channel).await?;
                }
                ChannelState::Error => {
                    warn!("channel is in error state, reconnecting...");
                    self.recreate_channel(is_publish_channel).await?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    async fn recreate_channel(&mut self, is_publish_channel: bool) -> AppResult<()> {
        let connection = self.conn_pool.get().await?;

        sleep(Self::RETRY_DELAY).await;

        match is_publish_channel {
            true => {
                self.publish_channel = connection.create_channel().await?;
            }
            false => {
                self.consume_channel = connection.create_channel().await?;
            }
        }

        Ok(())
    }
}
