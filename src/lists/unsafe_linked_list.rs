#![allow(dead_code)]

use std::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

pub struct UnsafeLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

pub struct Iter<T> {
    list: UnsafeLinkedList<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

impl<T> UnsafeLinkedList<T> {
    pub fn new() -> Self {
        UnsafeLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);

        node.prev = self.tail;
        let node = Some(Box::leak(Box::new(node)).into());

        match self.tail {
            None => self.head = node,
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
        }

        self.tail = node;
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);

        node.next = self.head;
        let node = Some(Box::leak(Box::new(node)).into());

        match self.head {
            None => self.tail = node,
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
        }

        self.head = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.tail.map(|tail| {
            let tail = unsafe { Box::from_raw(tail.as_ptr()) };
            self.tail = tail.prev;

            self.len -= 1;
            tail.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.head.map(|head| {
            let head = unsafe { Box::from_raw(head.as_ptr()) };
            self.head = head.next;

            self.len -= 1;
            head.data
        })
    }

    pub fn nth(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            None
        } else if index < self.len() / 2 {
            let mut node = self.head;

            for _ in 0..index {
                node = unsafe { node.unwrap().as_ref().next };
            }

            let node = unsafe { Box::from_raw(node.unwrap().as_ptr()) };
            Some(node.data)
        } else {
            let mut node = self.tail;

            for _ in 0..(self.len() - index - 1) {
                node = unsafe { node.unwrap().as_ref().prev };
            }

            let node = unsafe { Box::from_raw(node.unwrap().as_ptr()) };
            Some(node.data)
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.tail
            .as_ref()
            .map(|node| unsafe { &node.as_ref().data })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.tail
            .as_mut()
            .map(|node| unsafe { &mut node.as_mut().data })
    }

    pub fn back(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| unsafe { &node.as_ref().data })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node| unsafe { &mut node.as_mut().data })
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for UnsafeLinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { list: self }
    }
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for UnsafeLinkedList<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "[")?;

        let mut node = self.head;
        while let Some(n) = node {
            unsafe {
                write!(w, "{}", n.as_ref().data)?;
                node = n.as_ref().next;
            }

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
        let mut list = UnsafeLinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        assert_eq!(format!("{}", list), "[0, 1, 2]");
    }

    #[test]
    fn push_back() {
        let mut list = UnsafeLinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(format!("{}", list), "[2, 1, 0]");
    }

    #[test]
    fn pop_front() {
        let mut list = UnsafeLinkedList::new();

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
        let mut list = UnsafeLinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(0));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn nth() {
        let mut list = UnsafeLinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.nth(0), Some(0));
        assert_eq!(list.nth(1), Some(1));
        assert_eq!(list.nth(2), Some(2));
    }

    #[test]
    fn front() {
        let mut list = UnsafeLinkedList::new();

        assert_eq!(list.front(), None);

        list.push_front(0);
        assert_eq!(list.front(), Some(&0));

        list.push_front(1);
        assert_eq!(list.front(), Some(&1));

        list.push_front(2);
        assert_eq!(list.front(), Some(&2));
    }

    #[test]
    fn front_mut() {
        let mut list = UnsafeLinkedList::new();

        assert_eq!(list.front_mut(), None);

        list.push_front(0);
        assert_eq!(list.front_mut(), Some(&mut 0));

        list.push_front(1);
        assert_eq!(list.front_mut(), Some(&mut 1));

        list.push_front(2);
        assert_eq!(list.front_mut(), Some(&mut 2));
    }

    #[test]
    fn back() {
        let mut list = UnsafeLinkedList::new();

        assert_eq!(list.back(), None);

        list.push_back(0);
        assert_eq!(list.back(), Some(&0));

        list.push_back(1);
        assert_eq!(list.back(), Some(&1));

        list.push_back(2);
        assert_eq!(list.back(), Some(&2));
    }

    #[test]
    fn back_mut() {
        let mut list = UnsafeLinkedList::new();

        assert_eq!(list.back_mut(), None);

        list.push_back(0);
        assert_eq!(list.back_mut(), Some(&mut 0));

        list.push_back(1);
        assert_eq!(list.back_mut(), Some(&mut 1));

        list.push_back(2);
        assert_eq!(list.back_mut(), Some(&mut 2));
    }

    #[test]
    fn clear() {
        let mut list = UnsafeLinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.len(), 3);
        assert_eq!(format!("{}", list), "[0, 1, 2]");

        list.clear();
        assert_eq!(list.len(), 0);
        assert_eq!(format!("{}", list), "[]");
    }

    #[test]
    fn is_empty() {
        let mut list = UnsafeLinkedList::new();

        assert!(list.is_empty());

        list.push_front(0);
        assert!(!list.is_empty());

        list.push_front(1);
        assert!(!list.is_empty());

        list.push_front(2);
        assert!(!list.is_empty());

        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn len() {
        let mut list = UnsafeLinkedList::new();

        assert_eq!(list.len(), 0);

        list.push_front(0);
        assert_eq!(list.len(), 1);

        list.push_front(1);
        assert_eq!(list.len(), 2);

        list.pop_front();
        assert_eq!(list.len(), 1);

        list.push_front(2);
        assert_eq!(list.len(), 2);

        list.clear();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn iterator() {
        let mut list = UnsafeLinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        assert_eq!(list.into_iter().sum::<i32>(), 45);
    }
}
