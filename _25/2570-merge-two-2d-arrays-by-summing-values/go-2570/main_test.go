package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := mergeArrays([][]int{{1, 2}, {2, 3}, {4, 5}}, [][]int{{1, 4}, {3, 2}, {4, 1}})
	expected := [][]int{{1, 6}, {2, 3}, {3, 2}, {4, 6}}
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := mergeArrays([][]int{{2, 4}, {3, 6}, {5, 5}}, [][]int{{1, 3}, {4, 3}})
	expected := [][]int{{1, 3}, {2, 4}, {3, 6}, {4, 3}, {5, 5}}
	assert.Equal(t, expected, actual)
}
