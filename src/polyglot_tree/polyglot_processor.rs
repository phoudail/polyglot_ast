use crate::{context::TopoOrder, RawParseResult};

use super::polyglot_zipper::{PolyglotCursor, PolyglotZipper};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

/// A trait to allow processing over a polyglot tree.
/// This processing can be any kind of analysis, but starts at the root of the tree.
/// To start the tree analysis, call its apply method and pass the processor.
pub trait PolygotProcessor {
    /// The method starting the analysis of the tree for the implementing processor.
    /// This will always begin from the root of the tree, but navigation is left up to the implementer.
    fn process(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()>;
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

    pub fn process_impl(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()> {
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
        Ok(self)
    }
}

impl PolygotProcessor for TreePrinter {
    fn process(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()> {
        self.result = String::new();
        self.process_impl(zip)
    }
}

/// A simple processor that pretty prints the polyglot AST.
/// After processing a tree, use the `get_result` method to retrieve the generated string.
pub struct TreePrinterGlobal<'g> {
    global: &'g crate::context::GlobalContext,
    stack: Vec<(crate::context::InternalHandle, TopoOrder)>,
    indent_level: usize,
    result: String,
}

impl<'g> TreePrinterGlobal<'g> {
    /// Initializes a new TreePrinterGlobal instance.
    pub fn new(global: std::sync::Arc<&crate::context::GlobalContext>) -> TreePrinterGlobal<'g> {
        todo!("deprecated");
        // TreePrinterGlobal {
        //     stack: vec![],
        //     global,
        //     indent_level: 0,
        //     result: String::new(),
        // }
    }

    fn from(parent: &Self) -> TreePrinterGlobal {
        todo!("deprecated");
        // TreePrinterGlobal {
        //     stack: vec![],
        //     global: parent.global.clone(),
        //     indent_level: parent.indent_level,
        //     result: String::new(),
        // }
    }

    /// Returns a pretty printed version of the last processed polyglot tree.
    /// If this processor has not yet been applied to any tree, the string will be empty.
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }

    pub fn process_impl(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()> {
        let mut indent = String::from(" ").repeat(self.indent_level);
        todo!("deprecated");
        // let child = zip.child(0);
        // match child {
        //     Some(z) => {
        //         if Some(z.node().id())
        //             == self.global.sources[self.global.root.0]
        //                 .2
        //                 .get(0)
        //                 .map(|x| x.0 .0)
        //         {
        //             indent.push_str(&format!("{}\n", "polyglot_stuff"));
        //         } else {
        //             indent.push_str(&format!("{}\n", zip.kind()));
        //             self.result.push_str(&indent);
        //             self.indent_level += 1;
        //             let mut nextp = TreePrinterGlobal::from(self);
        //             nextp.process(z);
        //             self.result.push_str(nextp.get_result());
        //         }
        //     }
        //     None => {
        //         indent.push_str(&format!("{} : {}\n", zip.kind(), zip.code()));
        //         self.result.push_str(&indent);
        //     }
        // };

        // let sibling = zip.next_sibling();
        // if let Some(z) = sibling {
        //     let mut nextp = TreePrinterGlobal::from(self);
        //     nextp.process(z);
        //     self.result.push_str(nextp.get_result())
        // }
        // todo!()
    }

    // fn print_local(&mut self, h: &crate::context::InternalHandle) -> Option<()> {
    //     let (poly, tree, others) = self.global.get_raw_raw(h).unwrap();
    //     let cursor = super::polyglot_zipper::PreOrder2::from(tree.cst.as_ref().unwrap());
    //     loop {
    //         let Some((dir, z)) = cursor.next() else {
    //             return None;
    //         };

    //         if dir.is_down() {
    //             if Some(z.id())
    //                 == self.global.sources[self.global.root.0]
    //                     .2
    //                     .get(0)
    //                     .map(|x| x.0 .0)
    //             {
    //                 indent.push_str(&format!("{}\n", "polyglot_stuff"));
    //             } else {
    //                 indent.push_str(&format!("{}\n", cursor.kind()));
    //                 self.result.push_str(&indent);
    //                 self.indent_level += 1;
    //                 let mut nextp = TreePrinterGlobal::from(self);
    //                 nextp.process(z);
    //                 self.result.push_str(nextp.get_result());
    //             }
    //         } else {
    //         }

    //         match child {
    //             Some(z) => {}
    //             None => {
    //                 indent.push_str(&format!("{} : {}\n", zip.kind(), zip.code()));
    //                 self.result.push_str(&indent);
    //             }
    //         };

    //         let sibling = zip.next_sibling();
    //         if let Some(z) = sibling {
    //             let mut nextp = TreePrinterGlobal::from(self);
    //             nextp.process(z);
    //             self.result.push_str(nextp.get_result())
    //         }
    //     }
    // }

    // pub fn print_global(&mut self) {
    //     let indent_level = 0;
    //     let mut indent = String::from(" ").repeat(indent_level);

    //     let mut h = &self.global.root;

    //     self.stack = vec![];

    //     loop {
    //         self.print_local(h);
    //     }
    // }
}

impl<'g> PolygotProcessor for TreePrinterGlobal<'g> {
    fn process(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()> {
        self.result = String::new();
        self.process_impl(zip)
    }
}
pub struct PolyglotTreeSerializer<'g, Format = ()> {
    global: &'g crate::context::GlobalContext,
    _phantom: std::marker::PhantomData<Format>,
}

impl<'g, F> From<&'g crate::context::GlobalContext> for PolyglotTreeSerializer<'g, F> {
    fn from(global: &'g crate::context::GlobalContext) -> Self {
        Self {
            global,
            _phantom: Default::default(),
        }
    }
}

impl<'g> PolyglotTreeSerializer<'g, ()> {
    fn local(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        // h: &crate::context::InternalHandle,
        ele: &crate::context::AtributedEle,
        o: usize,
        cursor: &mut crate::polyglot_tree::polyglot_zipper::PreOrder2,
        indent: &mut usize,
    ) -> Result<Option<crate::context::InternalHandle>, std::fmt::Error> {
        loop {
            let Some((dir, z)) = cursor.next() else {
                return Ok(None);
            };

            if dir.is_down() {
                let p = ele.2.get(o);
                if Some(z.id()) == p.map(|x| x.0 .0) {
                    // dbg!(&ele.0);
                    // dbg!(&p);
                    cursor.skip_subtree();
                    return Ok(Some(p.unwrap().1));
                } else if z.child_count() > 0 {
                    writeln!(f, "{}{}", "    ".repeat(*indent), z.kind())?;
                    *indent += 1;
                }
            } else {
                if z.child_count() == 0 {
                    let k = z.kind();
                    let c = &ele.1.source[z.byte_range()];
                    if k == c {
                        writeln!(f, "{}{}", "    ".repeat(*indent), z.kind())?;
                    } else {
                        writeln!(f, "{}{} : {}", "    ".repeat(*indent), k, c)?;
                    }
                } else {
                    *indent = indent.saturating_sub(1);
                }
            }
        }
    }
}

impl<'g> Display for PolyglotTreeSerializer<'g, ()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.global.root.clone();
        let aaa = self.global.raw_internal(&h).unwrap();
        let cursor = super::polyglot_zipper::PreOrder2::from(aaa.1.cst.as_ref().unwrap());
        let mut stack = vec![(aaa, cursor, 0usize)];
        let mut indent = 0;
        while let Some(s) = stack.pop() {
            let (ss, mut cursor, o) = s;
            if let Some(h) = self.local(f, ss, o, &mut cursor, &mut indent)? {
                stack.push((ss, cursor, o + 1));
                let ss = self.global.raw_internal(&h).unwrap();
                writeln!(f, "{}==== start of of {} ====", "    ".repeat(indent), ss.0)?;
                let cursor = super::polyglot_zipper::PreOrder2::from(ss.1.cst.as_ref().unwrap());
                stack.push((ss, cursor, 0));
            } else if !stack.is_empty() {
                writeln!(f, "{}==== end of {} =====", "    ".repeat(indent + 1), ss.0)?;
            }
        }
        Ok(())
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

    fn process_impl(&mut self, zip: Option<PolyglotZipper>) -> Result<&mut Self, ()> {
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
        todo!()
    }
}

impl PolygotProcessor for DUBuilder {
    fn process(&mut self, zip: PolyglotZipper) -> Result<&mut Self, ()> {
        self.process_impl(Some(zip))
    }
}
