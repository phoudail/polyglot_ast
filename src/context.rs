use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use crate::{Language, PolyglotTree, RawParseResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handle {
    path: SrcOrPath,
    lang: Language,
} // TODO rename into SourceUniqIdentifier

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "{:#}", self)
        } else {
            match &self.path {
                SrcOrPath::Source(x) => write!(
                    f,
                    "{:?} -> {}",
                    self.lang,
                    &x[..x.len().min(20)].replace("\n", "\\n")
                ),
                SrcOrPath::Path(x) => write!(f, "{:?} -> {:?}", self.lang, x),
            }
        }
    }
}

impl From<(Language, &Path)> for Handle {
    fn from((lang, path): (Language, &Path)) -> Self {
        Handle {
            path: SrcOrPath::Path(path.into()),
            lang,
        }
    }
}
impl From<(Language, &std::sync::Arc<str>)> for Handle {
    fn from((lang, source): (Language, &std::sync::Arc<str>)) -> Self {
        Handle {
            path: SrcOrPath::Source(source.clone()),
            lang,
        }
    }
}
impl From<&dyn crate::PolyStuff> for Handle {
    fn from(value: &dyn crate::PolyStuff) -> Self {
        if let Some(source) = value.source() {
            (value.lang(), source).into()
        } else if let Some(path) = value.path() {
            (value.lang(), path).into()
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternalHandle(pub(crate) usize); // TODO rename into Handle

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SrcOrPath {
    Source(std::sync::Arc<str>),
    Path(PathBuf),
}
#[derive(Debug)]
pub struct TopoOrder(pub(crate) usize);

/// TODO short why
///
/// TODO NOTE: relative to previous way of doing things
///
/// TODO short example
///
/// TODO link to examples/*
pub struct GlobalContext {
    pub(crate) pwd: PathBuf,
    pub(crate) root: InternalHandle,
    pub(crate) sources: Vec<(Handle, RawParseResult, Vec<(TopoOrder, InternalHandle)>)>, // TODO refactoring into separarted struct with a vec backend and multiple indexes ((path|source)+lang?pwd, usize)
    pub(crate) queue: Vec<InternalHandle>,
}

pub(crate) type AtributedEle = (Handle, RawParseResult, Vec<(TopoOrder, InternalHandle)>);

impl GlobalContext {
    pub fn new(path: PathBuf, root: RawParseResult) -> Self {
        let root_dir = path.parent().unwrap().into();
        Self::with_root_dir((path, root), root_dir)
    }
    pub fn with_root_dir((path, root): (PathBuf, RawParseResult), root_dir: PathBuf) -> Self {
        GlobalContext {
            pwd: root_dir,
            root: InternalHandle(0),
            sources: vec![(
                Handle {
                    path: SrcOrPath::Path(path.clone()),
                    lang: root.language,
                },
                root,
                Default::default(),
            )],
            queue: vec![InternalHandle(0)],
        }
    }

    pub fn root_tree(&self) -> Option<PolyglotTree> {
        self.sources
            .get(self.root.0)
            .and_then(|(_, tree, _)| tree.try_into().ok())
    }

    pub fn get(&self, handle: &Handle) -> Option<&PolyglotTree> {
        // self.sources
        //     .iter()
        //     .find(|(h, _, _)| h == handle)
        //     .map(|(_, tree, _)| tree)
        todo!()
    }
    pub fn resolve_internal(&self, handle: &Handle) -> Option<InternalHandle> {
        self.sources
            .iter()
            .position(|x| &x.0 == handle)
            .map(|x| InternalHandle(x))
    }

    pub fn raw_internal<'a>(&'a self, handle: &InternalHandle) -> Option<&'a AtributedEle> {
        self.sources.get(handle.0)
    }

    pub fn raw(&self, handle: &InternalHandle) -> Option<&RawParseResult> {
        self.sources.get(handle.0).map(|(_, tree, _)| tree)
    }
    pub fn add_polyglot_tree(&mut self, handle: Handle, tree: RawParseResult) -> InternalHandle {
        let h = InternalHandle(self.sources.len());
        self.sources
            .push((handle.clone(), tree, Default::default()));
        self.queue.push(h.clone());
        h
    }

    pub fn set_solved(&mut self, root: InternalHandle) {
        self.queue.retain(|h| h != &root);
    }

    pub fn next_partial_polyglot_tree(&mut self) -> Option<(InternalHandle, RawParseResult)> {
        let handle = self.queue.pop()?;
        let (_, tree, _) = &self.sources[handle.0];
        Some((handle.clone(), tree.clone()))
    }

    pub(crate) fn add_use(&mut self, h: InternalHandle, pos: TopoOrder, h2: InternalHandle) {
        self.sources[h.0].2.push((pos, h2));
    }
}
