use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};
use std::ops::Deref;
use std::ptr;
use std::rc::{Rc, Weak};
use std::slice::Iter;

pub struct MinHeap<T, F>(Vec<T>, F)
where
    F: FnMut(&T, &T) -> Ordering;

impl<T: Clone, F> MinHeap<T, F>
where
    F: FnMut(&T, &T) -> Ordering,
{
    pub fn new(func: F) -> MinHeap<T, F> {
        MinHeap(Vec::new(), func)
    }

    fn _parent(i: usize) -> usize {
        i >> 1
    }

    fn _left(i: usize) -> usize {
        i << 1
    }
    fn _right(i: usize) -> usize {
        (i << 1) + 1
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn min_heapify(&mut self, index: usize) {
        let left = MinHeap::<T, F>::_left(index);
        let right = MinHeap::<T, F>::_right(index);
        let mut smallest = left;
        let vec_len = self.0.len();
        if left < vec_len
            && self.1(self.0.get(left).unwrap(), self.0.get(index).unwrap()) == Ordering::Less
        {
            smallest = left;
        } else {
            smallest = index;
        }
        if right < vec_len
            && self.1(self.0.get(right).unwrap(), self.0.get(smallest).unwrap()) == Ordering::Less
        {
            smallest = right;
        }
        if smallest != index {
            let at_index = (*self.0.get(index).unwrap()).clone();
            let at_smallest = (*self.0.get(smallest).unwrap()).clone();
            *self.0.get_mut(smallest).unwrap() = at_index;
            *self.0.get_mut(index).unwrap() = at_smallest;
            self.min_heapify(smallest);
        }
    }

    pub fn extract_min(&mut self) -> T {
        let min = self.0.swap_remove(0);
        self.min_heapify(0);
        min
    }

    pub fn heap_decrease_key(&mut self, index: usize, value: T) {
        let current = self.0.get(index).unwrap();
        if self.1(&value, current) == Ordering::Greater {
            panic!("New key is greater than current")
        }
        *self.0.get_mut(index).unwrap() = value;
        let mut idx = index;
        while idx > 0
            && self.1(
                self.0.get(MinHeap::<T, F>::_parent(idx)).unwrap(),
                self.0.get(idx).unwrap(),
            ) == Ordering::Greater
        {
            let parent = MinHeap::<T, F>::_parent(idx);
            let at_parent = self.0.get(parent).unwrap().clone();
            let at_idx = self.0.get(idx).unwrap().clone();
            *self.0.get_mut(parent).unwrap() = at_idx;
            *self.0.get_mut(idx).unwrap() = at_parent;
            idx = parent;
        }
    }

    pub fn insert(&mut self, value: T) {
        self.0.push(value.clone());
        self.heap_decrease_key(self.0.len() - 1, value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<T, F> Display for MinHeap<T, F>
where
    T: Display + Ord,
    F: FnMut(&T, &T) -> Ordering,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[');
        let length = self.0.len();
        for x in 0..length {
            let elem = self.0.get(x).unwrap();
            if x == 0 {
                f.write_str(format!("{}", elem).as_str());
            } else {
                f.write_str(format!(",{}", elem).as_str());
            }
        }
        f.write_char(']')
    }
}

struct FibonacciHeapNode<T> {
    value: T,
    degree: usize,
    left: *mut FibonacciHeapNode<T>,
    right: *mut FibonacciHeapNode<T>,
    parent: *mut FibonacciHeapNode<T>,
    child: *mut FibonacciHeapNode<T>,
    mark: bool
}

impl<T> FibonacciHeapNode<T> {
    fn new(value: T) -> FibonacciHeapNode<T> {
        FibonacciHeapNode {
            value,
            degree: 0,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
            child: ptr::null_mut(),
            mark: false
        }
    }
}

pub struct FibonacciHeap<T> {
    size: usize,
    min: *mut FibonacciHeapNode<T>
}

impl<T: Ord + Clone + Display> FibonacciHeap<T> {
    pub fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            size: 0,
            min: ptr::null_mut()
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, value: T) {
        if self.min.is_null() {
            let mut node = Box::new(FibonacciHeapNode::new(value));
            self.min = Box::into_raw(node);
            unsafe {
                (*self.min).left = self.min;
                (*self.min).right = self.min;
            }
            self.size = 1;
        } else {
            let mut node = Box::into_raw(Box::new(FibonacciHeapNode::new(value)));
            unsafe {
                let next = (*self.min).right;
                (*self.min).right = node;
                (*node).left = self.min;
                (*node).right = next;
                (*next).left = node;
                if (*node).value < (*self.min).value {
                    self.min = node;
                }
            }
            self.size += 1;
        }
    }

    pub fn root_level_nodes(&self) -> usize {
        let mut result = 0;
        let mut curr = self.min;
        let fst = curr;
        if !curr.is_null() {
            unsafe {
                loop {
                    result += 1;
                    curr = (*curr).right;
                    if curr == fst {
                        break;
                    }
                }
            }
        }
        result
    }

    unsafe fn heap_link(future_son: *mut FibonacciHeapNode<T>, future_parent: *mut FibonacciHeapNode<T>) {
        let prev = (*future_son).left;
        let next = (*future_son).right;
        (*prev).right = next;
        (*next).left = prev;
        let child = (*future_parent).child;
        if child.is_null() {
            (*future_son).left = future_son;
            (*future_son).right = future_son;
            (*future_son).parent = future_parent;
            (*future_parent).child = future_son;
            (*future_parent).degree = 1;
        } else {
            let child_right = (*child).right;
            (*future_son).parent = future_parent;
            (*child).right = future_son;
            (*future_son).right = child_right;
            (*future_son).left = child;
            (*child_right).left = future_son;
            (*future_parent).degree += 1;
        }
        (*future_son).mark = false;
    }

    fn consolidate(&mut self) {
        let logarithm_base = (1.0 + f32::sqrt(5.0)) / 2.0;
        let max_degree = f32::log(self.size as f32, logarithm_base).floor() as usize;
        let mut degrees: Vec<*mut FibonacciHeapNode<T>> = Vec::new();
        for _ in 0..(max_degree + 1) {
            degrees.push(ptr::null_mut());
        }
        let mut root_elem = self.min;
        unsafe {
            let root_nodes = self.root_level_nodes();
            for _ in 0..root_nodes {
                let mut curr = root_elem;
                root_elem = (*root_elem).right;
                let mut degree = (*curr).degree;
                while !degrees[degree].is_null() {
                    let mut other = degrees[degree];
                    if (*curr).value > (*other).value {
                        let tmp = other;
                        other = curr;
                        curr = tmp;
                    }
                    FibonacciHeap::heap_link(other, curr);
                    degrees[degree] = ptr::null_mut();
                    degree += 1;
                }
                degrees[degree] = curr;
            }
            self.min = ptr::null_mut();
        }
        for i in 0..(max_degree + 1) {
            unsafe {
                if !degrees[i].is_null() {
                    if self.min.is_null() {
                        self.min = degrees[i];
                        (*self.min).right = self.min;
                        (*self.min).left = self.min;
                    } else {
                       let right = (*self.min).right;
                        (*self.min).right = degrees[i];
                        (*degrees[i]).right = right;
                        (*right).left = degrees[i];
                        (*degrees[i]).left = self.min;
                        if (*degrees[i]).value < (*self.min).value {
                            self.min = degrees[i];
                        }
                    }
                }
            }
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let mut result = Option::None;
        if !self.min.is_null() {
            unsafe {
                let mut child = (*self.min).child;
                let fst = child;
                if !child.is_null() {
                    loop {
                        let next = (*child).right;
                        let min_next = (*self.min).right;
                        (*self.min).right = child;
                        (*child).left = self.min;
                        (*child).right = min_next;
                        (*min_next).left = child;
                        (*child).parent = ptr::null_mut();
                        child = next;
                        if child == fst {
                            break;
                        }
                    }
                }
                result = Option::Some((*self.min).value.clone());
                if self.min == (*self.min).right {
                    Box::from_raw(self.min);
                    self.min = ptr::null_mut();
                } else {
                    let right = (*self.min).right;
                    let left = (*self.min).left;
                    (*left).right = right;
                    (*right).left = left;
                    self.min = (*self.min).right;
                    self.consolidate();
                }
                self.size -= 1;
            }
        }
        result
    }
}