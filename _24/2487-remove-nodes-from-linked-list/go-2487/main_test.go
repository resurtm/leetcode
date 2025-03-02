package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	actual := removeNodes(&ListNode{
		Val: 5,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 13,
				Next: &ListNode{
					Val: 3,
					Next: &ListNode{
						Val:  8,
						Next: nil,
					},
				},
			},
		},
	})
	expected := &ListNode{
		Val: 13,
		Next: &ListNode{
			Val:  8,
			Next: nil,
		},
	}
	assert.Equal(t, expected, actual)
}

func TestCase2(t *testing.T) {
	actual := removeNodes(&ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 1,
					Next: &ListNode{
						Val:  1,
						Next: nil,
					},
				},
			},
		},
	})
	expected := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 1,
					Next: &ListNode{
						Val:  1,
						Next: nil,
					},
				},
			},
		},
	}
	assert.Equal(t, expected, actual)
}

func TestCase3(t *testing.T) {
	actual := removeNodes(&ListNode{
		Val: 9,
		Next: &ListNode{
			Val: 8,
			Next: &ListNode{
				Val: 7,
				Next: &ListNode{
					Val: 6,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	})
	expected := &ListNode{
		Val: 9,
		Next: &ListNode{
			Val: 8,
			Next: &ListNode{
				Val: 7,
				Next: &ListNode{
					Val: 6,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	}
	assert.Equal(t, expected, actual)
}
