package _map

import (
	"algorithms/list"
	"cmp"
	"encoding/binary"
	"hash/maphash"
)

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

type HashableInt int
type HashableString string

type Hashable interface {
	comparable
	Hash() uint64
}

var seed = maphash.MakeSeed()

func (h HashableInt) Hash() uint64 {
	buffer := make([]byte, 8)
	binary.LittleEndian.PutUint64(buffer, uint64(h))
	return maphash.Bytes(seed, buffer)
}

func (h HashableString) Hash() uint64 {
	return maphash.Bytes(seed, []byte(h))
}

type hashMapEntry[K Hashable, V any] struct {
	key   K
	value V
}

type hashMapBucket[K Hashable, V any] struct {
	entries list.LinkedList[hashMapEntry[K, V]]
}

type HashMap[K Hashable, V any] struct {
	buckets []hashMapBucket[K, V]
}

func NewHashMapWithBucketsNumber[K Hashable, V any](bucketSize int) HashMap[K, V] {
	buckets := make([]hashMapBucket[K, V], bucketSize)
	return HashMap[K, V]{
		buckets: buckets,
	}
}

func NewHashMap[K Hashable, V any]() HashMap[K, V] {
	return NewHashMapWithBucketsNumber[K, V](8)
}

func (hashMap *HashMap[K, V]) GetBucketsNumber() int {
	return len(hashMap.buckets)
}

func (hashMap *HashMap[K, V]) usedBuckets() int {
	counter := 0
	for _, item := range hashMap.buckets {
		if item.entries.GetSize() > 0 {
			counter++
		}
	}
	return counter
}

func (hashMap *HashMap[K, V]) calculateNewBuckets(newSize int) {
	newBuckets := make([]hashMapBucket[K, V], newSize)
	for _, bucket := range hashMap.buckets {
		iterator := bucket.entries.Iterator()
		for iterator.HasNext() {
			bucketEntry := iterator.GetNext()
			newIndex := bucketEntry.key.Hash() % uint64(newSize)
			newBuckets[newIndex].entries.AddLast(*bucketEntry)
		}
	}
	hashMap.buckets = newBuckets
}

func (hashMap *HashMap[K, V]) Insert(key K, value V) {
	index := key.Hash() % uint64(hashMap.GetBucketsNumber())
	entry := hashMapEntry[K, V]{
		key:   key,
		value: value,
	}
	hashMap.buckets[index].entries.AddLast(entry)
	if hashMap.usedBuckets() == hashMap.GetBucketsNumber() {
		newBucketsNumber := hashMap.GetBucketsNumber() << 1
		hashMap.calculateNewBuckets(newBucketsNumber)
	}
}

func (hashMap *HashMap[K, V]) Get(key K) *V {
	index := key.Hash() % uint64(hashMap.GetBucketsNumber())
	entries := &hashMap.buckets[index].entries
	iterator := entries.Iterator()
	for iterator.HasNext() {
		item := iterator.GetNext()
		if item.key == key {
			return &item.value
		}
	}
	return nil
}

func (hashMap *HashMap[K, V]) Contains(key K) bool {
	return hashMap.Get(key) != nil
}

func (hashMap *HashMap[K, V]) Delete(key K) {
	index := key.Hash() % uint64(hashMap.GetBucketsNumber())
	entries := &hashMap.buckets[index].entries
	entries.DeleteFirstMatching(func(entry hashMapEntry[K, V]) bool {
		return entry.key == key
	})
	if len(hashMap.buckets) > 2 && hashMap.usedBuckets() < (hashMap.GetBucketsNumber()>>1) {
		newBucketsNumber := hashMap.GetBucketsNumber() >> 1
		hashMap.calculateNewBuckets(newBucketsNumber)
	}
}
