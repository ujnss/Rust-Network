extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::thread;
use std::time;
use xio::{common::*, netio::*, *};

fn test01(io: &mut dyn NetIO, partyids: &Vec<u32>) {
  let partyid = io.partyid(); // self partyid
  let data = "0".to_string().into_bytes();

  for i in 0..5 {
    let msgid = i.to_string();
    for j in 0..5 {
      let st = time::Instant::now();
      io.broadcast(&msgid, &data).unwrap();
      for p in partyids.clone() {
        if p == partyid {
          continue;
        }
        let _ = io.recv(p, &msgid).unwrap();
      }
      println!(
        "partyid: {} loop: {}-{} elapsed(s): {}",
        partyid,
        i,
        j,
        st.elapsed().as_secs_f64()
      );
    }
  }
}

fn main() {
  // Set the log level and init the logger
  //! Note, you can replace `pretty_env_logger` with your favorite log library
  env::set_var("RUST_LOG", "error");
  pretty_env_logger::init_timed();

  let partyids = vec![1, 3, 5, 6, 7];
  let parties = partyids.len() as u32;
  let mut dispatchers = Vec::new();
  for partyid in partyids.clone() {
    let partyids_ = partyids.clone();
    let dispatcher = thread::spawn(move || {
      info!("partyid: {}", partyid);
      let participants = make_default_participants!(parties, &partyids_, 55555);
      let mut io = NetIOX::new(partyid, &participants).expect("new NetIO");
      test01(&mut io, &partyids_);
      println!("partyid: {} {:?}", partyid, io.stat());
      println!("partyid: {} {:?}", partyid, io.agg_stat(partyids_[0]));
      io.stop();
    });
    dispatchers.push(dispatcher);
  }

  for dispatcher in dispatchers {
    let _ = dispatcher.join();
  }

  info!("end ok");
}
