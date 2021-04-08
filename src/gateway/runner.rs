use std::{sync::mpsc, thread, time};
use std::sync::Arc;
use std::sync::Mutex;

use reqwest;
use serde_json;
use tungstenite::{connect, Message, WebSocket, client::AutoStream};
use url::Url;

use crate::constants;

pub struct GatePool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
    size: usize,
    receiver:  Arc<Mutex<mpsc::Receiver<Task>>>
}

pub enum Task {
    Heartbeat{metadata: String},
    Identify{metadata: String},
    Terminate{metadata: String},
}

type Ws = Arc<Mutex<WebSocket<AutoStream>>>;

impl GatePool {

    pub fn new(size: usize) -> GatePool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        GatePool {
            workers,
            sender,
            size,
            receiver,
        }
    }

    pub fn execute(&self, f: Task)
    {
        self.sender.send(f).unwrap();
    }

    fn get_sender(&self) -> mpsc::Sender<Task> {
        self.sender.clone()
    }

    fn populate(&mut self, stem: Arc<Mutex<WebSocket<AutoStream>>>) -> Arc<Mutex<WebSocket<AutoStream>>> {
        for id in 0..self.size {
            self.workers.push(Worker::new(id, Arc::clone(&self.receiver), Arc::clone(&stem)))
        }
        stem
    }

    fn find_gateway(&mut self) -> Url {
        let body = reqwest::blocking::get(constants::GET_GATEWAY).unwrap()
            .text().unwrap();
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        let socket_url = Url::parse(data.get("url").unwrap().as_str().unwrap()).unwrap();
        socket_url
    }

    fn start_socket(&mut self) -> Ws {
        let socket_url = self.find_gateway();
        let (mut socket, response) =
            connect(socket_url).expect("Can't connect");
        Arc::new(Mutex::new(socket))
    }

    fn find_heartbeat_duration(&mut self, streamer: Ws) -> (time::Duration, Ws) {
        let msg = streamer.lock().unwrap().read_message().expect("Error reading message");
        println!("Received: {}", msg);
        let text = msg.into_text().unwrap();
        let data: serde_json::Value = serde_json::from_str(&text).unwrap();
        let heartbeat = time::Duration::from_millis(data.get("d").unwrap().get("heartbeat_interval").unwrap().as_u64().unwrap());
        (heartbeat, streamer)
    }

    fn reader(&mut self, streamer: Ws, sender: mpsc::Sender<Task>, seq_code: Arc<Mutex<&str>>) {
        let reader = thread::spawn(move || {
            // Might wanna do first iteration out of thread to share the required delay
            loop {
                let msg = streamer.lock().unwrap().read_message().expect("Error reading message");
                println!("Received: {}", msg);
            }
        });
    }

    fn heartbeater(&mut self, streamer: Ws, sender: mpsc::Sender<Task>, heartbeat: time::Duration, seq_code: Arc<Mutex<&str>>) {
        let heartbeater = thread::spawn(move || {
            loop {
                sender.send(Task::Heartbeat {metadata: seq_code.lock().unwrap().to_string()});
                thread::sleep(heartbeat);
            }
        });
    }

    pub fn connect(&mut self) {
        let streamer = self.start_socket();
        let (heartbeat, streamer) = self.find_heartbeat_duration(streamer);
        let streamer = self.populate(streamer);
        let sequence_code = Arc::new(Mutex::new("null"));

        self.reader(Arc::clone(&streamer), self.get_sender(), Arc::clone(&sequence_code));
        self.heartbeater(Arc::clone(&streamer), self.get_sender(), heartbeat, Arc::clone(&sequence_code));

    }
}

impl Drop for GatePool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Task::Terminate { metadata: "None".to_string() }).unwrap();
        }


        for worker in &mut self.workers {

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>, streamer: Arc<Mutex<WebSocket<AutoStream>>>) -> Worker {
        let thread = Some(thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    Task::Heartbeat{metadata} => {
                        println!("Received heartbeat metadata: {} - Worker {}", metadata, id)
                    }
                    Task::Identify{metadata} => {
                        println!("Received identify metadata: {} - Worker {}", metadata, id)
                    }
                    Task::Terminate{metadata} => {
                        break;
                    }
                }
            }
        }));

        Worker {
            id,
            thread,
        }

    }
}
