use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

fn rc_to_node<'a, T>(rc: &'a Option<Rc<Node<T>>>) -> Option<&'a Node<T>> {
    rc.as_ref().map(|x|&(**x))
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: RefCell<Option<Rc<Node<T>>>>,
}

#[derive(Default, Debug)]
pub struct SimpleLinkedList<T> {
    len: usize,
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T> SimpleLinkedList<T> where T: Default + Copy {
    pub fn new() -> Self {
        Self{.. Self::default()}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let new_elem = Rc::new(Node{data: _element, next: RefCell::new(None)});
        let new_tail = Some(Rc::clone(&new_elem));
        let new_elem = Some(new_elem);
        if self.len() == 0 {
            self.head = new_elem;
            self.tail = new_tail;
        } else {
            self.tail = { // Needed, the borrowing of tail needs to be dropped before assigning it again
                let mut tmp = self.tail.as_ref().unwrap().next.borrow_mut();
                *tmp = new_elem;
                new_tail
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        self.len -= 1;

        // Store the data that was in the tail
        let data: Option<T> = self.tail.as_ref().map(|x|(**x).data);

        if self.len() == 0 {
            self.tail = None;
            self.head = None;
            return data;
        } else {
            // Move the cursor to one before the end
            let len = self.len() - 1;
            let mut cursor: Rc<Node<T>> = Rc::clone(self.head.as_ref().unwrap());
            for _ in 0 .. len {
                cursor = { // Needed, we need to drop the cursor borrowing after assigning it
                    let next = cursor.next.borrow(); 
                    Rc::clone(next.as_ref().unwrap())
                };
            }
            // Update tail
            *cursor.next.borrow_mut() = None;
            self.tail = Some(Rc::clone(&cursor));
        }
        return data
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        self.len -= 1;
        let data: Option<T> = self.head.as_ref().map(|x|(**x).data);

        if self.len() == 0 {
            self.tail = None;
            self.head = None;
            return data;
        } else {
            let head = rc_to_node(&self.head).unwrap();
            let new_head = Rc::clone(head.next.borrow().as_ref().unwrap());
            self.head = Some(new_head);
        }
        return data;
    }

    pub fn peek(&self) -> Option<&T> {
        self.tail.as_ref().map(|x|&(**x).data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        while self.len() > 0 {
            list.push(self.pop().unwrap());
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> where T: Default + Copy{
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> where T: Default + Copy{
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        while self.len() > 0 {
            v.push(self.pop_front().unwrap());
        }
        v
    }
}
