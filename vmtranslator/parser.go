package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Parser struct {
	fileName string
	commands []VMCommand
}

type commandType int

const (
	ADD commandType = iota
	SUB
	NEG
	EQ
	GT
	LT
	AND
	OR
	NOT
	POP
	PUSH
	LABEL
	GOTO
	IFGOTO
	FUNC
	CALL
	RET
)

type segmentType int

const (
	LOCAL segmentType = iota
	ARGUMENT
	THIS
	THAT
	CONSTANT
	STATIC
	POINTER
	TEMP
)

type VMCommand interface {
	isVMCommand()
}

func (_ Single) isVMCommand() {}
func (_ PP) isVMCommand()     {}
func (_ FC) isVMCommand()     {}
func (_ Func) isVMCommand()   {}

type Single struct {
	commandType commandType
	orig        string
}

func (s Single) String() string {
	return s.orig
}

type PP struct {
	commandType commandType
	segmentType segmentType
	segmentName string
	dst         uint64
	orig        string
}

func (p PP) String() string {
	return p.orig
}

type FC struct {
	commandType commandType
	label       string
	orig        string
}

func (fc FC) String() string {
	return fc.orig
}

type Func struct {
	commandType commandType
	name        string
	n           uint
	orig        string
}

func (f Func) String() string {
	return f.orig
}

func (f Func) GetN() uint {
	return f.n
}

func (f Func) GetName() string {
	return f.name
}

func (p *Parser) push(vc VMCommand) {
	p.commands = append(p.commands, vc)
}

func newParser(filename string) (*Parser, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	parser := Parser{fileName: filename}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		l := scanner.Text()
		if i := strings.Index(l, "//"); i > 0 {
			l = l[0:i]
		} else if i == 0 {
			continue
		}
		ls := strings.Fields(l)
		if len(ls) == 0 { // Empty line
			continue
		}
		commandType := commandType(-1)
		// Handle ADD, SUB and other arithmetics
		switch ls[0] {
		case "add":
			commandType = ADD
		case "sub":
			commandType = SUB
		case "neg":
			commandType = NEG
		case "eq":
			commandType = EQ
		case "gt":
			commandType = GT
		case "lt":
			commandType = LT
		case "and":
			commandType = AND
		case "or":
			commandType = OR
		case "not":
			commandType = NOT
		case "return":
			commandType = RET
		}
		if commandType >= 0 {
			if len(ls) != 1 {
				return nil, fmt.Errorf("Command %s malformed, wrong number of arguments", ls[0])
			}
			parser.push(
				&Single{
					commandType: commandType,
					orig:        strings.Join(ls, " "),
				},
			)
			continue
		}
		// Handle PUSH and POP
		switch ls[0] {
		case "push":
			commandType = PUSH
		case "pop":
			commandType = POP
		}
		if commandType >= 0 {
			if len(ls) != 3 {
				return nil, fmt.Errorf("Command %s malformed, wrong number of arguments", l)
			}
			dst, err := strconv.ParseUint(ls[2], 10, 64)
			if err != nil {
				return nil, fmt.Errorf("Command %s malformed, could't parse destination address, error was '%s'", ls[0], err.Error())
			}
			segment := segmentType(-1)
			switch ls[1] {
			case "local":
				segment = LOCAL
			case "argument":
				segment = ARGUMENT
			case "this":
				segment = THIS
			case "that":
				segment = THAT
			case "constant":
				segment = CONSTANT
			case "static":
				segment = STATIC
			case "pointer":
				segment = POINTER
			case "temp":
				segment = TEMP
			}
			if segment == -1 {
				return nil, fmt.Errorf("Command %s malformed, wrong segment name '%s'", ls[0], ls[1])
			}
			parser.push(
				&PP{
					commandType: commandType,
					segmentType: segment,
					segmentName: ls[1],
					dst:         dst,
					orig:        strings.Join(ls, " "),
				},
			)
			continue
		}
		// Handle Flow Control commands
		switch ls[0] {
		case "label":
			commandType = LABEL
		case "goto":
			commandType = GOTO
		case "if-goto":
			commandType = IFGOTO
		}
		if commandType >= 0 {
			if len(ls) != 2 {
				return nil, fmt.Errorf("Command %s malformed, wrong number of arguments", ls[0])
			}
			parser.push(
				&FC{
					commandType: commandType,
					label:       ls[1],
					orig:        strings.Join(ls, " "),
				},
			)
			continue
		}
		switch ls[0] {
		case "function":
			commandType = FUNC
		case "call":
			commandType = CALL
		}
		if commandType >= 0 {
			if len(ls) != 3 {
				return nil, fmt.Errorf("Command %s malformed, wrong number of arguments", ls[0])
			}
			n, err := strconv.ParseUint(ls[2], 10, 64)
			if err != nil {
				return nil, fmt.Errorf("Command %s malformed, could't parse number, error was '%s'", ls[0], err.Error())
			}
			parser.push(
				&Func{
					commandType: commandType,
					name:        ls[1],
					n:           uint(n),
					orig:        strings.Join(ls, " "),
				},
			)
			continue
		}
		if commandType == -1 {
			return nil, fmt.Errorf("Wrong command '%s'", ls[0])
		}
	}
	return &parser, nil

}
