use std::str::Pattern;

#[derive(Debug)]
enum Glob {
    MatchAll,
    MatchPrefix(String),
    MatchExact(String),
}

#[derive(Debug)]
pub struct GlobError(String);

#[derive(Debug)]
pub struct GlobSet {
    globs: Vec<Glob>,
}

impl GlobSet {
    fn from_globs(globs: &[&str]) -> Result<GlobSet, GlobError> {
        let mut gs = GlobSet{globs: Vec::new()};
        for glob in globs {
            try!(gs.add_glob(glob));
        }
        Ok(gs)
    }

    fn match_kind(&self, kind: &str) -> bool {
        self.globs.iter().any(|g| {
            match *g {
                Glob::MatchAll => true,
                Glob::MatchPrefix(ref ns) => ns.is_prefix_of(kind),
                Glob::MatchExact(ref k) => kind == *k,
            }
        })
    }

    fn add_glob(&mut self, glob: &str) -> Result<(), GlobError> {
        if glob == "*" {
            Ok(self.globs.push(Glob::MatchAll))
        } else if glob.contains("*") {
            if glob.ends_with("/*") {
                let prefix = glob.slice_to(glob.len() - 1);
                if prefix.contains("*") {
                    Err(GlobError(glob.to_string()))
                } else {
                    Ok(self.globs.push(Glob::MatchPrefix(prefix.to_string())))
                }
            } else {
                Err(GlobError(glob.to_string()))
            }
        } else {
            Ok(self.globs.push(Glob::MatchExact(glob.to_string())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GlobSet};
    use test::Bencher;

    #[test]
    fn test_single_match_all() {
        let gs = GlobSet::from_globs(&["*"]).unwrap();
        assert!(gs.match_kind("po/ta/to"));
    }

    #[test]
    fn test_single_prefix_yes() {
        let gs = GlobSet::from_globs(&["irc/*"]).unwrap();
        assert!(gs.match_kind("irc/in"));
    }

    #[test]
    fn test_single_prefix_no() {
        let gs = GlobSet::from_globs(&["irc/*"]).unwrap();
        assert!(!gs.match_kind("xyz/in"));
    }

    #[test]
    fn test_single_exact_yes() {
        let gs = GlobSet::from_globs(&["irc/in"]).unwrap();
        assert!(gs.match_kind("irc/in"));
    }

    #[test]
    fn test_single_exact_no() {
        let gs = GlobSet::from_globs(&["irc/in"]).unwrap();
        assert!(!gs.match_kind("irc/out"));
    }

    #[test]
    fn test_multi_exact() {
        let gs = GlobSet::from_globs(&["irc/out", "irc/in"]).unwrap();
        assert!(gs.match_kind("irc/in"));
    }

    fn test_multi_prefix() {
        let gs = GlobSet::from_globs(&["irc/out", "irc/*"]).unwrap();
        assert!(gs.match_kind("irc/in"));
    }

    #[test]
    fn test_glob_utf8() {
        let gs = GlobSet::from_globs(&["café/maïs/*"]).unwrap();
        assert!(gs.match_kind("café/maïs/sève"));
        assert!(!gs.match_kind("café/thé"));
    }

    #[bench]
    fn bench_single_short_exact(b: &mut Bencher) {
        let gs = GlobSet::from_globs(&["irc/in"]).unwrap();
        b.iter(|| gs.match_kind("irc/in"));
    }

    #[bench]
    fn bench_single_long_exact(b: &mut Bencher) {
        static KIND: &'static str = "this/is/a/much/longer/kind/that/should/be/slower";
        let gs = GlobSet::from_globs(&[KIND]).unwrap();
        b.iter(|| gs.match_kind(KIND));
    }

    #[bench]
    fn bench_single_short_prefix(b: &mut Bencher) {
        let gs = GlobSet::from_globs(&["irc/*"]).unwrap();
        b.iter(|| gs.match_kind("irc/in"));
    }

    #[bench]
    fn bench_multi_last(b: &mut Bencher) {
        let gs = GlobSet::from_globs(&[
            "irc/out/kick",
            "irc/kick",
            "irc/in",
        ]).unwrap();
        b.iter(|| gs.match_kind("irc/in"));
    }
}
