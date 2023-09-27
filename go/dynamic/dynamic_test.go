package dynamic

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestFibonacci(t *testing.T) {
	testCases := [][]int{
		{0, 0},
		{1, 1},
		{2, 1},
		{3, 2},
		{4, 3},
		{10, 55},
		{15, 610},
		{19, 4181},
	}
	for _, tc := range testCases {
		t.Run(fmt.Sprintf("Fibonacci of %d", tc[0]), func(tst *testing.T) {
			result := Fibonacci(tc[0])
			assert.Equal(tst, tc[1], result)
		})
	}
}
