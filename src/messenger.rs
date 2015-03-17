use std::collections::HashMap;
use std::old_io::BufferedWriter;
use std::old_io::net::pipe::UnixStream;
use std::sync::mpsc::Receiver;

use listener::{Event, Message};

/// Represents a remotely connected client.
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

/// Manage and broadcast events sent to the receiver.
pub fn broadcast(receiver: Receiver<Message>) {
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
