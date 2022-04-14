extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::thread;
use std::time;
use xio::{common::*, netio::*, *};

fn test01(io: &mut dyn NetIO) {
  let partyid = io.partyid(); // self partyid
  let data = "0".to_string().into_bytes();

  for i in 0..5 {
    let msgid = i.to_string();
    for j in 0..5 {
      let st = time::Instant::now();
      io.broadcast(&msgid, &data).unwrap();
      for p in 0..io.parties() {
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

  let parties = 5;
  let mut dispatchers = Vec::new();
  for partyid in 0..parties {
    let dispatcher = thread::spawn(move || {
      info!("partyid: {}", partyid);
      let participants = make_default_participants!(parties, 55555);
      let mut io = NetIOX::new(partyid, &participants).expect("new NetIO");
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
