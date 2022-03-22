extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use xio::common::*;
use xio::netio::*;

fn test01(io: &mut NetIO) {
  let partyid = io.partyid(); // self partyid
  let msgid1 = "testmessgeidone".to_string();
  let msgid2 = "testmessgeidtwo".to_string();
  let msgid3 = "testmessgeidthr".to_string();
  let msgid4 = "testmessgeidfou".to_string();

  let mut data = "0-abcd-0-efgh".to_string().into_bytes();
  data[0] = partyid as u8;

  for _ in 0..10 {
    thread::sleep(Duration::from_millis(1000));

    if partyid == 0 {
      thread::sleep(Duration::from_millis(500));
      data[7] = 1;
      info!("send data from {} to 1: {} {:?}", partyid, msgid1, data);
      io.send(1, &msgid1, &data).unwrap();

      data[7] = 2;
      info!("send data from {} to 1: {} {:?}", partyid, msgid2, data);
      io.send(1, &msgid2, &data).unwrap();

      data[7] = 3;
      info!("send data from {} to 1: {} {:?}", partyid, msgid3, data);
      io.send(1, &msgid3, &data).unwrap();

      data[7] = 4;
      info!("send data from {} to 1: {} {:?}", partyid, msgid4, data);
      io.send(1, &msgid4, &data).unwrap();

      //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
      let d = io.recv(2, &msgid1).unwrap();
      info!("recv data from 2 at {}: {} {:?}", partyid, msgid1, d);
    }
    if partyid == 1 {
      // recv unorder
      let d = io.recv(0, &msgid4).unwrap();
      info!("recv data from 0 at {}: {} {:?}", partyid, msgid4, d);

      let d = io.recv(0, &msgid1).unwrap();
      info!("recv data from 0 at {}: {} {:?}", partyid, msgid1, d);

      thread::sleep(Duration::from_millis(300));
      ///////////////////////////////////////////////////////////////////
      data[7] = 3;
      info!("send data from {} to 2: {} {:?}", partyid, msgid3, data);
      io.send(2, &msgid3, &data).unwrap();
      ///////////////////////////////////////////////////////////////////

      let d = io.recv(0, &msgid3).unwrap();
      info!("recv data from 0 at {}: {} {:?}", partyid, msgid3, d);

      let d = io.recv(0, &msgid2).unwrap();
      info!("recv data from 0 at {}: {} {:?}", partyid, msgid2, d);

      ///////////////////////////////////////////////////////////////////
      data[7] = 4;
      info!("send data from {} to 2: {} {:?}", partyid, msgid4, data);
      io.send(2, &msgid4, &data).unwrap();
      ///////////////////////////////////////////////////////////////////

      //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
      let d = io.recv(2, &msgid1).unwrap();
      info!("recv data from 2 at {}: {} {:?}", partyid, msgid1, d);
    }
    if partyid == 2 {
      let d = io.recv(1, &msgid4).unwrap();
      info!("recv data from 1 at {}: {} {:?}", partyid, msgid4, d);

      let d = io.recv(1, &msgid3).unwrap();
      info!("recv data from 1 at {}: {} {:?}", partyid, msgid3, d);

      thread::sleep(Duration::from_millis(800));

      //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
      data[7] = 11;
      io.broadcast(&msgid1, &data).unwrap();
    }
  }
}

fn main() {
  // Set the log level and init the logger
  //! Note, you can replace `pretty_env_logger` with your favorite log library
  env::set_var("RUST_LOG", "info");
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

  // Get the communication statistics
  info!("{:?}", io.stat());

  // Close the connections and stop the server.
  io.stop();

  info!("end ok");
}
