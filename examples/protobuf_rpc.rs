extern crate tokio_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate protobuf;

use futures::{future, Future};
use tokio_core::reactor::Core;
use tokio_service::{Service, NewService};
use std::{io, thread};
use std::time::Duration;
use protobuf::core::{Message, parse_from_bytes};

mod rpcpb;

struct Demo;

impl Service for Demo {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req = parse_from_bytes::<rpcpb::Request>(req.as_slice());
        if let Err(err) = req {
            return future::err(io::Error::new(io::ErrorKind::Other, err)).boxed();
        }

        let req = req.unwrap();
        let req = req.get_cmd_ping_req();
        println!("Request {:?}", req);

        let mut msg = rpcpb::CmdPingResponse::new();
        msg.set_message(req.get_message().to_string());
        let mut res = rpcpb::Response::new();
        res.set_field_type(rpcpb::MessageType::CmdPing);
        res.set_cmd_ping_res(msg);

        match res.write_to_bytes() {
            Ok(val) => future::ok(val).boxed(),
            Err(err) => future::err(io::Error::new(io::ErrorKind::Other, err)).boxed(),
        }
    }
}

impl NewService for Demo {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Instance = Demo;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Demo {})
    }
}

pub fn main() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let demo = Demo {};
    thread::spawn(move || { tokio_rpc::serve(addr, demo); });
    // A bit annoying, but we need to wait for the server to connect
    thread::sleep(Duration::from_millis(100));

    let handle = core.handle();
    core.run(tokio_rpc::Client::connect(&addr, &handle).and_then(|client| {
            let mut msg = rpcpb::CmdPingRequest::new();
            msg.set_message("Hello world!".to_string());

            let mut req = rpcpb::Request::new();
            req.set_field_type(rpcpb::MessageType::CmdPing);
            req.set_cmd_ping_req(msg);
            let buf = req.write_to_bytes().unwrap();
            client.call(buf)
                .and_then(|res| {
                              let res = parse_from_bytes::<rpcpb::Response>(res.as_slice());
                              println!("CLIENT Res: {:?}", res);
                              Ok(())
                          })
                .or_else(|err| {
                             println!("CLIENT Err: {:?}", err);
                             Ok(())
                         })
        }))
        .unwrap();
}
