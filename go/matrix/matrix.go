package matrix

import (
	"errors"
	"golang.org/x/exp/constraints"
	"runtime"
)

type Number interface {
	constraints.Float | constraints.Integer
}
type Matrix[T Number] struct {
	entries [][]T
}

func (matrix *Matrix[T]) GetRows() int {
	return len(matrix.entries)
}

func (matrix *Matrix[T]) GetColumns() int {
	return len(matrix.entries[0])
}

func NewMatrix[T Number](rows int, columns int) (*Matrix[T], error) {
	if rows <= 0 || columns <= 0 {
		return nil, errors.New("rows and columns should be greater than zero")
	}
	entries := make([][]T, rows)
	for idx := 0; idx < len(entries); idx++ {
		entries[idx] = make([]T, columns)
	}
	return &Matrix[T]{
		entries: entries,
	}, nil
}

func NewMatrixFromArray[T Number](entries [][]T) (*Matrix[T], error) {
	if len(entries) == 0 {
		return nil, errors.New("array length should be at least 1")
	}
	firstLen := len(entries[0])
	for _, entry := range entries {
		if len(entry) != firstLen {
			return nil, errors.New("every row should be the same size")
		}
	}
	return &Matrix[T]{
		entries: entries,
	}, nil
}

func (matrix *Matrix[T]) Get(row int, column int) *T {
	if row > matrix.GetRows()-1 || column > matrix.GetColumns()-1 {
		return nil
	}
	return &matrix.entries[row][column]
}

type multiplicationJob struct {
	lhsRow    int
	rhsColumn int
}

type multiplicationResult[T Number] struct {
	lhsRow    int
	rhsColumn int
	result    T
}

func multiply[T Number](lhs *Matrix[T],
	rhs *Matrix[T],
	jobs <-chan multiplicationJob,
	results chan<- multiplicationResult[T]) {
	for job := range jobs {
		var result T
		for idx := 0; idx < lhs.GetColumns(); idx++ {
			if idx == 0 {
				result = *lhs.Get(job.lhsRow, idx) * *rhs.Get(idx, job.rhsColumn)
			} else {
				result += *lhs.Get(job.lhsRow, idx) * *rhs.Get(idx, job.rhsColumn)
			}
		}
		results <- multiplicationResult[T]{
			lhsRow:    job.lhsRow,
			rhsColumn: job.rhsColumn,
			result:    result,
		}
	}
}

func (matrix *Matrix[T]) Multiply(rhs *Matrix[T]) (*Matrix[T], error) {
	if matrix.GetColumns() != rhs.GetRows() {
		return nil, errors.New("number of lhs columns should be the same as rhs rows")
	}
	result, err := NewMatrix[T](matrix.GetRows(), rhs.GetColumns())
	if err != nil {
		return nil, err
	}
	jobsNumber := matrix.GetRows() * rhs.GetColumns()
	jobs := make(chan multiplicationJob, jobsNumber)
	results := make(chan multiplicationResult[T], jobsNumber)
	for w := 0; w < runtime.NumCPU(); w++ {
		go multiply(matrix, rhs, jobs, results)
	}
	for row := 0; row < matrix.GetRows(); row++ {
		for column := 0; column < rhs.GetColumns(); column++ {
			jobs <- multiplicationJob{
				lhsRow:    row,
				rhsColumn: column,
			}
		}
	}
	close(jobs)
	for r := 0; r < jobsNumber; r++ {
		partialResult := <-results
		entry := result.Get(partialResult.lhsRow, partialResult.rhsColumn)
		*entry = partialResult.result
	}
	return result, nil
}
