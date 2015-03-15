#![feature(old_io, old_path, plugin)]
#![allow(deprecated)]
#![plugin(regex_macros)]

use std::old_io::BufferedStream;
use std::old_io::{Acceptor, Listener};
use std::old_io::net::pipe::{UnixListener, UnixStream};
use std::thread;

use uuid::Uuid;

extern crate regex;
extern crate "rustc-serialize" as serialize;
extern crate uuid;

mod plugin;

fn handle_stream(mut stream: UnixStream) {
    let buf = BufferedStream::new(stream);
}

fn main() {
    let path = format!("/tmp/{}.sock", Uuid::new_v4().to_simple_string());
    let sock = UnixListener::bind(path).unwrap();
    let mut acceptor = sock.listen().unwrap();

    loop {
        let mut stream = acceptor.accept().unwrap();
        thread::spawn(move || {
            handle_stream(stream);
        });
    }
}
