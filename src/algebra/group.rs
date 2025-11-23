use super::monoid::Monoid;

pub trait Group: Monoid {
    fn inv(a: &Self::Item) -> Self::Item;
}
