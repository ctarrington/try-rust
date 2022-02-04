use dioxus::prelude::*;

const NODE_COUNT: usize = 32;

#[derive(Clone)]
struct Node {
    position: i32,
}

impl Node {
    fn new() -> Self {
        Node { position: 0 }
    }

    fn tick(&mut self) {
        self.position = self.position + 1;
    }
}

#[derive(Clone)]
struct Scenario {
    count: usize,
    nodes: [Node; NODE_COUNT],
}

impl Scenario {
    fn new(count: usize) -> Self {
        let nodes = [0; NODE_COUNT].map(|_| Node::new());
        Scenario { count, nodes }
    }

    fn tick(&self) -> Self {
        let mut next = self.clone();
        for index in 0..next.count {
            next.nodes[index].tick();
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
    let (scenario, set_scenario) = use_state(&cx, || Scenario::new(2));
    let on_scenario_tick = move |_| {
        let next = scenario.tick();
        set_scenario(next);
    };

    let position = scenario.nodes[0].position;
    cx.render(rsx! {
        div {
            h1 {"Current Scenario: position {position}"}
            button { onclick: on_scenario_tick, "Scenario+"}
        }
    })
}
