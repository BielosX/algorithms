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

func TestCutRod_PriceArrayTooSmall(t *testing.T) {
	_, _, err := CutRodTopDown([]int{1, 2}, 5)

	assert.Error(t, err, "price array length should be at least rodLength + 1")
}

type cutRodTestCase struct {
	rodLength   int
	result      int
	optimalCuts []int
}

func TestCutRod(t *testing.T) {
	price := []int{0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30}
	testCases := []cutRodTestCase{
		{1, 1, []int{1}},
		{2, 5, []int{2}},
		{3, 8, []int{3}},
		{4, 10, []int{2, 2}},
		{5, 13, []int{2, 3}},
		{6, 17, []int{6}},
		{7, 18, []int{1, 6}},
		{8, 22, []int{2, 6}},
		{9, 25, []int{3, 6}},
		{10, 30, []int{10}},
	}
	for _, tc := range testCases {
		t.Run(fmt.Sprintf("CutRod of %d", tc.rodLength), func(tst *testing.T) {
			result, cuts, _ := CutRodTopDown(price, tc.rodLength)
			assert.Equal(tst, tc.result, result)
			assert.ElementsMatch(tst, tc.optimalCuts, cuts)
			result, cuts, _ = CutRodBottomUp(price, tc.rodLength)
			assert.Equal(tst, tc.result, result)
			assert.ElementsMatch(tst, tc.optimalCuts, cuts)
		})
	}
}

func TestMatrixMultiplicationBottomUp(t *testing.T) {
	sizes := []int{5, 10, 3, 12, 5, 50, 6}

	multiplications, ranges := MatrixMultiplicationBottomUp(sizes)

	assert.Equal(t, 2010, multiplications)
	assert.Equal(t, ranges, [][]int{
		{1, 6},
		{1, 2},
		{3, 6},
		{1, 1},
		{2, 2},
		{3, 4},
		{5, 6},
		{3, 3},
		{4, 4},
		{5, 5},
		{6, 6},
	})
}
