#![allow(dead_code)]
use crate::dbgt;
//use std::rc::{Rc, Weak};
//use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData; // core::marker::PhantomData;
use std::ptr;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>, // Box own the data they point to
    tail: *mut Node<T>,
    len: usize,
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: *mut Node<T>,
}

impl<T> LinkedList<T>
where
    T: fmt::Display + fmt::Debug,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        let p = (&*node).as_mut_ptr();
        if self.len == 0 {
            self.head = Some(node);
        } else {
            (*node).prev = self.tail;
            unsafe { (*self.tail).next = Some(node) }
        }
        self.tail = p;
        self.len += 1;
    }

    pub fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        if self.len == 0 {
            self.tail = (&*node).as_mut_ptr();
        } else {
            if let Some(first) = &mut self.head {
                first.prev = (&*node).as_mut_ptr();
            }
            node.next = self.head.take();
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        let mut ret = None::<Box<Node<T>>>;
        if self.len > 0 {
            if self.len == 1 {
                std::mem::swap(&mut ret, &mut self.head);
                self.tail = ptr::null_mut();
            } else {
                let pre; // TODO: immutable pre can have an &mut pre.next
                unsafe {
                    pre = &mut *(*self.tail).prev; // not null
                }
                std::mem::swap(&mut ret, &mut pre.next);
                self.tail = pre.as_mut_ptr();
                if let Some(node) = &mut ret {
                    node.prev = ptr::null_mut(); // need this?
                }
            }
            self.len -= 1;
        }
        ret
    }

    pub fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        let mut ret = None::<Box<Node<T>>>;
        if self.len > 0 {
            std::mem::swap(&mut ret, &mut self.head);
            if let Some(node) = &mut ret {
                std::mem::swap(&mut self.head, &mut node.next);
            }
            if let Some(node) = &mut self.head {
                node.prev = ptr::null_mut(); // need this?
            } else {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
        }
        ret
    }

    pub fn push_back(&mut self, v: T) {
        self.push_back_node(Box::new(Node::new(v))); // do not use box keyword
    }

    pub fn push_front(&mut self, v: T) {
        self.push_front_node(Box::new(Node::new(v)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.data)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.data)
    }

    pub fn remove_node(mut p: &mut Option<Box<Node<T>>>) -> Option<T> {
        // FIXME: this is not an associated method, (multiple mut borrow?)
        // so we can not modify self.len here
        let mut temp = None::<Box<Node<T>>>;
        std::mem::swap(&mut temp, &mut p);
        if let Some(to_remove) = &mut temp {
            std::mem::swap(p, &mut to_remove.next);
            if let Some(node) = &mut p {
                node.prev = to_remove.prev;
            }
            to_remove.prev = ptr::null_mut(); // need this?
        }
        temp.map(|node| (*node).data)
    }

    pub fn remove_at(&mut self, i: usize) -> Option<T> {
        if i > self.len - 1 {
            None
        } else if i == self.len - 1 {
            self.pop_back()
        } else {
            let mut p = &mut self.head;
            for _ in 0..i {
                if let Some(node) = p {
                    p = &mut node.next;
                }
            }
            self.len -= 1;
            Self::remove_node(p)
        }
    }

    pub fn remove_item(&mut self, item: T)
    where
        T: PartialEq,
    {
        let mut p = &mut self.head;
        let mut found = false;
        while let Some(node) = p {
            if let Some(next) = &node.next {
                if next.data == item {
                    found = true;
                }
            }
            p = &mut node.next;
            if found {
                self.len -= 1;
                Self::remove_node(p);
                break;
            }
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let mut lst = LinkedList::new();
        for item in iter {
            lst.push_back(item);
        }
        lst
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: match &self.head {
                Some(node) => (&**node).as_ptr(),
                None => ptr::null(),
            },
            marker: PhantomData,
        }
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|v| v == x)
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut is_first_time = true;
        let mut head = &self.head;
        while let Some(p) = head {
            if is_first_time {
                is_first_time = false
            } else {
                write!(f, " -> ")?;
            }
            write!(f, "{}", (*p).data)?;
            head = &(*p).next;
        }
        write!(f, ")")
    }
}

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Node {
            data: item,
            next: None,
            prev: ptr::null_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        // TODO: Does this unsafe?
        self as *const Self as usize as *mut Self
    }
}

pub struct Iter<'a, T> {
    head: *const Node<T>,
    marker: PhantomData<&'a Node<T>>, // NOTE: just for lifetime
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let ret = Some(&(*self.head).data);
                self.head = match &(*self.head).next {
                    Some(node) => (&**node).as_ptr(),
                    None => ptr::null(),
                };
                ret
            }
        }
    }
}

#[test]
//#[ignore]
fn test() {
    //let mut ll = LinkedList::from_iter(vec![9, 2, 3, 4, 5, 6].into_iter());
    let mut ll = LinkedList::new();
    for v in [4, 5, 6].into_iter() {
        ll.push_back(v);
    }
    for v in [3, 2, 9].into_iter() {
        ll.push_front(v);
    }

    let mut stack = vec![];
    stack.push(ll.pop_back());
    stack.push(ll.pop_front());
    stack.push(ll.pop_back());
    ll.push_back(42);
    ll.push_front(142);
    ll.push_front(12);
    stack.push(ll.pop_front());

    // [6, 9, 5, 12] + [142, 2, 3, 4, 42]
    while let Some(Some(v)) = stack.pop() {
        ll.push_front(v);
    }
    ll.remove_at(2);
    ll.remove_item(3);
    ll.remove_item(33);
    assert_eq!(7, ll.len());

    println!("{}", ll);
    assert!(ll.contains(&9));
    assert!(!ll.contains(&5));
    assert!(ll.contains(&142));
    assert!(!ll.contains(&3));
    assert!(ll.contains(&42));
}
