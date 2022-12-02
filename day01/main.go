package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

// Read in a files as a string based on a filepath
func read_file_as_str(path string) string {
	// read the whole content of file and pass it to file variable, in case of error pass it to err variable
	file, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
		panic(err)
	}
	// convert the file binary into a string using string
	fileContent := string(file)

	return fileContent
}

type elf struct {
	elf_id   int
	calories int
}

func newElf(elf_id int) *elf {
	p := elf{elf_id: elf_id}
	return &p
}

type elves []elf

func main() {
	inputFile := os.Args[1]
	all_elves := []elf{
		{elf_id: 0},
	}
	for _, line := range strings.Split(read_file_as_str(inputFile), "\n") {
		if line == "" {
			e := newElf(all_elves[len(all_elves)-1].elf_id + 1)
			all_elves = append(all_elves, *e)
		} else {
			int_value, err := strconv.Atoi(strings.TrimSpace(line))
			if err != nil {
				panic(err)
			}
			all_elves[len(all_elves)-1].calories += int_value
		}
	}

	sort.Slice(all_elves, func(i, j int) bool {
		return all_elves[i].calories > all_elves[j].calories
	})

	for i, an_elf := range all_elves {
		fmt.Printf("Rank %d, elf_id: %d has %d calories\n", i, an_elf.elf_id, an_elf.calories)
		if i == 2 {
			break
		}
	}
}
