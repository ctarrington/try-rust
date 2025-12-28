mod counter;
mod incrementer;

use leptos::prelude::*;

use counter::Counter;
use incrementer::Incrementer;

#[derive(Clone)]
struct CounterContext {
    count: ReadSignal<i32>,
    set_count: WriteSignal<i32>,
}

impl CounterContext {
    pub fn clear(&self) {
        self.set_count.set(0);
    }
}


#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    provide_context(CounterContext { count: count, set_count });

    view! {
        <div>
            <Incrementer />
            <Counter />
        </div>
    }
}