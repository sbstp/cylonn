use std::old_io::{BufferedReader, Command, File, IoResult, Process};
use std::old_io::process::StdioContainer;

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

#[derive(Debug, PartialEq)]
enum Line<'a> {
    Comment,
    Plugin(&'a str, &'a str),
    SyntaxError(&'static str),
}

fn parse_line<'a>(line: &'a str) -> Line {
    let ln = line.trim();
    if ln.starts_with('#') {
        Line::Comment
    } else {
        match ln.find(':') {
            None => Line::SyntaxError("No colon delimiter found"),
            Some(i) => {
                let name = ln[..i].trim();
                let cmd = ln[i+1..].trim();
                if name.is_empty() {
                    Line::SyntaxError("Plugin has no name")
                } else if cmd.is_empty() {
                    Line::SyntaxError("Plugin has no command")
                } else {
                    Line::Plugin(name, cmd)
                }
            },
        }
    }
}

pub fn read_init(path: &Path) -> IoResult<Vec<Plugin>> {
    let mut plugins = Vec::new();
    let file = try!(File::open(path));
    let mut reader = BufferedReader::new(file);
    while let Ok(ref line) = reader.read_line() {
        if let Line::Plugin(name, cmd) = parse_line(line) {
            plugins.push(Plugin::new(name, cmd));
        }
    }
    Ok(plugins)
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("test: run -a script"),
               Line::Plugin("test", "run -a script"));
}

#[test]
fn test_parse_line_spacey() {
    assert_eq!(parse_line("  test :  run -a script  "),
               Line::Plugin("test", "run -a script"));
}

#[test]
fn test_parse_line_comment() {
    assert_eq!(parse_line("# don't run -a script"),
               Line::Comment);
}

#[test]
fn test_parse_line_comment_spacey() {
    assert_eq!(parse_line("  # don't run -a script  "),
               Line::Comment);
}

#[test]
fn test_parse_line_hash_name() {
    assert_eq!(parse_line("hash#: this is valid"),
               Line::Plugin("hash#", "this is valid"));
}

#[test]
fn test_parse_line_err_no_name() {
    assert_eq!(parse_line(": cat /dev/null"),
               Line::SyntaxError("Plugin has no name"));
}

#[test]
fn test_parse_line_err_no_name_spacey() {
    assert_eq!(parse_line("  :  cat /dev/null   "),
               Line::SyntaxError("Plugin has no name"));
}

#[test]
fn test_parse_line_no_cmd() {
    assert_eq!(parse_line("nothing:"),
               Line::SyntaxError("Plugin has no command"));
}

#[test]
fn test_parse_line_no_cmd_spacey() {
    assert_eq!(parse_line("   nothing:  "),
               Line::SyntaxError("Plugin has no command"));
}
