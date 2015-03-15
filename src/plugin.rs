use std::old_io::{BufferedReader, Command, File, IoResult, Process};
use std::old_io::process::StdioContainer;

use regex::{Captures, Regex};

pub struct Plugin {
    pub name: String,
    pub cmd: String,
    pub running: bool,
    procc: Option<Process>,
}

impl Plugin {
    pub fn new(name: &str, cmd: &str) -> Plugin {
        Plugin{
            name: name.to_string(),
            cmd: cmd.to_string(),
            running: false,
            procc: None,
        }
    }

    pub fn load(&mut self, sock: &str) -> IoResult<()> {
        // Use sh -c "<command>" until a better way exists
        let procc = try!(Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", self.cmd, sock))
            .stdin(StdioContainer::InheritFd(0))
            .stdout(StdioContainer::InheritFd(1))
            .stderr(StdioContainer::InheritFd(2))
            .spawn());
        self.running = true;
        self.procc = Some(procc);
        Ok(())
    }

    // TODO does not work
    pub fn unload(&mut self) {
        if let Some(ref mut procc) = self.procc {
            procc.signal(15).unwrap(); // SIGTERM
            self.running = false;
        }
    }

    pub fn reload(&mut self, sock: &str) -> IoResult<()> {
        self.unload();
        self.load(sock)
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
    Ok(plugins)
}
