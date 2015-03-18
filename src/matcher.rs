pub fn kind_matches(kind: &str, globs: &[&str]) -> bool {
    globs.iter().any(|g| test_kind(kind, g))
}

fn test_kind(kind: &str, glob: &str) -> bool {
    if glob == "*" {
        true
    } else if glob.ends_with("/*") {
        kind.starts_with(glob.slice_to(glob.len() - 1))
    } else {
        glob == kind
    }
}

#[cfg(test)]
mod tests {
    use super::{test_kind, kind_matches};
    use test::Bencher;

    #[test]
    fn test_single_simple() {
        assert!(test_kind("irc/in", "irc/in"));
        assert!(!test_kind("irc/in", "irc/out"));
    }

    #[test]
    fn test_single_glob() {
        assert!(test_kind("irc/in", "irc/*"));
        assert!(test_kind("irc/in", "*"));
    }

    #[test]
    fn test_multi_glob() {
        assert!(kind_matches("irc/in", &["irc/out", "irc/in"]));
        assert!(kind_matches("irc/in", &["irc/out", "irc/*"]));
    }

    #[test]
    fn test_glob_utf8() {
        assert!(!test_kind("irc/in", "ircééé/*"));
        assert!(test_kind("foo/àd/éx/ûd", "foo/àd/*"));
    }

    #[bench]
    fn bench_single_short(b: &mut Bencher) {
        b.iter(|| test_kind("irc/in", "irc/in"));
    }

    #[bench]
    fn bench_single_long(b: &mut Bencher) {
        static TEXT: &'static str = "this/is/a/much/longer/kind/that/should/be/slower";
        b.iter(|| test_kind(TEXT, TEXT));
    }

    #[bench]
    fn bench_single_glob(b: &mut Bencher) {
        b.iter(|| test_kind("irc/in", "irc/*"));
    }

    #[bench]
    fn bench_multi_last(b: &mut Bencher) {
        b.iter(|| {
            kind_matches("irc/in", &["irc/out/kick", "irc/kick", "irc/in"]);
        });
    }
}
