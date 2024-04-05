package base64

func Base64encode(bytes []byte) []byte {
	var result []byte = nil
	carryBits := 2
	var previousCarry byte = 0
	length := len(bytes)
	for index, element := range bytes {
		var carryMask byte = ^(0xFF << carryBits)
		carry := element & carryMask
		value := (element & ^carryMask) >> carryBits
		value += previousCarry
		base64Value := mapValueToBase64(value)
		result = append(result, base64Value)
		previousCarry = carry << (6 - carryBits)
		if index == length-1 && carryBits < 6 {
			base64Value = mapValueToBase64(carry << (6 - carryBits))
			result = append(result, base64Value)
		}
		if carryBits == 6 {
			base64Value = mapValueToBase64(carry)
			result = append(result, base64Value)
			carryBits = 2
			previousCarry = 0
		} else {
			carryBits += 2
		}
	}
	padding := len(result) % 4
	for range 4 - padding {
		result = append(result, '=')
	}
	return result
}

var valueToChar = [...]byte{25, 'A', 51, 'a', 61, '0', 62, '+', 63, '/'}

func mapValueToBase64(value byte) byte {
	var firstValue byte = 0
	var result byte = 0
	for idx := 0; idx < len(valueToChar); idx += 2 {
		lastValue := valueToChar[idx]
		firstSymbol := valueToChar[idx+1]
		if value <= lastValue {
			offset := value - firstValue
			result = firstSymbol + offset
			break
		}
		firstValue = lastValue + 1
	}
	return result
}
