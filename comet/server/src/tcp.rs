use actix::Addr;
use core::Server;
use actix::Actor;
use actix::Context;
use actix::prelude::*;
use tokio_io::io::WriteHalf;
use tokio_io::AsyncRead;
use tokio_tcp::{TcpListener, TcpStream};
use std::net::SocketAddr;
use tokio_io::_tokio_codec::FramedRead;
use codec::GameCodec;
use actix_web::actix;
use session::ServerSession;
use futures::Stream;
use std::str::FromStr;

struct TcpServer {
    server: Addr<Server>
}

impl TcpServer {
    pub fn new(host: &str, port: u16, server: Addr<Server>) {
        let addr = SocketAddr::from_str(&format!("{}:{}", host, port)).unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        TcpServer::create(|ctx| {
            ctx.add_message_stream(
                listener
                    .incoming()
                    .map_err(|_| ())
                    .map(|s| TcpConnect(s)),
            );

            TcpServer { server }
        });
    }
}


impl Actor for TcpServer {
    type Context = Context<Self>;
}

#[derive(Message)]
struct TcpConnect(TcpStream);

/// Handle stream of TcpStream's
impl Handler<TcpConnect> for TcpServer {
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _: &mut Context<Self>) {
        let server = self.server.clone();
        ServerSession::create(|ctx| {
            let (r, w) = msg.0.split();
            ServerSession::add_stream(FramedRead::new(r, GameCodec), ctx);

            ServerSession::new(1, server, actix::io::FramedWrite::new(w, GameCodec, ctx))
        });
    }
}
