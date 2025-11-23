use super::Magma::Magma;

pub trait Quasigroup: Magma {
    fn div_left(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn div_right(a: &Self::Item, b: &Self::Item) -> Self::Item;
}
