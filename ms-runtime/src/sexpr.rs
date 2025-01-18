#[derive(Debug, Clone)]
pub enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}
