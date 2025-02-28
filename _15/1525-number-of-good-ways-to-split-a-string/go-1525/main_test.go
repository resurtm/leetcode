package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := numSplits("aacaba")
	expected := 2
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := numSplits("abcd")
	expected := 1
	assert.Equal(t, expected, actual)
}
