use std::fs::File;
use std::io::Read;

use tree_sitter::{Node, TextProvider};

use crate::{tree_sitter_utils::TreeSitterCST, util, Language, PolyglotTree};

//use super::EvalSource;

use self::use_solver::ReferenceAnalysis;

use super::{
    AnaError, BuildingContext, PolyglotBuilding, PolyglotDef, PolyglotKind, PolyglotUse,
    StuffPerLanguage,
};

#[derive(Debug, PartialEq, Eq)]
// enum not yet used in the code
// will be useful later
pub enum UnSolvedPolyglotUse<Node> {
    // partially solved
    EvalSource {
        eval: Node,
        name: Node,
    },
    // partially solved
    EvalBuilder {
        call: Node,
        name: Node,
    },
    // can be evaluated
    EvalInline {
        call: Node,
        inline: Node,
        lang: Language,
    },
    // can be evaluated if referenced file can be evaluated
    EvalPath {
        call: Node,
        path: Node,
        lang: Language,
    },
    // can be evaluated if referenced file can be evaluated
    Import {
        call: Node,
        path: Node,
        lang: Language,
    },
}
impl<Ref> UnSolvedPolyglotUse<Ref> {
    // will be useful later
    pub fn get_kind(&self) -> PolyglotKind {
        match self {
            UnSolvedPolyglotUse::EvalSource { .. } => PolyglotKind::Eval,
            UnSolvedPolyglotUse::EvalBuilder { .. } => PolyglotKind::Eval,
            UnSolvedPolyglotUse::EvalInline { .. } => PolyglotKind::Eval,
            UnSolvedPolyglotUse::EvalPath { .. } => PolyglotKind::Eval,
            UnSolvedPolyglotUse::Import { .. } => PolyglotKind::Import,
        }
    }
}
impl<'tree> UnSolvedPolyglotUse<use_solver::Node<'tree>> {
    pub(crate) fn solve<'a>(
        &self,
        // p: &impl crate::tree_sitter_utils::TextProvider<'a, I = &'a str, N<'tree> = use_solver::Node<'tree>>,
        p: &JavaBuilder<'tree, 'a>,
        ana: &impl ReferenceAnalysis,
    ) -> PolyglotUse {
        match self {
            UnSolvedPolyglotUse::EvalInline {
                call: eval,
                inline,
                lang,
            } => {
                if inline.kind() == "string_literal" {
                    use crate::tree_sitter_utils::TextProvider;
                    dbg!(eval.id());
                    let text = p.text(inline);
                    let text = &text[1..text.len().saturating_sub(1)];
                    PolyglotUse {
                        language: *lang,
                        position: eval.id(),
                        aux: crate::building::Aux::EvalSource { code: text.into() },
                    }
                } else {
                    todo!("{}", inline.kind())
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct JavaBuilder<'tree, 'text> {
    payload: TreeSitterCST<'tree, 'text>,
    // temporary stuff
}

impl<'tree, 'text> crate::tree_sitter_utils::TextProvider<'text> for &JavaBuilder<'tree, 'text> {
    type I = &'text str;
    type II = str;
    type N<'t> = tree_sitter::Node<'t>;
    fn text(&'text self, node: &Self::N<'_>) -> Self::I {
        self.node_to_code(node)
    }
    fn t(&self, node: &Self::N<'_>) -> &Self::II {
        self.node_to_code(node)
    }
}
impl<'tree, 'text> super::PolyglotBuilding for JavaBuilder<'tree, 'text> {
    type Node = tree_sitter::Node<'tree>;
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
impl<'tree, 'text> JavaBuilder<'tree, 'text> {
    pub fn node_to_code(&self, node: &tree_sitter::Node<'tree>) -> &'text str {
        &self.payload.node_to_code(node)
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
        if self.state == VisitState::Down {
            if self.cursor.goto_first_child() {
                self.state = VisitState::Down;
            } else {
                self.state = VisitState::Next;
                return self.next();
            }
        } else if self.state == VisitState::Next {
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
    fn visit(&mut self, cst: TreeSitterCST) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(node) = self.next() {
            dbg!(node);
            nodes.push(node);
        }
        return nodes;
    }
}

impl<'tree, 'text> StuffPerLanguage for JavaBuilder<'tree, 'text> {
    type UnsolvedUse = UnSolvedPolyglotUse<Self::Node>;
    fn find_polyglot_uses(&self) -> Vec<Self::UnsolvedUse> {
        let mut uses = Vec::new();
        let tree = self.payload.cst;

        for node in PreOrder::new(tree) {
            if let Some(us) = self.try_compute_polyglot_use(&node) {
                uses.push(us.unwrap());
            }
        }
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
        node: &Self::Node,
    ) -> Option<Result<Self::UnsolvedUse, AnaError>> {
        let call = self.get_polyglot_call(node)?;
        let s = self.payload.node_to_code(&call);
        if s == "eval" {
            let q = tree_sitter::Query::new(
                tree_sitter_java::language(),
                r#"(method_invocation 
    name: (identifier) @name
    (#match? @name "eval")
    arguments: [
        (argument_list
            (method_invocation 
                object: (identifier) @indirect_build
                name: (identifier) @build
                (#match? @build "build")
            )
        )
        (argument_list
            (method_invocation 
                object: (method_invocation) @direct_build
                name: (identifier) @build
                (#match? @build "build")
            )
        )
        (argument_list
            (identifier) @indirect
        )
        (argument_list
            (expression) @lang
            (expression) @code
        )
    ]
)"#,
            )
            .unwrap();
            dbg!(&q);
            let q_c = &mut tree_sitter::QueryCursor::new();
            let mut q_res = q_c.matches(&q, node.clone(), &self.payload);
            if let Some(m) = q_res.next() {
                assert_eq!(self.payload.node_to_code(&m.captures[0].node), "eval");
                match q.capture_names()[m.captures[1].index as usize].as_str() {
                    "indirect" => {
                        let lang = self.payload.node_to_code(&m.captures[1].node);
                        dbg!(lang);
                        dbg!(m.captures[1].node.to_sexp());
                        let indirect = self.payload.node_to_code(&m.captures[1].node);
                        dbg!(indirect);
                        let source = m.captures[1].node;
                        return Some(Ok(UnSolvedPolyglotUse::EvalInline {
                            call: *node,
                            inline: source,
                            lang: crate::Language::Python,
                        }));
                    }
                    "indirect_build" => {
                        let indirect_build = &m.captures[1].node;
                        dbg!(self.payload.node_to_code(indirect_build));
                        return Some(Ok(UnSolvedPolyglotUse::EvalBuilder {
                            call: *node,
                            name: indirect_build.clone(),
                        }));
                    }
                    "lang" => {
                        let lang =
                            util::strip_quotes(self.payload.node_to_code(&m.captures[1].node));
                        let code = &m.captures[2].node;
                        dbg!(&lang, &code);
                        return Some(Ok(UnSolvedPolyglotUse::EvalInline {
                            call: *node,
                            inline: code.clone(),
                            lang: crate::util::language_string_to_enum(&lang).unwrap(),
                        }));
                    }
                    "direct_build" => {
                        todo!("wait for extraction of Source.newBuilder");
                    }
                    x => {
                        dbg!(x);
                    }
                }
            }
            panic!();
        } else if s == "getMember" {
            let r = UnSolvedPolyglotUse::Import {
                call: todo!(),
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
        node: &Self::Node,
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
        call_expr: &<JavaBuilder<'tree, 'text> as PolyglotBuilding>::Node,
    ) -> Option<UnSolvedPolyglotUse<use_solver::Node<'tree>>> {
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

            let name = parameter;
            // let name = self.payload.node_to_code(&parameter);
            // let name = name.to_string();

            Some(UnSolvedPolyglotUse::EvalSource {
                name,
                eval: todo!(),
            })

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

pub(crate) use use_solver::DefaultRefAna;

mod use_solver {
    use std::{marker::PhantomData, path::PathBuf};

    use crate::{tree_sitter_utils::TreeSitterCST, util, Language};
    #[derive(Debug, PartialEq, Eq)]
    pub struct Reference<'tree>(pub Node<'tree>);
    pub(crate) enum TwoTransitions<T0, T1> {
        T0(T0),
        T1(T1),
    }
    pub(crate) enum ThreeTransitions<T0, T1, T2> {
        T0(T0),
        T1(T1),
        T2(T2),
    }
    #[derive(Debug)]
    pub struct SolvingError;
    #[derive(Debug)]
    pub struct NoSource<S> {
        pub(crate) content: S,
    }
    impl<'tree> NoSource<Reference<'tree>> {
        pub(crate) fn solve(
            self,
            refanalysis: &'tree impl ReferenceAnalysis<Reff = Reference<'tree>>,
        ) -> Result<NoSource<Node<'tree>>, SolvingError> {
            let source = refanalysis.solve(&self.content)?;
            Ok(NoSource { content: source })
        }
    }
    impl<'tree> NoSource<Node<'tree>> {
        pub(crate) fn solve(
            self,
        ) -> Result<
            ThreeTransitions<
                NoSource<Reference<'tree>>,
                NoBuilder<'tree, 'tree, Reference<'tree>>,
                PolygloteSource<Node<'tree>, Node<'tree>>,
            >,
            SolvingError,
        > {
            dbg!(self.content.to_sexp());
            todo!()
        }
    }
    #[derive(Debug)]
    pub struct NoBuilder<'tree, 'text, S> {
        pub(crate) local: S,
        pub(crate) global: &'tree TreeSitterCST<'tree, 'text>,
    }
    impl<'tree, T> NoBuilder<'tree, 'tree, T> {
        fn map<U>(self, local: U) -> NoBuilder<'tree, 'tree, U> {
            NoBuilder {
                local: local,
                global: self.global,
            }
        }
    }
    impl<'tree> NoBuilder<'tree, 'tree, Reference<'tree>> {
        pub(crate) fn solve(
            self,
            refanalysis: &'tree impl ReferenceAnalysis<Reff = Reference<'tree>>,
        ) -> Result<NoBuilder<Node<'tree>>, SolvingError> {
            let builder = refanalysis.solve(&self.local)?;
            Ok(self.map(builder))
        }
    }
    //not yet implemented and used
    fn process_file<'tree, 'text>(
        local: Node<'tree>,
        global: &TreeSitterCST<'tree, 'text>,
    ) -> Result<PolygloteSource<Node<'tree>, Node<'tree>>, SolvingError> {
        let q = tree_sitter::Query::new(
            tree_sitter_java::language(),
            r#"(object_creation expression) @direct
                (P) @indirect"#,
        )
        .unwrap();
        dbg!(&q);
        let q_c = &mut tree_sitter::QueryCursor::new();
        let mut q_res = q_c.captures(&q, local, global);
        if let Some(m) = q_res.next() {
            match q.capture_names()[m.1].as_str() {
                "direct" => {
                    todo!();
                }
                "indirect" => {
                    todo!();
                }
                x => {
                    panic!("{}", x)
                }
            }
        } else {
            panic!()
        }
    }

    fn process_lang<'tree, 'text>(
        local: Node<'tree>,
        global: &TreeSitterCST<'tree, 'text>,
    ) -> Result<PolygloteSource<Node<'tree>, Node<'tree>>, SolvingError> {
        let q = tree_sitter::Query::new(
            tree_sitter_java::language(),
            r#"(string_literal) @direct
                (identifier) @indirect"#,
        )
        .unwrap();
        dbg!(&q);
        let q_c = &mut tree_sitter::QueryCursor::new();
        let mut q_res = q_c.captures(&q, local, global);
        if let Some(m) = q_res.next() {
            let lang = &m.0.captures[0].node;
            match q.capture_names()[m.1].as_str() {
                "direct" => {
                    println!("DIRECT");
                    // let lang = util::strip_quotes(global.node_to_code(&m.0.captures[0].node));
                    // dbg!(lang);
                    let lang: PolygloteSource<Node<'tree>, Node<'tree>> = PolygloteSource {
                        language: m.0.captures[0].node,
                        //code: process_code(local, global),
                        code: todo!(),
                    };
                    dbg!(lang);
                    // return lang;
                    // dbg!(lang);
                    // dbg!(global.node_to_code(&lang));
                    // dbg!(lang.to_sexp());
                    todo!()
                }
                "indirect" => {
                    println!("INDIRECT");
                    //let mut lang = process_lang(lang.clone(), global);
                    // let mut lang:TwoTransitions<NoSource<Node<'tree>>, NoBuilder<'tree, 'tree, Reference<'tree>>> =
                    // NoSource {
                    //     content: Reference(lang.clone())
                    // }.solve(&global).unwrap();
                    // dbg!(&lang);
                    // lang
                    todo!()
                }
                x => {
                    panic!("{}", x)
                }
            }
        } else {
            panic!()
        }
    }

    fn process_code<'tree, 'text>(
        local: Node<'tree>,
        global: &TreeSitterCST<'tree, 'text>,
    ) -> Result<PolygloteSource<Node<'tree>, Node<'tree>>, SolvingError> {
        let q = tree_sitter::Query::new(
            tree_sitter_java::language(),
            r#"
                (string_literal) @direct
                (identifier) @code2
                (object_creation_expression
                    arguments: [
                        (argument_list
                            (string_literal) @code4
                        )
                        (argument_list
                            (identifier) @code4
                        )
                    ]
                )
"#,
        )
        .unwrap();
        println!("DEBUG PROCESS CODE");
        dbg!(&q);
        let q_c = &mut tree_sitter::QueryCursor::new();
        let mut q_res = q_c.captures(&q, local, global);
        for m in q_res {
            println!("PASSAGE DANS FOR");
            dbg!(q.capture_names()[m.1].as_str());
            // if m.0.captures[m.1].node != local {
            //     println!("PASSAGE DANS IF");
            //     continue;
            // }
            match q.capture_names()[m.1].as_str() {
                "direct" => {
                    println!("DIRECT");
                    let code = m.0.captures[0].node;
                    dbg!(&code);
                    dbg!(global.node_to_code(&code));
                    todo!()
                }
                "indirect" => {
                    println!("INDIRECT");
                    //TODO : appeler la méthode pour path ?
                    let code = process_code(m.0.captures[0].node.clone(), global);
                    dbg!(&code);
                    return code;
                }
                x => {
                    panic!("{}", x)
                }
            }
        }
        return Err(SolvingError);
    }

    fn process_new_builder<'tree, 'text>(
        local: Node<'tree>,
        global: &TreeSitterCST<'tree, 'text>,
    ) -> Result<PolygloteSource<Node<'tree>, Node<'tree>>, SolvingError> {
        let q = tree_sitter::Query::new(
            tree_sitter_java::language(),
            r#"(method_invocation
        object: (identifier) @Source
        (#match? @Source "Source")
        name: (identifier) @newBuilder
        (#match? @newBuilder "newBuilder")
        arguments: (argument_list
            (_) @lang
            (_) @code
        )
) @match "#,
        )
        .unwrap();
        dbg!(&q);
        let q_c = &mut tree_sitter::QueryCursor::new();
        let mut q_res = q_c.matches(&q, local, global);

        if let Some(m) = q_res.next() {
            if m.captures[0].node != local {
                eprintln!("did not match pattern exactly on given node");
                return Err(SolvingError);
            }
            dbg!(m.captures[1].node.to_sexp());
            let source = global.node_to_code(&m.captures[1].node);
            dbg!(source);
            let newBuilder = global.node_to_code(&m.captures[2].node);
            dbg!(newBuilder);

            let code = &m.captures[4].node;
            let lang = &m.captures[3].node;

            dbg!(global.node_to_code(code));
            dbg!(global.node_to_code(lang));

            let code = process_code(code.clone(), global);
            let lang = process_lang(lang.clone(), global);
            
            todo!()
            // return Ok(PolygloteSource {
            //     language: lang,
            //     code: code,
            // });
        } else {
            panic!()
        }
    }
    impl<'tree, 'text> NoBuilder<'tree, 'text, Node<'tree>> {
        pub(crate) fn solve(
            self,
        ) -> Result<
            TwoTransitions<
                NoBuilder<'tree, 'text, Reference<'tree>>,
                PolygloteSource<Node<'tree>, Node<'tree>>,
            >,
            SolvingError,
        > {
            dbg!(self.local.to_sexp());
            let q = tree_sitter::Query::new(
                tree_sitter_java::language(),
                r#"(local_variable_declaration
    type: (type_identifier)  @Builder
    (#match? @Builder "Builder")
    declarator: (variable_declarator 
        name: (identifier) 
        value: [
            (identifier) @indirect_build
            (method_invocation) @direct_build
        ]
    )
) @match "#,
            )
            .unwrap();
            dbg!(&q);
            let q_c = &mut tree_sitter::QueryCursor::new();
            let mut q_res = q_c.matches(&q, self.local, self.global);
            //dbg!(&q_res);
            if let Some(m) = q_res.next() {
                dbg!(&m);
                dbg!(&m.captures[0].node);
                dbg!(&q.capture_names()[m.captures[0].index as usize]);
                if &m.captures[0].node != &self.local {
                    eprintln!("did not match pattern exactly on given node");
                    return Err(SolvingError);
                }
                match q.capture_names()[m.captures[2].index as usize].as_str() {
                    "direct_build" => process_new_builder(m.captures[2].node.clone(), &self.global)
                        .map(|s| TwoTransitions::T1(s)),
                    "indirect_build" => {
                        let indirect_build = m.captures[2].node.clone();
                        dbg!(self.global.node_to_code(&indirect_build));
                        return Ok(TwoTransitions::T0(NoBuilder {
                            local: Reference(indirect_build),
                            global: self.global,
                        }));
                    }
                    x => {
                        unreachable!()
                    }
                }
            } else {
                panic!();
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct PolygloteSource<L, C> {
        pub(crate) language: L,
        pub(crate) code: C,
    }
    type CodeElement = (usize, usize);
    pub(crate) type Node<'tree> = tree_sitter::Node<'tree>;

    // #[cfg_attr(test, mockall::automock(type Reff=u8;))]
    pub trait ReferenceAnalysis {
        type Reff;
        fn solve<'tree>(&'tree self, reference: &Self::Reff) -> Result<Node<'tree>, SolvingError>;
    }

    #[derive(Default)]
    pub(crate) struct DefaultRefAna<'tree>(PhantomData<&'tree ()>);
    impl<'tree> ReferenceAnalysis for DefaultRefAna<'tree> {
        type Reff = Reference<'tree>;
        fn solve(&self, reff: &Reference<'_>) -> Result<Node<'tree>, SolvingError> {
            todo!()
        }
    }
    impl<'tree, C> PolygloteSource<Reference<'tree>, C> {
        fn solve(
            self,
            refanalysis: &'tree impl ReferenceAnalysis<Reff = Reference<'tree>>,
        ) -> Result<PolygloteSource<Node<'tree>, C>, SolvingError> {
            let language = refanalysis.solve(&self.language)?;
            Ok(PolygloteSource {
                language,
                code: self.code,
            })
        }
    }
    impl<'tree, C> PolygloteSource<Node<'tree>, C> {
        fn solve(
            self,
        ) -> Result<
            TwoTransitions<
                PolygloteSource<Reference<'tree>, C>,
                PolygloteSource<crate::Language, C>,
            >,
            SolvingError,
        > {
            todo!()
        }
    }
    impl<'tree> PolygloteSource<crate::Language, Reference<'tree>> {
        fn solve(
            self,
            refanalysis: &'tree impl ReferenceAnalysis<Reff = Reference<'tree>>,
        ) -> Result<PolygloteSource<crate::Language, Node<'tree>>, SolvingError> {
            let code = refanalysis.solve(&self.code)?;
            Ok(PolygloteSource {
                language: self.language,
                code,
            })
        }
    }
    impl<'tree> PolygloteSource<crate::Language, Node<'tree>> {
        fn solve(
            self,
        ) -> Result<
            PolygloteSource<
                crate::Language,
                ThreeTransitions<
                    PolygloteSource<crate::Language, Reference<'tree>>,
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
// Test section containing non regression tests on the Java builder (preorder_impl, visit, polyglot_use)
// Tests for the main types of polyglot programs :
// -direct 
// -direct2
// -indirect
// -indirect1
// -indirect2
mod test {
    #[cfg(test)]
    use mockall::*;
    use std::{collections::HashMap, fmt::Display};

    use crate::{
        building::{
            java::{PreOrder, UnSolvedPolyglotUse},
            BuildingContext, PolyglotBuilding, PolygloteTreeHandle, StuffPerLanguage,
        },
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
        cx.eval("python", "print('hello')");
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);
        let tree = tree.as_ref().unwrap();
        let mut pre_order = PreOrder::new(tree);
        assert_eq!(pre_order.node().kind(), "program");
        assert_eq!(
            pre_order.next().map(|n| n.kind()),
            Some("import_declaration")
        );
    }

    #[test]
    fn test_visit_function() {
        let main_content = r#"
        Context cx = Context.create();
        cx.eval("python", "print('hello')");
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let mut cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
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
        cx.eval(builder.build());
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);
        //dbg!(builder);

        let tree = tree.as_ref().unwrap();
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        dbg!(class.to_sexp());
        let meth = class.child(3).unwrap().child(1).unwrap();
        dbg!(meth.to_sexp());
        let poly_eval = meth.child(4).unwrap().child(3).unwrap().child(0).unwrap();
        dbg!(builder.node_to_code(&poly_eval));
        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(&r#use);
        let r#use = r#use.unwrap().unwrap();
        let expected_ref_node = poly_eval
            .child(3)
            .unwrap()
            .child(1)
            .unwrap()
            .child(0)
            .unwrap();
        assert_eq!(
            r#use,
            UnSolvedPolyglotUse::EvalBuilder {
                call: poly_eval,
                name: expected_ref_node
            }
        );
        if let UnSolvedPolyglotUse::EvalBuilder { call: eval, name } = r#use {
            assert_eq!(name, expected_ref_node);
            let decl = meth.child(4).unwrap().child(2).unwrap();
            dbg!(&decl.to_sexp());
            // let decl = super::use_solver::NoBuilder { content: decl, payload: builder.payload.cst };
            // match decl.solve().unwrap() {
            //     super::use_solver::TwoTransitions::T0(_) => panic!(),
            //     super::use_solver::TwoTransitions::T1(_) => todo!(),
            // }
        } else {
            panic!()
        }

        // todo!();

        // let extraction = builder.find_polyglot_uses();
        // dbg!(extraction);

        //extraction into find_polyglot_uses
        // let tree = tree.as_ref().unwrap();
        // dbg!(tree.root_node().to_sexp());
        // let extraction = JavaBuilder::find_polyglot_uses(builder);
        // dbg!(extraction);
    }

    #[test]
    fn direct() {
        let main_content = r#"
        Context cx = Context.create();
        cx.eval("python", "print('hello')");
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);

        let tree = tree.as_ref().unwrap();

        let class = tree.root_node().child(5).unwrap();

        let meth = class.child(3).unwrap().child(1).unwrap();

        let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap();

        let r#use = builder.try_compute_polyglot_use(&poly_eval);

        assert_eq!(
            r#use,
            Some(Ok(UnSolvedPolyglotUse::EvalInline {
                call: poly_eval,
                inline: poly_eval
                    .child_by_field_name("arguments")
                    .unwrap()
                    .named_child(1)
                    .unwrap(),
                lang: crate::Language::Python,
            },),)
        );
    }

    //Warning : this test is not working
    //direct_build case not yet implemented in try_compute_polyglot_use method
    #[test]
    fn direct2() {
        let main_content = r#"
        Context cx = Context.create();
        context.eval(Source.newBuilder("python", new File("TestSamples/pyprint.py")).build());
        "#;
        let file_content = main_wrap(main_content);
        dbg!(&file_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        dbg!(&tree);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        dbg!(&cst);
        let builder = &JavaBuilder::init(cst);
        dbg!(&builder);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
        let mut pre_order = PreOrder::new(tree);

        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        dbg!(class.to_sexp());
        let meth = class.child(3).unwrap().child(1).unwrap();
        dbg!(meth.to_sexp());
        //let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap().child(3).unwrap().child(1).unwrap().child(0).unwrap();
        let poly_eval = meth.child(4).unwrap().child(2).unwrap().child(0).unwrap();
        dbg!(poly_eval.child_count());
        // dbg!(poly_eval.child(0).unwrap().to_sexp());
        // dbg!(poly_eval.child(1).unwrap().to_sexp());
        dbg!(poly_eval.to_sexp());
        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        //dbg!(r#use);
        assert_eq!(r#use, Some(
            Ok(UnSolvedPolyglotUse::EvalBuilder {
                call: poly_eval,
                name: poly_eval
                    .child_by_field_name("arguments")
                    .unwrap()
                    .named_child(0)
                    .unwrap()
                    .child_by_field_name("object")
                    .unwrap(),
            },),)
        );
    }
    #[test]
    fn indirect() {
        //test for lang4 case
        let main_content = r#"
        Context cx = Context.create();

        Builder builder = Source.newBuilder("python", new File("TestSamples/pyprint.py"));
        context.eval(builder.build());
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        let meth = class.child(3).unwrap().child(1).unwrap();
        let meth_body = &meth.child(4).unwrap();
        dbg!(meth_body.to_sexp());
        let poly_eval = meth_body.child(3).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());
        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(&r#use);

        let r#use = r#use.unwrap().unwrap();
        let expected_ref_node = poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .named_child(0)
            .unwrap()
            .child_by_field_name("object")
            .unwrap();
        dbg!(poly_eval.to_sexp());
        dbg!(poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .to_sexp());
        dbg!(poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .named_child(0)
            .unwrap()
            .to_sexp());
        dbg!(expected_ref_node.to_sexp());
        assert_eq!(
            r#use,
            UnSolvedPolyglotUse::EvalBuilder {
                call: poly_eval,
                name: expected_ref_node
            }
        );
        // // r#use would have been resolved into builder_decl with a reference analysis
        // if let UnSolvedPolyglotUse::EvalBuilder { call: eval, name } = r#use {
        //     assert_eq!(name, expected_ref_node);
        //     let builder_decl = meth_body.child(2).unwrap();
        //     dbg!(&builder_decl.to_sexp());
        //     let decl = super::use_solver::NoBuilder {
        //         local: builder_decl,
        //         global: &builder.payload,
        //     };
        //     match decl.solve().unwrap() {
        //         super::use_solver::TwoTransitions::T0(x) => {
        //             dbg!(x.local.0.to_sexp());
        //             panic!()
        //         }
        //         super::use_solver::TwoTransitions::T1(x) => {
        //             dbg!(builder.node_to_code(&x.code));
        //             dbg!(builder.node_to_code(&x.language));
        //             panic!()
        //         }
        //     }
        // } else {
        //     panic!()
        // }
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
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);

        // TODO extract into find_polyglot_uses
        let tree = tree.as_ref().unwrap();
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        let meth = class.child(3).unwrap().child(1).unwrap();
        let meth_body = &meth.child(4).unwrap();
        dbg!(meth_body.to_sexp());
        let poly_eval = meth_body.child(3).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());
        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(&r#use);

        let r#use = r#use.unwrap().unwrap();
        let expected_ref_node = poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .named_child(0)
            .unwrap();
        //dbg!(poly_eval.child(0));
        dbg!(poly_eval.to_sexp());
        dbg!(poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .to_sexp());
        dbg!(poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .named_child(0)
            .unwrap()
            .to_sexp());
        dbg!(expected_ref_node.to_sexp());
        assert_eq!(
            r#use,
            UnSolvedPolyglotUse::EvalInline {
                call: poly_eval,
                inline: expected_ref_node,
                lang: crate::Language::Python,
            }
        );

        // if let UnSolvedPolyglotUse::EvalInline {
        //     call: poly_eval,
        //     inline: source,
        //     lang,
        // } = r#use
        // {
        //     assert_eq!(source, expected_ref_node);
        //     assert_eq!(lang, crate::Language::Python);
        //     let source_decl = meth_body.child(2).unwrap();
        //     dbg!(&source_decl.to_sexp());
        //     let decl = super::use_solver::NoBuilder {
        //         local: source_decl,
        //         global: &builder.payload,
        //     };
        //     match decl.solve().unwrap() {
        //         super::use_solver::TwoTransitions::T0(_) => panic!(),
        //         super::use_solver::TwoTransitions::T1(_) => todo!(),
        //     }
        // } else {
        //     panic!()
        // }
    }
    
    #[test]
    fn indirect2() {
        //TODO : c'est un eval context ???
        let main_content = r#"
        Context cx = Context.create();

        File file1 = new File("TestSamples/pyprint.py");
        Source source1 = Source.newBuilder("python", file1).build();
        context.eval(source1);
        "#;
        let file_content = main_wrap(main_content);
        let tree = crate::tree_sitter_utils::parse(&file_content);
        let cst = crate::tree_sitter_utils::into(tree.as_ref(), &file_content);
        let builder = &JavaBuilder::init(cst);

        let tree = tree.as_ref().unwrap();
        dbg!(tree.root_node().to_sexp());
        let class = tree.root_node().child(5).unwrap();
        let meth = class.child(3).unwrap().child(1).unwrap();
        let meth_body = &meth.child(4).unwrap();
        dbg!(meth_body.to_sexp());
        let poly_eval = meth_body.child(4).unwrap().child(0).unwrap();
        dbg!(poly_eval.to_sexp());

        let r#use = builder.try_compute_polyglot_use(&poly_eval);
        dbg!(&r#use);

        let r#use = r#use.unwrap().unwrap();
        let expected_ref_node = poly_eval
            .child_by_field_name("arguments")
            .unwrap()
            .named_child(0)
            .unwrap();
        assert_eq!(
            r#use,
            UnSolvedPolyglotUse::EvalInline {
                call: poly_eval,
                inline: expected_ref_node,
                lang: crate::Language::Python,
            }
        );
        // if let UnSolvedPolyglotUse::EvalInline {
        //     call: eval,
        //     inline: source,
        //     lang,
        // } = r#use
        // {
        //     assert_eq!(source, expected_ref_node);
        //     assert_eq!(lang, crate::Language::Python);
        //     let source_decl = meth_body.child(3).unwrap();
        //     dbg!(&source_decl.to_sexp());
        //     let decl = super::use_solver::NoBuilder {
        //         local: source_decl,
        //         global: &builder.payload,
        //     };
        //     match decl.solve().unwrap() {
        //         super::use_solver::TwoTransitions::T0(_) => panic!(),
        //         super::use_solver::TwoTransitions::T1(_) => todo!(),
        //     }
        // } else {
        //     panic!()
        // }
    }
}
