
use core::{
    option::Option,
    result::Result,
};

use rtic_sync::channel::{ NoReceiver, Receiver, Sender };
use crate::stream::{ ByteSink, ByteStream, Sink, Stream };

pub struct ChannelStream <'a, T, const N: usize> (Receiver<'a, T, N>);

impl <'a, T, const N: usize>  ChannelStream<'a, T, N> {
    pub fn new(receiver: Receiver<'a, T, N>) -> Self {
        ChannelStream(receiver)
    }
}

impl <'a, T, const N: usize> Stream for ChannelStream<'a, T, N> {
    type Item = T;

    async fn next(&mut self) -> Option<T> {
        match self.0.recv().await {
            Ok(byte) => Some(byte),
            Err(_) => None,
        }
    }
}

impl <'a, const N: usize> ByteStream for ChannelStream<'a, u8, N> {

}

pub struct ChannelSink<'a, T, const N: usize>(Sender<'a, T, N>);

impl <'a, T, const N: usize> ChannelSink<'a, T, N> {
    pub fn new(sender: Sender<'a, T, N>) -> Self {
        ChannelSink(sender)
    }
}

impl <'a, T, const N: usize> Sink for ChannelSink<'a, T, N> {
    type Item = T;
    type Error = NoReceiver<T>;

    async fn send(&mut self, item: T) -> Result<(), Self::Error> {
        self.0.send(item).await
    }
}

impl <'a, const N: usize> ByteSink for ChannelSink<'a, u8, N> {

}
