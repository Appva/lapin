extern crate lapin_futures as lapin;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
#[macro_use] extern crate nom;

//use std::net::TcpStream;
use std::iter::repeat;
use std::io::{Read,Write,Error};
use std::collections::HashMap;
use std::{thread,time};
use std::net::SocketAddr;

use nom::HexDisplay;
use lapin::*;
use lapin::client::Client;
use futures::future::{self,Future};
use futures_cpupool::CpuPool;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

fn main() {
  let mut core = Core::new().unwrap();
  let mut stream = TcpStream::connect(&"127.0.0.1:5672".parse::<SocketAddr>().unwrap(),  &core.handle());

  let client_future = TcpStream::connect(&"127.0.0.1:5672".parse::<SocketAddr>().unwrap(),  &core.handle()).then(|stream| {
    let stream =stream.unwrap();
    Client::new(stream)
  });

  let mut client = core.run(client_future).unwrap();
  println!("got client with connection state: {:?}", client.connection.state);
}