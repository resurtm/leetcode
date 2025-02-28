package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := maxEvents([][]int{{1, 2}, {2, 3}, {3, 4}})
	expected := 3
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := maxEvents([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 2}})
	expected := 4
	assert.Equal(t, expected, actual)
}

func TestCase3(t *testing.T) {
	actual := maxEvents([][]int{{1, 100_000}})
	expected := 1
	assert.Equal(t, expected, actual)
}

func TestCase4(t *testing.T) {
	actual := maxEvents([][]int{{1, 5}, {1, 5}, {1, 5}, {2, 3}, {2, 3}})
	expected := 5
	assert.Equal(t, expected, actual)
}

func TestCase5(t *testing.T) {
	actual := maxEvents([][]int{{3, 3}, {1, 3}, {2, 3}, {3, 4}, {3, 4}})
	expected := 4
	assert.Equal(t, expected, actual)
}
