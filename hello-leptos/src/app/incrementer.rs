// rust
use crate::app::CounterContext;
use leptos::prelude::*;

#[component]
pub fn Incrementer() -> impl IntoView {
    let counter_context = use_context::<CounterContext>().expect("Counter context should exist");
    let set_count = counter_context.set_count;

    view! {
        <button on:click=move |_| set_count.update(|c| *c += 1)>"increment"</button>
    }
}
