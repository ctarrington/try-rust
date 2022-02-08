use dioxus::prelude::*;
use ring_election_dioxus::model::{Network, NetworkConnection, Scenario};
use ring_election_dioxus::components::NetworkComponent;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (scenario, set_scenario) = use_state(&cx, || Scenario::new(3, Network::create_ring(3)));
    let on_scenario_tick = move |_| {
        let next = scenario.tick();
        set_scenario(next);
    };

    let process_list = (0..scenario.processor_count).map(|index| {
        let process = scenario.processes[index].clone();

        let mut input_values_formatted = vec![];
        let mut send_values_formatted = vec![];

        for value_index in 0..scenario.processor_count {
            let connection = scenario.network.get_connections()[value_index][index];
            let input_value_formatted: String = if connection == NetworkConnection::ALLOWED {
                format!("{:?}", process.get_input_values()[value_index])
            } else {
                "X".to_string()
            };
            input_values_formatted.push(input_value_formatted);

            let connection = scenario.network.get_connections()[index][value_index];
            let send_value_formatted: String = if connection == NetworkConnection::ALLOWED {
                format!("{:?}", process.get_send_values()[value_index])
            } else {
                "X".to_string()
            };
            send_values_formatted.push(send_value_formatted);
        }
        let input_values_formatted = input_values_formatted.join(", ");
        let send_values_formatted = send_values_formatted.join(", ");

        let process_summary = format!(
            "uid: {}, status: {:?}, halted: {:?}, inputs: [{}], sends: [{}]",
            process.get_uid(),
            process.get_status(),
            process.get_halted(),
            input_values_formatted,
            send_values_formatted,
        );

        rsx! {
            div {
                key: "{index}",
                "{process_summary}"
            }
        }
    });

    let network_list = (0..scenario.processor_count).map(|from| {
        let row_summary: Vec<String> = (0..scenario.processor_count)
            .map(|to| format!("{:?}", scenario.network.get_connections()[from][to]))
            .collect();
        let row_summary = row_summary.join(",");

        rsx! {
            div {
                key: "{from}",
                "{row_summary}"
            }
        }
    });

    cx.render(rsx!(
        div {
            NetworkComponent {network: scenario.network, processor_count: scenario.processor_count}
            h1 {"Current Scenario"}
            button { onclick: on_scenario_tick, "Scenario+"}
            network_list
            process_list
        }
    ))
}
