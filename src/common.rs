use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct OneData {
  pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct DataQueue {
  pub data: VecDeque<OneData>,
}

/// Participant info
#[derive(Debug, Clone)]
pub struct Participant {
  /// The party id, from 0 to n-1.
  pub partyid: u32,
  /// The node id. Not used now.
  pub nodeid: String,
  /// Now only supports valid IPv4.
  pub addr: String,
}

/// The communication statistics
#[derive(Debug, Clone)]
pub struct NetStat {
  /// Number of calls to `io.send`.
  pub sent_count: usize,
  /// The real data size.
  pub sent_bytes: usize,
  /// Total size. Protobuf serialized data (message id, real data, etc.) size.
  pub sent_bytes_all: usize,
}

/// For debug or simple usage.
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
