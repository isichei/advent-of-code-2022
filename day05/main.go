package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type command struct {
	move int
	from int
	to   int
}

type char_buffer struct {
	buff [4]rune
	i    int
}

func (sb *char_buffer) IsFull() bool {
	return sb.i == 3
}

func (sb *char_buffer) Add(c rune) {
	sb.i++
	sb.buff[sb.i] = c
}

func (sb *char_buffer) Pop() rune {
	c := sb.buff[sb.i]
	sb.i--
	return c
}

func (sb *char_buffer) Clear() {
	sb.i = -1
}

func NewCharBuffer() *char_buffer {
	cb := char_buffer{}
	cb.i = -1
	return &cb
}

// Used for part 1
func pop_stack(stack []string) ([]string, string) {
	x := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	return stack, x
}

// Used for part 2
func split_stack(stack []string, i int) ([]string, []string) {
	top := stack[len(stack)-i:]
	stack = stack[:len(stack)-i]
	return stack, top
}

func parse_command(line string) command {
	str_commands := strings.Fields(line)
	move, err := strconv.Atoi(str_commands[1])
	if err != nil {
		panic("Could not parse move value")
	}
	from, err := strconv.Atoi(str_commands[3])
	if err != nil {
		panic("Could not parse from value")
	}
	to, err := strconv.Atoi(str_commands[5])
	if err != nil {
		panic("Could not parse to value")
	}
	return command{move, from - 1, to - 1}
}

func str_to_rune_slice(s string) []rune {
	rs := []rune{}
	for _, r := range s {
		rs = append(rs, r)
	}
	return rs
}

func parse_block_diagram(lines []string) [][]string {
	columns := strings.Fields(lines[len(lines)-1])
	last_column, err := strconv.Atoi(columns[len(columns)-1])
	if err != nil {
		fmt.Printf("columns `%s`\n", columns)
		panic(err)
	}

	// Init the stack
	var stacks [][]string
	for i := 0; i < last_column; i++ {
		stacks = append(stacks, []string{})
	}

	buffer := NewCharBuffer()
	for i := len(lines) - 2; i >= 0; i-- {
		// s, e, win_len := 0, 4, 4
		stack_counter := 0
		buffer.Clear()
		for _, c := range lines[i] {
			if buffer.IsFull() {
				if buffer.buff[1] != ' ' {
					stacks[stack_counter] = append(stacks[stack_counter], string(buffer.buff[1]))
				}
				stack_counter++
				buffer.Clear()
			}
			buffer.Add(c)
		}
		if buffer.buff[1] != ' ' {
			stacks[stack_counter] = append(stacks[stack_counter], string(buffer.buff[1]))
		}

	}
	return stacks
}

func readlines(path string) ([]string, []string) {
	// read the whole content of file and pass it to file variable, in case of error pass it to err variable
	commands := []string{}
	block_diagram := []string{}

	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	// convert the file binary into a string using string
	fileContent := string(file)
	for _, line := range strings.Split(fileContent, "\n") {
		s := strings.Trim(line, " ")
		switch {
		case strings.HasPrefix(s, "["):
			block_diagram = append(block_diagram, line)
		case strings.HasPrefix(s, "1"):
			block_diagram = append(block_diagram, line)
		case strings.HasPrefix(s, "move"):
			commands = append(commands, line)
		default:
			fmt.Printf("Unknown line: `%s`\n", line)
		}

	}
	return commands, block_diagram
}

func main() {
	fp := os.Args[1]
	commands_str, block_diagram := readlines(fp)

	stacks := parse_block_diagram(block_diagram)
	var x []string
	for _, c := range commands_str {
		command := parse_command(c)
		stacks[command.from], x = split_stack(stacks[command.from], command.move)
		stacks[command.to] = append(stacks[command.to], x...)
	}
	fmt.Print("Top: ")
	for i := 0; i < len(stacks); i++ {
		fmt.Printf("[%s] ", stacks[i][len(stacks[i])-1])
	}
	fmt.Print("\n")
}
