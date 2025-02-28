package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := findMissingAndRepeatedValues([][]int{{1, 3}, {2, 2}})
	expected := []int{2, 4}
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := findMissingAndRepeatedValues([][]int{{9, 1, 7}, {8, 9, 2}, {3, 4, 6}})
	expected := []int{9, 5}
	assert.Equal(t, expected, actual)
}
