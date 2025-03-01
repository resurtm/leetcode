package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := countMatchingSubarrays([]int{1, 2, 3, 4, 5, 6}, []int{1, 1})
	expected := 4
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := countMatchingSubarrays([]int{1, 4, 4, 1, 3, 5, 5, 3}, []int{1, 0, -1})
	expected := 2
	assert.Equal(t, expected, actual)
}
