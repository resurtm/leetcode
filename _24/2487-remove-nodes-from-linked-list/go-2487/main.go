package main

import "math"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNodes(curr *ListNode) *ListNode {
	// return removeNodesStack(curr)
	return removeNodesReverse(curr)
}

func removeNodesStack(curr *ListNode) *ListNode {
	st := []*ListNode{{Val: math.MaxInt}}
	for curr != nil {
		for st[len(st)-1].Val < curr.Val {
			st = st[:len(st)-1]
		}
		st[len(st)-1].Next = curr
		st = append(st, curr)
		curr = curr.Next
	}
	return st[0].Next
}

func reverseList(head *ListNode) *ListNode {
	curr := head
	var prev *ListNode
	for curr != nil {
		curr.Next, prev = prev, curr.Next
		prev, curr = curr, prev
	}
	return prev
}

func removeNodesReverse(curr *ListNode) *ListNode {
	curr = reverseList(curr)
	head := curr
	for curr != nil {
		next := curr.Next
		for next != nil && next.Val < curr.Val {
			next = next.Next
		}
		curr.Next = next
		curr = next
	}
	return reverseList(head)
}

type Entry struct {
	Ok   bool
	Node *ListNode
}

func removeNodesSlow1(curr *ListNode) *ListNode {
	st := []Entry{}
	for curr != nil {
		for idx, entry := range st {
			if !entry.Ok {
				continue
			}
			if entry.Node.Val < curr.Val {
				st[idx].Ok = false
			}
		}
		st = append(st, Entry{Ok: true, Node: curr})
		curr = curr.Next
	}
	var res *ListNode = nil
	var node *ListNode = nil
	for _, entry := range st {
		if !entry.Ok {
			continue
		}
		if res == nil {
			res = entry.Node
		}
		if node == nil {
			node = entry.Node
		}
		node.Next = entry.Node
		node = entry.Node
	}
	return res
}

func removeNodesSlow2(curr *ListNode) *ListNode {
	st := []*ListNode{}
	for curr != nil {
		st_next := []*ListNode{}
		for _, node := range st {
			if node.Val >= curr.Val {
				st_next = append(st_next, node)
			}
		}
		st = append([]*ListNode{curr}, st_next...)
		curr = curr.Next
	}
	for idx, node := range st {
		if idx == 0 {
			node.Next = nil
		} else {
			node.Next = st[idx-1]
		}
	}
	return st[len(st)-1]
}
