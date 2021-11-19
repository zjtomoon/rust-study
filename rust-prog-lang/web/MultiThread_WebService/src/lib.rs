use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//pub struct ThreadPool;

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    //sender: mpsc::Sender<Job>,
    sender: mpsc::Sender<Message>,
}

//struct Job;
enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

//type Job = Box<FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /*pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }*/

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static{
        let job = Box::new(f);

        //self.sender.send(job).unwrap();
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        //let mut threads = Vec::with_capacity(size);

        let (sender,receiver) = mpsc::channel();

        let receiver= Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            //workers.push(Worker::new(id,receiver));
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool {
            //threads
            workers,
            sender,
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}",worker.id);

            //worker.thread.join().unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    //thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    //fn new(id: usize,receiver: mpsc::Receiver<Job>) -> Worker {
    /*fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            //receiver;
            /*loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job;executing.",id);

                (*job)();
            }*/
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job;executing.",id);

                job.call_box();
            }
        });

        Worker {
            id,
            //thread,
            thread: Some(thread),
        }
    }*/

    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job)=> {
                        println!("Worker {} got a job;executing.",id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.",id);

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}