package matrix

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestNewMatrix_ZeroRows(t *testing.T) {
	_, err := NewMatrix[int](0, 5)

	assert.Error(t, err, "rows and columns should be greater than zero")
}

func TestNewMatrix_ZeroColumns(t *testing.T) {
	_, err := NewMatrix[int](5, 0)

	assert.Error(t, err, "rows and columns should be greater than zero")
}

func TestNewMatrix(t *testing.T) {
	matrix, _ := NewMatrix[int](5, 7)

	assert.Equal(t, 5, matrix.GetRows())
	assert.Equal(t, 7, matrix.GetColumns())
}

func TestNewMatrixFromArray_DifferentRowSize(t *testing.T) {
	_, err := NewMatrixFromArray[int]([][]int{
		{1, 2, 3},
		{4, 5},
	})

	assert.Error(t, err, "every row should be the same size")
}

func TestNewMatrixFromArray(t *testing.T) {
	matrix, _ := NewMatrixFromArray[int]([][]int{
		{1, 2},
		{3, 4},
	})

	assert.Equal(t, 1, *matrix.Get(0, 0))
	assert.Equal(t, 2, *matrix.Get(0, 1))
	assert.Equal(t, 3, *matrix.Get(1, 0))
	assert.Equal(t, 4, *matrix.Get(1, 1))
}

func TestMatrix_Multiply_DifferentSize(t *testing.T) {
	first, _ := NewMatrix[int](5, 7)
	second, _ := NewMatrix[int](2, 7)

	_, err := first.Multiply(second)

	assert.Error(t, err, "number of lhs columns should be the same as rhs rows")
}

func TestMatrix_Multiply(t *testing.T) {
	first, _ := NewMatrixFromArray([][]int{
		{1, 2, 3},
		{4, 5, 6},
	})
	second, _ := NewMatrixFromArray([][]int{
		{2, 1},
		{3, 1},
		{4, 1},
	})

	result, _ := first.Multiply(second)

	assert.Equal(t, 20, *result.Get(0, 0))
	assert.Equal(t, 6, *result.Get(0, 1))
	assert.Equal(t, 47, *result.Get(1, 0))
	assert.Equal(t, 15, *result.Get(1, 1))
}

func TestMultiplyMatrices_NoMatrixProvided(t *testing.T) {
	_, err := MultiplyMatrices[int]()

	assert.Error(t, err, "no matrix provided")
}

func TestMultiplyMatrices_SingleMatrixProvided(t *testing.T) {
	matrix, _ := NewMatrixFromArray([][]int{
		{1, 2, 3},
		{4, 5, 6},
	})

	result, _ := MultiplyMatrices(*matrix)
	assert.Equal(t, result, matrix)
}

func TestMultiplyMatrices_SizeDoesNotMatch(t *testing.T) {
	first, _ := NewMatrixFromArray([][]int{
		{1, 2, 3},
		{4, 5, 6},
	})
	second, _ := NewMatrixFromArray([][]int{
		{2, 1},
		{3, 1},
	})

	_, err := MultiplyMatrices(*first, *second)
	assert.Error(t, err, "matrix size does not match")
}

func TestMultiplyMatrices(t *testing.T) {
	first, _ := NewMatrixFromArray([][]int{
		{1, 2, 3},
		{4, 5, 6},
	})
	second, _ := NewMatrixFromArray([][]int{
		{2, 1},
		{3, 1},
		{4, 1},
	})
	third, _ := NewMatrixFromArray([][]int{
		{1, 2, 3},
		{4, 5, 6},
	})
	expected, _ := NewMatrixFromArray([][]int{
		{44, 70, 96},
		{107, 169, 231},
	})

	result, _ := MultiplyMatrices(*first, *second, *third)
	assert.Equal(t, expected, result)
}
