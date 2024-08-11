use std::future::Future;
use std::sync::Arc;

use futures_util::StreamExt;
use lapin::{BasicProperties, Channel, options::*, types::FieldTable};
use log::{error, info};
use tokio::runtime::Handle;

pub use {
    lapin::ExchangeKind,
    lapin::message::{Delivery, DeliveryResult},
};

use crate::MEDULLAH;
use crate::prelude::{AppResult, OnceLockHelper};

pub mod conn;

#[derive(Clone)]
pub struct RabbitMQ {
    pub publish_channel: Channel,
    pub consume_channel: Channel,
    pub nack_on_failure: bool,
}

#[derive(Default)]
pub struct RabbitMQOptions {
    pub nack_on_failure: bool,
}

impl RabbitMQ {
    // Create a new instance and connect to the RabbitMQ server
    pub async fn new() -> AppResult<Self> {
        Self::new_opt(RabbitMQOptions {
            nack_on_failure: true,
        })
        .await
    }

    pub async fn new_opt(opt: RabbitMQOptions) -> AppResult<Self> {
        let connection = MEDULLAH.rabbitmq();
        let publish_channel = connection.create_channel().await?;
        let consume_channel = connection.create_channel().await?;
        Ok(Self {
            publish_channel,
            consume_channel,
            nack_on_failure: opt.nack_on_failure,
        })
    }

    pub fn set_nack_on_failure(&mut self, state: bool) {
        self.nack_on_failure = state;
    }

    // Declare an exchange
    pub async fn declare_exchange(
        &self,
        exchange: &str,
        kind: lapin::ExchangeKind,
    ) -> AppResult<()> {
        self.publish_channel
            .exchange_declare(
                exchange,
                kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }

    // Declare a queue
    pub async fn declare_queue(&self, queue: &str) -> AppResult<()> {
        self.publish_channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;
        Ok(())
    }

    // Bind a queue to an exchange with a routing key
    pub async fn bind_queue(
        &self,
        queue: &str,
        exchange: &str,
        routing_key: &str,
    ) -> AppResult<()> {
        self.publish_channel
            .queue_bind(
                queue,
                exchange,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }

    // Publish a message to a specified exchange and routing key
    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> AppResult<()> {
        self.publish_channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        Ok(())
    }

    // Consume messages from a specified queue and execute an async function on each message
    pub async fn consume<F, Fut>(&'static self, queue: &str, tag: &str, func: F) -> AppResult<()>
    where
        F: FnOnce(Arc<Self>, Delivery) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        info!("subscribing to {}...", queue);

        let mut consumer = self
            .consume_channel
            .basic_consume(
                queue,
                tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let instance = Arc::new(self.clone());

        Handle::current().spawn(async move {
            while let Some(result) = consumer.next().await {
                if let Ok(delivery) = result {
                    let instance = instance.clone();

                    Handle::current().spawn(async move {
                        let delivery_tag = delivery.delivery_tag;
                        match func(instance, delivery).await {
                            Ok(_) => {}
                            Err(err) => {
                                if self.nack_on_failure {
                                    let _ = self.nack(delivery_tag, true).await;
                                }

                                error!("[consume-executor] returned error: {:?}", err);
                            }
                        }
                    });
                }
            }
        });

        Ok(())
    }

    // Acknowledge a message
    pub async fn ack(&self, delivery_tag: u64) -> AppResult<()> {
        self.consume_channel
            .basic_ack(delivery_tag, BasicAckOptions::default())
            .await?;
        Ok(())
    }

    // Negatively acknowledge a message
    pub async fn nack(&self, delivery_tag: u64, requeue: bool) -> AppResult<()> {
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
}
