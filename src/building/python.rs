use std::fs::File;
use std::io::Read;

struct Tree {
    pub language: Python,
    pub map_source: SourceMap,
    pub map_file: FileMap,
}

impl PolyglotTree {
    fn get_polyglot_call(&self, node: Node) -> Option<&str> {
        let child = node.child(0)?;
        if node.kind().eq("call") && child.kind().eq("attribute") {
            return Some(self.node_to_code(child));
        }
        None
    }

    fn make_subtree(&self, node: &Node) -> Option<PolyglotTree> {
        let arg1 = node.child(1)?.child(1)?.child(0)?;
        let arg2 = node.child(1)?.child(3)?.child(0)?;

        let mut new_code: Option<String> = None;
        let mut new_lang: Option<String> = None;
        let mut path: Option<PathBuf> = None;

        // Python polyglot calls use a single function and differentiate by argument names, which are mandatory.
        // We need to check both arguments for each possible case, and then check again at the end we have enough information.
        match self.node_to_code(arg1) {
            "path" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                path = Some(self.working_dir.clone());
                let new_path = match PathBuf::from_str(tmp.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                };
                path = path.map(|mut p| {
                    p.push(new_path);
                    p
                });
            }

            "language" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                new_code = Some(String::from(tmp.as_str()));
            }
            other => {
                eprintln!(
                    "Warning: unable to handle polyglot call argument {other} at position {}",
                    arg1.start_position()
                );
                return None;
            }
        }

        match self.node_to_code(arg2) {
            "path" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));
                path = Some(self.working_dir.clone());
                let new_path = match PathBuf::from_str(tmp.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                };
                path = path.map(|mut p| {
                    p.push(new_path);
                    p
                });
            }

            "language" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));

                new_code = Some(String::from(tmp.as_str()));
            }

            other => {
                eprintln!(
                    "Warning: unable to handle polyglot call argument {other} at position {}",
                    arg2.start_position()
                );
                return None;
            }
        }

        // We convert the language, if there was one
        let new_lang = match new_lang {
            Some(s) => match util::language_string_to_enum(s.as_str()) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}");
                    return None;
                }
            },
            None => {
                eprintln!(
                    "Warning: no language argument provided for polyglot call at position {}",
                    node.start_position()
                );
                return None;
            }
        };

        let subtree = match new_code {
            Some(c) => Self::from_directory(c, new_lang, self.working_dir.clone())?,
            None => Self::from_path(
                // No raw code, check for a path
                match path {
                    Some(p) => p,
                    None => {
                        // No path either -> we cant build the tree
                        eprintln!("Warning:: no path or string argument provided to Python polyglot call at position {}", node.start_position());
                        return None;
                    }
                },
                new_lang,
            )?,
        };
        Some(subtree)
    }
}
