use std::iter::DoubleEndedIterator;
use std::iter::FusedIterator;

pub struct Triple<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Triple<T> {
    /// Iterate by reference
    pub fn iter(&self) -> Box<dyn DoubleEndedIteratorHelper<Item = &T> + '_> {
        Box::new(TripleIterator::new(self))
    }

    /// Iterate by mutable reference
    pub fn iter_mut(&mut self) -> Box<dyn DoubleEndedIteratorHelper<Item = &mut T> + '_> {
        Box::new(TripleIteratorMut::new(self))
    }

    fn next_index(i: &mut u8) -> Option<u8> {
        if *i < 4 {
            *i += 1;
            Some(*i)
        } else {
            None
        }
    }
}

impl<T: 'static> IntoIterator for Triple<T> {
    type Item = T;
    type IntoIter = Box<
        dyn DoubleEndedIteratorHelper<
                Item = <TripleIntoIterator<T> as DoubleEndedIteratorHelper>::Item,
            > + 'static,
    >;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(TripleIntoIterator::new(self))
    }
}

impl<'a, T> IntoIterator for &'a Triple<T> {
    type Item = &'a T;
    type IntoIter = Box<
        dyn DoubleEndedIteratorHelper<
                Item = <TripleIterator<'a, T> as DoubleEndedIteratorHelper>::Item,
            > + 'a,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Triple<T> {
    type Item = &'a mut T;
    type IntoIter = Box<
        dyn DoubleEndedIteratorHelper<
                Item = <TripleIteratorMut<'a, T> as DoubleEndedIteratorHelper>::Item,
            > + 'a,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub enum IteratorDirection {
    Forward,
    Backward,
}

pub trait DoubleEndedIteratorHelper {
    type Item;
    fn next_in_direction(&mut self, direction: IteratorDirection) -> Option<Self::Item>;
}

impl<I> Iterator for dyn DoubleEndedIteratorHelper<Item = I> + '_ {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_in_direction(IteratorDirection::Forward)
    }
}

impl<I> DoubleEndedIterator for dyn DoubleEndedIteratorHelper<Item = I> + '_ {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_in_direction(IteratorDirection::Backward)
    }
}

impl<I> FusedIterator for dyn DoubleEndedIteratorHelper<Item = I> + '_ {}

/// Iterator that iterates by value
pub struct TripleIntoIterator<T> {
    front: u8,
    back: u8,
    triple: Triple<Option<T>>,
}

impl<T> TripleIntoIterator<T> {
    fn new(triple: Triple<T>) -> Self {
        let triple = Triple {
            a: Some(triple.a),
            b: Some(triple.b),
            c: Some(triple.c),
        };
        Self {
            front: 0,
            back: 0,
            triple,
        }
    }
}

impl<T> DoubleEndedIteratorHelper for TripleIntoIterator<T> {
    type Item = T;

    fn next_in_direction(&mut self, direction: IteratorDirection) -> Option<Self::Item> {
        if self.front == 4 - self.back {
            return None;
        }
        let i = match direction {
            IteratorDirection::Forward => Triple::<T>::next_index(&mut self.front),
            IteratorDirection::Backward => Triple::<T>::next_index(&mut self.back),
        };

        match i? {
            1 => std::mem::take(&mut self.triple.a),
            2 => std::mem::take(&mut self.triple.b),
            3 => std::mem::take(&mut self.triple.c),
            _ => None,
        }
    }
}

/// Iterator that iterates by reference
pub struct TripleIterator<'a, T> {
    front: u8,
    back: u8,
    triple: &'a Triple<T>,
}

impl<'a, T> TripleIterator<'a, T> {
    fn new(triple: &'a Triple<T>) -> Self {
        Self {
            front: 0,
            back: 0,
            triple,
        }
    }
}

impl<'a, T> DoubleEndedIteratorHelper for TripleIterator<'a, T> {
    type Item = &'a T;

    fn next_in_direction(&mut self, direction: IteratorDirection) -> Option<Self::Item> {
        if self.front == 4 - self.back {
            return None;
        }
        let i = match direction {
            IteratorDirection::Forward => Triple::<T>::next_index(&mut self.front),
            IteratorDirection::Backward => Triple::<T>::next_index(&mut self.back),
        };

        match i? {
            1 => Some(&self.triple.a),
            2 => Some(&self.triple.b),
            3 => Some(&self.triple.c),
            _ => None,
        }
    }
}

/// Iterator that iterates by mutable reference
pub struct TripleIteratorMut<'a, T> {
    front: u8,
    back: u8,
    triple: Triple<Option<&'a mut T>>,
}

impl<'a, T> TripleIteratorMut<'a, T> {
    fn new(triple: &'a mut Triple<T>) -> Self {
        let triple = Triple {
            a: Some(&mut triple.a),
            b: Some(&mut triple.b),
            c: Some(&mut triple.c),
        };
        Self {
            front: 0,
            back: 0,
            triple,
        }
    }
}

impl<'a, T> DoubleEndedIteratorHelper for TripleIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next_in_direction(&mut self, direction: IteratorDirection) -> Option<Self::Item> {
        if self.front == 4 - self.back {
            return None;
        }
        let i = match direction {
            IteratorDirection::Forward => Triple::<T>::next_index(&mut self.front),
            IteratorDirection::Backward => Triple::<T>::next_index(&mut self.back),
        };

        match i? {
            1 => std::mem::take(&mut self.triple.a),
            2 => std::mem::take(&mut self.triple.b),
            3 => std::mem::take(&mut self.triple.c),
            _ => None,
        }
    }
}

pub fn main() {
    let mut t = Triple { a: 1, b: 2, c: 3 };

    for e in &t {
        println!("{e}");
    }

    for e in &mut t {
        *e += 10;
    }

    for e in t.iter().rev() {
        println!("{e}");
    }

    for e in t.iter_mut().rev() {
        *e += 10;
    }

    for e in t.into_iter().rev() {
        println!("{e}");
    }
}
