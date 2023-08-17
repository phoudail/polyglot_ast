use std::fs::File;
use std::io::Read;

use tree_sitter::Node;

use crate::{tree_sitter_utils::TreeSitterCST, PolyglotTree};

use super::{
    AnaError, BuildingContext, PolyglotBuilding, PolyglotDef, PolyglotUse, StuffPerLanguage,
};

struct JavaBuilder<'tree, 'text> {
    payload: TreeSitterCST<'tree, 'text>,
    // temporary stuff
}
impl<'tree, 'text> super::PolyglotBuilding for JavaBuilder<'tree, 'text> {
    type Node<'a> = tree_sitter::Node<'a>;
    type Ctx = TreeSitterCST<'tree, 'text>;

    fn init(payload: TreeSitterCST<'tree, 'text>) -> Self {
        Self { payload }
    }

    fn compute(self) -> PolyglotTree {
        todo!()
    }
}
impl<'tree, 'text> StuffPerLanguage for JavaBuilder<'tree, 'text> {
    fn find_polyglot_uses(&self) -> Vec<super::PolyglotUse> {
        todo!()
    }

    fn find_polyglot_exports(&self) -> Vec<super::PolyglotDef> {
        todo!()
    }

    fn try_compute_polyglot_use(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<super::PolyglotUse, AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.payload.node_to_code(&call);
        if s == "eval" {
            let r = if s == "" {
                PolyglotUse::Eval {
                    path: todo!(),
                    lang: todo!(),
                }
            } else {
                let lang = self
                    .payload
                    .node_to_code(&node.child(3).unwrap().child(1).unwrap());
                let lang = crate::util::strip_quotes(lang);
                dbg!(&lang);
                PolyglotUse::EvalSource {
                    source: self
                        .payload
                        .node_to_code(&node.child(3).unwrap().child(3).unwrap())
                        .to_string(),
                    lang: crate::util::language_string_to_enum(&lang).unwrap(),
                }
            };
            Some(Ok(r))
        } else if s == "getMember" {
            let r = PolyglotUse::Import {
                path: todo!(),
                lang: todo!(),
            };
            Some(Ok(r))
        } else {
            None
        }
    }

    fn try_compute_polyglot_def(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<super::PolyglotDef, AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.payload.node_to_code(node);
        if s == "putMember" {
            let r = PolyglotDef::ExportValue {
                name: todo!(),
                value: todo!(),
            };
            Some(Ok(r))
        } else {
            None
        }
    }
}

impl<'tree, 'text> JavaBuilder<'tree, 'text> {
    fn get_polyglot_call<'n>(&self, node: &tree_sitter::Node<'n>) -> Option<tree_sitter::Node<'n>> {
        if node.kind().ne("method_invocation") {
            return None;
        }
        let child = node.child(2)?;
        if child.kind().eq("identifier") {
            return Some(child);
        }
        None
    }

    fn compute_polyglot_use(
        &self,
        call_expr: &<JavaBuilder<'tree, 'text> as PolyglotBuilding>::Node<'_>,
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

            let name = self.payload.node_to_code(&parameter);
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
            let s = util::strip_quotes(self.payload.node_to_code(&language));

            let new_lang = match util::language_string_to_enum(&s) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}",);
                    return None;
                }
            };
            let new_code = util::strip_quotes(self.payload.node_to_code(&code));
            println!("{}", new_code);
            todo!()
        } else {
            None
        }
    }
}

mod use_solver {
    use std::path::PathBuf;
    struct Reference(String);
    enum TwoTransitions<T0, T1> {
        T0(T0),
        T1(T1),
    }
    enum ThreeTransitions<T0, T1, T2> {
        T0(T0),
        T1(T1),
        T2(T2),
    }
    struct SolvingError;
    enum Nev {}
    struct NoSource<S> {
        source: S,
    }
    impl NoSource<Reference> {
        fn solve<'tree>(
            self,
            refanalysis: &'tree impl ReferenceAnalysis,
        ) -> Result<NoSource<Node<'tree>>, SolvingError> {
            let source = refanalysis.solve(&self.source)?;
            Ok(NoSource { source })
        }
    }
    impl NoSource<Node<'_>> {
        fn solve<'tree>(
            self,
            refanalysis: &'tree impl ReferenceAnalysis,
        ) -> Result<
            TwoTransitions<NoSource<Reference>, PolygloteSource<Node<'tree>, Node<'tree>>>,
            SolvingError,
        > {
            todo!()
        }
    }

    struct PolygloteSource<L, C> {
        language: L,
        code: C,
    }
    type CodeElement = (usize, usize);
    type Node<'tree> = tree_sitter::Node<'tree>;
    trait ReferenceAnalysis {
        fn solve<'tree>(&'tree self, reference: &Reference) -> Result<Node<'tree>, SolvingError>;
    }
    impl<C> PolygloteSource<Reference, C> {
        fn solve<'tree>(
            self,
            refanalysis: &'tree impl ReferenceAnalysis,
        ) -> Result<PolygloteSource<Node<'tree>, C>, SolvingError> {
            let language = refanalysis.solve(&self.language)?;
            Ok(PolygloteSource {
                language,
                code: self.code,
            })
        }
    }
    impl<C> PolygloteSource<Node<'_>, C> {
        fn solve(
            self,
        ) -> Result<
            TwoTransitions<PolygloteSource<Reference, C>, PolygloteSource<crate::Language, C>>,
            SolvingError,
        > {
            todo!()
        }
    }
    impl PolygloteSource<crate::Language, Reference> {
        fn solve<'tree>(
            self,
            refanalysis: &'tree impl ReferenceAnalysis,
        ) -> Result<PolygloteSource<crate::Language, Node<'tree>>, SolvingError> {
            let code = refanalysis.solve(&self.code)?;
            Ok(PolygloteSource {
                language: self.language,
                code,
            })
        }
    }
    impl PolygloteSource<crate::Language, Node<'_>> {
        fn solve<'tree>(
            self,
        ) -> Result<
            PolygloteSource<
                crate::Language,
                ThreeTransitions<
                    PolygloteSource<crate::Language, Reference>,
                    PolygloteSource<crate::Language, PathBuf>,
                    PolygloteSource<crate::Language, String>,
                >,
            >,
            SolvingError,
        > {
            todo!()
        }
    }
    impl<C> PolygloteSource<crate::Language, C> {
        fn lang(&self) -> &crate::Language {
            &self.language
        }
    }
    impl<L> PolygloteSource<L, PathBuf> {
        fn code(&self) -> &PathBuf {
            &self.code
        }
    }
    impl<L> PolygloteSource<L, String> {
        fn code(&self) -> &String {
            &self.code
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, fmt::Display};

    use crate::{
        building::{BuildingContext, PolyglotBuilding, StuffPerLanguage},
        tree_sitter_utils::TreeSitterCST,
        PolyglotTree,
    };

    use super::JavaBuilder;

    fn main_wrap(main_content: impl Display) -> String {
        format!(
            "{}{}{}",
            r#"import java.io.File;

        import javax.naming.Context;
        import javax.xml.transform.Source;
        
        import org.graalvm.polyglot.Context;
        import org.graalvm.polyglot.Value;
        
        public class JavaTest2 {
            public static void main(String[] args) {"#,
            main_content.to_string(),
            r#"}}"#
        )
    }
    #[test]
    fn direct() {
        let main_content = r#"
        Context cx = Context.create();
        context.eval("python", "print('hello')");
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
        let class = tree.root_node().child(5).unwrap();
        let meth = class.child(3).unwrap().child(1).unwrap();
        let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());

        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(r#use);
    }
    #[test]
    fn direct2() {
        let main_content = r#"
        Context cx = Context.create();
        context.eval(Source.newBuilder("python", new File("TestSamples/pyprint.py")).build());
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
        let class = tree.root_node().child(5).unwrap();
        let meth = class.child(3).unwrap().child(1).unwrap();
        let meth_body = &meth.child(4).unwrap();
        dbg!(meth_body.to_sexp());
        let poly_eval = meth_body.child(2).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());

        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(r#use);
    }
    #[test]
    fn indirect() {
        let main_content = r#"
        Context cx = Context.create();

        Builder builder = Source.newBuilder("python", new File("TestSamples/pyprint.py"));
        context.eval(builder.build());
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
    }
    #[test]
    fn indirect1() {
        let main_content = r#"
        Context cx = Context.create();

        Source source1 = Source.newBuilder("python", new File("TestSamples/pyprint.py")).build();
        context.eval(source1);
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
    }
    #[test]
    fn indirect2() {
        let main_content = r#"
        Context cx = Context.create();

        File file1 = new File("TestSamples/pyprint.py");
        Source source1 = Source.newBuilder("python", file1).build();
        context.eval(source1);
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
    }
}
