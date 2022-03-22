use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct OneData {
  pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DataQueue {
  pub data: VecDeque<OneData>,
}

#[derive(Debug, Clone)]
pub struct Participant {
  pub partyid: u32,
  pub nodeid: String, // not use now
  pub addr: String,   // valid IPv4 or domain
}

#[derive(Debug, Clone)]
pub struct NetStat {
  pub sent_count: usize,
  pub sent_bytes: usize,     // the real data size
  pub sent_bytes_all: usize, // total size. protobuf serialized data (message id, real data, etc.)
}

// for debug or simple useage
pub fn get_default_participants(parties: u32) -> Vec<Participant> {
  let mut participants = Vec::new();
  for i in 0..parties {
    let participant = Participant {
      partyid: i,
      nodeid: "node".to_string() + &i.to_string(),
      addr: "127.0.0.1:".to_string() + &(i + 40000).to_string(),
    };
    participants.push(participant);
  }
  return participants;
}
