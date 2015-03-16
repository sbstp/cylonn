use std::old_io::{BufferedReader, File, IoResult};

use plugin::Plugin;

// Syntax error messages
static ERR_NOCOLON: &'static str = "No colon delimiter found";
static ERR_NONAME: &'static str = "Plugin has no name";
static ERR_NOCMD: &'static str = "Plugin has no command";

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
            None => Line::SyntaxError(ERR_NOCOLON),
            Some(i) => {
                let name = ln[..i].trim();
                let cmd = ln[i+1..].trim();
                if name.is_empty() {
                    Line::SyntaxError(ERR_NONAME)
                } else if cmd.is_empty() {
                    Line::SyntaxError(ERR_NOCMD)
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
        // TODO: handle syntax error case
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
               Line::SyntaxError(ERR_NONAME));
}

#[test]
fn test_parse_line_err_no_name_spacey() {
    assert_eq!(parse_line("  :  cat /dev/null   "),
               Line::SyntaxError(ERR_NONAME));
}

#[test]
fn test_parse_line_no_cmd() {
    assert_eq!(parse_line("nothing:"),
               Line::SyntaxError(ERR_NOCMD));
}

#[test]
fn test_parse_line_no_cmd_spacey() {
    assert_eq!(parse_line("   nothing:  "),
               Line::SyntaxError(ERR_NOCMD));
}
