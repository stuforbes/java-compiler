
pub struct ResolvedSymbol<'symbol> {
    object_class: &'symbol String,
    field_class: &'symbol String,
    descriptor: &'symbol String,
}