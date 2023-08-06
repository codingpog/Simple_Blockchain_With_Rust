use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        //todo!(); // create the channels; start the worker threads; record their JoinHandles
        // create channels to safely communicate between threads
        // a single-producer multiple-consumer channel for the tasks: they will by enqueued by the main thread, and be received by workers
        let (send_tasks, recv_tasks) = spmc::channel();

        //a multiple-producer single-consumer channel for results: they will be sent by workers as they are produced, and received by the main thread.
        let (send_output, recv_output) = mpsc::channel();

        // create worker threads and store their joinHandles in the vector
        let mut workers = Vec::with_capacity(n_workers);
        for _ in 0..n_workers {
            // clone for concurrency safety
            let recv_tasks_clone = recv_tasks.clone();
            //let send_output_shared_clone = Arc::clone(&send_ouput_shared);
            let send_output_clone = send_output.clone();

            let handle = thread::spawn(move || {
                WorkQueue::run(recv_tasks_clone, send_output_clone);
            });

            workers.push(handle);
        }
        WorkQueue {
            send_tasks: Some(send_tasks),
            recv_tasks,
            recv_output,
            workers,
        }

    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            //arc and mutex to share the sender side of the result channel among the workers
            let send_output_shared = Arc::new(Box::new(send_output.clone()));           

            // task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
            let task_result = recv_tasks.recv();

            let result;
            match task_result {
                Ok(task) => {
                    result = task.run();
                }
                Err(e) => {
                    break;
                }
            }

            match result {
                Some(output) => {
                    send_output.send(output).expect("Failed to send task output");
                }
                None => {
                    //println!("Incorrect");
                }
            }


        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // to do
        // send this task to a worker 
        match &mut self.send_tasks {
            Some(sender) => sender.send(t),
            None => Err(spmc::SendError(t)),
        }
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // Destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        if let Some(sender) = self.send_tasks.take() {
            drop(sender);

            while let Ok(_) = self.recv_tasks.recv() {}

            for handle in self.workers.drain(..) {
                handle.join().expect("Worker thread panicked");
            }
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
