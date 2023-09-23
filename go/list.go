package list

import "fmt"

type doublyLinkedListNode[T any] struct {
	value T
	next  *doublyLinkedListNode[T]
	prev  *doublyLinkedListNode[T]
}

type DoublyLinkedList[T any] struct {
	head *doublyLinkedListNode[T]
	tail *doublyLinkedListNode[T]
	size uint64
}

func NewDoublyLinkedList[T any]() DoublyLinkedList[T] {
	return DoublyLinkedList[T]{
		head: nil,
		tail: nil,
		size: 0,
	}
}

func (lst *DoublyLinkedList[T]) addFirstElement(value T) {
	node := &doublyLinkedListNode[T]{
		value: value,
		next:  nil,
		prev:  nil,
	}
	lst.head = node
	lst.tail = node
	lst.size = 1
}

func (lst *DoublyLinkedList[T]) AddLast(value T) {
	if lst.head == nil && lst.tail == nil {
		lst.addFirstElement(value)
	} else {
		node := &doublyLinkedListNode[T]{
			value: value,
			next:  nil,
			prev:  lst.tail,
		}
		lst.tail.next = node
		lst.tail = node
		lst.size++
	}
}

func (lst *DoublyLinkedList[T]) AddFirst(value T) {
	if lst.head == nil && lst.tail == nil {
		lst.addFirstElement(value)
	} else {
		node := &doublyLinkedListNode[T]{
			value: value,
			next:  lst.head,
			prev:  nil,
		}
		lst.head.prev = node
		lst.head = node
		lst.size++
	}
}

type IndexOutOfBoundsError struct {
	index uint64
	size  uint64
}

func (e *IndexOutOfBoundsError) Error() string {
	return fmt.Sprintf("List of size %d accessed with index %d", e.size, e.index)
}

func newIndexOutOfBoundsError(index uint64, size uint64) *IndexOutOfBoundsError {
	return &IndexOutOfBoundsError{
		index: index,
		size:  size,
	}
}

func (lst *DoublyLinkedList[T]) iterateTo(index uint64) *doublyLinkedListNode[T] {
	current := lst.head
	for idx := uint64(0); idx < index; idx++ {
		current = current.next
	}
	return current
}

func (lst *DoublyLinkedList[T]) AddAt(value T, index uint64) error {
	if index >= lst.size {
		return newIndexOutOfBoundsError(index, lst.size)
	} else {
		current := lst.iterateTo(index)
		node := &doublyLinkedListNode[T]{
			value: value,
			prev:  current.prev,
			next:  current,
		}
		if current.prev != nil {
			current.prev.next = node
		}
		current.prev = node
		if index == 0 {
			lst.head = node
		}
		lst.size++
		return nil
	}
}

func (lst *DoublyLinkedList[T]) GetAt(index uint64) (*T, error) {
	if index >= lst.size {
		return nil, newIndexOutOfBoundsError(index, lst.size)
	} else {
		current := lst.iterateTo(index)
		return &current.value, nil
	}
}

func (lst *DoublyLinkedList[T]) GetLast() *T {
	if lst.tail == nil {
		return nil
	} else {
		return &lst.tail.value
	}
}

func (lst *DoublyLinkedList[T]) GetFirst() *T {
	if lst.head == nil {
		return nil
	} else {
		return &lst.head.value
	}
}
