use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

struct LinkedListNode<T> {
    value: T,
    next: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

impl<T> LinkedListNode<T> {
    fn new(value: T) -> LinkedListNode<T> {
        LinkedListNode {
            value,
            next: Option::None,
        }
    }

    fn is_last(&self) -> bool {
        self.next.is_none()
    }

    fn next_is_last(&self) -> bool {
        self.next
            .as_ref()
            .map(|a| a.borrow().is_last())
            .unwrap_or(false)
    }

    fn add_next(&mut self, next: LinkedListNode<T>) {
        match &self.next {
            None => {
                self.next = Option::Some(Rc::new(RefCell::new(next)));
            }
            Some(nxt) => {
                nxt.borrow_mut().add_next(next);
            }
        }
    }
}

pub struct LinkedList<T> {
    first: Option<Rc<RefCell<LinkedListNode<T>>>>,
    last: Option<Rc<RefCell<LinkedListNode<T>>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            first: Option::None,
            last: Option::None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        match &self.last {
            None => {
                let v = Rc::new(RefCell::new(LinkedListNode::new(value)));
                self.last = Option::Some(Rc::clone(&v));
                self.first = Option::Some(v);
            }
            Some(lst) => {
                lst.borrow_mut().add_next(LinkedListNode::new(value));
                self.last = Option::Some(Rc::clone(lst));
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut result = Option::None;
        let r = &mut self.first;
        match r {
            None => {}
            Some(fst) => {
                result = Option::Some(fst.borrow().value.clone());
                let nxt = fst.borrow().next.as_ref().map(|a| Rc::clone(a));
                *r = nxt;
                self.size -= 1;
            }
        }
        if let Some(elem) = r {
            if elem.borrow().is_last() {
                self.last = Option::Some(Rc::clone(elem));
            }
        } else {
            self.last = Option::None;
        }
        result
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut result = Option::None;
        match &self.first {
            None => {}
            Some(fst) => {
                if fst.borrow().is_last() {
                    result = Option::Some(fst.borrow().value.clone());
                    self.first = Option::None;
                    self.last = Option::None;
                    self.size = 0;
                } else {
                    let mut curr = Rc::clone(fst);
                    while !curr.borrow().next_is_last() {
                        let next = curr.borrow().next.as_ref().map(|a| Rc::clone(a));
                        if let Some(nxt) = next {
                            curr = nxt;
                        }
                    }
                    result = curr
                        .borrow()
                        .next
                        .as_ref()
                        .map(|a| a.borrow().value.clone());
                    curr.borrow_mut().next = Option::None;
                    self.last = Option::Some(curr);
                    self.size -= 1;
                }
            }
        }
        result
    }

    pub fn first(&self) -> Option<T> {
        self.first.as_ref().map(|a| a.borrow().value.clone())
    }

    pub fn last(&self) -> Option<T> {
        self.last.as_ref().map(|a| a.borrow().value.clone())
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter::new(self)
    }
}

pub struct LinkedListIter<T> {
    curr: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

impl<T: Clone> LinkedListIter<T> {
    pub fn new(list: &LinkedList<T>) -> LinkedListIter<T> {
        LinkedListIter {
            curr: list.first.as_ref().map(|a| Rc::clone(a)),
        }
    }
}

impl<T: Clone> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr.as_ref().map(|a| a.borrow().value.clone());
        self.curr = self
            .curr
            .as_ref()
            .map(|a| a.borrow().next.as_ref().map(|b| Rc::clone(b)))
            .flatten();
        result
    }
}

struct DoublyLinkedListNode<T> {
    value: T,
    next: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    prev: Option<Weak<RefCell<DoublyLinkedListNode<T>>>>,
}

impl<T> DoublyLinkedListNode<T> {
    fn new(value: T) -> DoublyLinkedListNode<T> {
        DoublyLinkedListNode {
            value,
            next: Option::None,
            prev: Option::None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    size: usize,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            first: Option::None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        match &self.first {
            None => {
                self.first = Option::Some(Rc::new(RefCell::new(DoublyLinkedListNode::new(value))));
            }
            Some(fst) => {
                let maybe_prev = fst.borrow().prev.as_ref().map(|a| a.upgrade()).flatten();
                let mut node = DoublyLinkedListNode::new(value);
                node.next = Option::Some(Rc::clone(fst));
                if let Some(prev) = maybe_prev {
                    node.prev = Option::Some(Rc::downgrade(&prev));
                    prev.borrow_mut().next = Option::Some(Rc::new(RefCell::new(node)));
                } else {
                    node.prev = Option::Some(Rc::downgrade(fst));
                    let node_ref = Rc::new(RefCell::new(node));
                    fst.borrow_mut().prev = Option::Some(Rc::downgrade(&node_ref));
                    fst.borrow_mut().next = Option::Some(node_ref);
                }
            }
        }
        self.size += 1;
    }

    pub fn first(&self) -> Option<T> {
        self.first.as_ref().map(|a| a.borrow().value.clone())
    }

    pub fn last(&self) -> Option<T> {
        let mut result = Option::None;
        match self.first.as_ref() {
            None => {}
            Some(fst) => {
                if let Some(prev) = fst.borrow().prev.as_ref().map(|a| a.upgrade()).flatten() {
                    result = Option::Some(prev.borrow().value.clone());
                } else {
                    result = Option::Some(fst.borrow().value.clone());
                }
            }
        }
        result
    }
}
