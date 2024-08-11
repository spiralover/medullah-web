use std::future::Future;
use std::sync::Arc;

use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicNackOptions, ExchangeDeclareOptions,
    QueueBindOptions, QueueDeclareOptions,
};
use lapin::{Channel, Connection, ExchangeKind};

use lapin::types::FieldTable;
use log::{error, info};

use crate::prelude::AppMessage;
use tokio::runtime::Handle;

use crate::results::AppResult;

#[derive(Clone)]
pub struct RabbitService {
    rabbit: Arc<Connection>,
}

impl RabbitService {
    pub async fn new(rabbit: Arc<Connection>) -> RabbitService {
        RabbitService { rabbit }
    }

    pub async fn consume<F, Fut>(
        &self,
        queue: String,
        tag: &str,
        blocking: bool,
        func: F,
    ) -> AppResult<()>
    where
        F: FnOnce(Delivery) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let channel = self.rabbit.create_channel().await?;

        let _rmq_queue = channel
            .queue_declare(
                queue.clone().as_str(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut consumer = channel
            .basic_consume(
                queue.clone().as_str(),
                tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("subscribing to {}...", queue);

        while let Some(result) = consumer.next().await {
            if let Ok(delivery) = result {
                if blocking {
                    match func(delivery).await {
                        Ok(_) => {}
                        Err(err) => {
                            error!("[consume-executor] returned error: {:?}", err);
                        }
                    }
                } else {
                    Handle::current().spawn(async move {
                        match func(delivery).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("[consume-executor] returned error: {:?}", err);
                            }
                        }
                    });
                }
            }
        }

        Ok(())
    }

    pub async fn ack(d: Delivery, ctx: String) -> AppResult<()> {
        match d.ack(BasicAckOptions::default()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to acknowledge({}): {:?}", ctx, err);
                Err(AppMessage::RabbitmqError(err))
            }
        }
    }

    pub async fn nack(d: Delivery, ctx: String) -> AppResult<()> {
        match d.nack(BasicNackOptions::default()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to not-acknowledge({}): {:?}", ctx, err);
                Err(AppMessage::RabbitmqError(err))
            }
        }
    }

    #[allow(dead_code)]
    async fn make_channel(
        rabbit: &Arc<Connection>,
        queue: String,
        exchange: String,
        routing_key: String,
    ) -> AppResult<Channel> {
        let channel = rabbit.create_channel().await?;

        channel
            .exchange_declare(
                &exchange,
                ExchangeKind::default(),
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let _rmq_queue = channel
            .queue_declare(
                &queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                &queue,
                &exchange,
                &routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(channel)
    }
}
