use dioxus::prelude::*;

const PROCESS_COUNT: usize = 10;

#[derive(Debug, Clone, Copy)]
enum Message {
    UID(u32),
    CORONATION(u32),
}

#[derive(Debug, Clone)]
enum Status {
    UNKNOWN,
    LEADER,
    FOLLOWER(u32),
}

/// Simple synchronous leader election scheme for a ring of processes
/// More or less an implementation of the LCR algorithm as described in
/// Destributed Algorithms by Nancy Lynch
/// https://learning.oreilly.com/library/view/distributed-algorithms/9781558603486/OEBPS/B9781558603486500031.htm
#[derive(Debug, Clone)]
struct Process {
    uid: u32,
    send_value: Option<Message>,
    input_value: Option<Message>,
    status: Status,
    halted: bool,
}

impl Process {
    fn new() -> Self {
        Process {
            uid: 0,
            send_value: None,
            input_value: None,
            status: Status::UNKNOWN,
            halted: false,
        }
    }

    fn tick(&mut self) {
        self.send_value = Some(Message::UID(self.uid));
    }
}

#[derive(Clone, Debug)]
struct Network {
    processes: [Process; PROCESS_COUNT],
    processor_count: usize,
}

impl Network {
    fn new(processor_count: usize) -> Self {
        let processes = [0; PROCESS_COUNT].map(|_| Process::new());
        Network {
            processes,
            processor_count,
        }
    }

    fn tick(&self) -> Self {
        let mut next = self.clone();
        for index in 0..next.processor_count {
            next.processes[index].tick();
        }
        next
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (network, set_network) = use_state(&cx, || Network::new(3));
    /*
    let on_network_tick = move |_| {
        let next = network.tick();
        set_network(next);
    };
     */

    let processor_count = network.processor_count;
    cx.render(rsx! {
        div {
            h1 {"Current Network: {processor_count}"}
        }
    })
}
