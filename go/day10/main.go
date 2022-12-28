package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type CommandType string
type Pixel string

const (
	ADDX  CommandType = "addx"
	NOOP  CommandType = "noop"
	LIGHT Pixel       = "#"
	DARK  Pixel       = "."
)

type Command struct {
	cmd         CommandType
	cycle_cost  int
	x           int
	has_started bool
}

func readlines(fp string) []Command {
	var cmds []Command
	file, err := ioutil.ReadFile(fp)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}

	for _, line := range strings.Split(string(file), "\n") {
		if strings.HasPrefix(line, "addx") {
			split := strings.SplitN(line, " ", 2)
			x, err := strconv.Atoi(split[1])
			if err != nil {
				panic(err)
			}
			cmds = append(cmds, Command{ADDX, 1, x, false})
		} else {
			cmds = append(cmds, Command{NOOP, 0, 0, false})
		}
	}

	return cmds
}

func print_status(cycle int, x int, cmd Command, w string) {
	fmt.Printf("cycle: %d, x: %d, cmd: %s, at: %s\n", cycle, x, cmd.cmd, w)
}

func main() {
	fp := os.Args[1]
	cmds := readlines(fp)

	cmd_iter := 0
	cycle := 0
	x := 1
	signal_math := 0
	catch_cycle := 20
	crt := ""
	cmd_return_cycle := 0
	crt_offset := 0
	// var current_cmd Command
	for cmd_iter < len(cmds) {

		// increment cycle and begin cmd
		cycle++
		if cycle-crt_offset > 40 {
			crt_offset += 40
		}
		fmt.Printf("Cycle at %d\n", cycle)
		if !cmds[cmd_iter].has_started {
			cmd_return_cycle = cycle + cmds[cmd_iter].cycle_cost
			cmds[cmd_iter].has_started = true
			fmt.Printf("  Start cmd %s add %d (will end at %d)\n", cmds[cmd_iter].cmd, cmds[cmd_iter].x, cmd_return_cycle)
		}

		// mid cycle (do check)
		// part 1
		fmt.Printf("  X: %d\n", x)
		if cycle == catch_cycle {
			signal_math += cycle * x
			catch_cycle += 40
		}
		// part 2
		if cycle-crt_offset >= x && cycle-crt_offset <= x+2 {
			crt += "#"
		} else {
			crt += "."
		}
		// fmt.Printf("  CRT: %s\n", crt)

		// check cmd and add to x
		if cycle == cmd_return_cycle {
			x += cmds[cmd_iter].x
			fmt.Printf("  Cmd has returned. x=%d\n", x)
			cmd_iter++
		}
	}

	fmt.Println(signal_math)

	for line_count := 0; line_count < 6; line_count++ {
		s := line_count * 40
		e := ((line_count + 1) * 40)
		fmt.Println(crt[s:e])
	}
}
