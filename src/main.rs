use crossbeam::{
    atomic::AtomicCell,
    channel::{unbounded, Receiver, Sender},
    queue::ArrayQueue,
    sync::WaitGroup,
};
use rand::Rng;
use std::sync::Arc;
use std::thread;

fn main() {
    // AtomicCell Example
    println!("Starting AtomicCell example...");
    let atomic_value: AtomicCell<u32> = AtomicCell::new(12);
    let arc = Arc::new(atomic_value);

    let mut thread_handles_ac: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..10 {
        thread_handles_ac.push(run_thread(arc.clone(), i, i % 2 == 0));
    }

    thread_handles_ac
        .into_iter()
        .for_each(|th| th.join().expect("can't join thread"));

    println!("value after threads finished: {}", arc.load());
    println!("AtomicCell example finished!");
    thread::sleep(std::time::Duration::from_millis(500));

    // ArrayQueue Example
    println!("---------------------------------------");
    println!("Starting ArrayQueue example...");
    let q: ArrayQueue<u32> = ArrayQueue::new(100);
    let arc_q = Arc::new(q);

    let mut thread_handles_aq: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 1..5 {
        thread_handles_aq.push(run_producer(arc_q.clone(), i));
    }

    for i in 1..5 {
        thread_handles_aq.push(run_consumer(arc_q.clone(), i));
    }

    thread_handles_aq
        .into_iter()
        .for_each(|th| th.join().expect("can't join thread"));
    println!("values in q after threads finished: {}", arc_q.len());
    println!("ArrayQueue example finished!");
    thread::sleep(std::time::Duration::from_millis(500));

    // channel Example
    println!("---------------------------------------");
    println!("Starting channel example...");
    let (s, r) = unbounded();

    for i in 1..5 {
        run_producer_chan(s.clone(), i);
    }
    drop(s);

    for i in 1..5 {
        run_consumer_chan(r.clone(), i);
    }

    println!("channel example finished!");
    thread::sleep(std::time::Duration::from_millis(500));

    // WaitGroup Example
    println!("---------------------------------------");
    println!("Starting WaitGroup example...");
    let wg = WaitGroup::new();

    for i in 0..50 {
        let wg_clone = wg.clone();
        thread::spawn(move || {
            do_work(i);
            drop(wg_clone);
        });
    }

    println!("waiting for all threads to finish...!");
    wg.wait();
    println!("all threads finished!");
    println!("WaitGroup example finished!");
}

fn do_work(thread_num: i32) {
    let num = rand::thread_rng().gen_range(100..500);
    thread::sleep(std::time::Duration::from_millis(num));
    let mut sum = 0;
    for i in 0..10 {
        sum += sum + num * i;
    }
    println!(
        "thread {} calculated sum: {}, num: {}",
        thread_num, sum, num
    );
    thread::sleep(std::time::Duration::from_millis(num));
}

fn run_producer_chan(s: Sender<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - pushing...!", num);
        for _ in 0..1000 {
            s.send(num).expect("send failed");
        }
    })
}

fn run_consumer_chan(r: Receiver<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut i = 0;
        println!("Hello from producer thread {} - popping!", num);
        loop {
            if let Err(_) = r.recv() {
                println!(
                    "last sender dropped - stopping consumer thread, messages received: {}",
                    i
                );
                break;
            }
            i += 1;
        }
    })
}

fn run_producer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - pushing...!", num);
        for _ in 0..20 {
            q.push(num).expect("pushing failed");
        }
    })
}

fn run_consumer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - popping!", num);
        for _ in 0..20 {
            q.pop();
        }
    })
}

fn run_thread(val: Arc<AtomicCell<u32>>, num: u32, store: bool) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if store {
            val.fetch_add(1);
        }
        println!("Hello from thread {}! value: {}", num, val.load());
    })
}
