use std::thread::{Builder, JoinHandle};
use std::io;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> io::Result<Worker>{
        let builder = Builder::new();
        let thread = builder.spawn(|| {})?; 
        Ok(Worker{ id, thread })
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            if let Ok(worker) = Worker::new(id) {
                workers.push(worker);
            }
        }

        assert!(workers.len() > 0);
        ThreadPool{ workers }
    }

    pub fn execute<F> (&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
    }
}