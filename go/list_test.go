package list

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestGetLastReturnsNilWhenListIsEmpty(t *testing.T) {
	uat := NewDoublyLinkedList[int]()

	assert.Empty(t, uat.GetLast())
}

func TestGetLastReturnsTailOfSingleElementListAddLast(t *testing.T) {
	uat := NewDoublyLinkedList[int]()
	uat.AddLast(5)

	assert.Equal(t, *uat.GetLast(), 5)
}

func TestGetLastReturnsTailOfMultiElementListAddLast(t *testing.T) {
	uat := NewDoublyLinkedList[int]()
	uat.AddLast(5)
	uat.AddLast(6)
	uat.AddLast(7)

	assert.Equal(t, *uat.GetLast(), 7)
}

func TestAddAtReturnsErrorWhenIndexOutOfBounds(t *testing.T) {
	uat := NewDoublyLinkedList[int]()

	err := uat.AddAt(5, 0)

	assert.Error(t, err)
}

func TestAddAtIndexZeroWhenSingleElement(t *testing.T) {
	uat := NewDoublyLinkedList[int]()
	uat.AddLast(5)
	_ = uat.AddAt(6, 0)
	value, _ := uat.GetAt(0)

	assert.Equal(t, *uat.GetLast(), 5)
	assert.Equal(t, *value, 6)
}

func TestAddAtBetweenTwoValues(t *testing.T) {
	uat := NewDoublyLinkedList[int]()
	uat.AddLast(5)
	uat.AddLast(6)

	_ = uat.AddAt(7, 1)

	first, _ := uat.GetAt(0)
	second, _ := uat.GetAt(1)
	third, _ := uat.GetAt(2)

	assert.Equal(t, *first, 5)
	assert.Equal(t, *second, 7)
	assert.Equal(t, *third, 6)
}

func TestGetFirstReturnsHeadOfMultiElementListAddFirst(t *testing.T) {
	uat := NewDoublyLinkedList[int]()
	uat.AddFirst(5)
	uat.AddFirst(6)
	uat.AddFirst(7)

	assert.Equal(t, *uat.GetFirst(), 7)
}

func TestGetAtReturnsErrorWhenIndexOutOfBounds(t *testing.T) {
	uat := NewDoublyLinkedList[int]()

	_, err := uat.GetAt(10)

	assert.Error(t, err)
	assert.Equal(t, err.Error(), "List of size 0 accessed with index 10")
}

func TestGetFirstReturnsNilWhenListIsEmpty(t *testing.T) {
	uat := NewDoublyLinkedList[int]()

	value := uat.GetFirst()

	assert.Empty(t, value)
}
