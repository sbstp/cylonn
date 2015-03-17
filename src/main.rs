#![feature(old_io, old_path)]
#![feature(plugin)]
#![allow(dead_code, deprecated)]

use std::collections::HashMap;
use std::old_io::{BufferedReader, BufferedWriter};
use std::old_io::{Acceptor, Listener};
use std::old_io::net::pipe::{UnixListener, UnixStream};
use std::thread;
use std::sync::mpsc::{self, Sender};

use uuid::Uuid;

extern crate "rustc-serialize" as serialize;
extern crate uuid;

mod init;
mod plugin;

struct Builder {
    client_id: u32,
}

impl Builder {

    fn new(client_id: u32) -> Self {
        Builder { client_id: client_id }
    }

    fn message(&self, event: Event) -> Message {
        Message { client_id: self.client_id, event: event }
    }

    fn line(&self, line: String) -> Message {
        Message { client_id: self.client_id, event: Event::Line(line) }
    }

    fn stream(&self, stream: UnixStream) -> Message {
        Message { client_id: self.client_id, event: Event::Stream(stream) }
    }

}

struct Message {
    client_id: u32,
    event: Event,
}

enum Event {
    Line(String),
    Stream(UnixStream),
}

struct Client {
    client_id: u32,
    writer: BufferedWriter<UnixStream>,
    // TODO: plugin
}

impl Client {

    fn new(client_id: u32, stream: UnixStream) -> Self {
        Client {
            client_id: client_id,
            writer: BufferedWriter::new(stream),
        }
    }

}

fn handle(id: u32, sock: UnixStream, sender: Sender<Message>) {
    let builder = Builder::new(id);
    let mut reader = BufferedReader::new(sock.clone());
    // TODO: use Result of send() call
    sender.send(builder.stream(sock.clone()));

    // TODO: handle errors
    while let Ok(line) = reader.read_line() {
        println!("new line");
        sender.send(builder.line(line));
    }
}

fn accept(path: String, sender: Sender<Message>) {
    let listener = UnixListener::bind(&path[..]).unwrap();
    let mut acceptor = listener.listen().unwrap();
    // Associate each client a unique id.
    let mut client_id = 0u32;

    while let Ok(sock) = acceptor.accept() {
        let s = sender.clone();
        thread::spawn(move || {
            handle(client_id, sock, s);
        });
        client_id += 1;
    }
}

fn main() {
    // Path of the UnixSocket.
    let path = format!("/tmp/{}.sock", Uuid::new_v4().to_simple_string());
    // Channel that send events.
    let (sender, receiver) = mpsc::channel::<Message>();

    // Create the server thread.
    // It creates a UnixSocket and waits for connections.
    {
        let path = path.clone();
        thread::spawn(move || {
            accept(path, sender);
        });
    }


    // Read the plugins from the init file.
    let mut plugins = match init::read_init(&Path::new("init")) {
        Ok(plugins) => plugins,
        Err(err) => panic!("{}", err),
    };

    // Launch the plugins.
    for p in plugins.iter_mut() {
        if let Err(ref err) = p.load(&path[..]) {
            println!("{}", err);
        }
    }

    // List of clients.
    let mut clients: HashMap<u32, Client> = HashMap::new();

    // Await for events.
    // TODO: handle errors
    while let Ok(message) = receiver.recv() {
        match message.event {
            Event::Line(line) => {
                println!("{}: {}", message.client_id, line);
                for (client_id, client) in clients.iter_mut() {
                    // Do not broadcast the message to the sender.
                    if *client_id != message.client_id {
                        client.writer.write_str(&line[..]);
                        client.writer.flush();
                    }
                }
            }
            Event::Stream(stream) => {
                println!("{}: stream received", message.client_id);
                clients.insert(message.client_id, Client::new(message.client_id, stream));
            }
        }
    }
}
