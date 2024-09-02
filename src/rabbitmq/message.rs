use crate::prelude::AppResult;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicNackOptions};
use lapin::types::ShortString;

pub struct Message {
    delivery: Delivery,
}

impl Message {
    pub fn new(delivery: Delivery) -> Self {
        Self { delivery }
    }

    pub fn delivery(&self) -> &Delivery {
        &self.delivery
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.delivery.data
    }

    pub fn str(&self) -> AppResult<&str> {
        Ok(std::str::from_utf8(&self.delivery.data)?)
    }

    pub fn routing_key(&self) -> &ShortString {
        &self.delivery.routing_key
    }

    pub fn deserialize<T>(&self) -> AppResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(serde_json::from_slice(&self.delivery.data)?)
    }

    pub async fn ack(&self) -> AppResult<()> {
        self.delivery.acker.ack(BasicAckOptions::default()).await?;
        Ok(())
    }

    pub async fn nack(&self) -> AppResult<()> {
        self.delivery
            .acker
            .nack(BasicNackOptions::default())
            .await?;
        Ok(())
    }
}
