use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct Topic<T> {
    pub path: &'static str,
    t: PhantomData<T>,
}

impl<T> Topic<T> {
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            t: PhantomData,
        }
    }
}

pub struct Output<T: Serialize> {
    pub topic: Topic<T>,
}

impl<T: Serialize> Output<T> {
    pub fn new(topic: Topic<T>) -> Self {
        Self { topic }
    }
    pub fn send(&self, value: &T) {
        todo!()
    }
}

pub struct Input<T: Deserialize<'static>> {
    pub topic: Topic<T>,
}

impl<'a, T: Deserialize<'static>> Input<T> {
    pub fn new(topic: Topic<T>) -> Self {
        Self { topic }
    }
    pub async fn recv(&self) -> Result<T, RecvError> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum RecvError {
    #[error("Invalid data received")]
    InvalidData(Vec<u8>),
    #[error("Shutdown")]
    Shutdown,
}
