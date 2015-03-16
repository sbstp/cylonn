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

fn parse_line<'a>(line: &'a str) -> Option<(&'a str, &'a str)> {
    static RE_LINE: Regex = regex!(r"^\s*([^#\s]+)\s*:\s*((\s*[^#\s]+)+)\s*$");

    match RE_LINE.captures(line) {
        Some(cap) => {
            match (cap.at(1), cap.at(2)) {
                (Some(name), Some(cmd)) => Some((name, cmd)),
                _ => None,
            }
        },
        _ => None,
    }
}

pub fn read_init(path: &Path) -> IoResult<Vec<Plugin>> {
    let mut plugins = Vec::new();
    let file = try!(File::open(path));
    let mut reader = BufferedReader::new(file);
    while let Ok(ref line) = reader.read_line() {
        if let Some((name, cmd)) = parse_line(line) {
            plugins.push(Plugin::new(name, cmd));
        }
    }
    Ok(plugins)
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("test: run -a script"),
               Some(("test", "run -a script")));
}

#[test]
fn test_parse_line_trim() {
    assert_eq!(parse_line("  test  :   run -a script  "),
               Some(("test", "run -a script")));
}

#[test]
fn test_parse_line_comment() {
    assert_eq!(parse_line("# don't run -a script"),
               None);
}
