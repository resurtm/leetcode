package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := getEncryptedString("dart", 3)
	expected := "tdar"
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := getEncryptedString("aaa", 1)
	expected := "aaa"
	assert.Equal(t, expected, actual)
}
