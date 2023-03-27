use super::polyglot_zipper::PolyglotZipper;

/// A trait to allow processing over a polyglot tree. 
/// This processing can be any kind of analysis, but starts at the root of the tree.
/// To start the tree analysis, call its apply method and pass the processor.
pub trait PolygotProcessor {
    /// The method starting the analysis of the tree for the implementing processor.
    /// This will always begin from the root of the tree, but navigation is left up to the implementer.
    fn process(&mut self, zip: PolyglotZipper);
}

/// A simple processor that pretty prints the polyglot AST. 
/// After processing a tree, use the `get_result` method to retrieve the generated string.
pub struct TreePrinter {
    indent_level: usize,
    result: String,
}

impl Default for TreePrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl TreePrinter {
    /// Initializes a new TreePrinter instance.
    pub fn new() -> TreePrinter {
        TreePrinter {
            indent_level: 0,
            result: String::new(),
        }
    }

    fn from(&self) -> TreePrinter {
        TreePrinter {
            indent_level: self.indent_level,
            result: String::new(),
        }
    }

    /// Returns a pretty printed version of the last processed polyglot tree.
    /// If this processor has not yet been applied to any tree, the string will be empty.
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }

    fn process_impl(&mut self, zip: PolyglotZipper) {
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
        if let Some(z) = sibling {
            let mut nextp = self.from();
            nextp.process(z);
            self.result.push_str(nextp.get_result())
        }
    }
}

impl PolygotProcessor for TreePrinter {
    fn process(&mut self, zip: PolyglotZipper) {
        self.result = String::new();
        self.process_impl(zip);
    }
}
