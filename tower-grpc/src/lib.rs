#![deny(warnings, missing_debug_implementations)]
//#![deny(missing_docs)]

extern crate base64;
extern crate bytes;
#[macro_use]
extern crate futures;
extern crate h2;
extern crate http;
#[macro_use]
extern crate log;
extern crate percent_encoding;
extern crate tower_http;
extern crate tower_service;
extern crate tower_util;

#[cfg(feature = "pb")]
extern crate prost;
#[cfg(feature = "tower-h2")]
extern crate tower_h2;
#[cfg(feature = "tower-hyper")]
extern crate tower_hyper;

pub mod client;
pub mod generic;
pub mod metadata;

mod body;
mod error;
mod request;
mod response;
mod status;

pub use body::{Body, BoxBody};
pub use request::Request;
pub use response::Response;
pub use status::{Code, Status};

#[cfg(feature = "pb")]
pub mod server;

/// Type re-exports used by generated code
#[cfg(feature = "pb")]
pub mod codegen;

#[cfg(feature = "pb")]
mod codec;

#[cfg(feature = "pb")]
pub use codec::{Encode, Streaming};
