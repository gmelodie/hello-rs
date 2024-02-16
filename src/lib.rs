use std::{
    sync::mpsc,
    sync::mpsc::channel,
    sync::{Arc, Mutex},
    thread,
    thread::JoinHandle,
};

fn worker(rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let f = rx.lock().unwrap().recv().unwrap();
        f();
    }
}

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    tx: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        let (tx, rx) = channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let mut pool = ThreadPool {
            threads: Vec::with_capacity(n),
            tx,
        };
        for _ in 0..n {
            let rx_clone = Arc::clone(&rx);
            pool.threads.push(thread::spawn(move || worker(rx_clone)));
        }
        pool
    }
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // create Job
        let job: Job = Box::new(f);
        let _ = self.tx.send(job);
        println!("finished posting job");
    }
}
