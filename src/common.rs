use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct OneData {
  pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DataQueue {
  pub data: VecDeque<OneData>,
}

#[derive(Debug)]
pub struct Participant {
  pub partyid: u32,
  pub nodeid: String, // not use now
  pub addr: String,   // valid IPv4 or domain
}

// for debug or simple useage
pub fn get_default_participants(parties: u32) -> Vec<Participant> {
  let mut participants = Vec::new();
  for i in 0..parties {
    let participant = Participant {
      partyid: i,
      nodeid: "node".to_string() + &i.to_string(),
      addr: "127.0.0.1:".to_string() + &(i + 10000).to_string(),
    };
    participants.push(participant);
  }
  return participants;
}
