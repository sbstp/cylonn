use std::fmt;
use std::old_io::{Command, IoResult, Process};
use std::old_io::process::StdioContainer;

pub struct Plugin {
    pub name: String,
    pub cmd: String,
    pub running: bool,
    procc: Option<Process>,
}

impl fmt::Debug for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Same as derive(Debug), but without procc
        write!(f,
               "Plugin {{ name: {}, cmd: {}, running: {} }}",
               self.name,
               self.cmd,
               self.running)
    }
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
