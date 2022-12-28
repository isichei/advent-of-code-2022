package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Direction int

type Pos struct {
	i int
	j int
}

func readlines(fp string) [][]int {
	var trees [][]int

	file, err := ioutil.ReadFile(fp)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	for i, line := range strings.Split(string(file), "\n") {
		trees = append(trees, []int{})
		for _, height := range strings.Split(line, "") {
			x, err := strconv.Atoi(height)
			if err != nil {
				panic(err)
			}
			trees[i] = append(trees[i], x)
		}
	}
	return trees
}

func is_visable_left(trees [][]int, pos *Pos) (bool, int) {
	height := trees[pos.i][pos.j]
	counter := 0
	is_visable := true
	for j := pos.j - 1; j >= 0; j-- {
		counter++
		if trees[pos.i][j] >= height {
			is_visable = false
			break
		}
	}
	return is_visable, counter
}

func is_visable_right(trees [][]int, pos *Pos) (bool, int) {
	height := trees[pos.i][pos.j]
	counter := 0
	is_visable := true
	for j := pos.j + 1; j < len(trees[pos.i]); j++ {
		counter++
		if trees[pos.i][j] >= height {
			is_visable = false
			break
		}
	}
	return is_visable, counter
}

func is_visable_up(trees [][]int, pos *Pos) (bool, int) {
	height := trees[pos.i][pos.j]
	counter := 0
	is_visable := true
	for i := pos.i - 1; i >= 0; i-- {
		counter++
		if trees[i][pos.j] >= height {
			is_visable = false
			break
		}
	}
	return is_visable, counter
}

func is_visable_down(trees [][]int, pos *Pos) (bool, int) {
	height := trees[pos.i][pos.j]
	counter := 0
	is_visable := true
	for i := pos.i + 1; i < len(trees); i++ {
		counter++
		if trees[i][pos.j] >= height {
			is_visable = false
			break
		}
	}
	return is_visable, counter
}

func is_visable(trees [][]int, pos *Pos) bool {
	v, _ := is_visable_left(trees, pos)
	if v {
		return v
	}

	v, _ = is_visable_right(trees, pos)
	if v {
		return v
	}

	v, _ = is_visable_up(trees, pos)
	if v {
		return v
	}

	v, _ = is_visable_down(trees, pos)
	if v {
		return v
	} else {
		return false
	}
}

func score_view(trees [][]int, pos *Pos) int {
	_, l := is_visable_left(trees, pos)
	_, r := is_visable_right(trees, pos)
	_, u := is_visable_up(trees, pos)
	_, d := is_visable_down(trees, pos)
	return l * r * u * d
}

func part1(trees [][]int) int {
	len_i := len(trees)
	len_j := len(trees[0])

	counter := 0
	for i := 0; i < len_i; i++ {
		for j := 0; j < len_j; j++ {
			if is_visable(trees, &Pos{i, j}) {
				counter += 1
			}
		}
	}

	return counter
}

func part2(trees [][]int) (Pos, int) {
	var s int

	len_i := len(trees)
	len_j := len(trees[0])

	best_pos := Pos{0, 0} // Would be zero
	best_score := 0

	// ignore edges
	for i := 1; i < len_i-1; i++ {
		for j := 1; j < len_j-1; j++ {
			p := Pos{i, j}
			s = score_view(trees, &p)
			if s > best_score {
				best_pos = p
				best_score = s
			}
		}
	}
	return best_pos, best_score
}

func main() {
	fp := os.Args[1]
	trees := readlines(fp)
	counter := part1(trees)
	pos, score := part2(trees)
	fmt.Printf("Part 1: %d\n", counter)
	fmt.Printf("Part 2: pos (%d, %d) | score %d\n", pos.i, pos.j, score)
}
