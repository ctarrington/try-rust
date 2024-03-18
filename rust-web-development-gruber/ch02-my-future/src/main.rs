use std::future::Future;

#[tokio::main]
async fn main() {
    let value = find(5).await;
    println!("Value: {}", value);

}

struct MyFuture {
    steps: usize,
    current_step: usize,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<i32> {
        self.current_step += 1;
        if self.current_step < self.steps {
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }

        std::task::Poll::Ready(self.current_step as i32)
    }
}
fn find(steps: usize) -> impl Future<Output = i32> {
    MyFuture { current_step: 0, steps: steps }
}
