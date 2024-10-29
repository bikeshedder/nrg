use std::marker::PhantomData;

use thiserror::Error;
use tokio::sync::Mutex;
use tokio_modbus::client::{Context, Reader, Writer};

pub trait Type: Sized {
    const LEN: u16;
    fn decode(data: &[u16]) -> Option<Self>;
    fn encode(&self) -> Box<[u16]>;
}

impl Type for u16 {
    const LEN: u16 = 1;
    fn decode(data: &[u16]) -> Option<Self> {
        let [w0] = *data else { return None };
        Some(w0)
    }
    fn encode(&self) -> Box<[u16]> {
        Box::new([*self])
    }
}

impl Type for u32 {
    const LEN: u16 = 2;
    fn decode(data: &[u16]) -> Option<Self> {
        let [w1, w0] = *data else { return None };
        Some(((w1 as u32) << 16) + (w0 as u32))
    }
    fn encode(&self) -> Box<[u16]> {
        Box::new([(self << 16) as u16, *self as u16])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Register<T: Type> {
    pub name: &'static str,
    pub addr: u16,
    t: PhantomData<T>,
}

impl<T: Type> Register<T> {
    pub const fn new(name: &'static str, addr: u16) -> Self {
        Self {
            name,
            addr,
            t: PhantomData,
        }
    }
}

pub async fn read_register<T: Type>(
    ctx: &Mutex<Context>,
    reg: Register<T>,
) -> Result<T, ModbusError> {
    let data = ctx
        .lock()
        .await
        .read_holding_registers(reg.addr, T::LEN)
        .await??;
    Ok(T::decode(&data).unwrap())
}

#[derive(Debug, Error)]
pub enum ModbusError {
    #[error("Modbus error")]
    Error(#[from] tokio_modbus::Error),
    #[error("Modbus exception")]
    Exception(#[from] tokio_modbus::ExceptionCode),
}

pub async fn write_register(
    ctx: &Mutex<Context>,
    reg: Register<u16>,
    value: u16,
) -> Result<(), ModbusError> {
    ctx.lock()
        .await
        .write_single_register(reg.addr, value)
        .await??;
    Ok(())
}
