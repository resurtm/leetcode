package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := applyOperations([]int{1, 2, 2, 1, 1, 0})
	expected := []int{1, 4, 2, 0, 0, 0}
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := applyOperations([]int{0, 1})
	expected := []int{1, 0}
	assert.Equal(t, expected, actual)
}
