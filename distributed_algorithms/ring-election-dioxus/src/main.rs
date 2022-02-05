use dioxus::prelude::*;
use ring_election_dioxus::{Message, Process, Scenario, Status};

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (scenario, set_scenario) = use_state(&cx, || Scenario::new(3));
    let on_scenario_tick = move |_| {
        let next = scenario.tick();
        set_scenario(next);
    };

    let process_list = (0..scenario.processor_count).map(|index| {
        let process_summary = format!("{:?}", scenario.processes[index]);

        rsx! {
            div {
                key: "{index}",
                "{process_summary}"
            }
        }
    });

    let network_index = scenario.processes[0].network_index;
    cx.render(rsx! {
        div {
            h1 {"Current Scenario network_index: {network_index}"}
            process_list
            button { onclick: on_scenario_tick, "Scenario+"}
        }
    })
}
