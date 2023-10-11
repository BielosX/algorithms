package dynamic

import (
	"errors"
	"math"
)

func Fibonacci(num int) int {
	if num == 0 {
		return 0
	} else {
		prev := [2]int{0, 1}
		result := 1
		for i := 2; i <= num; i++ {
			result = prev[0] + prev[1]
			prev[0] = prev[1]
			prev[1] = result
		}
		return result
	}
}

func cutRodTopDown(price []int, rodLength int, savedResults []int, optimalFirstCut []int) int {
	if savedResults[rodLength] >= 0 {
		return savedResults[rodLength]
	}
	result := 0
	if rodLength > 0 {
		result = math.MinInt
		prevResult := math.MinInt
		for i := 1; i <= rodLength; i++ {
			prevResult = result
			result = max(result, price[i]+cutRodTopDown(price, rodLength-i, savedResults, optimalFirstCut))
			if result > prevResult {
				optimalFirstCut[rodLength] = i
			}
		}
	}
	savedResults[rodLength] = result
	return result
}

func reconstructSolution(rodLength int, optimalFirstCut []int) []int {
	var solution []int
	length := rodLength
	for length != 0 {
		firstCut := optimalFirstCut[length]
		solution = append(solution, firstCut)
		length -= firstCut
	}
	return solution
}

var priceArrayLengthError = errors.New("price array length should be at least rodLength + 1")

// CutRodTopDown price array index is rod length, price[0] should be 0
func CutRodTopDown(price []int, rodLength int) (int, []int, error) {
	if len(price) < rodLength+1 {
		return 0, []int{}, priceArrayLengthError
	}
	savedResults := make([]int, rodLength+1)
	optimalFirstCut := make([]int, rodLength+1)
	for idx := 0; idx < len(savedResults); idx++ {
		savedResults[idx] = math.MinInt
	}
	result := cutRodTopDown(price, rodLength, savedResults, optimalFirstCut)
	solution := reconstructSolution(rodLength, optimalFirstCut)
	return result, solution, nil
}

// CutRodBottomUp price array index is rod length, price[0] should be 0
func CutRodBottomUp(price []int, rodLength int) (int, []int, error) {
	if len(price) < rodLength+1 {
		return 0, []int{}, priceArrayLengthError
	}
	savedResults := make([]int, rodLength+1)
	optimalFirstCut := make([]int, rodLength+1)
	for currentRodLength := 1; currentRodLength <= rodLength; currentRodLength++ {
		result := math.MinInt
		for cut := 1; cut <= currentRodLength; cut++ {
			cutResult := price[cut] + price[currentRodLength-cut]
			if result < cutResult {
				result = cutResult
				optimalFirstCut[currentRodLength] = cut
			}
		}
		savedResults[currentRodLength] = result
	}
	solution := reconstructSolution(rodLength, optimalFirstCut)
	return savedResults[rodLength], solution, nil
}
