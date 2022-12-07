package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

// Created a char buffer on day 05 thx me!
// Also created an empty set on day 03 so can use
// that with the buffer. One day I will look up
// I will do go packages to reuse for this year AoC
var exists = struct{}{}

type set struct {
	m map[rune]struct{}
}

func NewSet() *set {
	s := &set{}
	s.m = make(map[rune]struct{})
	return s
}
func (s *set) Add(value rune) {
	s.m[value] = exists
}

type char_buffer struct {
	buff [14]rune
	i    int
	full bool // Go default values were messing up my unique buffer check ...
}

func (sb *char_buffer) Add(c rune) {
	if sb.i == len(sb.buff)-1 {
		sb.i = 0
	} else {
		sb.i++
		if !sb.full {
			sb.full = sb.i == len(sb.buff)-1
		}
	}
	sb.buff[sb.i] = c
}

func (sb *char_buffer) IsUnique() bool {
	s := NewSet()
	for _, c := range sb.buff {
		s.Add(c)
	}
	return len(s.m) == len(sb.buff) && sb.full
}

func NewCharBuffer() *char_buffer {
	cb := char_buffer{}
	cb.i = -1
	cb.full = false
	return &cb
}

// Return the value of the first char
func start_of_packet(s string) int {
	buff := NewCharBuffer()
	for i, c := range s {
		if buff.IsUnique() {
			return i
		} else {
			buff.Add(c)
		}
		// fmt.Printf("rune: %c size: %d\n", c, buff.IsUnique())
	}
	return -1
}

// Not proper testing but who cares
func do_tests() {
	var tests = []string{
		"mjqjpqmgbljsphdztnvjfqwrcgsmlb",
		"bvwbjplbgvbhsrlpgdmjqwftvncz",
		"nppdvjthqldpwncqszvftbrmjlhg",
		"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
		"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
	}
	var ans_p1 = []int{7, 5, 6, 10, 11}
	var ans_p2 = []int{19, 23, 23, 29, 26}
	for i, test := range tests {
		fmt.Printf("expected: %d or %d got: %d | %s\n", ans_p1[i], ans_p2[i], start_of_packet(test), test)
	}
}

func read_message(path string) string {
	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	fileContent := string(file)
	for _, line := range strings.Split(fileContent, "\n") {
		return line
	}
	return ""
}

func main() {
	fp := os.Args[1]
	if fp == "test" {
		do_tests()
	} else {
		s := read_message(fp)
		fmt.Println(start_of_packet(s))
	}
}
