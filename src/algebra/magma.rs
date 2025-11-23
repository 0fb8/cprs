pub trait Magma {
    type Item: Clone;
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
}
