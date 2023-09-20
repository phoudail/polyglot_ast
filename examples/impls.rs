//! add you language


use polyglot_ast::PolyglotTree;
use::polyglot_ast::building::PolyglotBuilding;

fn main() {
    todo!("How to add a language implementing PolygloteBuilding");
}

struct MyLang;

impl PolyglotBuilding for MyLang {
    type Node = ();
    type Ctx = ();
    fn init(ctx: Self::Ctx) -> Self {
        todo!()
    }
    fn compute(self) -> PolyglotTree {
        todo!()
    }
}