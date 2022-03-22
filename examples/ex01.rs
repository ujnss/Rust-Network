extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use structopt::StructOpt;
use xio::common::*;
use xio::netio::*;

fn _testlog() {
  env::set_var("RUST_LOG", "trace");
  pretty_env_logger::init();

  error!("this is a error");
  warn!("this is a warning");
  info!("this is a information");
  debug!("this is a debug");
  trace!("this is a trace");
}

fn test01(io: &mut NetIO) {
  let partyid = io.partyid(); // self partyid
  let msgid = "for test".to_string();
  let mut data = "0-abcd-efgh-ijkl-mnop".to_string().into_bytes();
  data[0] = partyid as u8;
  info!("msgid {:?}", msgid);
  info!("send data from {}: {:?}", partyid, data);

  if partyid == 0 {
    io.send(1, msgid.clone(), &data).unwrap();
    let d = io.recv(2, msgid.clone()).unwrap();
    info!("recv data from 2: {:?}", d);
  }
  if partyid == 1 {
    let d = io.recv(0, msgid.clone()).unwrap();
    info!("recv data from 0: {:?}", d);
    io.send(2, msgid.clone(), &data).unwrap();
  }
  if partyid == 2 {
    let d = io.recv(1, msgid.clone()).unwrap();
    info!("recv data from 1: {:?}", d);
    io.send(0, msgid.clone(), &data).unwrap();
  }
}

fn main() {
  env::set_var("RUST_LOG", "trace");
  pretty_env_logger::init_timed();

  // get "--party_id <id>" from console command
  let opt = NetIOCommandOpt::from_args();
  let partyid = opt.party_id;
  info!("partyid {}", partyid);

  // Set the participants, from default value[for debug/test or simple usage]
  // You can set it(struct Participant) according to your needs.
  let participants = get_default_participants(3);

  // New a NetIO
  let mut io = NetIO::new(partyid, &participants).expect("new NetIO");

  // Then, do what you want to do
  test01(&mut io);

  // Todo!!! Close the connections and stop the server.
  io.stop();

  info!("end ok");
}
