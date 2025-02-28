package main

import (
	"container/heap"
	"sort"
)

type IntHeap []int

func (h IntHeap) Len() int {
	return len(h)
}

func (h IntHeap) Less(i, j int) bool {
	return h[i] < h[j]
}

func (h IntHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func maxEvents(events [][]int) int {
	days := events[0][1]
	sort.Slice(events, func(i, j int) bool {
		if events[i][1] > days {
			days = events[i][1]
		}
		if events[j][1] > days {
			days = events[j][1]
		}
		if events[i][0] == events[j][0] {
			return events[i][1] < events[j][1]
		}
		return events[i][0] < events[j][0]
	})

	h := &IntHeap{}
	heap.Init(h)

	day, eventId, c := 1, 0, 0
	for day <= days {
		// fast forward time, if no events to visit available
		if eventId < len(events) && h.Len() == 0 {
			day = events[eventId][0]
		}

		// add all events starting today to heap
		for eventId < len(events) && events[eventId][0] <= day {
			heap.Push(h, events[eventId][1])
			eventId += 1
		}

		// remove all ended events from heap
		for h.Len() > 0 && (*h)[0] < day {
			heap.Pop(h)
		}

		// visit earliest (in heap) event
		if h.Len() > 0 {
			heap.Pop(h)
			c += 1
		} else if eventId >= len(events) {
			break
		}

		day += 1
	}
	return c
}
