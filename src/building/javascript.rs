use std::{fs::File, path::PathBuf};
use std::io::Read;

use tree_sitter::Node;

use crate::{PolyglotTree, SourceMap, FileMap, util};

use super::{BuildingContext, StuffPerLanguage, PolyglotUse, PolyglotBuilding};

struct JavaScriptBuilder<'ctx, 'str> {
    ctx: &'ctx mut BuildingContext,
    source: &'src str,
}
impl<'ctx,'str> super::PolyglotBuilding for JavaScriptBuilder<'ctx,'str> {
    type Node<'a> = tree_sitter::Node<'a>;
    type Ctx = BuildingContext;

    fn init(ctx: BuildingContext) -> Self {
        todo!()
    }

    fn compute(self, node: &Self::Node<'_>) -> PolyglotTree {
        todo!()
    }
}

impl<'ctx,'str> StuffPerLanguage for JavaScriptBuilder<'ctx,'str> {
    fn find_polyglot_uses(&self) -> Vec<super::PolyglotUse> {
        todo!()
    }

    fn find_polyglot_exports(&self) -> Vec<super::PolyglotDef> {
        todo!()
    }

    fn try_compute_polyglot_use(&self, node: &Self::Node<'_>) -> Option<super::PolyglotUse> {
        let call = self.get_polyglot_call(node)?;
        let s = self.node_to_code(node);
        if s == "eval" {
            let r = 
                if s=="" {
                    PolyglotUse::Eval { path: (), lang: () }
                } else {
                    PolyglotUse::EvalSource { source: (), lang: () }
                };   
            Some(r)
        } else if s == "getMember" {
            let r = PolyglotUse::Import { path: (), lang: () };
            Some(r)
        } else {
            None
        }
    }

    fn try_compute_polyglot_def(&self, node: &Self::Node<'_>) -> Option<super::PolyglotDef> {
        todo!()
    }
}

impl<'ctx,'str> JavaScriptBuilder<'ctx,'str> {
    fn node_to_code(&self, node: &tree_sitter::Node<'_>) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }
    fn get_polyglot_call(&self, node: &tree_sitter::Node<'_>) -> Option<tree_sitter::Node<'_>> {
        let child = node.child(2)?;
        if node.kind().eq("method_invocation") && child.kind().eq("identifier") {
            return Some(child);
        }
        None
    }
    fn compute_polyglot_use(
        &self,
        node: &<JavaScriptBuilder<'ctx, 'str> as PolyglotBuilding>::Node<'_>,
    ) -> Option<PolyglotTree> {


    }


    fn make_subtree(&self, node: &Node) -> Option<PolyglotTree> {
        let call_type = node.child(0)?.child(2)?; // function name
        let arg1 = node.child(1)?.child(1)?; // language
        let arg2 = node.child(1)?.child(3)?; // code

        // JavaScript uses a different function for evaluating raw code and files, so we have two cases
        match self.node_to_code(&call_type) {
            "eval" => {
                // Arguments are positional, and always at the same spot
                let tmp_lang = util::strip_quotes(self.node_to_code(&arg1));
                let tmp_code = util::strip_quotes(self.node_to_code(&arg2));

                let new_lang = match util::language_string_to_enum(tmp_lang.as_str()) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!(
                            "Could not convert argument {} to language due to error: {e}",
                            tmp_lang.as_str()
                        );
                        return None;
                    }
                };

                let new_code = String::from(tmp_code.as_str());
                Self::from_directory(new_code, new_lang, self.working_dir.clone())
            }

            "evalFile" => {
                let tmp_lang = util::strip_quotes(self.node_to_code(&arg1));

                let new_lang = match util::language_string_to_enum(tmp_lang.as_str()) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!(
                            "Could not convert argument {} to language due to error: {e}",
                            tmp_lang.as_str()
                        );
                        return None;
                    }
                };

                let tmp_path = util::strip_quotes(self.node_to_code(&arg2));

                let mut path = self.working_dir.clone();

                let new_path = match PathBuf::from_str(tmp_path.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp_path.as_str()
                        );
                        return None;
                    }
                };

                path.push(new_path);

                Self::from_path(path, new_lang)
            }

            other => {
                eprintln!(
                    "Warning: unable to identify polyglot function call {other} at position {}",
                    node.start_position()
                );
                None
            }
        }
    }
}