use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (name, set_name) = use_state(&cx, || "Jane");

    let circles = (0..=10).map(|value| {
        let x_position = value*10;
        let y_position = value*10;

       rsx! {
            circle {
                cx: "{x_position}",
                cy: "{y_position}",
                r: "5",
            }
       }
    });
    cx.render(rsx! {
        div {
            div { "Hello to {name}" }
            button { onclick: move |_| set_name("Fred"), "Fred"}
            button { onclick: move |_| set_name("Ted"), "Ted"}
            svg {
                view_box: "0 0  100 100",
                circles
            }
            }
    })
}
