use std::error;
use std::fmt;
use std::old_io::{BufferedReader, File, IoError, IoResult};

use plugin::Plugin;

#[derive(Copy, PartialEq, Eq, Debug)]
pub enum SyntaxErrorKind {
    NoColon,
    NoName,
    NoCommand,
}

impl fmt::Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyntaxErrorKind::NoColon => write!(f, "{}", "No colon delimiter found"),
            SyntaxErrorKind::NoName => write!(f, "{}", "Plugin has no name"),
            SyntaxErrorKind::NoCommand => write!(f, "{}", "Plugin has no command"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub line_no: u32,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} on line {}", self.kind, self.line_no)
    }
}

#[derive(Debug)]
pub enum ReadError {
    IoError(IoError),
    SyntaxError(SyntaxError),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::IoError(ref err) => write!(f, "I/O Error: {}", err),
            ReadError::SyntaxError(ref err) => write!(f, "Syntax Error: {}", err),
        }
    }
}

impl error::FromError<IoError> for ReadError {
    fn from_error(err: IoError) -> Self {
        ReadError::IoError(err)
    }
}

impl error::FromError<SyntaxError> for ReadError {
    fn from_error(err: SyntaxError) -> Self {
        ReadError::SyntaxError(err)
    }
}

fn parse_line<'a>(line_no: u32, line: &'a str) -> Result<Option<Plugin>, SyntaxError> {
    let ln = line.trim();
    if ln.is_empty() || ln.starts_with('#') {
        Ok(None)
    } else {
        match ln.find(':') {
            None => Err(SyntaxError{kind: SyntaxErrorKind::NoColon,
                                    line_no: line_no}),
            Some(i) => {
                let name = ln[..i].trim();
                let cmd = ln[i+1..].trim();
                if name.is_empty() {
                    Err(SyntaxError{kind: SyntaxErrorKind::NoName,
                                    line_no: line_no})
                } else if cmd.is_empty() {
                    Err(SyntaxError{kind: SyntaxErrorKind::NoCommand,
                                    line_no: line_no})
                } else {
                    Ok(Some(Plugin::new(name, cmd)))
                }
            },
        }
    }
}

pub fn read_init(path: &Path) -> Result<Vec<Plugin>, ReadError> {
    let mut plugins = Vec::new();
    let file = try!(File::open(path));
    let mut reader = BufferedReader::new(file);
    let mut line_no = 0;
    while let Ok(ref line) = reader.read_line() {
        line_no += 1;
        if let Some(plugin) = try!(parse_line(line_no, line)) {
            plugins.push(plugin);
        }
    }
    Ok(plugins)
}

#[test]
fn test_parse_line() {
    let plugin = parse_line(1, "test: run -a script").unwrap().unwrap();
    assert_eq!(plugin.name, "test");
    assert_eq!(plugin.cmd, "run -a script");
}

#[test]
fn test_parse_line_spacey() {
    let plugin = parse_line(1, "  test :  run -a script  ").unwrap().unwrap();
    assert_eq!(plugin.name, "test");
    assert_eq!(plugin.cmd, "run -a script");
}

#[test]
fn test_parse_line_comment() {
    assert!(parse_line(1, "# don't run -a script").unwrap().is_none());
}

#[test]
fn test_parse_line_comment_spacey() {
    assert!(parse_line(1, "  # don't run -a script  ").unwrap().is_none());
}

#[test]
fn test_parse_line_empty() {
    assert!(parse_line(1, "").unwrap().is_none());
}

#[test]
fn test_parse_line_empty_spacey() {
    assert!(parse_line(1, "    ").unwrap().is_none());
}

#[test]
fn test_parse_line_hash_name() {
    let plugin = parse_line(1, "hash#: this is valid").unwrap().unwrap();
    assert_eq!(plugin.name, "hash#");
    assert_eq!(plugin.cmd, "this is valid");
}

#[test]
fn test_parse_line_err_no_colon() {
    assert_eq!(parse_line(1, "I AM ERROR").unwrap_err().kind,
               SyntaxErrorKind::NoColon);
}

#[test]
fn test_parse_line_err_no_name() {
    assert_eq!(parse_line(1, ": cat /dev/null").unwrap_err().kind,
               SyntaxErrorKind::NoName);
}

#[test]
fn test_parse_line_err_no_name_spacey() {
    assert_eq!(parse_line(1, "  :  cat /dev/null   ").unwrap_err().kind,
               SyntaxErrorKind::NoName);
}

#[test]
fn test_parse_line_no_cmd() {
    assert_eq!(parse_line(1, "nothing:").unwrap_err().kind,
               SyntaxErrorKind::NoCommand);
}

#[test]
fn test_parse_line_no_cmd_spacey() {
    assert_eq!(parse_line(1, "   nothing:  ").unwrap_err().kind,
               SyntaxErrorKind::NoCommand);
}
