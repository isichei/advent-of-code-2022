package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type Outcome int
type Choice int

const (
	Win      Outcome = 6
	Draw     Outcome = 3
	Lose     Outcome = 0
	Rock     Choice  = 1
	Paper    Choice  = 2
	Scissors Choice  = 3
)

func (c Choice) Beats() Choice {
	if c == 1 {
		return 3
	} else {
		return c - 1
	}
}
func (c Choice) LosesTo() Choice {
	if c == 3 {
		return 1
	} else {
		return c + 1
	}
}

func ChoiceFromStringCode(s string) Choice {
	switch {
	case s == "A" || s == "X":
		return Rock
	case s == "B" || s == "Y":
		return Paper
	case s == "C" || s == "Z":
		return Scissors
	default:
		panic("Unknown choice")
	}
}

func OutcomeFromString(s string) Outcome {
	switch s {
	case "X":
		return Lose
	case "Y":
		return Draw
	case "Z":
		return Win
	default:
		panic("OH OH!")
	}
}

type Game struct {
	opp     Choice
	me      Choice
	outcome Outcome
}

func (g Game) solve_my_choice() Game {
	switch g.outcome {
	case Draw:
		g.me = g.opp
	case Win:
		g.me = g.opp.LosesTo()
	case Lose:
		g.me = g.opp.Beats()
	default:
		panic("Whoops!")
	}
	return g
}

func (g Game) score_match() int {
	return int(g.me) + int(g.outcome)
}

func NewGameFromStringInput(s string) Game {
	str_vals := strings.Split(strings.TrimSpace(s), " ")
	opp := ChoiceFromStringCode(str_vals[0])
	m := ChoiceFromStringCode(str_vals[1]) // part1
	out := OutcomeFromString(str_vals[1])  // part2
	return Game{me: m, opp: opp, outcome: out}
}

// Read full file and return slice of Games
func read_file_as_games(path string) []Game {
	var rps_games []Game

	// read the whole content of file and pass it to file variable, in case of error pass it to err variable
	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	// convert the file binary into a string using string
	fileContent := string(file)
	for _, line := range strings.Split(fileContent, "\n") {
		rps_games = append(rps_games, NewGameFromStringInput(line))
	}
	return rps_games
}

func main() {
	s := os.Args[1]
	games := read_file_as_games(s)
	var total_score int
	for _, g := range games {
		g = g.solve_my_choice() // part 2
		total_score += g.score_match()
	}
	fmt.Printf("Total score %d\n", total_score)
}
