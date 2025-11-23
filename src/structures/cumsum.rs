use crate::algebra::group::Group;
use crate::algebra::monoid::Monoid;

pub struct CumSum<M: Monoid> {
    data: Vec<M::Item>,
}

impl<M: Monoid> CumSum<M> {
    pub fn new(arr: &[M::Item]) -> Self {
        let mut data = Vec::with_capacity(arr.len() + 1);
        data.push(M::e());
        for x in arr {
            let last = data.last().unwrap().clone();
            data.push(M::op(&last, x));
        }
        CumSum { data }
    }

    pub fn prefix(&self, i: usize) -> M::Item {
        self.data[i].clone()
    }
}

impl<G: Group> CumSum<G> {
    pub fn raange(&self, l: usize, r: usize) -> G::Item {
        G::op(&self.data[r], &G::inv(&self.data[l]))
    }
}
