use std::boxed::Box;
use std::iter::FromIterator;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

#[derive(Default, Debug)]
pub struct SimpleLinkedList<T> {
    len: usize,
    tail: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, elem: T) {
        self.len += 1;
        self.tail = Some(Box::new(Node {
            data: elem,
            prev: self.tail.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            self.len -= 1;
            self.tail = node.prev;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.tail.as_ref().map(|x| &x.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cursor = &self.tail;
        while let Some(node) = cursor {
            list.push(node.data);
            cursor = &node.prev;
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T>
where
    T: Default + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::<T>::new();
        for e in _iter {
            list.push(e);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Default + Copy,
{
    fn into(mut self) -> Vec<T> {
        let mut v = vec![T::default(); self.len()];
        for i in (0..self.len()).rev() {
            v[i] = self.pop().unwrap();
        }
        v
    }
}
