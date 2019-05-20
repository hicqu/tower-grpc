use body::{BoxBody, HttpBody};
use generic::{DecodeBuf, EncodeBuf};

use bytes::{Buf, BufMut};
use futures::{Poll, Stream};
use http;
use protobuf::{CodedInputStream, CodedOutputStream, Message, ProtobufError};

use std::fmt;
use std::marker::PhantomData;

/// Protobuf codec
#[derive(Debug)]
pub struct Codec<T, U>(PhantomData<(T, U)>);

#[derive(Debug)]
pub struct Encoder<T>(PhantomData<T>);

#[derive(Debug)]
pub struct Decoder<T>(PhantomData<T>);

/// A stream of inbound gRPC messages
pub type Streaming<T, B = BoxBody> = ::generic::Streaming<Decoder<T>, B>;

pub(crate) use generic::Direction;

/// A pb encoded gRPC response body
pub struct Encode<T>
where
    T: Stream,
{
    inner: ::generic::Encode<Encoder<T::Item>, T>,
}

// ===== impl Codec =====

impl<T, U> Codec<T, U>
where
    T: Message,
    U: Message + Default,
{
    /// Create a new pb codec
    pub fn new() -> Self {
        Codec(PhantomData)
    }
}

impl<T, U> ::generic::Codec for Codec<T, U>
where
    T: Message,
    U: Message + Default,
{
    type Encode = T;
    type Encoder = Encoder<T>;
    type Decode = U;
    type Decoder = Decoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        Encoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        Decoder(PhantomData)
    }
}

impl<T, U> Clone for Codec<T, U> {
    fn clone(&self) -> Self {
        Codec(PhantomData)
    }
}

// ===== impl Encoder =====

impl<T> Encoder<T>
where
    T: Message,
{
    pub fn new() -> Self {
        Encoder(PhantomData)
    }
}

impl<T> ::generic::Encoder for Encoder<T>
where
    T: Message,
{
    type Item = T;

    /// Protocol buffer gRPC content type
    const CONTENT_TYPE: &'static str = "application/grpc+proto";

    fn encode(&mut self, item: T, buf: &mut EncodeBuf) -> Result<(), ::Status> {
        let len = item.compute_size() as usize;
        if buf.remaining_mut() < len {
            buf.reserve(len);
            assert!(unsafe { buf.bytes_mut().len() } >= len);
        }
        unsafe {
            item.write_to(&mut CodedOutputStream::bytes(buf.bytes_mut()))
                .unwrap();
            buf.advance_mut(len);
        }
        Ok(())
    }
}

impl<T> Clone for Encoder<T> {
    fn clone(&self) -> Self {
        Encoder(PhantomData)
    }
}

// ===== impl Decoder =====

impl<T> Decoder<T>
where
    T: Message + Default,
{
    /// Returns a new decoder
    pub fn new() -> Self {
        Decoder(PhantomData)
    }
}

#[allow(dead_code)]
fn from_decode_error(error: ProtobufError) -> ::Status {
    // Map Protobuf parse errors to an INTERNAL status code, as per
    // https://github.com/grpc/grpc/blob/master/doc/statuscodes.md
    ::Status::new(::Code::Internal, error.to_string())
}

impl<T> ::generic::Decoder for Decoder<T>
where
    T: Message + Default,
{
    type Item = T;

    fn decode(&mut self, buf: &mut DecodeBuf) -> Result<T, ::Status> {
        impl<'a> ::std::io::Read for DecodeBuf<'a> {
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
                let len = ::std::cmp::min(Buf::bytes(self).len(), buf.len());
                unsafe { ::std::ptr::copy(&Buf::bytes(self)[0], &mut buf[0], len) };
                self.advance(len);
                Ok(len)
            }
        }

        impl<'a> ::std::io::BufRead for DecodeBuf<'a> {
            fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
                Ok(Buf::bytes(self))
            }
            fn consume(&mut self, amt: usize) {
                self.advance(amt);
            }
        }

        let mut t = T::new();
        let mut s = CodedInputStream::from_buffered_reader(buf);
        t.merge_from(&mut s).unwrap();
        Ok(t)
    }
}

impl<T> Clone for Decoder<T> {
    fn clone(&self) -> Self {
        Decoder(PhantomData)
    }
}

// ===== impl Encode =====

impl<T> Encode<T>
where
    T: Stream<Error = ::Status>,
    T::Item: ::protobuf::Message,
{
    pub(crate) fn new(inner: ::generic::Encode<Encoder<T::Item>, T>) -> Self {
        Encode { inner }
    }
}

impl<T> HttpBody for Encode<T>
where
    T: Stream<Error = ::Status>,
    T::Item: ::protobuf::Message,
{
    type Data = <::generic::Encode<Encoder<T::Item>, T> as HttpBody>::Data;
    type Error = <::generic::Encode<Encoder<T::Item>, T> as HttpBody>::Error;

    fn is_end_stream(&self) -> bool {
        false
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.inner.poll_data()
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, Self::Error> {
        self.inner.poll_trailers()
    }
}

impl<T> fmt::Debug for Encode<T>
where
    T: Stream + fmt::Debug,
    T::Item: fmt::Debug,
    T::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Encode")
            .field("inner", &self.inner)
            .finish()
    }
}
