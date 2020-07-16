#![allow(dead_code)]

use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    prev: WeakLink<T>,
    next: Link<T>,
}

pub struct SafeLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

pub struct Iter<T> {
    list: SafeLinkedList<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }
}

impl<T> SafeLinkedList<T> {
    pub fn new() -> Self {
        SafeLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.len += 1;
        let new_tail = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new_tail.clone());

                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.len += 1;
        let new_head = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);

                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // Remove the head pointer if there's 1 element left to prevent strong_count to be 2 for tail
        if self.len() == 1 {
            self.head = None;
        }

        self.tail.take().map(|old_tail| {
            self.len -= 1;

            if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                let new_tail = new_tail.upgrade();

                self.tail = new_tail.clone();

                if let Some(new_tail) = new_tail {
                    new_tail.borrow_mut().next = None;
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // Remove the tail pointer if there's 1 element left to prevent strong_count to be 2 for head
        if self.len() == 1 {
            self.tail = None;
        }

        self.head.take().map(|old_head| {
            self.len -= 1;

            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;

                self.head = Some(new_head);
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.data))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |node| &node.data))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.data))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|head| RefMut::map(head.borrow_mut(), |node| &mut node.data))
    }

    // pub fn nth(&self, index: usize) -> Option<Ref<T>> {
    //     if index >= self.len() {
    //         None
    //     } else if index < self.len() / 2 {
    //         let mut i = 0;
    //         let mut node = self.head.clone();

    //         while i < index {
    //             i += 1;

    //             node = node.unwrap().borrow().next.clone();
    //         }

    //         node.as_ref()
    //             .map(|x| Ref::map(x.borrow(), |node| &node.data))
    //     } else {
    //         let mut i = 0;
    //         let mut node = self.tail.clone();

    //         while i >= index {
    //             i -= 1;

    //             node = node.unwrap().borrow().prev.clone().unwrap().upgrade();
    //         }

    //         node.as_ref()
    //             .map(|x| Ref::map(x.borrow(), |node| &node.data))
    //     }
    // }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for SafeLinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { list: self }
    }
}

impl<T: fmt::Display> fmt::Display for SafeLinkedList<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "[")?;

        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();

            if let Some(_) = node {
                write!(w, ", ")?;
            }
        }

        write!(w, "]")
    }
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_front() {
        let mut list = SafeLinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        assert_eq!(format!("{}", list), "[0, 1, 2]");
    }

    #[test]
    fn push_back() {
        let mut list = SafeLinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(format!("{}", list), "[2, 1, 0]");
    }

    #[test]
    fn pop_front() {
        let mut list = SafeLinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut list = SafeLinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(0));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_front() {
        let mut list = SafeLinkedList::new();

        assert!(list.peek_front().is_none());

        list.push_front(0);
        assert_eq!(&*list.peek_front().unwrap(), &0);

        list.push_front(1);
        assert_eq!(&*list.peek_front().unwrap(), &1);

        list.push_front(2);
        assert_eq!(&*list.peek_front().unwrap(), &2);
    }

    #[test]
    fn peek_back() {
        let mut list = SafeLinkedList::new();

        assert!(list.peek_back().is_none());

        list.push_back(0);
        assert_eq!(&*list.peek_back().unwrap(), &0);

        list.push_back(1);
        assert_eq!(&*list.peek_back().unwrap(), &1);

        list.push_back(2);
        assert_eq!(&*list.peek_back().unwrap(), &2);
    }

    #[test]
    fn peek_front_mut() {
        let mut list = SafeLinkedList::new();

        assert!(list.peek_front().is_none());

        list.push_front(0);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 0);

        list.push_front(1);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 1);

        list.push_front(2);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 2);
    }

    #[test]
    fn peek_back_mut() {
        let mut list = SafeLinkedList::new();

        assert!(list.peek_back().is_none());

        list.push_back(0);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 0);

        list.push_back(1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);

        list.push_back(2);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 2);
    }

    #[test]
    fn iterator() {
        let mut list = SafeLinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        assert_eq!(list.into_iter().sum::<i32>(), 45);
    }
}
