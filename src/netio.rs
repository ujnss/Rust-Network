use crate::common::*;
use crate::node_service::*;
use crate::rsio::*;
use crate::rsio_grpc::*;
use anyhow::format_err;
use crossbeam_channel::*;
// use futures::executor::{ThreadPool, ThreadPoolBuilder};
use grpcio::*;
use log::*;
use std::collections::{HashMap, HashSet};
// use std::env;
// use std::io::Read;
use std::result;
use std::sync::{Arc, Mutex};
use std::time::Duration;
// use std::{io, result, thread};
use ::protobuf::Message;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake_case")]
pub struct NetIOCommandOpt {
  #[structopt(short, long, default_value = "-1")]
  pub party_id: u32,
}

pub struct NetIO {
  partyid: u32, // self party id
  server: Server,
  clients: HashMap<String, NodeServiceClient>, // {nodeid: client}

  // nodeid_partyid: HashMap<String, u32>, // {nodeid: partyid}
  partyid_nodeid: HashMap<u32, String>, // {partyid: nodeid}, including self

  msg_dispatcher: Arc<Mutex<MyMessageDispatcher>>,
  msg_rxs: HashMap<String, Receiver<OneData>>, // msgid --> message // Arc<Mutex<HashMap<String, Receiver<OneData>>>>
  msgid_lock: Arc<Mutex<HashSet<String>>>,     // msgid locker
  cachedid: HashSet<String>,                   // cached msgid for quickly check

  stat: Arc<Mutex<NetStat>>, // the communication statistics
}

impl NetIO {
  pub fn new(partyid: u32, participants: &Vec<Participant>) -> result::Result<Self, anyhow::Error> {
    // todo: valid check for partyid, addr, etc.
    let parties = participants.len();
    info!("participants: {}/{} {:?}", partyid, parties, participants);

    // init
    let mut clients = HashMap::new();
    // let mut nodeid_partyid = HashMap::new();
    let mut partyid_nodeid = HashMap::new();
    let (step_tx, step_rx) = unbounded::<StepCallRequest>();

    ////////////////////////////////////////////////
    // threads
    // let threads = ThreadPoolBuilder::new()
    //   .pool_size(8)
    //   .name_prefix("task")
    //   .create()
    //   .map_err(|e| format_err!("Create threadpool Err: {}", e))
    //   .unwrap();

    ////////////////////////////////////////////////
    let msg_dispatcher = Arc::new(Mutex::new(MyMessageDispatcher {
      step_rx: step_rx.clone(),
      msg_txs: Arc::new(Mutex::new(HashMap::new())),
    }));
    {
      let mut _dispatcher = msg_dispatcher.lock().map_err(|_| error!("")).unwrap();
      _dispatcher.spawn();
    }

    ////////////////////////////////////////////////
    // server
    let env = Arc::new(Environment::new(1));
    let msg_node = MyNodeService { step_tx: step_tx };
    let msg_node_service = create_node_service(msg_node);

    let quota = ResourceQuota::new(Some("ServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone())
      .set_resource_quota(quota)
      .keepalive_time(Duration::from_secs(6))
      .keepalive_timeout(Duration::from_secs(21));

    let self_node = participants.get(partyid as usize).unwrap();
    let vs: Vec<&str> = self_node.addr.splitn(2, ":").collect();
    let host = vs[0].to_string();
    let port = vs[1].to_string().parse::<u16>().unwrap();
    let mut server = ServerBuilder::new(env)
      .register_service(msg_node_service)
      .bind(host, port)
      .channel_args(ch_builder.build_args())
      .build()
      .map_err(|e| format_err!("Build server Err: {}", e))?;
    server.start();

    for (host, port) in server.bind_addrs() {
      info!("listening on {}:{}", host, port);
    }

    ////////////////////////////////////////////////
    // connect to clients
    for p in participants {
      // nodeid_partyid.insert(p.nodeid.clone(), p.partyid);
      partyid_nodeid.insert(p.partyid, p.nodeid.clone());
      if partyid != p.partyid {
        info!("connect to {} {}", p.partyid, p.addr);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(&p.addr);
        let client = NodeServiceClient::new(ch);
        clients.insert(p.nodeid.clone(), client);
      }
    }
    let stat = NetStat {
      sent_bytes: 0,
      sent_bytes_all: 0,
      sent_count: 0,
    };

    let s = Self {
      partyid: partyid,
      server: server,
      clients: clients,

      // nodeid_partyid: nodeid_partyid,
      partyid_nodeid: partyid_nodeid,

      msg_dispatcher: msg_dispatcher,
      msg_rxs: HashMap::new(),
      msgid_lock: Arc::new(Mutex::new(HashSet::new())),
      cachedid: HashSet::new(),

      stat: Arc::new(Mutex::new(stat)),
    };
    Ok(s)
  }

  pub fn init(&mut self) {}
  pub fn stop(&mut self) {
    // todo! close & shutdown
    // close client

    // close server
    self.server.shutdown();
    {
      let mut _dispatcher = self.msg_dispatcher.lock().map_err(|_| error!("")).unwrap();
      _dispatcher.stop();
    }
  }
  pub fn partyid(&self) -> u32 {
    return self.partyid;
  }
  pub fn stat(&self) -> NetStat {
    self.stat.lock().unwrap().clone()
  }

  fn make_partyid_msgid(&mut self, partyid: u32, msgid: &String) -> String {
    // todo: optimized
    let msgid_ = partyid.to_string() + msgid;
    return msgid_;
  }

  fn register_channel(&mut self, msgid: &String) {
    // quickly check
    if self.cachedid.contains(msgid) {
      return;
    }

    let mut msgid_lock = self.msgid_lock.lock().unwrap();
    if msgid_lock.contains(msgid) {
      return;
    }

    // if not registered
    let (msg_tx, msg_rx) = unbounded::<OneData>();
    let mut _dispatcher = self.msg_dispatcher.lock().map_err(|_| error!("")).unwrap();
    _dispatcher.register(msgid.clone(), msg_tx);

    self.msg_rxs.insert(msgid.clone(), msg_rx);
    msgid_lock.insert(msgid.clone());
    self.cachedid.insert(msgid.clone());
  }

  pub fn recv(&mut self, partyid: u32, msgid: &String) -> result::Result<Vec<u8>, anyhow::Error> {
    let msgid_ = self.make_partyid_msgid(partyid, msgid);
    self.register_channel(&msgid_);

    // todo: optimized
    let rx = self.msg_rxs.get_mut(&msgid_).unwrap();

    let mut loop_counter = 1;
    let errmsg;
    loop {
      loop_counter = loop_counter + 1;
      if loop_counter > 60 {
        errmsg = format!("recv timeout loop_counter:{}", loop_counter);
        break;
      }

      let res = rx.recv_timeout(Duration::from_secs(1));
      match res {
        Err(err) => match err {
          crossbeam_channel::RecvTimeoutError::Timeout => {
            debug!("channel recv timeout, retry");
          }
          crossbeam_channel::RecvTimeoutError::Disconnected => {
            errmsg = format!("channel recv err {:?}", err);
            break;
          }
        },
        Ok(vu8) => {
          // todo: optimized
          return Ok(vu8.data);
        }
      }
    }

    warn!("{}", errmsg);
    Err(format_err!("{}", errmsg))
  }
  pub fn send(
    &mut self,
    partyid: u32,
    msgid: &String,
    data: &Vec<u8>,
  ) -> result::Result<usize, anyhow::Error> {
    let msgid_ = self.make_partyid_msgid(self.partyid, msgid);
    self.register_channel(&msgid_);

    // let call_opt = CallOption::default().wait_for_ready(true).timeout(Duration::from_secs(5));
    let call_opt = CallOption::default().wait_for_ready(true);
    let mut req = StepCallRequest::default();
    req.set_msgid(msgid_);
    req.set_content(data.clone());
    {
      let mut stat = self.stat.lock().unwrap();
      stat.sent_count += 1;
      stat.sent_bytes += data.len();
      stat.sent_bytes_all += req.compute_size() as usize;
    }

    let nodeid = self.partyid_nodeid.get(&partyid).unwrap();
    let client = self.clients.get_mut(nodeid).unwrap();
    let reply = client.step_call_opt(&req, call_opt);
    debug!("send reply {:?}", reply);
    match reply {
      Err(err) => Err(format_err!("send reply Err: {}", err)),
      Ok(res) => {
        if res.status == 0 {
          Ok(data.len())
        } else {
          Err(format_err!("send reply Err: {:?}", res))
        }
      }
    }
  }
  pub fn broadcast(
    &mut self,
    msgid: &String,
    data: &Vec<u8>,
  ) -> result::Result<usize, anyhow::Error> {
    // ??cannot borrow `*self` as mutable because it is also borrowed as immutable
    let mut peerids = Vec::new();
    for k in self.partyid_nodeid.keys() {
      if *k != self.partyid {
        peerids.push(*k);
      }
    }
    for peerid in peerids {
      self.send(peerid, msgid, data)?;
    }
    Ok(data.len())
  }

  // todo: nodeid version
  // fn make_nodeid_msgid(&mut self, nodeid: String, msgid: String) -> String {
  //   let msgid_ = "".to_string();
  //   return msgid_;
  // }
  // pub fn recv(&mut self, nodeid: String, msgid: String) -> Vec<u8> {
  //   return 0;
  // }
  // pub fn send(&mut self, nodeid: String, msgid: String, data: &Vec<u8>) -> usize {
  //   return 0;
  // }
}
