use super::polyglot_zipper::PolyglotZipper;

pub trait PolygotProcessor {
    fn process(&self, zip: PolyglotZipper);
}
