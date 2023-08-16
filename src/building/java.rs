use std::fs::File;
use std::io::Read;

use crate::PolyglotTree;

use super::{BuildingContext, StuffPerLanguage, PolyglotUse, PolyglotBuilding, AnaError, PolyglotDef};

struct JavaBuilder<'ctx, 'src> {
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

    fn try_compute_polyglot_use(&self, node: &Self::Node<'_>) -> Option<Result<super::PolyglotUse,AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.node_to_code(&call);
        if s == "eval" {
            let r = if s=="" {
                PolyglotUse::Eval { path: todo!(), lang: todo!() }
            } else {
                PolyglotUse::EvalSource { source: todo!(), lang: todo!() }
            };
            Some(Ok(r))
        } else if s == "getMember" {
            let r = PolyglotUse::Import { path: todo!(), lang: todo!() };
            Some(Ok(r))
        } else {
            None
        }
    }

    fn try_compute_polyglot_def(&self, node: &Self::Node<'_>) -> Option<Result<super::PolyglotDef,AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.node_to_code(node);
        if s == "puMember" {
            let r = PolyglotDef::ExportValue { name: todo!(), value: todo!() };
            Some(Ok(r))
        } else {
            None
        }
    }
}



impl<'ctx,'src> JavaBuilder<'ctx,'src> {
    fn node_to_code(&self, node: &tree_sitter::Node<'_>) -> &'src str {
        &self.source[node.start_byte()..node.end_byte()]
    }
    fn get_polyglot_call<'n>(&self, node: &tree_sitter::Node<'n>) -> Option<tree_sitter::Node<'n>> {
        if node.kind().ne("method_invocation") {
            return None;
        }
        let child = node.child(2)?;
        if  child.kind().eq("identifier") {
            return Some(child);
        }
        None
    }

    fn compute_polyglot_use(
        &self,
        call_expr: &<JavaBuilder<'ctx, 'src> as PolyglotBuilding>::Node<'_>
    ) -> Option<PolyglotUse> {
        // Java uses positional arguments, so they will always be accessible with the same route.

        //if node.child(3) has 1 child => code, if node.child(3) has 2 children: 1st => language, 2nd => code
        let parameters = &call_expr.child(3)?;
        let parameter_count = parameters.child_count();

        if parameter_count == 1 {
            // NOTE match something like:
            // context.eval(source);
            // where context is a org.graalvm.polyglot.Context
            // where source is a org.graalvm.polyglot.Source
            let parameter = parameters.child(1)?;

            let t = parameter.kind();
            assert_eq!(t, "identifer");

            let name = self.node_to_code(&parameter);
            let name = name.to_string();

            Some(PolyglotUse::EvalVariable { name })

            // dbg!(name);
            // let a = {
            //     tree_sitter::Query::new(tree_sitter_java::language(), r#"
            //     (local_variable_declaration 
            //         type: "Source" | "org.graalvm.polyglot.Source" @variable.type
            //         declarator: (variable_declarator 
            //             name: (identifier @variable.name)
            //             value: (* @variable.value)))

            //     (local_variable_declaration  (arguments (identifier)))
            //     "#);
            //     todo!("the rest")
            // };

            // if let Ok(a) = a {
            //     return a;
            // }

            // let b = {
            //     todo!("with spoon")
            // };

            // // etc

            // todo!("use some robust static analysis, ie use existing lsp for Java, or Spoon, or Jdt, or tree-sitter query")
        } else if parameter_count == 5 {
            // NOTE match something like:
            // context.eval("python", "print(42)");
            // or:
            // context.eval("python", source);
            // where context is a org.graalvm.polyglot.Context
            // where source is a string or a file (or anything that can be turned into a string ?)
            
            //getting language and code
            let language = parameters.child(1)?;
            let code = parameters.child(3)?;
            use crate::util;
            let s = util::strip_quotes(self.node_to_code(&language));

            let new_lang = match util::language_string_to_enum(&s) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}",);
                    return None;
                }
            };
            let new_code = util::strip_quotes(self.node_to_code(&code));
            println!("{}", new_code);
            todo!()
        } else {
            None
        }
    }
}
