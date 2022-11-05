use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        assert!(thread_count > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Self { tx, workers }
    }

    pub fn execute<T>(&self, t: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(t);
        self.tx.send(Message::NewJob(job)).unwrap();
    }
}

// TODO: If we need to, implement a Terminate message and also do impl Drop.
enum Message {
    NewJob(Job),
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                    eprintln!("worker {} got a job!", id);
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
