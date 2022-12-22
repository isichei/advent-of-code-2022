package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Direction string

const (
	Up    Direction = "U"
	Down  Direction = "D"
	Left  Direction = "L"
	Right Direction = "R"
)

type Pos struct {
	i int
	j int
}

type Move struct {
	direction Direction
	steps     int
}

func (m Move) str() string {
	return fmt.Sprintf("(%s, %d)", m.direction, m.steps)
}

func conv_str_to_direction(s string) Direction {
	switch s {
	case "U":
		return Up
	case "D":
		return Down
	case "L":
		return Left
	case "R":
		return Right
	default:
		panic(fmt.Sprintf("Was expecting valid direction got %s", s))
	}
}

func conv_direction_to_pos(d Direction) Pos {
	switch d {
	case Up:
		return Pos{-1, 0}
	case Down:
		return Pos{1, 0}
	case Left:
		return Pos{0, -1}
	case Right:
		return Pos{0, 1}
	default:
		panic(fmt.Sprintf("Unknown direction %s", d))
	}
}

func get_new_pos(upstream *Pos, current *Pos) Pos {
	diff_pos := Pos{upstream.i - current.i, upstream.j - current.j}
	if diff_pos.i > 2 || diff_pos.i < -2 || diff_pos.j < -2 || diff_pos.j > 2 {
		panic("Change too large")
	}
	switch diff_pos {
	// Diag
	case Pos{-2, 1}, Pos{-1, 2}, Pos{-2, 2}:
		return Pos{-1, 1}
	case Pos{1, 2}, Pos{2, 1}, Pos{2, 2}:
		return Pos{1, 1}
	case Pos{1, -2}, Pos{2, -1}, Pos{2, -2}:
		return Pos{1, -1}
	case Pos{-1, -2}, Pos{-2, -1}, Pos{-2, -2}:
		return Pos{-1, -1}
	// Normal
	case Pos{0, 2}:
		return Pos{0, 1}
	case Pos{0, -2}:
		return Pos{0, -1}
	case Pos{2, 0}:
		return Pos{1, 0}
	case Pos{-2, 0}:
		return Pos{-1, 0}
	default:
		return Pos{0, 0}
	}
}

func apply_move(rope []Pos, move_pos Pos) {

	// Move the Head
	rope[0].i += move_pos.i
	rope[0].j += move_pos.j
	for k := 1; k < len(rope); k++ {
		change := get_new_pos(&rope[k-1], &rope[k])
		fmt.Printf("change: %v\n", change)
		rope[k].i += change.i
		rope[k].j += change.j
	}
}

func readlines(fp string) []Move {
	var moves []Move
	file, err := ioutil.ReadFile(fp)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}

	for _, line := range strings.Split(string(file), "\n") {
		split := strings.SplitN(line, " ", 2)
		d := conv_str_to_direction(split[0])
		steps, err := strconv.Atoi(split[1])
		if err != nil {
			panic(err)
		}

		moves = append(moves, Move{d, steps})
	}

	return moves
}

func main() {
	fp := os.Args[1]
	fmt.Println(fp)
	moves := readlines(fp)
	// rope := []Pos{{0, 0}, {0, 0}} part 1
	rope := []Pos{{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}} // part 2
	tail_idx := len(rope) - 1
	tail_pos := map[Pos]int{}

	for i, move := range moves {
		step := 0
		move_pos := conv_direction_to_pos(move.direction)
		for step < move.steps {
			apply_move(rope, move_pos)
			v, prs := tail_pos[rope[tail_idx]]
			if prs {
				tail_pos[rope[tail_idx]] = v + 1
			} else {
				tail_pos[rope[tail_idx]] = 1
			}
			step++
			fmt.Printf("%d, step: %s%d, %v\n", i, move.direction, step, rope)
		}
	}

	fmt.Println(len(tail_pos))
}
