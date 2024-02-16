use std::{
    sync::mpsc,
    sync::mpsc::channel,
    sync::{mpsc::TryRecvError, Arc, Mutex},
    thread,
    thread::JoinHandle,
};

fn worker(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let msg = rx.lock().unwrap().try_recv();
        match msg {
            Err(TryRecvError::Disconnected) => {
                println!("worker thread {id} stopping, channel disconnected");
                break;
            }
            Err(TryRecvError::Empty) => continue,
            Ok(f) => f(),
        }
    }
}

pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    tx: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        let (tx, rx) = channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let mut pool = ThreadPool {
            threads: Vec::with_capacity(n),
            tx: Some(tx),
        };
        for id in 0..n {
            let rx_clone = Arc::clone(&rx);
            pool.threads
                .push(Some(thread::spawn(move || worker(id, rx_clone))));
        }
        pool
    }
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // create Job
        let job: Job = Box::new(f);
        let _ = self.tx.clone().unwrap().send(job);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Cleaning up ThreadPool");
        // signal stop to threads
        drop(self.tx.take());

        // then wait for threads to finish
        for t in self.threads.iter_mut() {
            if let Some(thread) = t.take() {
                let _ = thread.join();
            }
        }
    }
}
