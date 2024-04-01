package base64

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestBase64encode(t *testing.T) {
	result := Base64encode([]byte("Test String"))

	assert.Equal(t, []byte("VGVzdCBTdHJpbmc="), result)
}

func TestBase64encodeSingleCharacter(t *testing.T) {
	result := Base64encode([]byte("T"))

	assert.Equal(t, []byte("VA=="), result)
}
