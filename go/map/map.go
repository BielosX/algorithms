package _map

import "cmp"

type Color int8

const (
	Red   Color = 0
	Black       = 1
)

type redBlackTreeNode[K cmp.Ordered, V any] struct {
	color               Color
	key                 K
	value               V
	left, right, parent *redBlackTreeNode[K, V]
}

func (node *redBlackTreeNode[K, V]) hasParent() bool {
	return node.parent != nil
}

func (node *redBlackTreeNode[K, V]) hasParentParent() bool {
	if !node.hasParent() {
		return false
	} else {
		return node.parent.parent != nil
	}
}

type TreeMap[K cmp.Ordered, V any] struct {
	root *redBlackTreeNode[K, V]
	size uint64
}

func NewTreeMap[K cmp.Ordered, V any]() TreeMap[K, V] {
	return TreeMap[K, V]{}
}

func (tree *TreeMap[K, V]) updateParentSon(node *redBlackTreeNode[K, V], newSon *redBlackTreeNode[K, V]) {
	if node.parent == nil {
		tree.root = newSon
	} else if node == node.parent.left {
		node.parent.left = newSon
	} else {
		node.parent.right = newSon
	}
}

func (tree *TreeMap[K, V]) leftRotate(node *redBlackTreeNode[K, V]) {
	nodeRight := node.right
	node.right = nodeRight.left
	if nodeRight.left != nil {
		nodeRight.left.parent = node
	}
	nodeRight.parent = node.parent
	tree.updateParentSon(node, nodeRight)
	nodeRight.left = node
	node.parent = nodeRight
}

func (tree *TreeMap[K, V]) rightRotate(node *redBlackTreeNode[K, V]) {
	nodeLeft := node.left
	node.left = nodeLeft.right
	if nodeLeft.right != nil {
		nodeLeft.right.parent = node
	}
	nodeLeft.parent = node.parent
	tree.updateParentSon(node, nodeLeft)
	nodeLeft.right = node
	node.parent = nodeLeft
}

func (tree *TreeMap[K, V]) insertFixup(node *redBlackTreeNode[K, V]) {
	current := node
	for current.hasParent() && current.hasParentParent() && current.parent.color == Red {
		if current.parent == current.parent.parent.left {
			parentParentRight := current.parent.parent.right
			if parentParentRight.color == Red {
				current.parent.color = Black
				parentParentRight.color = Black
				current.parent.parent.color = Red
				current = current.parent.parent
			} else {
				if current == current.parent.right {
					current = current.parent
					tree.leftRotate(current)
				}
				current.parent.color = Black
				current.parent.parent.color = Red
				tree.rightRotate(current)
			}
		} else {
			parentParentLeft := current.parent.parent.left
			if parentParentLeft.color == Red {
				current.parent.color = Black
				parentParentLeft.color = Black
				current.parent.parent.color = Red
				current = current.parent.parent
			} else {
				if current == current.parent.left {
					current = current.parent
					tree.rightRotate(current)
				}
				current.parent.color = Black
				current.parent.parent.color = Red
				tree.leftRotate(current)
			}
		}
	}
	tree.root.color = Black
}

func (tree *TreeMap[K, V]) Insert(key K, value V) {
	if tree.root == nil {
		tree.root = &redBlackTreeNode[K, V]{
			color: Black,
			key:   key,
			value: value,
		}
		tree.size = 1
	} else {
		var currentNode *redBlackTreeNode[K, V] = nil
		iterator := tree.root
		for iterator != nil {
			currentNode = iterator
			if key < iterator.key {
				iterator = iterator.left
			} else {
				iterator = iterator.right
			}
		}
		node := &redBlackTreeNode[K, V]{
			parent: currentNode,
			color:  Red,
			value:  value,
			key:    key,
		}
		if key < currentNode.key {
			currentNode.left = node
		} else {
			currentNode.right = node
		}
		tree.insertFixup(node)
		tree.size++
	}
}

func (tree *TreeMap[K, V]) findKey(key K) *redBlackTreeNode[K, V] {
	current := tree.root
	for current != nil && current.key != key {
		if key > current.key {
			current = current.right
		}
		if key < current.key {
			current = current.left
		}
	}
	return current
}

func (tree *TreeMap[K, V]) Contains(key K) bool {
	current := tree.findKey(key)
	if current == nil {
		return false
	} else {
		return true
	}
}

func (tree *TreeMap[K, V]) Get(key K) *V {
	current := tree.findKey(key)
	if current != nil {
		return &current.value
	} else {
		return nil
	}
}
