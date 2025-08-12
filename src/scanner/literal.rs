
#[derive(Debug, Copy, Clone)]
pub enum Literal<'a> {
    String(&'a str)
}
