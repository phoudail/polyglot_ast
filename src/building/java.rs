use std::fs::File;
use std::io::Read;

use crate::PolyglotTree;

use super::{BuildingContext, StuffPerLanguage, PolyglotUse};

struct JavaBuilder<'ctx, 'str> {
    ctx: &'ctx mut BuildingContext,
    source: &'src str,
}
impl<'ctx,'str> super::PolyglotBuilding for JavaBuilder<'ctx,'str> {
    type Node<'a> = tree_sitter::Node<'a>;
    type Ctx = BuildingContext;

    fn init(ctx: BuildingContext) -> Self {
        todo!()
    }

    fn compute(self, node: &Self::Node<'_>) -> PolyglotTree {
        todo!()
    }
}
impl<'ctx,'str> StuffPerLanguage for JavaBuilder<'ctx,'str> {
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
            let r = if s=="" {
                PolyglotUse::Eval { path: (), lang: () }
            } else {
                PolyglotUse::EvalSource { source: (), lang: () }
            }
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
impl<'ctx,'str> JavaBuilder<'ctx,'str> {
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
        node: &Self::Node<'_>
    ) -> Option<PolyglotTree> {
        // Java uses positional arguments, so they will always be accessible with the same route.

        //if node.child(3) has 1 child => code, if node.child(3) has 2 children: 1st => language, 2nd => code
        let nb_child: usize = node.child(3)?.child_count();

        if nb_child == 3 {
            let variable_name = node.child(3)?.child(1)?;

            let a = {
                tree_sitter::Query::new(tree_sitter_java::language(), r#"
                (local_variable_declaration 
                    type: "Source" | "org.graalvm.polyglot.Source" @variable.type
                    declarator: (variable_declarator 
                        name: (identifier @variable.name)
                        value: (* @variable.value)))

                (local_variable_declaration  (arguments (identifier)))
                "#);
                todo!("the rest")
            };

            if let Ok(a) = a {
                return a;
            }

            let b = {
                todo!("with spoon")
            };

            // etc

            todo!("use some robust static analysis, ie use existing lsp for Java, or Spoon, or Jdt, or tree-sitter query")
        } else if nb_child == 5 {
            //getting language and code
            let language = node.child(3)?.child(1)?;
            let code = node.child(3)?.child(3)?;
            use crate::util;
            let s = util::strip_quotes(self.node_to_code(language));

            let new_lang = match util::language_string_to_enum(&s) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}",);
                    return None;
                }
            };
            let new_code = util::strip_quotes(self.node_to_code(code));
            println!("{}", new_code);
            //return CST
            return Self::from_directory(new_code, new_lang, self.working_dir.clone());
        }
        //return none if no child or more than 2
        return None;
    }
    fn make_subtree(
        &self,
        node: &Node,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
    ) -> Option<PolyglotTree> {
        // Java uses positional arguments, so they will always be accessible with the same route.

        //if node.child(3) has 1 child => code, if node.child(3) has 2 children: 1st => language, 2nd => code
        let nb_child: usize = node.child(3)?.child_count();

        if nb_child == 3 {
            //getting code
            let code = node.child(3)?.child(1)?;

            //getting source
            let source = map_source.get(self.node_to_code(code));

            //guetting language
            let tmp_language = &source.expect("").0;

            //getting file
            let file: Option<&String> = map_file.get(&source.expect("source not found").1);

            //open the file of the path
            let mut openfile = File::open(file.expect("file not found")).unwrap();

            //putting file code in a string
            let mut code = String::new();
            openfile.read_to_string(&mut code).unwrap();

            //return CST
            return Self::from_directory(code, tmp_language.clone(), self.working_dir.clone());
        } else if nb_child == 5 {
            //getting language and code
            let language = node.child(3)?.child(1)?;
            let code = node.child(3)?.child(3)?;
            use crate::util;
            let s = util::strip_quotes(self.node_to_code(language));

            let new_lang = match util::language_string_to_enum(&s) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}",);
                    return None;
                }
            };
            let new_code = util::strip_quotes(self.node_to_code(code));
            println!("{}", new_code);
            //return CST
            return Self::from_directory(new_code, new_lang, self.working_dir.clone());
        }
        //return none if no child or more than 2
        return None;
    }
}
