use crate::{Language, PolyglotTree, SourceFilePath};

mod java;
// mod javascript;
// mod javascript;
// mod python;

#[derive(Debug)]
pub enum PolyglotUse {
    // partially solved
    EvalVariable {
        name: String,
    },
    // can be evaluated
    EvalSource {
        source: String,
        lang: Language,
    },
    // can be evaluated if referenced file can be evaluated
    Eval {
        path: SourceFilePath,
        lang: Language,
    },
    // can be evaluated if referenced file can be evaluated
    Import {
        path: SourceFilePath,
        lang: Language,
    },
}
// TODO rename into PolygloteUse, the old  PolygloteUse becomes UnSolvedPolyglotUse
pub enum SolvedPolyglotUse {
    Eval(PolygloteTreeHandle),
    Import(PolygloteTreeHandle),
}

pub struct PolygloteTreeHandle(usize);

impl PolyglotUse {
    pub fn get_kind(&self) -> PolyglotKind {
        todo!()
    }
}

pub enum PolyglotDef {
    ExportValue { name: String, value: String },
}
impl PolyglotDef {
    pub fn get_kind(&self) -> PolyglotKind {
        todo!()
    }
}

enum PolyglotDefOrUse {
    Def(PolyglotDef),
    Use(PolyglotUse),
}

impl PolyglotDefOrUse {
    pub fn get_kind(&self) -> PolyglotKind {
        todo!()
    }
}

impl From<PolyglotUse> for PolyglotDefOrUse {
    fn from(value: PolyglotUse) -> Self {
        PolyglotDefOrUse::Use(value)
    }
}

pub enum PolyglotKind {
    Eval,
    Import,
    Export,
}

pub trait PolyglotBuilding {
    type Node<'a>;
    type Ctx;
    fn init(ctx: Self::Ctx) -> Self;
    fn compute(self) -> PolyglotTree;
}

#[derive(Debug)]
pub enum AnaError {}

trait StuffPerLanguage: PolyglotBuilding {
    fn find_polyglot_uses(&self) -> Vec<PolyglotUse>;
    fn find_polyglot_exports(&self) -> Vec<PolyglotDef>;

    fn try_compute_polyglot_element(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<PolyglotDefOrUse, AnaError>> {
        if let Some(def) = self.try_compute_polyglot_def(node) {
            Some(def.map(|def| PolyglotDefOrUse::Def(def)))
        } else {
            self.try_compute_polyglot_use(node)
                .map(|uze| uze.map(|uze| uze.into()))
        }
    }
    fn try_compute_polyglot_use(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<PolyglotUse, AnaError>>;
    fn try_compute_polyglot_def(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<PolyglotDef, AnaError>>;

    // fn is_polyglot_eval_call(&self, node: Node) -> bool {
    //     match self.language {
    //         Language::Python => {
    //             matches!(self.get_polyglot_call_python(node), Some("polyglot.eval"))
    //         }
    //         Language::JavaScript => matches!(
    //             self.get_polyglot_call_js(node),
    //             Some("Polyglot.eval") | Some("Polyglot.evalFile")
    //         ),
    //         Language::Java => matches!(self.get_polyglot_call_java(node), Some("eval")),
    //     }
    // }

    // fn is_polyglot_import_call(&self, node: Node) -> bool {
    //     match self.language {
    //         Language::Python => matches!(
    //             self.get_polyglot_call_python(node),
    //             Some("polyglot.import_value")
    //         ),
    //         Language::JavaScript => {
    //             matches!(self.get_polyglot_call_js(node), Some("Polyglot.import"))
    //         }
    //         Language::Java => matches!(self.get_polyglot_call_java(node), Some("getMember")),
    //     }
    // }

    // fn is_polyglot_export_call(&self, node: Node) -> bool {
    //     match self.language {
    //         Language::Python => matches!(
    //             self.get_polyglot_call_python(node),
    //             Some("polyglot.export_value")
    //         ),
    //         Language::JavaScript => {
    //             matches!(self.get_polyglot_call_js(node), Some("Polyglot.export"))
    //         }
    //         Language::Java => matches!(self.get_polyglot_call_java(node), Some("putMember")),
    //     }
    // }
}

/// take inpiration from grall_utils.rs
struct BuildingContext {
    pwd: std::path::PathBuf,
    map_source: crate::SourceMap,
    map_file: crate::FileMap,
}
