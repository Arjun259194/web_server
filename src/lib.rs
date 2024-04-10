use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[allow(unused)] // temp, remove later
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create new ThreadPool
    ///
    /// The n is the number of threads in the pool
    ///
    /// # Panic
    ///
    /// The `new` function will panic if the n is less then 0 (zero)
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0); // expression inside should be true at runtime
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(n);

        for id in 0..=n {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[allow(unused)] // temp, remove later
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got job; executing.", id);
            job();
        });
        Worker { id, thread }
    }
}
