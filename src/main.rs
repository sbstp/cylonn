#![feature(old_io, old_path)]
#![feature(core)]
#![feature(test)]
#![allow(dead_code, deprecated)]

extern crate "rustc-serialize" as serialize;
extern crate uuid;

#[cfg(test)]
extern crate test;

mod init;
mod listener;
mod messenger;
pub mod glob;
mod plugin;

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

    messenger::broadcast(receiver);
}
