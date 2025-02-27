package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := rearrangeArray([]int{3, 1, -2, -5, 2, -4})
	expected := []int{3, -2, 1, -5, 2, -4}
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := rearrangeArray([]int{-1, 1})
	expected := []int{1, -1}
	assert.Equal(t, expected, actual)
}
