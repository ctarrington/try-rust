use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct Scenario {
    count: i32,
}

impl Scenario {
    fn new() -> Self {
        Scenario { count: 0 }
    }

    fn tick(self) -> Self {
        let mut next = self.clone();
        next.count = next.count + 1;
        next
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
    //dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (scenario, set_scenario) = use_state(&cx, || Scenario::new());
    let on_scenario_tick = move |_| {
        let next = scenario.tick();
        set_scenario(next);
    };

    /*
    use_coroutine(&cx, || async move {
        loop {
            // set_count(count + 1);

        }
    });
     */

    cx.render(rsx! {
        div {
            h1 {"Current Scenario: count {scenario.count}"}
            button { onclick: on_scenario_tick, "Scenario+"}
        }
    })
}
