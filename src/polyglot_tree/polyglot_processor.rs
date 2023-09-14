use crate::context::TopoOrder;

use super::polyglot_zipper::PolyglotZipper;
use std::collections::{HashMap, HashSet};

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

    fn from(parent: &Self) -> TreePrinter {
        TreePrinter {
            indent_level: parent.indent_level,
            result: String::new(),
        }
    }

    /// Returns a pretty printed version of the last processed polyglot tree.
    /// If this processor has not yet been applied to any tree, the string will be empty.
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }

    pub fn process_impl(&mut self, zip: PolyglotZipper) {
        let mut indent = String::from(" ").repeat(self.indent_level);

        let child = zip.child(0);
        match child {
            Some(z) => {
                z.node().id();
                indent.push_str(&format!("{}\n", zip.kind()));
                self.result.push_str(&indent);
                self.indent_level += 1;
                let mut nextp = TreePrinter::from(self);
                nextp.process(z);
                self.result.push_str(nextp.get_result())
            }
            None => {
                indent.push_str(&format!("{} : {}\n", zip.kind(), zip.code()));
                self.result.push_str(&indent);
            }
        };

        let sibling = zip.next_sibling();
        if let Some(z) = sibling {
            let mut nextp = TreePrinter::from(self);
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

/// A simple processor that pretty prints the polyglot AST.
/// After processing a tree, use the `get_result` method to retrieve the generated string.
pub struct TreePrinterGlobal {
    global: std::sync::Arc<crate::context::GlobalContext>,
    stack: Vec<(crate::context::InternalHandle, TopoOrder)>,
    indent_level: usize,
    result: String,
}

impl TreePrinterGlobal {
    /// Initializes a new TreePrinterGlobal instance.
    pub fn new(global: std::sync::Arc<crate::context::GlobalContext>) -> TreePrinterGlobal {
        TreePrinterGlobal {
            stack: vec![],
            global,
            indent_level: 0,
            result: String::new(),
        }
    }

    fn from(parent: &Self) -> TreePrinterGlobal {
        TreePrinterGlobal {
            stack: vec![],
            global: parent.global.clone(),
            indent_level: parent.indent_level,
            result: String::new(),
        }
    }

    /// Returns a pretty printed version of the last processed polyglot tree.
    /// If this processor has not yet been applied to any tree, the string will be empty.
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }

    pub fn process_impl(&mut self, zip: PolyglotZipper) {
        let mut indent = String::from(" ").repeat(self.indent_level);

        let child = zip.child(0);
        match child {
            Some(z) => {
                if Some(z.node().id())
                    == self.global.sources[self.global.root.0]
                        .2
                        .get(0)
                        .map(|x| x.0 .0)
                {
                    indent.push_str(&format!("{}\n", "polyglot_stuff"));
                } else {
                    indent.push_str(&format!("{}\n", zip.kind()));
                    self.result.push_str(&indent);
                    self.indent_level += 1;
                    let mut nextp = TreePrinterGlobal::from(self);
                    nextp.process(z);
                    self.result.push_str(nextp.get_result());
                }
            }
            None => {
                indent.push_str(&format!("{} : {}\n", zip.kind(), zip.code()));
                self.result.push_str(&indent);
            }
        };

        let sibling = zip.next_sibling();
        if let Some(z) = sibling {
            let mut nextp = TreePrinterGlobal::from(self);
            nextp.process(z);
            self.result.push_str(nextp.get_result())
        }
    }
}

impl PolygotProcessor for TreePrinterGlobal {
    fn process(&mut self, zip: PolyglotZipper) {
        self.result = String::new();
        self.process_impl(zip);
    }
}

pub struct DUBuilder {
    imports: HashMap<String, HashSet<(u32, u32)>>,
    exports: HashMap<String, (u32, u32)>,
}

impl Default for DUBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DUBuilder {
    pub fn new() -> Self {
        DUBuilder {
            imports: HashMap::new(),
            exports: HashMap::new(),
        }
    }

    fn from(parent: Self) -> Self {
        DUBuilder {
            imports: parent.imports,
            exports: parent.exports,
        }
    }

    fn update_maps(&mut self, son: &Self) {
        self.imports.extend(son.imports.clone().into_iter());

        self.exports.extend(son.exports.clone().into_iter());
    }

    pub fn get_imports(&self) -> &HashMap<String, HashSet<(u32, u32)>> {
        &self.imports
    }

    pub fn get_exports(&self) -> &HashMap<String, (u32, u32)> {
        &self.exports
    }

    pub fn print_inconsistencies(&self) {
        for imp in self.imports.keys() {
            if !self.exports.contains_key(imp) {
                println!("Imported but not exported: {imp}")
            }
        }

        for exp in self.exports.keys() {
            if !self.exports.contains_key(exp) {
                println!("Exported but not imported: {exp}")
            }
        }
    }

    fn process_impl(&mut self, zip: Option<PolyglotZipper>) {
        if zip.is_some() {
            let zip = zip.unwrap();

            if zip.is_polyglot_import_call() {
                todo!()
            }

            if zip.is_polyglot_export_call() {
                todo!()
            }

            self.process_impl(zip.child(0));
            self.process_impl(zip.next_sibling());
        }
    }
}

impl PolygotProcessor for DUBuilder {
    fn process(&mut self, zip: PolyglotZipper) {
        self.process_impl(Some(zip));
    }
}
