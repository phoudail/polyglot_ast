use std::fs::File;
use std::io::Read;

use tree_sitter::Node;

use crate::{Language, tree_sitter_utils::TreeSitterCST, PolyglotTree};

//use super::EvalSource;

use super::{
    AnaError, BuildingContext, PolyglotBuilding, PolyglotDef, PolyglotUse, StuffPerLanguage,
    UnSolvedPolyglotUse,
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
        todo!();
        // return PolyglotTree {
        //     tree: self.payload.cst,
        //     code: self.payload.code,
        //     working_dir: self.payload.working_dir,
        //     language: crate::Language::Java,
        //     node_to_subtrees_map: HashMap::new(),
        // };
    }
}

trait Visit {
    fn visit(&self, node: &PolyglotTree) -> Vec<Node>;
    fn display(&self, node: &PolyglotTree);
}

struct PreOrder<'tree> {
    cursor: tree_sitter::TreeCursor<'tree>,
    state: VisitState,
}
#[derive(PartialEq, Eq)]
enum VisitState {
    Down,
    Next,
    Up,
}

impl<'tree> Iterator for PreOrder<'tree> {

    type Item = Node<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.state == VisitState::Down{
            if self.cursor.goto_first_child() {
                self.state = VisitState::Down;
            } else {
                self.state = VisitState::Next;
                return self.next();
            }
        } else if self.state == VisitState::Next{
            if self.cursor.goto_next_sibling() {
                self.state = VisitState::Down;
            } else {
                self.state = VisitState::Up;
                return self.next();
            }
        } else if self.state == VisitState::Up {
            if self.cursor.goto_parent() {
                self.state = VisitState::Next;
                return self.next(); // TODO caution, might stack overflow 
            } else {
                // finish
                //println!("ICI NEXT FAIT CRASH POLYGLOT USE");
                return None;
            }
        }

        Some(self.cursor.node())
    }
}

impl<'tree> PreOrder<'tree> {
    pub fn new(tree: &'tree tree_sitter::Tree) -> Self {
        let cursor = tree.walk();
        let state = VisitState::Down;
        Self { cursor, state }
    }
    fn node(&self) -> Node<'tree> {
        self.cursor.node()
    }

    //the visit function must use the next function to go to the next node
    fn visit(&mut self, cst:TreeSitterCST) -> Vec<Node> {
        println!("PASSAGE DANS VISIT");
        let mut nodes = Vec::new();

        while let Some(node) = self.next() {
            dbg!(node);
            nodes.push(node);
        }
        println!("FIN WHILE");

        return nodes;
    }

    fn display(&self, node: &PolyglotTree) {
        dbg!("{}",node);
    }

}

impl<'tree, 'text> StuffPerLanguage for JavaBuilder<'tree, 'text> {        

    fn find_polyglot_uses(&self) -> Vec<super::UnSolvedPolyglotUse> {
        println!("PASSAGE DANS FIND POLYGLOT USES");
        let mut uses = Vec::new();
        let tree = self.payload.cst;

        //let mut stack = vec![tree.root_node()];

        for node in PreOrder::new(tree) {
            dbg!(node);
            dbg!(node.kind());
            dbg!(node.to_sexp());
            dbg!(node.child_count());
            //dbg!(node.child(0));
            if node.kind().eq("import_declaration") {
                let r#use = UnSolvedPolyglotUse::Import {
                    path: self
                        .payload
                        .node_to_code(&node.child(1).unwrap().child(0).unwrap())
                        .to_string(),
                    lang: crate::Language::Java,
                };
                println!("DEBUG");
                dbg!(&r#use);
                eprintln!("{:?}",uses);
                uses.push(r#use);
            } else if node.kind().eq("method_invocation") {
                let r#use = UnSolvedPolyglotUse::Eval {
                    path: "??".to_string(),
                    lang: crate::Language::Java,
                };
                dbg!(&r#use);
                uses.push(r#use);
            //anciennement local_variable_declaration
            } else if node.kind().eq(".") {
                println!("PASSAGE DANS .");
                let r#use = UnSolvedPolyglotUse::EvalVariable {
                    name: self
                        .payload
                        .node_to_code(&node.child(1).unwrap().child(0).unwrap())
                        .to_string(),
                };
                dbg!(&r#use);
                uses.push(r#use);
            } else if node.kind().eq("identifier") {
                println!("PASSAGE DANS IDENTIFIER");
                // dbg!(self.payload.node_to_code(&node.child(1).unwrap().child(0).unwrap()));
                // dbg!(self
                //     .payload
                //     .node_to_code(&node.child(1).unwrap().child(0).unwrap())
                //     .to_string());
                let r#use = UnSolvedPolyglotUse::EvalSource {
                    source: self
                        .payload
                        .node_to_code(&node)
                        .to_string(),
                    lang: crate::Language::Java,
                };
                println!("FIN D'INSTANCIATION DE r#use");
                dbg!(&r#use);
                uses.push(r#use);
            }
            else{
                println!("autre")
            }
            //node.children()
        }
        println!("FIN WHILE FIND POLYGLOT USES");
        return dbg!(uses);
    }

    fn find_polyglot_exports(&self) -> Vec<super::PolyglotDef> {
        let mut exports = Vec::new();
        let tree = self.payload.cst;
        let mut stack = vec![tree.root_node()];
        while let Some(node) = stack.pop() {
            if self.payload.node_to_code(&tree.root_node()) == "??" {
                let r#use = PolyglotDef::ExportValue {
                    name: self
                        .payload
                        .node_to_code(&tree.root_node().child(1).unwrap())
                        .to_string(),
                    value: self
                        .payload
                        .node_to_code(&tree.root_node().child(3).unwrap())
                        .to_string(),
                };
                exports.push(r#use);
            }
        }
        return exports;
    }

    fn try_compute_polyglot_use(
        &self,
        node: &Self::Node<'_>,
    ) -> Option<Result<super::UnSolvedPolyglotUse, AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.payload.node_to_code(&call);
        if s == "eval" {
            let r = if s == "" {
                UnSolvedPolyglotUse::Eval {
                    path: todo!(),
                    lang: todo!(),
                }
            } else {
                let lang = self
                    .payload
                    .node_to_code(&node.child(3).unwrap().child(1).unwrap());
                let lang = crate::util::strip_quotes(lang);
                dbg!(&lang);
                UnSolvedPolyglotUse::EvalSource {
                    source: self
                        .payload
                        .node_to_code(&node.child(3).unwrap().child(3).unwrap())
                        .to_string(),
                    lang: crate::util::language_string_to_enum(&lang).unwrap(),
                }
            };
            Some(Ok(r))
        } else if s == "getMember" {
            let r = UnSolvedPolyglotUse::Import {
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
    ) -> Option<UnSolvedPolyglotUse> {
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

            Some(UnSolvedPolyglotUse::EvalVariable { name })

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
        building::{BuildingContext, PolyglotBuilding, PolygloteTreeHandle, StuffPerLanguage, java::PreOrder, UnSolvedPolyglotUse},
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
    fn test_preorder_implem() {
        let main_content = r#"
        Context cx = Context.create();
        context.eval("python", "print('hello')");
        "#;
        println!("TEST PREORDER IMPLEM");
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);
        let tree = tree.as_ref().unwrap();
        let mut pre_order = PreOrder::new(tree);
        assert_eq!(pre_order.node().kind(), "program");
        assert_eq!(pre_order.next().map(|n| n.kind()), Some("import_declaration"));
    }

    #[test]
    fn test_visit_function(){
        let main_content = r#"
        Context cx = Context.create();
        context.eval("python", "print('hello')");
        "#;
        println!("TEST VISIT FUNCTION");
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let mut cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let tree = tree.as_ref().unwrap();
        let mut pre_order = PreOrder::new(tree);

        let nodes = PreOrder::visit(&mut pre_order, cst);
        assert_eq!(nodes.len(), 111);
    }


    #[test]
    fn test_polyglot_use() {
        let main_content = r#"
        Context cx = Context.create();

        Builder builder = Source.newBuilder("python", new File("TestSamples/pyprint.py"));
        context.eval(builder.build());
        "#;
        println!("TEST POLYGLOT USE");
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(&tree, &file_content);
        let builder = &JavaBuilder::init(cst);
        //dbg!(builder);

        let tree = tree.as_ref().unwrap();
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        dbg!(class.to_sexp());
        let meth = class.child(3).unwrap().child(1).unwrap();
        dbg!(meth.to_sexp());
        let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());
        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        let extraction = builder.find_polyglot_uses();
        dbg!(extraction);

        //extraction into find_polyglot_uses
        // let tree = tree.as_ref().unwrap();
        // println!("TEST POLYGLOT USE");
        // dbg!(tree.root_node().to_sexp());
        // let extraction = JavaBuilder::find_polyglot_uses(builder);
        // dbg!(extraction);
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
        println!("TEST DIRECT");
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        dbg!(class.to_sexp());
        let meth = class.child(3).unwrap().child(1).unwrap();
        dbg!(meth.to_sexp());
        let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());
        let r#use: Option<Result<UnSolvedPolyglotUse, crate::building::AnaError>> = builder.try_compute_polyglot_use(&poly_eval);
        //dbg!(r#use);
        assert_eq!(r#use,Some(
            Ok(
                UnSolvedPolyglotUse::EvalSource {
                    source: ("\"print('hello')\"").to_string(),
                    lang: crate::Language::Python,
                },
            ),
        ));
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
        let mut pre_order = PreOrder::new(tree);

        dbg!(pre_order.node().kind());
        dbg!(pre_order.next().map(|n| n.kind()));
        dbg!(pre_order.next());
        dbg!(pre_order.next());
        dbg!(pre_order.next());
        dbg!(pre_order.next());
        dbg!(pre_order.next());

        println!("DIRECT2");
        dbg!(tree.root_node().to_sexp());
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
    //todo
    //faire test for u in find_polyglot_uses
    //commencer par petits cas avec solved(s) puis faire cas importants avec eval(e) et eval(s)
    //faire des assert equals dans les tests pour vérifier qu'on a bien les valeurs qu'on veut

    //les eval E et eval S correspondent au todo à compléter
    //ce qui est polyglotte c'est pas la ligne entière de code mais juste le eval(s)

    //pour les noms de déclaration, possibilité de modifier les enums et d'en rajouter
    //ils ne sont pas spécialement adaptés
    //pas obligés d'avoir les bon noms pour les bons use il faut juste prendre tous les cas poluyglottes
}
