use rand::random;

const PROCESS_COUNT: usize = 9;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    UID(u32),
    CORONATION(u32),
}

#[derive(Debug, Clone, Copy)]
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
    send_connections: [NetworkConnection; PROCESS_COUNT],
    send_values: [Option<Message>; PROCESS_COUNT],
    input_values: [Option<Message>; PROCESS_COUNT],
    status: Status,
    halted: bool,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NetworkConnection {
    BLOCKED = 0,
    ALLOWED = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Network {
    connections: [[NetworkConnection; PROCESS_COUNT]; PROCESS_COUNT],
}

impl Network {
    pub fn create_ring(size: usize) -> Self {
        let mut connections = [[NetworkConnection::BLOCKED; PROCESS_COUNT]; PROCESS_COUNT];

        for to in 0..size {
            let from = if to == 0 { size - 1 } else { to - 1 };
            connections[from][to] = NetworkConnection::ALLOWED;
        }

        Network { connections }
    }

    pub fn get_connections(&self) -> [[NetworkConnection; PROCESS_COUNT]; PROCESS_COUNT] {
        self.connections
    }

    pub fn get_connections_from(&self, from_index: usize) -> [NetworkConnection; PROCESS_COUNT] {
        self.connections[from_index]
    }
}

#[derive(Clone, Debug)]
pub struct Scenario {
    pub processes: [Process; PROCESS_COUNT],
    pub processor_count: usize,
    pub network: Network,
}

impl Process {
    fn new(send_connections: [NetworkConnection; PROCESS_COUNT]) -> Self {
        let uid = create_uid();

        let mut send_values: [Option<Message>; PROCESS_COUNT] = [None; PROCESS_COUNT];
        for index in 0..PROCESS_COUNT {
            if send_connections[index] == NetworkConnection::ALLOWED {
                send_values[index] = Some(Message::UID(uid));
            }
        }

        Process {
            uid,
            send_connections,
            send_values,
            input_values: [None; PROCESS_COUNT],
            status: Status::UNKNOWN,
            halted: false,
        }
    }

    pub fn get_uid(&self) -> u32 {
        self.uid
    }

    pub fn get_send_connections(&self) -> [NetworkConnection; PROCESS_COUNT] {
        self.send_connections
    }

    pub fn get_send_values(&self) -> [Option<Message>; PROCESS_COUNT] {
        self.send_values
    }

    pub fn get_input_values(&self) -> [Option<Message>; PROCESS_COUNT] {
        self.input_values
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_halted(&self) -> bool {
        self.halted
    }

    fn tick(&mut self) {
        let input_value = self
            .input_values
            .clone()
            .into_iter()
            .filter(|value| if let Some(_) = value { true } else { false })
            .next();

        let input_value = match input_value {
            Some(wrapped_message) => wrapped_message,
            None => None,
        };

        let send_value;
        match input_value {
            Some(Message::UID(uid)) => {
                if uid > self.uid {
                    send_value = Some(Message::UID(uid));
                } else if uid == self.uid {
                    send_value = Some(Message::CORONATION(self.uid));
                    self.status = Status::LEADER;
                } else {
                    send_value = None;
                }
            }

            Some(Message::CORONATION(uid)) => {
                if uid > self.uid {
                    self.status = Status::FOLLOWER(uid);
                    send_value = Some(Message::CORONATION(uid));
                } else if uid == self.uid {
                    self.halted = true;
                    send_value = None;
                } else {
                    send_value = None;
                }
            }

            None => {
                send_value = None;
            }
        }

        for index in 0..PROCESS_COUNT {
            self.send_values[index] = if self.send_connections[index] == NetworkConnection::ALLOWED
            {
                send_value
            } else {
                None
            }
        }
    }
}

impl Scenario {
    pub fn new(processor_count: usize, network: Network) -> Self {
        let processes: Vec<Process> = (0..PROCESS_COUNT)
            .map(|index| Process::new(network.get_connections_from(index)))
            .collect();

        let processes: [Process; PROCESS_COUNT] = processes.try_into().unwrap();

        Scenario {
            processes,
            processor_count,
            network,
        }
    }

    pub fn tick(&self) -> Self {
        let mut next = self.clone();
        for from in 0..self.processor_count {
            for to in 0..self.processor_count {
                if self.network.connections[from][to] == NetworkConnection::ALLOWED {
                    next.processes[to].input_values[from] = self.processes[from].send_values[to];
                }
            }
        }
        for index in 0..next.processor_count {
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
    const SIZE: usize = 3;

    let scenario = Scenario::new(SIZE, Network::create_ring(SIZE));
    let mut unique_uids = std::collections::HashSet::new();
    for index in 0..SIZE {
        unique_uids.insert(scenario.processes[index].uid);
    }
    assert_eq!(SIZE, unique_uids.len());
}

#[test]
fn test_election() {
    const SIZE: usize = 3;
    let mut scenario = Scenario::new(SIZE, Network::create_ring(SIZE));
    let required_rounds = 2 * SIZE;
    for _ in 0..required_rounds {
        scenario = scenario.tick();
    }
    let leaders: Vec<Process> = scenario
        .processes
        .clone()
        .into_iter()
        .filter(|process| matches!(process.status, Status::LEADER))
        .collect();
    assert_eq!(1, leaders.len());
    assert!(leaders.get(0).unwrap().halted);

    let leader_uid: u32 = leaders.get(0).unwrap().uid;
    let followers: Vec<Process> = scenario
        .processes
        .clone()
        .into_iter()
        .filter(|process| {
            let is_follower = if let Status::FOLLOWER(following) = process.status {
                following == leader_uid
            } else {
                false
            };
            is_follower
        })
        .collect();
    assert_eq!(SIZE - 1, followers.len());
}
