use std::iter::DoubleEndedIterator;
use std::iter::FusedIterator;

pub struct Triple<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Triple<T> {
    /// Iterate by reference
    pub fn iter(&self) -> TripleIterator<T> {
        TripleIterator::new(self)
    }

    /// Iterate by mutable reference
    pub fn iter_mut(&mut self) -> TripleIteratorMut<T> {
        TripleIteratorMut::new(self)
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

impl<T> IntoIterator for Triple<T> {
    type Item = T;
    type IntoIter = TripleIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TripleIntoIterator::new(self)
    }
}

impl<'a, T> IntoIterator for &'a Triple<T> {
    type Item = &'a T;
    type IntoIter = TripleIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Triple<T> {
    type Item = &'a mut T;
    type IntoIter = TripleIteratorMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub enum IteratorDirection {
    Forward,
    Backward,
}

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

    fn take_index(&mut self, i: u8) -> Option<T> {
        match i {
            1 => std::mem::take(&mut self.triple.a),
            2 => std::mem::take(&mut self.triple.b),
            3 => std::mem::take(&mut self.triple.c),
            _ => None,
        }
    }
}

impl<T> Iterator for TripleIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = Triple::<T>::next_index(&mut self.front)?;
        self.take_index(i)
    }
}

impl<T> FusedIterator for TripleIntoIterator<T> {}

impl<T> DoubleEndedIterator for TripleIntoIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = 4 - Triple::<T>::next_index(&mut self.back)?;
        self.take_index(i)
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

impl<'a, T> Iterator for TripleIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match Triple::<T>::next_index(&mut self.front)? {
            1 => Some(&self.triple.a),
            2 => Some(&self.triple.b),
            3 => Some(&self.triple.c),
            _ => None,
        }
    }
}

impl<T> FusedIterator for TripleIterator<'_, T> {}

impl<'a, T> DoubleEndedIterator for TripleIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match 4 - Triple::<T>::next_index(&mut self.back)? {
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

impl<'a, T> Iterator for TripleIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match Triple::<T>::next_index(&mut self.front)? {
            1 => std::mem::take(&mut self.triple.a),
            2 => std::mem::take(&mut self.triple.b),
            3 => std::mem::take(&mut self.triple.c),
            _ => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for TripleIteratorMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match 4 - Triple::<T>::next_index(&mut self.back)? {
            1 => std::mem::take(&mut self.triple.a),
            2 => std::mem::take(&mut self.triple.b),
            3 => std::mem::take(&mut self.triple.c),
            _ => None,
        }
    }
}

impl<T> FusedIterator for TripleIteratorMut<'_, T> {}

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
