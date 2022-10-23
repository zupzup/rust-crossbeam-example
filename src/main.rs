use crossbeam::atomic::AtomicCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let atomic_value: AtomicCell<u32> = AtomicCell::new(12);
    let arc = Arc::new(atomic_value);

    let thread1_handle = run_thread_1(arc.clone());
    let thread2_handle = run_thread_2(arc.clone());
    let thread3_handle = run_thread_3(arc.clone());

    thread1_handle.join().expect("cant join thread");
    thread2_handle.join().expect("cant join thread");
    thread3_handle.join().expect("cant join thread");

    println!("value after threads finished: {}", arc.load());

    println!("Hello, world!");
}

fn run_thread_1(val: Arc<AtomicCell<u32>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from thread 1! value: {}", val.load());
    })
}

fn run_thread_2(val: Arc<AtomicCell<u32>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        val.store(13);
        println!("Hello from thread 2! value: {}", val.load());
    })
}

fn run_thread_3(val: Arc<AtomicCell<u32>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from thread 3! value: {}", val.load());
    })
}
