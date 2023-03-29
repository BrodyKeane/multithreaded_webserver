use std::{
    thread::{Builder, JoinHandle},
    sync::{mpsc, Arc, Mutex},
    io,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; 

impl ThreadPool { 
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel(); 
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            if let Ok(worker) = Worker::new(id, Arc::clone(&receiver)) {
                workers.push(worker);
            }
        }
        assert!(workers.len() > 0);
        ThreadPool{ workers, sender }
    }


    pub fn execute<F> (&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> io::Result<Worker>{
        let builder = Builder::new();
        let thread = builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker: {id} got a job; executing.");
            job();
        })?; 
        
        Ok(Worker{ id, thread })
    }
}