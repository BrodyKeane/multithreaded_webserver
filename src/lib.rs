use std::{
    thread::{self, Builder},
    sync::{mpsc, Arc, Mutex},
    io,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        ThreadPool{
            workers, 
            sender: Some(sender),
        }
    }


    pub fn execute<F> (&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> io::Result<Worker>{
        let builder = Builder::new();
        let thread = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker: {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker: {id} dissconnected; shutting down.");
                    break;
                }
            }
        })?; 
        
        Ok(Worker{
            id,
            thread: Some(thread)
        })
    }
}   