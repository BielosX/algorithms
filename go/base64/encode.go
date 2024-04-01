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

func mapValueToBase64(value byte) byte {
	if value <= 25 {
		return 'A' + value
	}
	if value <= 51 {
		offset := value - 26
		return 'a' + offset
	}
	if value <= 61 {
		offset := value - 52
		return '0' + offset
	}
	if value == 62 {
		return '+'
	}
	if value == 63 {
		return '/'
	}
	return 0
}
