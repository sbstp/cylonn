use std::old_io::BufferedReader;
use std::old_io::IoResult;
use std::old_io::File;

use regex::{Captures, Regex};

pub struct Plugin {
    pub name: String,
    pub cmd: String,
}

impl Plugin {

    fn new(name: &str, cmd: &str) -> Plugin {
        Plugin{
            name: name.to_string(),
            cmd: cmd.to_string(),
        }
    }

}

fn read_line<'a>(line: &'a str) -> Option<Captures<'a>> {
    static RE_LINE: Regex = regex!(r"^\s*([^#\s]+)\s*:\s*([^#\r\n]+)\s*$");
    RE_LINE.captures(line)
}

pub fn read_init(path: &Path) -> IoResult<Vec<Plugin>> {
    let mut plugins = Vec::new();
    let file = try!(File::open(path));
    let mut reader = BufferedReader::new(file);
    while let Ok(ref line) = reader.read_line() {
        if let Some(cap) = read_line(line) {
            if let (Some(name), Some(cmd)) = (cap.at(1), cap.at(2)) {
                plugins.push(Plugin::new(name, cmd));
            }
        }
    }
    Ok(Vec::new())
}
