use std::collections::HashMap;
use std::path::PathBuf;

use crate::{Language, PolyglotTree};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Handle {
    path: SrcOrPath,
    lang: Language,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SrcOrPath {
    Source(String), // TODO use Arc<str>
    Path(PathBuf),
}

struct GlobalContext {
    pwd: PathBuf,
    root: Handle,
    sources: HashMap<Handle, PolyglotTree>, // TODO refactoring into separarted struct with a vec backend and multiple indexes ((path|source)+lang?pwd, usize)
}
