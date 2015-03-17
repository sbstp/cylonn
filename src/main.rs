#![feature(old_io, old_path)]
#![feature(plugin)]
#![allow(dead_code, deprecated)]

use std::collections::HashMap;
use std::old_io::BufferedStream;
use std::old_io::{Acceptor, Listener};
use std::old_io::net::pipe::{UnixListener, UnixStream};
use std::thread;
use std::sync::mpsc::{self, Sender};

use uuid::Uuid;

extern crate "rustc-serialize" as serialize;
extern crate uuid;

mod init;
mod plugin;

enum Event {
    Line(u32, String),
    Stream(u32, UnixStream),
}

fn handle(id: u32, sock: UnixStream, sender: Sender<Event>) {
    let mut reader = BufferedStream::new(sock.clone());
    // TODO: use Result of send() call
    sender.send(Event::Stream(id, sock.clone()));

    while let Ok(line) = reader.read_line() {
        println!("new line");
        sender.send(Event::Line(id, line));
    }
}

fn accept(path: String, sender: Sender<Event>) {
    let listener = UnixListener::bind(&path[..]).unwrap();
    let mut acceptor = listener.listen().unwrap();
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
    let (sender, receiver) = mpsc::channel::<Event>();

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

    // List of streams.
    let mut streams = HashMap::new();

    // Await for events.
    while let Ok(event) = receiver.recv() {
        match event {
            Event::Line(id, line) => println!("{}: {}", id, line),
            Event::Stream(id, stream) => {
                println!("{}: stream received", id);
                streams.insert(id, stream);
            }
        }
    }
}
