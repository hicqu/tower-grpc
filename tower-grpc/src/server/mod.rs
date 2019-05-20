pub mod client_streaming;
pub mod server_streaming;
pub mod streaming;
pub mod unary;
pub mod unimplemented;

use codec::{Codec, Streaming};
use generic::server::{
    ClientStreamingService, Grpc, ServerStreamingService, StreamingService, UnaryService,
};
use Body;

use http;
use protobuf;

pub fn unary<T, B, R>(service: T, request: http::Request<B>) -> unary::ResponseFuture<T, B, R>
where
    T: UnaryService<R>,
    R: protobuf::Message + Default,
    T::Response: protobuf::Message,
    B: Body,
{
    let mut grpc = Grpc::new(Codec::new());
    let inner = grpc.unary(service, request);
    unary::ResponseFuture::new(inner)
}

pub fn client_streaming<T, R, B>(
    service: &mut T,
    request: http::Request<B>,
) -> client_streaming::ResponseFuture<T, Streaming<R, B>>
where
    T: ClientStreamingService<Streaming<R, B>>,
    R: protobuf::Message + Default,
    T::Response: protobuf::Message,
    B: Body,
{
    let mut grpc = Grpc::new(Codec::new());
    let inner = grpc.client_streaming(service, request);
    client_streaming::ResponseFuture::new(inner)
}

pub fn server_streaming<T, B, R>(
    service: T,
    request: http::Request<B>,
) -> server_streaming::ResponseFuture<T, B, R>
where
    T: ServerStreamingService<R>,
    R: protobuf::Message + Default,
    T::Response: protobuf::Message,
    B: Body,
{
    let mut grpc = Grpc::new(Codec::new());
    let inner = grpc.server_streaming(service, request);
    server_streaming::ResponseFuture::new(inner)
}

pub fn streaming<T, R, B>(
    service: &mut T,
    request: http::Request<B>,
) -> streaming::ResponseFuture<T, Streaming<R, B>>
where
    T: StreamingService<Streaming<R, B>>,
    R: protobuf::Message + Default,
    T::Response: protobuf::Message,
    B: Body,
{
    let mut grpc = Grpc::new(Codec::new());
    let inner = grpc.streaming(service, request);
    streaming::ResponseFuture::new(inner)
}

pub fn unimplemented(message: String) -> unimplemented::ResponseFuture {
    unimplemented::ResponseFuture::new(message)
}
