use itertools::izip;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
enum Message {
    UID(Uuid),
    CORONATION(Uuid),
}

#[derive(Debug)]
enum Status {
    UNKNOWN,
    LEADER,
    FOLLOWER(Uuid),
}

#[derive(Debug)]
struct Process {
    uid: Uuid,
    send_value: Option<Message>,
    input_value: Option<Message>,
    status: Status,
    halted: bool,
}

impl Process {
    fn new(uid: Uuid) -> Self {
        Process {
            uid,
            send_value: Some(Message::UID(uid)),
            input_value: None,
            status: Status::UNKNOWN,
            halted: false,
        }
    }

    fn round(&mut self) {
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

        println!("process: {:?}", self);
    }
}

fn main() {
    let processor_count: usize = 5;

    let uuids = (0..processor_count).map(|_| Uuid::new_v4());

    let mut senders = vec![];
    let mut receivers = vec![];
    for _ in 0..processor_count {
        let (sender, receiver) = mpsc::channel::<Message>();
        senders.push(sender);
        receivers.push(receiver);
    }

    // move the last receiver to the front of the line
    // so senders 3 2 1 line up with
    // receivers  1 3 2
    let first_receiver = receivers.remove(0);
    receivers.push(first_receiver);

    // reverse so it goes in clockwise direction
    let senders = senders.into_iter().rev();
    let receivers = receivers.into_iter().rev();

    // make a process list from the tuples of uuids, senders and receivers
    let process_inputs = izip!(uuids, senders, receivers);

    // We want the rounds to be in lock step for this scenario so we give each process half of the
    // interval to catch a message and let it sleep the balance
    // Each round has a target end time based on the epoch and the interval and the round counter
    let epoch = Instant::now();
    let interval = Duration::from_millis(1000);

    let halted_original = Arc::new(Mutex::new(false));

    let handles: Vec<JoinHandle<Process>> = process_inputs
        .map(|(uuid, sender, receiver)| {
            let halted = halted_original.clone();
            thread::spawn(move || {
                let mut process = Process::new(uuid);
                let mut round_counter = 0;
                loop {
                    // these messages are cheap to copy so don't bother with Arcs
                    if let Some(message) = process.send_value {
                        sender.send(message).expect("unable to send message");
                    }

                    let received = receiver.recv_timeout(interval / 2);
                    match received {
                        Ok(message) => {
                            process.input_value = Some(message);
                        }
                        Err(_) => {
                            process.input_value = None;
                        }
                    }
                    process.round();

                    if process.halted {
                        *halted.lock().unwrap() = true;
                    }

                    if *halted.lock().unwrap() {
                        break process;
                    }

                    round_counter += 1;
                    let duration = epoch + round_counter * interval - Instant::now();
                    thread::sleep(duration);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("unable to join handle");
    }
}
