use std::{fmt::Debug, marker::PhantomData};

pub struct IdMap<I: Id, T> {
    map: Vec<T>,
    __: PhantomData<fn() -> I>,
}

impl<I: Id, T> IdMap<I, T> {
    pub fn new() -> Self {
        Self {
            map: vec![],
            __: PhantomData,
        }
    }

    pub fn insert(&mut self, t: T) -> I {
        self.map.push(t);
        I::from_usize(self.map.len() - 1)
    }

    pub fn get(&self, i: I) -> &T {
        &self.map[i.as_usize()]
    }
}

pub trait Id: Debug + Copy {
    fn as_usize(self) -> usize;
    fn from_usize(id: usize) -> Self;
}
