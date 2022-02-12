use futures::executor::block_on;
use std::future::Future;
use std::thread;
//use std::time::Duration;
//use tokio::time::{sleep, Duration};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use std::sync::mpsc::Receiver;

trait SimpleTrait {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

struct Join<FutureA, FutureB>
where
    FutureA: SimpleTrait<Output = ()>,
    FutureB: SimpleTrait<Output = ()>,
{
    a: Option<FutureA>, // ready 后置 None, 防止重复 poll
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleTrait for Join<FutureA, FutureB>
where
    FutureA: SimpleTrait<Output = ()>,
    FutureB: SimpleTrait<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(future) = &mut self.a {
            if let Poll::Ready(()) = future.poll(wake) {
                self.a.take();
                // self.a = None;
            }
        }
        if let Some(future) = &mut self.b {
            if let Poll::Ready(()) = future.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

struct AndThen<FutureA, FutureB>
where
    FutureA: SimpleTrait<Output = ()>,
    FutureB: SimpleTrait<Output = ()>,
{
    first: Option<FutureA>,
    second: FutureB, // will not poll again after FutureB is finished!
}

impl<FutureA, FutureB> SimpleTrait for AndThen<FutureA, FutureB>
where
    FutureA: SimpleTrait<Output = ()>,
    FutureB: SimpleTrait<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<()> {
        if let Some(future) = &mut self.first {
            if let Poll::Pending = future.poll(wake) {
                return Poll::Pending;
            } else {
                self.first.take();
            }
        }
        self.second.poll(wake)
    }
}

// trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: Content<'_>) -> Poll<Self::Output>;
// }

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new() -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let shared_state2 = shared_state.clone();
        thread::spawn(move || {
            println!("processing...");
            thread::sleep(Duration::from_secs(5));
            let mut shared_state = shared_state2.lock().unwrap();
            shared_state.completed = true;

            if let Some(waker) = shared_state.waker.take() {
                println!("waking...");
                // why we need .take()
                waker.wake();
            }
            println!("I'm awake, I'm awake!");
        });
        Self { shared_state }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

struct Task {
    future:
}


async fn async_main() {
    // thread::sleep(Duration::from_secs(5));
    // let future = TimerFuture::new();
    // block_on(future);
    let f1 = sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
    thread::sleep(Duration::from_secs(10));
}

async fn learn_song() {
    println!("learning song...");
}

async fn sing_song() {
    println!("singing song...");
}

async fn dance() {
    println!("dancing...");
}

async fn sing() {
    learn_song().await;
    //thread::sleep(Duration::from_secs(3));
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(3));
        block_on(sing_song());
    });
    //handle.join();
    //sing_song().await;
}
