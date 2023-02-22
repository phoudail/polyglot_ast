use super::polyglot_zipper::PolyglotZipper;

pub trait PolygotProcessor {
    fn process(&mut self, zip: PolyglotZipper);
}

pub struct TreePrinter {
    indent_level: usize,
    result: String,
}

impl TreePrinter {
    pub fn new() -> TreePrinter {
        TreePrinter {
            indent_level: 0,
            result: String::new(),
        }
    }

    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }
}

impl PolygotProcessor for TreePrinter {
    fn process(&mut self, zip: PolyglotZipper) {
        let mut res = String::from(" ").repeat(self.indent_level);
        res.push_str(&format!("{}\n", zip.kind()));
        self.result.push_str(&res);

        let child = zip.child(0);
        match child {
            Some(z) => self.process(z),
            None => (),
        };

        let sibling = zip.next_sibling();
        match sibling {
            Some(z) => self.process(z),
            None => (),
        }
    }
}
