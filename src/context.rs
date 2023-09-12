use std::collections::HashMap;
use std::path::PathBuf;

use crate::{Language, PolyglotTree};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Handle {
    path: SrcOrPath,
    lang: Language,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InternalHandle(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SrcOrPath {
    Source(std::sync::Arc<str>),
    Path(PathBuf),
}
struct TopoOrder(usize);
struct GlobalContext {
    pwd: PathBuf,
    root: InternalHandle,
    sources: Vec<(Handle, PolyglotTree, Vec<(TopoOrder, Handle)>)>, // TODO refactoring into separarted struct with a vec backend and multiple indexes ((path|source)+lang?pwd, usize)
    queue: Vec<InternalHandle>,
}
