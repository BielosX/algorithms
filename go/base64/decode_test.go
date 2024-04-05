package base64

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestBase64decode(t *testing.T) {
	testCases := []string{
		"VGVzdCBTdHJpbmc=",
		"VA==",
		"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdA==",
	}
	expected := [][]byte{
		[]byte("Test String"),
		[]byte("T"),
		[]byte("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
	}

	for idx, tc := range testCases {
		t.Run(fmt.Sprintf("Base64decode of %s", tc), func(tst *testing.T) {
			result := Base64decode([]byte(tc))

			assert.Equal(tst, expected[idx], result)
		})
	}
}

func TestMapBase64ToValue(t *testing.T) {
	testCases := []byte{'+', '/', '=', 'V', 'c', '7'}
	expectedResults := []byte{62, 63, 0, 21, 28, 59}

	for idx, tc := range testCases {
		t.Run(fmt.Sprintf("mapBase64ToValue of %c", tc), func(tst *testing.T) {
			result := mapBase64ToValue(tc)

			assert.Equal(tst, expectedResults[idx], result)
		})
	}
}
