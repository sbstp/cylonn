#![feature(old_io, old_path, plugin)]
#![allow(dead_code, deprecated)]

use std::old_io::BufferedStream;
use std::old_io::{Acceptor, Listener};
use std::old_io::net::pipe::{UnixListener, UnixStream};
use std::old_io::timer::sleep;
use std::old_io::IoResult;
use std::time::duration::Duration;
use std::thread;
use std::sync::mpsc::{self, Sender};

use uuid::Uuid;

extern crate "rustc-serialize" as serialize;
extern crate uuid;

mod init;
mod plugin;

enum Event {
    Line(String),
    Stream(UnixStream),
}

fn handle(sock: UnixStream, sender: Sender<Event>) {
    let mut reader = BufferedStream::new(sock.clone());
    sender.send(Event::Stream(sock.clone()));

    while let Ok(line) = reader.read_line() {
        println!("new line");
        sender.send(Event::Line(line));
    }
}

fn accept(path: String, sender: Sender<Event>) {
    let listener = UnixListener::bind(path.as_slice()).unwrap();
    let mut acceptor = listener.listen().unwrap();

    while let Ok(sock) = acceptor.accept() {
        println!("new client");
        let s = sender.clone();
        thread::spawn(move || {
            handle(sock, s);
        });
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
    let mut plugins = init::read_init(&Path::new("init")).unwrap();

    // Launch the plugins.
    for p in plugins.iter_mut() {
        if let Err(ref err) = p.load(path.as_slice()) {
            println!("{}", err);
        }
    }

    // List of streams.
    let mut streams = Vec::new();

    // Await for events.
    while let Ok(event) = receiver.recv() {
        match event {
            Event::Line(line) => println!("{}", line),
            Event::Stream(stream) => {
                println!("stream received");
                streams.push(stream);
            }
        }
    }
}
