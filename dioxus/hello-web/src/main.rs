use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (name, set_name) = use_state(&cx, || "Jane");
    let (red_x_position, set_red_x_position) = use_state(&cx, || 0);

    let circles = (0..=10).map(|value| {
        let x_position = value*10;
        let y_position = value*10;

       rsx! {
            circle {
                key: "{value}",
                fill: "black",
                cx: "{x_position}",
                cy: "{y_position}",
                r: "5",
            }
       }
    });

    let red_circle = rsx! {
        circle {
            fill: "red",
            cx: "{red_x_position}",
            cy: "40",
            r: "3",
        }
    };

    cx.render(rsx! {
        div {
            div { "Hello to {name}" }
            button { onclick: move |_| set_name("Fred"), "Fred"}
            button { onclick: move |_| set_name("Ted"), "Ted"}
            button { onclick: move |_| set_red_x_position(red_x_position + 1), "X+"}
            svg {
                view_box: "0 0  100 100",
                circles
                red_circle
            }
            }
    })
}
