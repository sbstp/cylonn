use std::iter::Iterator;

pub fn kind_matches(kind: &str, globs: &[&str]) -> bool {
    for glob in globs.iter() {
        if test_kind(kind, glob) {
            return true;
        }
    }
    false
}

/// This is a variant of `zip` that stops when both underlying iterators yield None.
struct GreedyZip<'d, A, B> {
    a: Box<Iterator<Item=A> + 'd>,
    b: Box<Iterator<Item=B> + 'd>,
}

impl<'d, A, B> GreedyZip<'d, A, B> {

    fn new<'e, IA, IB>(a: IA, b: IB) -> GreedyZip<'e, A, B>
            where IA: Iterator<Item=A> + 'e, IB: Iterator<Item=B> + 'e {
        GreedyZip {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

}

impl<'d, A, B> Iterator for GreedyZip<'d, A, B> {
    type Item = (Option<A>, Option<B>);

    fn next(&mut self) -> Option<(Option<A>, Option<B>)> {
        match (self.a.next(), self.b.next()) {
            (Some(a), Some(b)) => Some((Some(a), Some(b))),
            (Some(a), None) => Some((Some(a), None)),
            (None, Some(b)) => Some((None, Some(b))),
            (None, None) => None,
        }
    }

}

fn test_kind(kind: &str, glob: &str) -> bool {
    let mut it = GreedyZip::new(kind.chars(), glob.chars());
    for o in it {
        match o {
            (Some(kch), Some(gch)) => {
                if gch == '*' {
                    // Rest of glob does not matter.
                    return true;
                } else if kch != gch {
                    // Characters are not equal.
                    return false;
                }
            }
            // (Some,None) or (None,Some) means no match.
            _ => return false,
        }
    }
    true
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
