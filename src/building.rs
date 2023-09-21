//! File that contains the main building functions
//! 
//! This file has been created to facilitate the addition of new languages to the polyglot AST.
//! 
//! To add a new langage to PolylgotAST : 
//! - add the language to the Language enum in the lib.rs and util.rs files
//! - add a new file in the building folder named name_of_the_langage.rs
//! - impl StuffPerLanguage for the new file

use crate::{Language, PolyglotTree, SourceFilePath};

pub(crate) mod java;
// mod javascript;
// mod javascript;
// mod python;
#[derive(Debug)]
pub struct PolyglotUse {
    aux: Aux,
    language: Language,
    position: usize,
}

#[derive(Debug)]
enum Aux {
    // For example: eval(source1)
    EvalSource {
        code: std::sync::Arc<str>,
    },

    // For example: eval(Source.newBuilder("python", file2).build());
    // where file2 is a path to a file containing python code
    EvalPath {
        path: SourceFilePath,
    },

    // For example: import polyglot
    // it often symbolizes the beginning of a polyglot block
    Import {
        // language: Language,
        // path: SourceFilePath,
    },
}

#[derive(Debug)]
pub struct PolygloteTreeHandle(usize);

impl PolyglotUse {
    /// Returns the kind of the polyglot use
    pub fn get_kind(&self) -> PolyglotKind {
        match self.aux {
            Aux::EvalSource { .. } => PolyglotKind::Eval,
            Aux::EvalPath { .. } => PolyglotKind::Eval,
            Aux::Import { .. } => PolyglotKind::Import,
        }
    }
}

impl crate::PolyStuff for PolyglotUse {
    // Returns the kind of the polyglot use
    fn kind(&self) -> self::PolyglotKind {
        self.kind()
    }

    // Returns the language of the polyglot use
    fn lang(&self) -> Language {
        self.language
    }

    // Returns the path of the polyglot use
    // useful only for EvalPath polyglot uses
    fn path(&self) -> Option<&std::path::Path> {
        match &self.aux {
            Aux::EvalSource { .. } => None,
            Aux::EvalPath { path, .. } => Some(path.as_ref()),
            Aux::Import { .. } => todo!(),
        }
    }

    // Returns the source of the polyglot use
    // useful only for EvalSource polyglot uses
    fn source(&self) -> Option<&std::sync::Arc<str>> {
        match &self.aux {
            Aux::EvalSource { code, .. } => Some(code),
            Aux::EvalPath { .. } => None,
            Aux::Import { .. } => todo!(),
        }
    }

    // Returns the position of the polyglot use
    fn position(&self) -> crate::context::TopoOrder {
        crate::context::TopoOrder(self.position)
    }
}

// Enum that represents a polyglot definition
pub enum PolyglotDef {
    ExportValue { name: String, value: String },
}
impl PolyglotDef {
    // Returns the kind of the polyglot definition
    pub fn get_kind(&self) -> PolyglotKind {
        match self {
            PolyglotDef::ExportValue { .. } => PolyglotKind::Export,
        }
    }
}

// Enum that represents a polyglot definition or use
pub(crate) enum PolyglotDefOrUse {
    Def(PolyglotDef),
    Use(PolyglotUse),
}

impl PolyglotDefOrUse {
    // Returns the kind of the polyglot definition or use
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

// 3 main kinds of polyglot elements
//todo
pub enum PolyglotKind {
    Eval,
    Import,
    Export,
}

// 
pub trait PolyglotBuilding {
    type Node;
    type Ctx;
    fn init(ctx: Self::Ctx) -> Self;
    fn compute(self) -> PolyglotTree;
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnaError {}

// Main trait for polyglot building
// it is implemented for each language
// each new language must implement this trait with this 5 methods
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

}

/// take inpiration from graal_utils.rs
struct BuildingContext {
    pwd: std::path::PathBuf,
    map_source: crate::SourceMap,
    map_file: crate::FileMap,
}
