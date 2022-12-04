package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type task_range [2]int

func _between(x int, r task_range) bool {
	return (x >= r[0] && x <= r[1])
}

func overlap(r1 task_range, r2 task_range) bool {
	return _between(r1[0], r2) || _between(r1[1], r2) || _between(r2[0], r1) || _between(r2[1], r1)
}

func contains(r1 task_range, r2 task_range) bool {
	return (r1[0] >= r2[0] && r1[1] <= r2[1]) || (r2[0] >= r1[0] && r2[1] <= r1[1])
}

func _tasks_str_to_task_range(tasks_str string) task_range {
	tasks := strings.Split(tasks_str, "-")
	v1, err1 := strconv.Atoi(strings.TrimSpace(tasks[0]))
	v2, err2 := strconv.Atoi(strings.TrimSpace(tasks[1]))
	if err1 != nil || err2 != nil {
		panic("Error spliting the tasks string")
	}
	return [2]int{v1, v2}
}

func parse_line(line string) (task_range, task_range) {
	tasks_str := strings.Split(line, ",")
	r1 := _tasks_str_to_task_range(tasks_str[0])
	r2 := _tasks_str_to_task_range(tasks_str[1])
	return r1, r2
}

func readlines(path string) []string {
	// read the whole content of file and pass it to file variable, in case of error pass it to err variable
	var lines []string
	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	// convert the file binary into a string using string
	fileContent := string(file)
	for _, line := range strings.Split(fileContent, "\n") {
		lines = append(lines, line)
	}
	return lines
}

func main() {
	fp := os.Args[1]
	lines := readlines(fp)
	part_1_counter, part_2_counter := 0, 0
	for _, line := range lines {
		if contains((parse_line(line))) {
			part_1_counter += 1
		}
		if overlap((parse_line(line))) {
			part_2_counter += 1
		}
	}
	fmt.Printf("Pairs that fully overlap: %d\n", part_1_counter)
	fmt.Printf("Pairs that overlap: %d\n", part_2_counter)
}
