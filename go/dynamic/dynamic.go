package dynamic

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
