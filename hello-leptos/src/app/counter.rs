use crate::app::CounterContext;
use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let counter_context = use_context::<CounterContext>().expect("Counter context should exist");
    view! { <div>
        <button on:click=move |_| counter_context.set_count.set(0)>"clear"</button>
        <span>counter: {counter_context.count}</span></div> }
}
