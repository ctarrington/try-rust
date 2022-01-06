use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone, Copy)]
enum Message {
    UID { value: u32 },
    CORONATION { uid: u32 },
}

#[derive(Debug)]
enum Status {
    UNKNOWN,
    LEADER,
    FOLLOWER,
}

#[derive(Debug)]
struct Process {
    uid: u32,
    send_value: Option<Message>,
    status: Status,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    last_update: SystemTime,
    round_duration: Duration,
}

impl Process {
    fn new(
        uid: u32,
        sender: Sender<Message>,
        receiver: Receiver<Message>,
        round_duration: Duration,
    ) -> Self {
        Process {
            uid,
            send_value: Some(Message::UID { value: uid }),
            status: Status::UNKNOWN,
            sender,
            receiver,
            round_duration,
            last_update: SystemTime::now(),
        }
    }

    fn round(&mut self) {
        self.last_update = SystemTime::now();

        // these messages are cheap to copy so don't bother with Arcs
        if let Some(message) = self.send_value {
            self.sender.send(message);
        }

        let received = self.receiver.recv_timeout(self.round_duration / 2);
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
    let interval = Duration::from_millis(1000);
    let (sender_1, receiver_2) = mpsc::channel::<Message>();
    let (sender_2, receiver_3) = mpsc::channel::<Message>();
    let (sender_3, receiver_1) = mpsc::channel::<Message>();

    let mut process_1 = Process::new(1, sender_1, receiver_1, interval);
    let mut process_2 = Process::new(2, sender_2, receiver_2, interval);
    let mut process_3 = Process::new(3, sender_3, receiver_3, interval);

    let process_list = vec![process_1, process_2, process_3];
    let epoch = Instant::now();

    let mut handles = vec![];

    for mut process in process_list {
        let handle = thread::spawn(move || {
            let mut round = 0;
            loop {
                process.round();
                round += 1;
                let duration = epoch + round * interval - Instant::now();
                println!("duration: {:?}", duration);
                thread::sleep(duration);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("unable to join handle");
    }
}
