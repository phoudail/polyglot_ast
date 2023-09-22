use std::{ops::Deref, path::PathBuf};

use crate::{context::GlobalContext, Language};

pub struct PolyglotAstBuilder<T = ()> {
    state: T,
}

impl PolyglotAstBuilder<()> {
    pub fn set_entry_point(path: impl Into<PathBuf>) -> PolyglotAstBuilder<PathBuf> {
        PolyglotAstBuilder { state: path.into() }
    }
}

impl PolyglotAstBuilder<PathBuf> {
    pub fn set_entry_lang(self, language: Language) -> PolyglotAstBuilder<(PathBuf, Language)> {
        PolyglotAstBuilder {
            state: (self.state, language),
        }
    }
}

#[derive(Debug)]
pub enum BuildingError {}

impl PolyglotAstBuilder<(PathBuf, Language)> {
    pub fn build(self) -> Result<GlobalContext, BuildingError> {
        let path = self.state.0;
        let text = std::fs::read_to_string(&path).unwrap();
        let language = self.state.1;
        let root = crate::parse(text.into(), language);
        let mut global = GlobalContext::new(path, root);
        while let Some((h, partial)) = global.next_partial_polyglot_tree() {
            dbg!(&h);
            if let Some(v) = partial.compute_polyglot_stuff() {
                dbg!(&v);
                for x in v {
                    dbg!(&x);
                    // TODO x.get_path() or get_code() and get_lang()
                    let lang = x.lang();
                    if global.resolve_internal(&x.deref().into()).is_none() {
                        dbg!(&x);
                        if let Some(code) = x.source() {
                            // snap.analysis.polyglot_tree(id, lang)
                            //todo!("snap.analysis.polyglot_tree() but without file id but an Arc<str> ie. code variable")
                            let pos = x.position();
                            let parsed = crate::parse(code.clone(), lang);
                            let handle = (lang, code).into();
                            let h2 = global.add_polyglot_tree(handle, parsed);
                            global.add_use(h, pos, h2);
                        } else {
                            // TODO
                            // let path = x.path().unwrap();
                            // let url = lsp_types::Url::from_file_path(path).unwrap();
                            // let id = from_proto::file_id(&snap, &url)?;
                            // let parsed = snap.analysis.polyglot_tree(id, lang).unwrap();
                            // let handle = (lang,path).into();
                            // global.add_polyglot_tree(handle,parsed);
                        };
                    } else {
                        // TODO
                    };
                }
                global.set_solved(h);
            };
        }
        Ok(global)
    }
}
