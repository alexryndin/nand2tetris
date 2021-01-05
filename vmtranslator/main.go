package main

import (
	"fmt"
	"io/ioutil"
	"log"
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
	var outputFileName string
	fi, err := os.Stat(inputFileName)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}
	writeInit := false
	switch mode := fi.Mode(); {
	case mode.IsDir():
		if len(os.Args) == 3 {
			outputFileName = os.Args[2]
		} else {
			absPath, _ := filepath.Abs(inputFileName)
			baseName := fmt.Sprintf("%s.asm", filepath.Base(absPath))
			outputFileName = filepath.Join(absPath, baseName)
		}
		writeInit = true
		files, err := ioutil.ReadDir(inputFileName)
		if err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}

		t, err := newEmptyTranslator(outputFileName, writeInit)
		if err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}
		defer t.close()
		log.Println("Starting translator")

		for _, f := range files {
			ext := filepath.Ext(f.Name())
			if ext != ".vm" {
				log.Printf("Skipping %s\n", f.Name())
				continue
			}
			p, err := newParser(filepath.Join(inputFileName, f.Name()))
			if err != nil {
				fmt.Println(err.Error())
				os.Exit(1)
			}
			t.init(p)
			if err := t.translate(writeInit); err != nil {
				fmt.Println(err.Error())
				os.Exit(1)
			}
		}
	case mode.IsRegular():
		if len(os.Args) == 3 {
			outputFileName = os.Args[2]
		} else {
			outputFileName = fmt.Sprintf("%s.asm", strings.TrimSuffix(inputFileName, filepath.Ext(inputFileName)))
		}
		p, err := newParser(inputFileName)
		if err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}
		t, err := newTranslator(p, outputFileName, writeInit)
		if err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}
		defer t.close()
		if err := t.translate(writeInit); err != nil {
			fmt.Println(err.Error())
			os.Exit(1)
		}
	}

}
