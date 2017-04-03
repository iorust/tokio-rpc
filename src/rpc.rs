//! A simple client and server implementation for a multiplexed RPC.

#![deny(warnings, missing_docs)]

use std::{io, str};
use std::net::SocketAddr;

use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::{TcpClient, TcpServer};
use tokio_proto::multiplex::{RequestId, ServerProto, ClientProto, ClientService};
use tokio_service::{Service, NewService};
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};

const MAGIC_VER: u8 = 42;
const MAGIC_VER_VAL: u64 = 42 << 56;

/// RPC
struct RPC<T> {
    inner: T,
}

/// Client
pub struct Client {
    inner: RPC<ClientService<TcpStream, RPCProto>>,
}

/// RPCCodec is multiplexed codec
struct RPCCodec;

/// Protocol definition
struct RPCProto;

/// Start a server, listening for connections on `addr`.
///
/// For each new connection, `new_service` will be used to build a `Service`
/// instance to process requests received on the new connection.
///
/// This function will block as long as the server is running.
pub fn serve<T>(addr: SocketAddr, new_service: T)
    where T: NewService<Request = Vec<u8>, Response = Vec<u8>, Error = io::Error> + Send + Sync + 'static
{
    TcpServer::new(RPCProto {}, addr).serve(RPC { inner: new_service });
}

impl<T> Service for RPC<T>
    where T: Service<Request = Vec<u8>, Response = Vec<u8>, Error = io::Error>,
          T::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(self.inner.call(req))
    }
}

impl<T> NewService for RPC<T>
    where T: NewService<Request = Vec<u8>, Response = Vec<u8>, Error = io::Error>,
          <T::Instance as Service>::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Instance = RPC<T::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(RPC { inner: inner })
    }
}

impl Client {
    /// Establish a connection to a multiplexed protobuf protocol server at the
    /// provided `addr`.
    pub fn connect(addr: &SocketAddr,
                   handle: &Handle)
                   -> Box<Future<Item = Client, Error = io::Error>> {
        let ret = TcpClient::new(RPCProto)
            .connect(addr, handle)
            .map(|client_service| {
                     let s = RPC { inner: client_service };
                     Client { inner: s }
                 });

        Box::new(ret)
    }
}

impl Service for Client {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req)
    }
}

/// Implementation of the multiplexed protobuf protocol.
/// # An example frame:
///
/// +-- MAGIC_VER: 1 --+--- request_id: 7 ---+-- payload_len: 4 --+-- payload --+
/// |  0b00101010, 42  |  0b00000000000001   | 0xffffffff, 4G - 1 |   message   |
/// +------------------+---------------------+--------------------+-------------+
///
impl Decoder for RPCCodec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf_len = buf.len();
        if buf_len < 12 {
            return Ok(None);
        }
        if buf[0] != MAGIC_VER {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("invalid magic flag {}, must be {}",
                                              buf[8],
                                              MAGIC_VER)));
        }

        let payload_len = BigEndian::read_u32(&(buf.as_ref()[8..12])) as usize;
        let data_len = 12 + payload_len;
        if buf_len < data_len {
            return Ok(None);
        }

        let mut data = buf.split_to(data_len);
        data[0] = 0; // remove MAGIC_VER
        let request_id = BigEndian::read_u64(&data[0..8]);
        data.split_to(12);
        Ok(Some((request_id as RequestId, data.to_vec())))
    }
}

impl Encoder for RPCCodec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        let (request_id, msg) = msg;
        let payload_len = msg.len();
        let len = 12 + payload_len + buf.len();
        buf.reserve(len); // Reserve enough space for the frame

        let id = request_id as u64;
        if id & MAGIC_VER_VAL != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, format!("invalid request_id {}", id)));
        }
        buf.put_u64::<BigEndian>(id | MAGIC_VER_VAL);
        buf.put_u32::<BigEndian>(payload_len as u32);
        buf.put_slice(msg.as_slice());
        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for RPCProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;

    type Transport = Framed<T, RPCCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RPCCodec))
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for RPCProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;

    type Transport = Framed<T, RPCCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(RPCCodec))
    }
}
