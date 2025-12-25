mod counter;

use counter::Counter;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct CounterContext {
    pub count: ReadSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    provide_context(CounterContext { count: count });
    view! {
        <div>
            <button on:click=move |_| *set_count.write() += 1>"add"</button>
            <Counter />
        </div>
    }
}