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
    pub fn iter_mut(&mut self) -> TripleIteratorMut<'_, T> {
        TripleIteratorMut::new(self)
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

/// Iterator that iterates by value
pub struct TripleIntoIterator<T> {
    i: u8,
    triple: Triple<Option<T>>,
}

impl<T> TripleIntoIterator<T> {
    fn new(triple: Triple<T>) -> Self {
        let triple = Triple {
            a: Some(triple.a),
            b: Some(triple.b),
            c: Some(triple.c),
        };
        Self { i: 0, triple }
    }
}

impl<T> Iterator for TripleIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.i {
            0 => std::mem::take(&mut self.triple.a),
            1 => std::mem::take(&mut self.triple.b),
            2 => std::mem::take(&mut self.triple.c),
            _ => None,
        };
        if self.i < 3 {
            self.i += 1;
        }
        res
    }
}

impl<T> FusedIterator for TripleIntoIterator<T> {}

/// Iterator that iterates by reference
pub struct TripleIterator<'a, T> {
    i: u8,
    triple: &'a Triple<T>,
}

impl<'a, T> TripleIterator<'a, T> {
    fn new(triple: &'a Triple<T>) -> Self {
        Self { i: 0, triple }
    }
}

impl<'a, T> Iterator for TripleIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.i {
            0 => Some(&self.triple.a),
            1 => Some(&self.triple.b),
            2 => Some(&self.triple.c),
            _ => None,
        };
        if self.i < 3 {
            self.i += 1;
        }
        res
    }
}

impl<T> FusedIterator for TripleIterator<'_, T> {}

/// Iterator that iterates by mutable reference
pub struct TripleIteratorMut<'a, T> {
    i: u8,
    triple: Triple<Option<&'a mut T>>,
}

impl<'a, T> TripleIteratorMut<'a, T> {
    fn new(triple: &'a mut Triple<T>) -> Self {
        let triple = Triple {
            a: Some(&mut triple.a),
            b: Some(&mut triple.b),
            c: Some(&mut triple.c),
        };
        Self { i: 0, triple }
    }
}

impl<'a, T> Iterator for TripleIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.i {
            0 => std::mem::take(&mut self.triple.a),
            1 => std::mem::take(&mut self.triple.b),
            2 => std::mem::take(&mut self.triple.c),
            _ => None,
        };
        if self.i < 3 {
            self.i += 1;
        }
        res
    }
}

impl<T> FusedIterator for TripleIteratorMut<'_, T> {}

pub fn main() {
    let mut t = Triple { a: 1, b: 2, c: 3 };

    for e in &t {
        println!("{e}")
    }

    for e in &mut t {
        *e += 10;
    }

    for e in t {
        println!("{e}")
    }
}
