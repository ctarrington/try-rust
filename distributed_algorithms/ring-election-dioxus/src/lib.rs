use rand::random;

const PROCESS_COUNT: usize = 10;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    UID(u32),
    CORONATION(u32),
}

#[derive(Debug, Clone)]
pub enum Status {
    UNKNOWN,
    LEADER,
    FOLLOWER(u32),
}
/// Simple synchronous leader election scheme for a ring of processes
/// More or less an implementation of the LCR algorithm as described in
/// Distributed Algorithms by Nancy Lynch
/// https://learning.oreilly.com/library/view/distributed-algorithms/9781558603486/OEBPS/B9781558603486500031.htm
#[derive(Debug, Clone)]
pub struct Process {
    uid: u32,
    pub network_index: usize,
    send_value: Option<Message>,
    input_value: Option<Message>,
    status: Status,
    halted: bool,
}

impl Process {
    fn new() -> Self {
        Process {
            uid: 0,
            network_index: 0,
            send_value: None,
            input_value: None,
            status: Status::UNKNOWN,
            halted: false,
        }
    }

    fn tick(&mut self) {
        match self.input_value {
            Some(Message::UID(uid)) => {
                if uid > self.uid {
                    self.send_value = Some(Message::UID(uid));
                } else if uid == self.uid {
                    self.send_value = Some(Message::CORONATION(self.uid));
                    self.status = Status::LEADER;
                } else {
                    self.send_value = None;
                }
            }

            Some(Message::CORONATION(uid)) => {
                if uid > self.uid {
                    self.status = Status::FOLLOWER(uid);
                    self.send_value = Some(Message::CORONATION(uid));
                } else if uid == self.uid {
                    self.halted = true;
                    self.send_value = None;
                } else {
                    self.send_value = None;
                }
            }

            None => {
                self.send_value = None;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scenario {
    pub processes: [Process; PROCESS_COUNT],
    pub processor_count: usize,
}

impl Scenario {
    pub fn new(processor_count: usize) -> Self {
        let mut processes = [0; PROCESS_COUNT].map(|_| Process::new());

        for index in 0..processor_count {
            let uid = create_uid();
            processes[index].network_index = index;
            processes[index].uid = uid;
            processes[index].send_value = Some(Message::UID(uid));
        }

        Scenario {
            processes,
            processor_count,
        }
    }

    pub fn tick(&self) -> Self {
        let mut next = self.clone();
        for index in 0..next.processor_count {
            let source_index = if index == 0 {
                next.processor_count - 1
            } else {
                index - 1
            };
            next.processes[index].input_value = self.processes[source_index].send_value;
            next.processes[index].tick();
        }
        next
    }
}

/// lame uid based on a random number
fn create_uid() -> u32 {
    random()
}

#[test]
fn test_construction() {
    let scenario = Scenario::new(3);
    assert_eq!(scenario.processes[1].uid, 1);
}
