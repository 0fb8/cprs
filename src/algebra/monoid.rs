pub trait Monoid {
    type Item: Clone;
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn e() -> Self::Item;
}
