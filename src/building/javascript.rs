use std::fs::File;
use std::io::Read;

struct Tree {
    pub language: JavaScript,
    pub map_source: SourceMap,
    pub map_file: FileMap,
}

impl PolyglotTree {
    fn get_polyglot_call(&self, node: Node) -> Option<&str> {
        let child = node.child(0)?;
        if node.kind().eq("call_expression") && child.kind().eq("member_expression") {
            return Some(self.node_to_code(child));
        }
        None
    }

    fn make_subtree(&self, node: &Node) -> Option<PolyglotTree> {
        let call_type = node.child(0)?.child(2)?; // function name
        let arg1 = node.child(1)?.child(1)?; // language
        let arg2 = node.child(1)?.child(3)?; // code

        // JavaScript uses a different function for evaluating raw code and files, so we have two cases
        match self.node_to_code(call_type) {
            "eval" => {
                // Arguments are positional, and always at the same spot
                let tmp_lang = util::strip_quotes(self.node_to_code(arg1));
                let tmp_code = util::strip_quotes(self.node_to_code(arg2));

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
                let tmp_lang = util::strip_quotes(self.node_to_code(arg1));

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

                let tmp_path = util::strip_quotes(self.node_to_code(arg2));

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
