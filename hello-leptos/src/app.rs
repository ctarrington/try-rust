mod counter;
mod incrementer;

use leptos::prelude::*;

use counter::Counter;
use incrementer::Incrementer;

#[derive(Clone)]
pub struct CounterContext {
    pub count: ReadSignal<i32>,
    pub set_count: WriteSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    provide_context(CounterContext { count: count, set_count: set_count });

    view! {
        <div>
            <Incrementer />
            <Counter />
        </div>
    }
}