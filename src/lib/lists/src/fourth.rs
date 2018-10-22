use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head: Rc<RefCell<Node<T>>>| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail: Rc<RefCell<Node<T>>>| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            Ref::map(node.borrow(), |node:&Node<T>| &node.elem)
        })
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            Ref::map(node.borrow(), |node:&Node<T>| &node.elem)
        })
    }
    pub fn peek_mut_front(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            RefMut::map(node.borrow_mut(), |node: &mut Node<T>| &mut node.elem)
        })
    }
    pub fn peek_mut_back(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            RefMut::map(node.borrow_mut(), |node: &mut Node<T>| &mut node.elem)
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use std::cell::{Ref, RefMut};

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);

        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_mut_front().is_none());
        assert!(list.peek_mut_back().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek_front().map(|i: Ref<i32>| *i), Some(3));
        assert_eq!(list.peek_back().map(|i: Ref<i32>| *i), Some(1));
        assert_eq!(list.peek_mut_front().map(|i: RefMut<i32>| *i), Some(3));
        assert_eq!(list.peek_mut_back().map(|i: RefMut<i32>| *i), Some(1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}