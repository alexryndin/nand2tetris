package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	if l := len(os.Args); l != 2 && l != 3 {
		fmt.Printf("Usage: %s src\n", os.Args[0])
		os.Exit(0)
	}
	inputFileName := os.Args[1]
	outputFileName := fmt.Sprintf("%s.asm", strings.TrimSuffix(inputFileName, filepath.Ext(inputFileName)))
	if len(os.Args) == 3 {
		outputFileName = os.Args[2]
	}
	p, err := newParser(inputFileName)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}
	t, err := newTranslator(p, inputFileName)
	if err := t.translate(outputFileName); err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

}
