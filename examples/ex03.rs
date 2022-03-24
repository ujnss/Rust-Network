extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::thread;
use xio::common::*;
use xio::netio::*;

fn test01(io: &mut NetIO) {
  let partyid = io.partyid(); // self partyid
  let msgid1 = "testmessgeidone".to_string();
  let msgid2 = "testmessgeidtwo".to_string();

  let mut data = "0-abcd-0-efgh".to_string().into_bytes();
  data[0] = partyid as u8;

  // case 1
  if partyid == 0 {
    data[7] = 11;
    io.broadcast(&msgid1, &data).unwrap();
  } else {
    let d = io.recv(0, &msgid1).unwrap();
    info!("1* recv data from 0 at {}: {} {:?}", partyid, msgid1, d);
  }

  // case 2
  data[7] = 12;
  io.broadcast(&msgid2, &data).unwrap();

  for p in 0..io.parties() {
    if p == partyid {
      continue;
    }
    let d = io.recv(p, &msgid2).unwrap();
    info!("2* recv data from {} at {}: {} {:?}", p, partyid, msgid2, d);
  }
}

fn main() {
  // Set the log level and init the logger
  //! Note, you can replace `pretty_env_logger` with your favorite log library
  env::set_var("RUST_LOG", "info");
  pretty_env_logger::init_timed();

  let parties = 5;
  let mut dispatchers = Vec::new();
  for partyid in 0..parties {
    let dispatcher = thread::spawn(move || {
      info!("partyid: {}", partyid);
      let participants = get_default_participants(parties);
      let mut io = NetIO::new(partyid, &participants).expect("new NetIO");
      test01(&mut io);
      info!("partyid: {} {:?}", partyid, io.stat());
      io.stop();
    });
    dispatchers.push(dispatcher);
  }

  for dispatcher in dispatchers {
    let _ = dispatcher.join();
  }

  info!("end ok");
}
