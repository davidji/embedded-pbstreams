
pub mod channel;

use core::{
    future::Future, result::Result
};

/// A trait for an asynchronous stream of items.
/// This trasit is very simular to the `Stream` trait in the `futures` crate
/// but see Sink below
pub trait Stream {
    type Item;

    fn next(&mut self) -> impl Future<Output = Option<Self::Item>>;
}

pub trait ByteStream: Stream<Item = u8> {}

/// A trait for a sink that can accept items asynchronously.
/// This trait is similar to the `Sink` trait in the `futures` crate,
/// but is much simpler and more closely aligned to rtc_sync's channel model.
/// It's convenient not to unify them for now, and SinkExt.send() would make that
/// change easier.
pub trait Sink {
    type Item;
    type Error;

    fn send(&mut self, item: Self::Item) -> impl Future<Output = Result<(), Self::Error>>;
}

pub async fn relay<I, O, T, E>(input: &mut I,  output: &mut O) -> Result<(), E>
where
    I: Stream<Item = T>,
    O: Sink<Item = T, Error = E>,
{
    while let Some(item) = input.next().await {
        output.send(item).await?;
    }
    Ok(())
}

pub trait ByteSink: Sink<Item = u8> {}
