use crate::greeter_generated::foo::bar::HelloReply as FbsHelloReply;
use bytes::{Bytes, BytesMut};

pub struct HelloReply<T> {
    pub buffer: T,
}
impl From<Bytes> for HelloReply<Bytes> {
    fn from(buffer: Bytes) -> Self {
        Self { buffer }
    }
}

impl From<BytesMut> for HelloReply<BytesMut> {
    fn from(buffer: BytesMut) -> Self {
        Self { buffer }
    }
}

impl<'a> From<&'a [u8]> for HelloReply<&'a [u8]> {
    fn from(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
}

pub trait HelloReplyRead {
    fn message(&self) -> &str;
}

// pub trait HelloReplyWrite {
//     fn set_message(&mut self, message: &str);
// }

impl<T> HelloReplyRead for HelloReply<T>
where
    T: std::borrow::Borrow<[u8]>,
{
    fn message(&self) -> &str {
        let hello_reply = flatbuffers::get_root::<FbsHelloReply>(self.buffer.borrow());
        hello_reply.message().unwrap()
    }
}

// impl<T> HelloReplyWrite for HelloReply<T>
// where
//     T: std::borrow::BorrowMut<[u8]>,
// {
//     fn set_message(&mut self, message: &str) {}
// }
