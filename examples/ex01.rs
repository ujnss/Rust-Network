extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use structopt::StructOpt;
use xio::{common::*, netio::*, *};

fn test01(io: &mut dyn NetIO) {
  let partyid = io.partyid(); // self partyid
  let msgid = "for test".to_string(); // "".to_string();
  let mut data = "0-abcd-efgh-ijkl-mnop".to_string().into_bytes();
  data[0] = partyid as u8;
  info!("msgid {:?}", msgid);
  info!("send data from {}: {:?}", partyid, data);

  if partyid == 0 {
    io.send(1, &msgid, &data).unwrap();
    let d = io.recv(2, &msgid).unwrap();
    info!("recv data from 2: {:?}", d);
  }
  if partyid == 1 {
    let d = io.recv(0, &msgid).unwrap();
    info!("recv data from 0: {:?}", d);
    io.send(2, &msgid, &data).unwrap();
  }
  if partyid == 2 {
    let d = io.recv(1, &msgid).unwrap();
    info!("recv data from 1: {:?}", d);
    io.send(0, &msgid, &data).unwrap();
  }
}

fn main() {
  // Set the log level and init the logger
  //! Note, you can replace `pretty_env_logger` with your favorite log library
  env::set_var("RUST_LOG", "info");
  pretty_env_logger::init_timed();

  // get "--party_id <id>" from console command
  let opt = CommandLineOpt::from_args();
  let partyid = opt.party_id;
  info!("netio option: {:?}", opt);

  // Set the participants, from default value[for debug/test or simple usage]
  // You can set it(struct Participant) according to your needs.
  let participants = make_default_participants!(3);

  // New a NetIO
  let io: &mut dyn NetIO = &mut NetIOX::new(partyid, &participants).expect("new NetIO");

  // Then, do what you want to do
  test01(io);

  // Get the communication statistics
  info!("{:?}", io.stat());

  // Close the connections and stop the server.
  io.stop();

  info!("end ok");
}
