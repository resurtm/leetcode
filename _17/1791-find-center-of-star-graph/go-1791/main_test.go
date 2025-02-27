package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := findCenter([][]int{{1, 2}, {2, 3}, {4, 2}})
	expected := 2
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := findCenter([][]int{{1, 2}, {5, 1}, {1, 3}, {1, 4}})
	expected := 1
	assert.Equal(t, expected, actual)
}
