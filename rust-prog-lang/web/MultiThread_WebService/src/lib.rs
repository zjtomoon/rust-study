use std::thread;

//pub struct ThreadPool;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /*pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }*/

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static{

    }
    pub fn new(size: usize) -> ThreadPool {
        assert_eq!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {

        }

        ThreadPool {
            threads
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}