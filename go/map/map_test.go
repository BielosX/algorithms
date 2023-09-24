package _map

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestEmptyMap(t *testing.T) {
	uat := NewTreeMap[int, string]()

	assert.False(t, uat.Contains(5))
}

func TestFindInsertedValue(t *testing.T) {
	uat := NewTreeMap[int, string]()

	uat.Insert(5, "Test")
	uat.Insert(2, "Test2")
	uat.Insert(7, "Test3")
	uat.Insert(1, "Test4")

	assert.Equal(t, "Test", *uat.Get(5))
	assert.Equal(t, "Test2", *uat.Get(2))
	assert.Equal(t, "Test3", *uat.Get(7))
	assert.Equal(t, "Test4", *uat.Get(1))
}
