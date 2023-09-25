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

func TestLinkedList_AddFirst(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddFirst(1)
	uat.AddFirst(2)

	assert.Equal(t, 1, *uat.Get(1))
	assert.Equal(t, 2, *uat.Get(0))
}
func TestLinkedList_AddLast(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddLast(1)
	uat.AddLast(2)

	assert.Equal(t, 1, *uat.Get(0))
	assert.Equal(t, 2, *uat.Get(1))
}

func TestLinkedList_DeleteFirstMatching(t *testing.T) {
	uat := NewLinkedList[int]()
	uat.AddLast(1)
	uat.AddLast(2)
	uat.AddLast(3)
	uat.AddLast(4)

	uat.DeleteFirstMatching(func(x int) bool {
		return x == 3
	})

	assert.Equal(t, uint64(3), uat.size)
	assert.Equal(t, 1, *uat.Get(0))
	assert.Equal(t, 2, *uat.Get(1))
	assert.Equal(t, 4, *uat.Get(2))
}

type TestStruct struct {
	key   int
	value string
}

func TestLinkedList_FindFirst(t *testing.T) {
	uat := NewLinkedList[TestStruct]()
	uat.AddLast(TestStruct{
		key:   1,
		value: "Test1",
	})
	uat.AddLast(TestStruct{
		key:   1,
		value: "Test2",
	})

	value := uat.FindFirst(func(x TestStruct) bool {
		return x.key == 1
	})

	assert.Equal(t, "Test1", value.value)
}

func TestLinkedList_AnyMatchEmptyList(t *testing.T) {
	uat := NewLinkedList[int]()

	assert.False(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}

func TestLinkedList_AnyMatchFalse(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddLast(1)
	uat.AddLast(2)
	uat.AddLast(3)

	assert.False(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}

func TestLinkedList_AnyMatchTrue(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddLast(1)
	uat.AddLast(5)
	uat.AddLast(3)

	assert.False(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}

func TestLinkedList_AllMatchEmptyList(t *testing.T) {
	uat := NewLinkedList[int]()

	assert.False(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}

func TestLinkedList_AllMatchFalse(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddLast(1)
	uat.AddLast(5)
	uat.AddLast(3)

	assert.False(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}
func TestLinkedList_AllMatchTrue(t *testing.T) {
	uat := NewLinkedList[int]()

	uat.AddLast(5)
	uat.AddLast(5)
	uat.AddLast(5)

	assert.True(t, uat.AllMatch(func(x int) bool {
		return x == 5
	}))
}
