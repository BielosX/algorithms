use std::ptr;
use crate::tree::NodeColor::BLACK;

struct BinaryTreeNode<T> {
    value: T,
    left: *mut BinaryTreeNode<T>,
    right: *mut BinaryTreeNode<T>,
    parent: *mut BinaryTreeNode<T>,
}

impl<T> Drop for BinaryTreeNode<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.left.is_null() {
                Box::from_raw(self.left);
            }
            if !self.right.is_null() {
                Box::from_raw(self.right);
            }
        }
    }
}

impl<T> BinaryTreeNode<T> {
    fn new(value: T) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut()
        }
    }
}

pub struct BinaryTree<T> {
    root: *mut BinaryTreeNode<T>,
    size: usize
}

impl<T: Ord + Clone> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: ptr::null_mut(),
            size: 0
        }
    }

    pub fn contains(&self, value: T) -> bool {
        let mut result = false;
        let mut curr = self.root;
        while !result && !curr.is_null() {
            unsafe {
                if value > (*curr).value {
                    curr = (*curr).right;
                } else if value < (*curr).value {
                    curr = (*curr).left;
                } else {
                    result = true;
                }
            }
        }
        result
    }

    pub fn add(&mut self, value: T) {
        if self.root.is_null() {
            let node = Box::new(BinaryTreeNode::new(value));
            unsafe {
                self.root = Box::into_raw(node);
            }
        } else {
            let mut curr = self.root;
            let mut finished = false;
            while !finished {
                unsafe {
                    if value > (*curr).value {
                        if (*curr).right.is_null() {
                            let mut node = Box::new(BinaryTreeNode::new(value.clone()));
                            node.parent = curr;
                            (*curr).right = Box::into_raw(node);
                            finished = true;
                        } else {
                            curr = (*curr).right;
                        }
                    } else if value < (*curr).value {
                        if (*curr).left.is_null() {
                            let mut node = Box::new(BinaryTreeNode::new(value.clone()));
                            node.parent = curr;
                            (*curr).left = Box::into_raw(node);
                            finished = true;
                        } else {
                            curr = (*curr).left;
                        }
                    } else {
                        finished = true;
                    }
                }
            }
        }
    }

    pub fn preorder_dfs_iter(&self) -> BinaryTreeDfsPreorderIterator<'_, T> {
        BinaryTreeDfsPreorderIterator::new(self)
    }
}

impl<T> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        if !self.root.is_null() {
            unsafe {
                Box::from_raw(self.root);
            }
        }
    }
}

pub struct BinaryTreeDfsPreorderIterator<'a, T> {
    tree: &'a BinaryTree<T>,
    curr: *mut BinaryTreeNode<T>
}

impl<'a, T> BinaryTreeDfsPreorderIterator<'a, T> {
    pub fn new(tree: &BinaryTree<T>) -> BinaryTreeDfsPreorderIterator<T> {
        unsafe {
            if tree.root.is_null() {
                BinaryTreeDfsPreorderIterator {
                    tree,
                    curr: ptr::null_mut()
                }
            } else {
                BinaryTreeDfsPreorderIterator {
                    tree,
                    curr: tree.root
                }
            }
        }
    }
}

impl<'a, T> Iterator for BinaryTreeDfsPreorderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        unsafe {
            result = self.curr.as_ref().map(|a| (&a.value as *const T).as_ref()).flatten();
            let mut found_next = false;
            let mut curr = self.curr;
            let mut prev = curr;
            while !found_next && !curr.is_null() {
                if prev == (*curr).left && !(*curr).right.is_null() {
                    prev = curr;
                    self.curr = (*curr).right;
                    found_next = true;
                } else if prev == (*curr).right {
                    prev = curr;
                    curr = (*curr).parent;
                } else {
                    if !(*curr).left.is_null() {
                        self.curr = (*curr).left;
                        found_next = true;
                    } else if !(*curr).right.is_null() {
                        self.curr = (*curr).right;
                        found_next = true;
                    } else {
                        if self.curr == curr {
                            if !(*curr).parent.is_null() {
                                prev = curr;
                                curr = (*curr).parent;
                            } else {
                                self.curr = ptr::null_mut();
                                found_next = true;
                            }
                        } else {
                            self.curr = curr;
                            found_next = true;
                        }
                    }
                }
            }
            if prev == self.tree.root && curr.is_null() {
                self.curr = ptr::null_mut();
            }
        }
        result
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum NodeColor {
    RED,
    BLACK
}

struct RedBlackTreeNode<T> {
    value: T,
    left: *mut RedBlackTreeNode<T>,
    right: *mut RedBlackTreeNode<T>,
    parent: *mut RedBlackTreeNode<T>,
    color: NodeColor
}

impl<T> RedBlackTreeNode<T> {

    fn new(value: T, color: NodeColor) -> RedBlackTreeNode<T> {
        RedBlackTreeNode {
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
            color,
            value
        }
    }

    fn new_red(value: T) -> RedBlackTreeNode<T> {
        RedBlackTreeNode::new(value, NodeColor::RED)
    }

    fn new_black(value: T) -> RedBlackTreeNode<T> {
        RedBlackTreeNode::new(value, NodeColor::BLACK)
    }

    fn parent_color(&self) -> NodeColor {
        if self.parent.is_null() {
            NodeColor::BLACK
        } else {
            unsafe {
                (*self.parent).color
            }
        }
    }

    fn parent_parent(&self) -> Option<*mut RedBlackTreeNode<T>> {
        if self.parent.is_null() {
            Option::None
        } else {
            unsafe {
                let parent_parent = (*self.parent).parent;
                if parent_parent.is_null() {
                    Option::None
                } else {
                    Option::Some(parent_parent)
                }
            }
        }
    }

    fn parent_parent_right(&self) -> Option<*mut RedBlackTreeNode<T>> {
        unsafe {
            self.parent_parent().map(|p| (*p).right)
        }
    }

    fn parent_parent_left(&self) -> Option<*mut RedBlackTreeNode<T>> {
        unsafe {
            self.parent_parent().map(|p| (*p).left)
        }
    }

    fn is_left_child(&self) -> bool {
        if self.parent.is_null() {
            false
        } else {
            let self_ptr = self as *const RedBlackTreeNode<T>;
            unsafe {
                if self_ptr == (*self.parent).left {true} else {false}
            }
        }
    }

    fn is_right_child(&self) -> bool {
        if self.parent.is_null() {
            false
        } else {
            let self_ptr = self as *const RedBlackTreeNode<T>;
            unsafe {
                if self_ptr == (*self.parent).right {true} else {false}
            }
        }
    }
}

pub struct RedBlackTree<T> {
    root: *mut RedBlackTreeNode<T>,
    size: usize
}

impl<T: Ord + Clone> RedBlackTree<T> {
    pub fn new() -> RedBlackTree<T> {
        RedBlackTree {
            root: ptr::null_mut(),
            size: 0
        }
    }

    unsafe fn update_parent_son(&mut self,
                                node: *mut RedBlackTreeNode<T>,
                                new_son: *mut RedBlackTreeNode<T>) {
        if (*node).parent.is_null() {
            self.root = new_son;
        } else if (*node).is_left_child() {
            (*(*node).parent).left = new_son;
        } else {
            (*(*node).parent).right = new_son;
        }
    }

    unsafe fn left_rotate(&mut self, node: *mut RedBlackTreeNode<T>) {
        let mut right = (*node).right;
        (*node).right = (*right).left;
        if !(*right).left.is_null() {
            (*(*right).left).parent = node;
        }
        (*right).parent = (*node).parent;
        self.update_parent_son(node, right);
        (*right).left = node;
        (*node).parent = right;
    }

    unsafe fn right_rotate(&mut self, node: *mut RedBlackTreeNode<T>) {
        let mut left = (*node).left;
        (*node).left = (*left).right;
        if !(*left).right.is_null() {
            (*(*left).right).parent = node;
        }
        (*left).parent = (*node).parent;
        self.update_parent_son(node, left);
        (*left).right = node;
        (*node).parent = left;
    }

    unsafe fn node_color(node: *mut RedBlackTreeNode<T>) -> NodeColor {
        if node.is_null() {
            NodeColor::BLACK
        } else {
            (*node).color
        }
    }

    unsafe fn fixup(&mut self,
                    curr: *mut RedBlackTreeNode<T>,
                    parent_parent: *mut RedBlackTreeNode<T>,
                    uncle: *mut RedBlackTreeNode<T>,
                    side_check: fn(*mut RedBlackTreeNode<T>) -> bool,
                    node_rotation: unsafe fn(&mut RedBlackTree<T>, *mut RedBlackTreeNode<T>),
                    parent_parent_rotation: unsafe fn(&mut RedBlackTree<T>, *mut RedBlackTreeNode<T>)) -> *mut RedBlackTreeNode<T> {
        let mut current = curr;
        if RedBlackTree::node_color(uncle) == NodeColor::RED {
            (*(*current).parent).color = NodeColor::BLACK;
            if !uncle.is_null() {
                (*uncle).color = BLACK;
            }
            (*parent_parent).color = NodeColor::RED;
            current = parent_parent;
        } else {
            if side_check(curr) {
                current = (*current).parent;
                node_rotation(self, current);
            }
            (*(*current).parent).color = NodeColor::BLACK;
            (*parent_parent).color = NodeColor::RED;
            parent_parent_rotation(self, parent_parent);
        }
        current
    }

    fn insert_fixup(&mut self, node: *mut RedBlackTreeNode<T>) {
        unsafe {
            let mut current = node;
            while !(*current).parent.is_null() && (*current).parent_color() == NodeColor::RED {
                if let Some(parent_parent) = (*current).parent_parent() {
                    if (*(*current).parent).is_left_child() {
                        let mut uncle = (*parent_parent).right;
                        self.fixup(current,
                                   parent_parent,
                                   uncle,
                                   |p| (*p).is_right_child(),
                                   RedBlackTree::left_rotate,
                                   RedBlackTree::right_rotate);
                    } else {
                        let mut uncle = (*parent_parent).left;
                        self.fixup(current,
                                   parent_parent,
                                   uncle,
                                   |p| (*p).is_left_child(),
                                   RedBlackTree::right_rotate,
                                   RedBlackTree::left_rotate);
                    }
                }
            }
            (*self.root).color = NodeColor::BLACK;
        }
    }

    pub fn contains(&self, value: T) -> bool {
        let mut result = false;
        let mut curr: *mut RedBlackTreeNode<T> = self.root;
        while !result && !curr.is_null() {
            unsafe {
                if value > (*curr).value {
                    curr = (*curr).right;
                } else if value < (*curr).value {
                    curr = (*curr).left;
                } else {
                    result = true;
                }
            }
        }
        result
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_null() {
            let node = Box::new(RedBlackTreeNode::new_black(value));
            self.root = Box::into_raw(node);
            self.size = 1;
        } else {
            unsafe {
                let mut curr = self.root;
                while !curr.is_null() {
                    if value < (*curr).value {
                        let left = (*curr).left;
                        if left.is_null() {
                            let mut node = Box::new(RedBlackTreeNode::new_red(value.clone()));
                            node.parent = curr;
                            let node_ptr = Box::into_raw(node);
                            (*curr).left = node_ptr;
                            self.size += 1;
                            self.insert_fixup(node_ptr);
                        }
                        curr = left;
                    } else if value > (*curr).value {
                        let right = (*curr).right;
                        if right.is_null() {
                            let mut node = Box::new(RedBlackTreeNode::new_red(value.clone()));
                            node.parent = curr;
                            let node_ptr = Box::into_raw(node);
                            (*curr).right = node_ptr;
                            self.size += 1;
                            self.insert_fixup(node_ptr);
                        }
                        curr = right;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}