package base64

func Base64decode(encoded []byte) []byte {
	var result []byte
	var previousValue byte = 0
	var borrowShift = 6
	prevZero := true
	for index, element := range encoded {
		if element == '=' {
			break
		}
		var valueMask byte = ^(0xFF << borrowShift)
		decodedValue := mapBase64ToValue(element)
		value := valueMask & decodedValue
		if index != 0 {
			if !prevZero {
				borrow := ^valueMask & decodedValue
				previousValue += borrow >> borrowShift
				result = append(result, previousValue)
			}
			if index == len(encoded)-1 && borrowShift != 0 {
				lastValue := value << borrowShift
				result = append(result, lastValue)
			}
		}
		previousValue = value << (8 - borrowShift)
		borrowShift -= 2
		if borrowShift < 0 {
			prevZero = true
			borrowShift = 6
		} else {
			prevZero = false
		}
	}
	return result
}

var charToValue = [...]byte{'+', '+', 62, '/', '/', 63, '0', '9', 52, 'A', 'Z', 0, 'a', 'z', 26}

func mapBase64ToValue(encoded byte) byte {
	if encoded == '=' {
		return 0
	}
	for idx := 0; idx < len(charToValue); idx += 3 {
		first := charToValue[idx]
		last := charToValue[idx+1]
		firstValue := charToValue[idx+2]
		if encoded >= first && encoded <= last {
			offset := encoded - first
			return firstValue + offset
		}
	}
	return 0
}
