#![feature(old_io, plugin)]
#![allow(deprecated)]
#![plugin(regex_macros)]

use std::old_io::{BufferedReader, BufferedStream};
use std::old_io::{Acceptor, Listener};
use std::old_io::{IoError, IoResult};
use std::old_io::{File};
use std::old_io::net::pipe::{UnixListener, UnixStream};
use std::thread;

use regex::{Captures, Regex};
use uuid::Uuid;

extern crate regex;
extern crate regex_macros;
extern crate "rustc-serialize" as serialize;
extern crate uuid;
//
//
// fn handle_stream(mut stream: UnixStream) {
//     let buf = BufferedStream::new(stream);
//     buf.read_line();
// }

struct Plugin {
    pub name: String,
    pub cmd: String,
}


fn read_line<'a>(line: &'a str) -> Option<Captures<'a>> {
    static RE_LINE: Regex = regex!(r"^\s*([^#\s]+)\s*:\s*([^#\r\n]+)\s*$");
    RE_LINE.captures(line)
}

fn read_init() -> IoResult<Vec<Plugin>> {
    let mut plugins = Vec::new();
    let path = Path::new("init");
    let file = try!(File::open(&path));
    let mut reader = BufferedReader::new(file);
    while let Ok(ref line) = reader.read_line() {
        if let Some(cap) = read_line(line) {
            if let (Some(name), Some(cmd)) = (cap.at(1), cap.at(2)) {
                plugins.push(Plugin{
                    name: name.to_string(),
                    cmd: cmd.to_string(),
                });
            }
        }
    }
    Ok(Vec::new())
}

fn main() {
    read_init();
    // let path = format!("/tmp/{}.sock", Uuid::new_v4().to_simple_string());
    // let sock = UnixListener::bind(path).unwrap();
    // let mut acceptor = sock.listen().unwrap();
    //
    // loop {
    //     let mut stream = acceptor.accept().unwrap();
    //     thread::spawn(move || {
    //         handle_stream(stream);
    //     });
    // }
}
