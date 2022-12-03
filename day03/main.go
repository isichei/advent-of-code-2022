package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

// Stolen from https://www.davidkaya.com/sets-in-golang/
var exists = struct{}{}

type set struct {
	m map[string]struct{}
}

func NewSet() *set {
	s := &set{}
	s.m = make(map[string]struct{})
	return s
}
func (s *set) Add(value string) {
	s.m[value] = exists
}
func (s *set) Remove(value string) {
	delete(s.m, value)
}
func (s *set) Contains(value string) bool {
	_, c := s.m[value]
	return c
}

type counter struct {
	m map[string]int
}

func NewCounter() *counter {
	s := &counter{}
	s.m = make(map[string]int)
	return s
}
func (s *counter) Add(k string) {
	v, prs := s.m[k]
	if prs {
		s.m[k] = v + 1
	} else {
		s.m[k] = 1
	}
}

type Rucksack struct {
	left  []string
	right []string
}

func find_duplicate(lists [][]string) string {

	counter := NewCounter()

	for _, list := range lists {
		set := NewSet()
		for _, item := range list {
			set.Add(item)
		}
		for k := range set.m {
			counter.Add(k)
		}
	}

	for k, v := range counter.m {
		if v == len(lists) {
			return k
		}
	}
	panic("Oh Oh! Couldn't find a match...")
}

func NewRucksack(items string) Rucksack {
	l := len(items)
	ruck := Rucksack{left: []string{}, right: []string{}}
	for i, str := range strings.Split(items, "") {
		if i < l/2 {
			ruck.left = append(ruck.left, str)
		} else {
			ruck.right = append(ruck.right, str)
		}
	}
	return ruck
}

func priority_map() map[string]int {
	m := make(map[string]int)
	priority := 1
	for i := 'a'; i <= 'z'; i++ {
		m[string(i)] = priority
		priority++
	}
	for i := 'A'; i <= 'Z'; i++ {
		m[string(i)] = priority
		priority++
	}
	return m
}

// Return string
func read_lines(path string) []Rucksack {
	// read the whole content of file and pass it to file variable, in case of error pass it to err variable
	var bags []Rucksack
	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	// convert the file binary into a string using string
	fileContent := string(file)
	for _, line := range strings.Split(fileContent, "\n") {
		bags = append(bags, NewRucksack(line))
	}
	return bags
}

func part1(input string) {
	bags := read_lines(input)
	pm := priority_map()
	score := 0
	for _, bag := range bags {
		lists := make([][]string, 2)
		lists[0] = bag.left
		lists[1] = bag.right
		dup := find_duplicate(lists)
		p := pm[dup]
		score += p
	}
	fmt.Printf("Part 1: %d\n", score)
}

func part2(input string) {
	bags := read_lines(input)
	pm := priority_map()
	score := 0
	bag_group_size := 3
	lists := make([][]string, 3)
	for i, bag := range bags {
		bag_counter := i % bag_group_size
		if bag_counter == 0 {
			// Reset
			lists = make([][]string, bag_group_size)
		}
		lists[bag_counter] = append(bag.left, bag.right...)
		if bag_counter == 2 {
			dup := find_duplicate(lists)
			p := pm[dup]
			score += p
		}
	}
	fmt.Printf("Part 2: %d\n", score)
}

func main() {
	path := os.Args[1]
	part1(path)
	part2(path)
}
