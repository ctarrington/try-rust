use itertools::izip;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
enum Message {
    UID { value: Uuid },
    CORONATION { uid: Uuid },
}

#[derive(Debug)]
enum Status {
    UNKNOWN,
    LEADER,
    FOLLOWER,
}

#[derive(Debug)]
struct Process {
    uid: Uuid,
    send_value: Option<Message>,
    status: Status,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Process {
    fn new(uid: Uuid, sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Process {
            uid,
            send_value: Some(Message::UID { value: uid }),
            status: Status::UNKNOWN,
            sender,
            receiver,
        }
    }

    fn round(&mut self, receive_timeout: Duration) {
        // these messages are cheap to copy so don't bother with Arcs
        if let Some(message) = self.send_value {
            println!("uid: {}, sending: {:?}", self.uid, message);
            self.sender.send(message).expect("unable to send message");
        }

        let received = self.receiver.recv_timeout(receive_timeout);
        match received {
            Ok(Message::UID { value }) if value > self.uid => {
                self.send_value = Some(Message::UID { value });
            }
            Ok(Message::UID { value }) if value == self.uid => {
                self.send_value = Some(Message::CORONATION { uid: self.uid });
                self.status = Status::LEADER;
            }
            Ok(Message::UID { value: _ }) => {
                self.send_value = None;
            }
            Ok(Message::CORONATION { uid }) if uid > self.uid => {
                self.status = Status::FOLLOWER;
                self.send_value = Some(Message::CORONATION { uid: uid });
            }
            Ok(Message::CORONATION { uid: _ }) => {
                self.send_value = None;
            }
            Err(_) => {
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
    let process_list = izip!(uuids, senders, receivers)
        .map(|(uuid, sender, receiver)| Process::new(uuid, sender, receiver));

    // We want the rounds to be in lock step for this scenario so we give each process half of the
    // interval to catch a message and let it sleep the balance
    // Each round has a target end time based on the epoch and the interval and the round counter
    let epoch = Instant::now();
    let interval = Duration::from_millis(1000);

    let handles: Vec<JoinHandle<Process>> = process_list
        .map(|mut process| {
            thread::spawn(move || {
                let mut round_counter = 0;
                loop {
                    process.round(interval / 2);
                    round_counter += 1;
                    let duration = epoch + round_counter * interval - Instant::now();
                    thread::sleep(duration);
                }
            })
        })
        .collect();

    println!("about to join handles");
    for handle in handles {
        println!("joining handle");
        handle.join().expect("unable to join handle");
    }
}
