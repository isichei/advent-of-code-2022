package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type OutputType int

const (
	TOTAL_SPACE int = 70000000
	REQUIRED_UNUSED_SPACE int = 30000000

	OT_CD_IN	OutputType = 0
	OT_CD_OUT   OutputType = 1
	OT_LS      	OutputType = 2
	OT_DIR      OutputType = 3
	OT_FILE     OutputType = 4
)

type basic_file struct {
	name string
	size int
	files []int
}

func (bf basic_file) print() {
	fmt.Printf("%s: (%d), [", bf.name, bf.size)
	for _, i := range bf.files {
		fmt.Printf("%d, ", i)
	}
	fmt.Print("]\n")
}

func pop_stack(stack []int) ([]int, int) {
	x := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	return stack, x
}

func NewBasicFile(name string, size int) basic_file {
	return basic_file{name: name, size: size, files: []int{}}
}

func parse_cmd(line string) (OutputType, string, int) {
	if strings.HasPrefix(line, "$ cd") {
		dirname := strings.SplitAfterN(line, "cd ", 2)[1]
		if dirname == ".." {
			return OT_CD_OUT, "", 0
		} else {
			return OT_CD_IN, dirname, 0
		}
	} else if line == "$ ls" {
		return OT_LS, "", 0
	} else if strings.HasPrefix(line, "dir"){
		dirname := strings.SplitAfterN(line, " ", 2)[1]
		return OT_DIR, dirname, 0
	} else {
		out := strings.SplitAfterN(line, " ", 2)
		filename := out[1]
		size, err := strconv.Atoi(strings.TrimSpace(out[0]))
		if err != nil {
			panic(err)
		}
		return OT_FILE, filename, size
	}
}

func get_id_by_name(name string, list_of_ids []int, file_lookup map[int]basic_file) int {
	for _, i := range list_of_ids {
		if file_lookup[i].name == name {
			return i
		}
	}
	panic("Not found")
}

func get_dir_size(current_dir basic_file, file_lookup map[int]basic_file) int {
	size := 0
	for _, i := range current_dir.files {
		size += file_lookup[i].size
	}
	return size
}

func readlines(fp string) []string {
	var lines []string
	file, err := ioutil.ReadFile(fp)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	for _, line := range strings.Split(string(file), "\n") {
		lines = append(lines, line)
	}
	return lines
}

func walk_the_commands(lines []string) map[int]basic_file {
	file_lookup := make(map[int]basic_file)
	file_lookup[0] = NewBasicFile("/", 0)
	filo := []int{0}
	current_id := 0
	current_dir := file_lookup[0]

	for i, line := range(lines[1:]){
		file_id := i+1  // as starting from 1
		cmd, name, s := parse_cmd(line)
		switch cmd {
			case OT_DIR, OT_FILE:
				// fmt.Print("... adding file or dir\n")
				file_lookup[file_id] = NewBasicFile(name, s)
				current_dir.files = append(current_dir.files, file_id)
			case OT_CD_IN:
				// Update the current dir in the lookup
				file_lookup[current_id] = current_dir
				current_id = get_id_by_name(name, current_dir.files, file_lookup)
				filo = append(filo, current_id)
				current_dir = file_lookup[current_id]
				// fmt.Printf("... entering dir: %s\n", name)
			case OT_CD_OUT:
				current_dir.size = get_dir_size(current_dir, file_lookup) 
				// old_dir_name := current_dir.name
				file_lookup[current_id] = current_dir // save it back
				filo, _ = pop_stack(filo)
				current_id = filo[len(filo)-1] // Get top of filo but don't pop
				current_dir = file_lookup[current_id]
				// new_dir_name := current_dir.name
				// fmt.Printf("... existing dir (%s -> %s)\n", old_dir_name, new_dir_name)
			default:
				// fmt.Println("... skip")
		}
	}
	// Save whatever dir finished
	current_dir.size = get_dir_size(current_dir, file_lookup) 
	file_lookup[current_id] = current_dir

	// Finalise the root dir
	current_dir = file_lookup[0]
	current_dir.size = get_dir_size(current_dir, file_lookup)
	file_lookup[0] = current_dir 
	return file_lookup
}

func main(){
	fp := os.Args[1]
	lines := readlines(fp)
	file_map := walk_the_commands(lines)
	part1_answer := 0
	thres := REQUIRED_UNUSED_SPACE - (TOTAL_SPACE - file_map[0].size)
	fmt.Printf("total: %d | thres: %d\n", file_map[0].size, thres)
	min_dir := NewBasicFile("fake dir", TOTAL_SPACE)
	for _, v := range file_map {
		if len(v.files) != 0 && v.size <= 100000 {
			part1_answer += v.size
		}
		if len(v.files) != 0 && v.size >= thres && v.size < min_dir.size {
			min_dir = v
		}
	}
	fmt.Printf("Part 1: %d\n", part1_answer)
	fmt.Printf("Part 2: %d (dir %s)\n", min_dir.size, min_dir.name)
}