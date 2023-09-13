use crate::{Language, PolyglotTree, SourceFilePath};

pub(crate) mod java;
// mod javascript;
// mod javascript;
// mod python;
#[derive(Debug)]
pub enum PolyglotUse {
    EvalSource{
        language: Language,
        code: std::sync::Arc<str>,
    },
    EvalPath{
        language: Language,
        path: SourceFilePath,
    },
    Import{
        // language: Language,
        // path: SourceFilePath,
    },
}

#[derive(Debug)]
pub struct PolygloteTreeHandle(usize);

impl PolyglotUse {
    pub fn get_kind(&self) -> PolyglotKind {
        match self {
            PolyglotUse::EvalSource { .. } => PolyglotKind::Eval,
            PolyglotUse::EvalPath { .. } => PolyglotKind::Eval,
            PolyglotUse::Import { .. } => PolyglotKind::Import,
        }
    }
}


impl crate::PolyStuff for PolyglotUse {
    fn kind(&self) -> self::PolyglotKind {
        self.kind()
    }

    fn lang(&self) -> Language {
        match self {
            PolyglotUse::EvalSource { language, .. } => *language,
            PolyglotUse::EvalPath { language, .. } => *language,
            PolyglotUse::Import { .. } => todo!(),
        }
    }

    fn path(&self) -> Option<&std::path::Path> {
        match self {
            PolyglotUse::EvalSource { .. } => None,
            PolyglotUse::EvalPath { path, .. } => Some(path.as_ref()),
            PolyglotUse::Import { .. } => todo!(),
        }
    }

    fn source(&self) -> Option<&std::sync::Arc<str>> {
        match self {
            PolyglotUse::EvalSource { code, .. } => Some(code),
            PolyglotUse::EvalPath { .. } => None,
            PolyglotUse::Import { .. } => todo!(),
        }
    }
}

pub enum PolyglotDef {
    ExportValue { name: String, value: String },
}
impl PolyglotDef {
    pub fn get_kind(&self) -> PolyglotKind {
        match self {
            PolyglotDef::ExportValue { .. } => PolyglotKind::Export,
        }
    }
}

pub(crate) enum PolyglotDefOrUse {
    Def(PolyglotDef),
    Use(PolyglotUse),
}

impl PolyglotDefOrUse {
    pub fn get_kind(&self) -> PolyglotKind {
        match self {
            PolyglotDefOrUse::Def(def) => def.get_kind(),
            PolyglotDefOrUse::Use(use_) => use_.get_kind(),
        }
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
    type Node;
    type Ctx;
    fn init(ctx: Self::Ctx) -> Self;
    fn compute(self) -> PolyglotTree;
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnaError {}

pub(crate) trait StuffPerLanguage: PolyglotBuilding {
    type UnsolvedUse;
    fn find_polyglot_uses(&self) -> Vec<Self::UnsolvedUse>;
    fn find_polyglot_exports(&self) -> Vec<PolyglotDef>;

    fn try_compute_polyglot_element(
        &self,
        node: &Self::Node,
    ) -> Option<Result<PolyglotDefOrUse, AnaError>> {
        if let Some(def) = self.try_compute_polyglot_def(node) {
            Some(def.map(|def| PolyglotDefOrUse::Def(def)))
        } else {
            self.try_compute_polyglot_use(node).map(|uze| {
                uze.map(|uze| {
                    // uze.into()
                    todo!()
                })
            })
        }
    }
    fn try_compute_polyglot_use(
        &self,
        node: &Self::Node,
    ) -> Option<Result<Self::UnsolvedUse, AnaError>>;
    fn try_compute_polyglot_def(&self, node: &Self::Node) -> Option<Result<PolyglotDef, AnaError>>;

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
