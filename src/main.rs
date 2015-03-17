#![feature(old_io, old_path)]
#![feature(plugin)]
#![allow(dead_code, deprecated)]

use std::collections::HashMap;
use std::old_io::BufferedWriter;
use std::old_io::net::pipe::UnixStream;

use listener::Event;

extern crate "rustc-serialize" as serialize;
extern crate uuid;

mod init;
mod listener;
mod plugin;


/// Represents a remotely connected client.
pub struct Client {
    client_id: u32,
    writer: BufferedWriter<UnixStream>,
    // TODO: plugin
}

impl Client {

    pub fn new(client_id: u32, stream: UnixStream) -> Self {
        Client {
            client_id: client_id,
            writer: BufferedWriter::new(stream),
        }
    }

}

fn main() {
    // Path of the UnixSocket.
    let (path, receiver) = listener::create();

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
