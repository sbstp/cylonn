pub fn kind_matches(kind: &str, globs: &[&str]) -> bool {
    for glob in globs.iter() {
        if test_kind(kind, glob) {
            return true;
        }
    }
    false
}

fn test_kind(kind: &str, glob: &str) -> bool {
    let mut kit = kind.chars();
    let mut git = glob.chars();
    loop {
        // Manual zip
        match (kit.next(), git.next()) {
            (Some(kch), Some(gch)) => {
                if gch == '*' {
                    // Rest of glob does not matter.
                    return true;
                } else if kch != gch {
                    // Characters are not equal.
                    return false;
                }
            }
            // Both iterators reached the end at the same time.
            (None, None) => return true,
            // (Some,None) or (None,Some) means no match.
            _ => return false,
        }
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

    #[bench]
    fn bench_single_short(b: &mut Bencher) {
        b.iter(|| test_kind("irc/in", "irc/in"));
    }

    #[bench]
    fn bench_single_long(b: &mut Bencher) {
        static text: &'static str = "this/is/a/much/longer/kind/that/should/be/slower";
        b.iter(|| test_kind(text, text));
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
