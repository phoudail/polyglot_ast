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

    pub fn from(&self) -> TreePrinter{
        TreePrinter {
            indent_level: self.indent_level,
            result: String::new(),
        }
    }

    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }
}

impl PolygotProcessor for TreePrinter {
    fn process(&mut self, zip: PolyglotZipper) { // TODO fix indent
        let mut indent = String::from(" ").repeat(self.indent_level);

        let child = zip.child(0);
        match child {
            Some(z) => {
                indent.push_str(&format!("{}\n", zip.kind()));
                self.result.push_str(&indent);
                self.indent_level += 1;
                self.process(z);
            }
            None => {
                indent.push_str(&format!("{} : {}\n", zip.kind(), zip.code()));
                self.result.push_str(&indent);
            }
        };

        let sibling = zip.next_sibling();
        match sibling {
            Some(z) => {let mut nextp = self.from(); nextp.process(z); self.result.push_str(nextp.get_result()) },
            None => (),
        }
    }
}
