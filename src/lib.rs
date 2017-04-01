//! A simple client and server implementation fo a multiplexed

#![deny(warnings, missing_docs)]

extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;

pub use rpc::{Client, serve};

mod rpc;