use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let (color, set_color) = use_state(&cx, || "red");

    let onclick_handler = move |_| {
        set_color(match *color {
            "green" => "orange",
            "orange" => "red",
            "red" => "green",
            _ => "red",
        })
    };

    cx.render(rsx!(
        div {
            font_size: "18px",
            div {
                color: "blue",
                font_size: "28px",
                "Hi from Dioxus! My favorite color is " Widget { color: color.to_string() } "!"
            }
            button {
                onclick: onclick_handler,
                span {
                    font_size: "20px",
                    "Change Color"
                }
            }
            p {}
        }
    ))
}

#[derive(Props, PartialEq)]
struct WidgetProps {
    color: String,
}

#[allow(non_snake_case)]
fn Widget(cx: Scope<WidgetProps>) -> Element {
    let WidgetProps { color } = cx.props;

    cx.render(rsx!(
        span {
            color: "{color}",
            "{color}"
        }
    ))
}
