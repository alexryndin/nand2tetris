package main

import (
	"fmt"
	"os"
	"strings"
)

const (
	THISADDR = 3
	THATADDR = 4
	TEMPADDR = 5
)

type Translator struct {
	parser   *Parser
	filename string
	id       uint64
}

func newTranslator(parser *Parser, filename string) (*Translator, error) {
	return &Translator{parser: parser, filename: filename}, nil
}

func (t *Translator) getID() uint64 {
	t.id++
	return t.id
}

func (t *Translator) translate(filename string) error {
	file, err := os.Create(filename)
	defer file.Close()
	if err != nil {
		return err
	}
	var sout []string
	for _, v := range t.parser.commands {
		sout = append(sout, "// ")
		switch ct := v.(type) {
		case *PP:
			if translated, err := t.translatePP(ct); err == nil {
				sout = append(sout, ct.orig, "\n", translated)
			} else {
				return err
			}
		case *Single:
			if translated, err := t.translateSingle(ct); err == nil {
				sout = append(sout, ct.orig, "\n", translated)
			} else {
				return err
			}
		default:
			return fmt.Errorf("Unknown command to translate '%T'", ct)
		}
		sout = append(sout, "\n")

	}
	sout = append(sout, strings.TrimSpace(HEADER))
	sout = append(sout, "\n")
	out := []byte(strings.Join(sout, ""))
	if _, err := file.Write(out); err != nil {
		return err
	}
	return nil

}

func format(ss *[]string) *[]string {
	for i, s := range *ss {
		if !(strings.HasPrefix(s, "(") && strings.HasSuffix(s, ")")) {
			(*ss)[i] = fmt.Sprintf("\t%s", s)
		}
	}
	return ss
}

func (translator *Translator) translateSingle(s *Single) (string, error) {
	var sout []string
	switch s.commandType {
	case ADD:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("AM=M-1"))
		sout = append(sout, fmt.Sprintf("D=M"))
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=D+M"))
	case SUB:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("AM=M-1"))
		sout = append(sout, fmt.Sprintf("D=M"))
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=M-D"))
	case NEG:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=-M"))
	case EQ:
		returnLabel := fmt.Sprintf("RETURN_%d", translator.getID())
		sout = append(sout, fmt.Sprintf("@%s", returnLabel))
		sout = append(sout, fmt.Sprintf("D=A"))
		sout = append(sout, fmt.Sprintf("@EQ_PROCEDURE"))
		sout = append(sout, fmt.Sprintf("0;JMP"))
		sout = append(sout, fmt.Sprintf("(%s)", returnLabel))
	case LT:
		returnLabel := fmt.Sprintf("RETURN_%d", translator.getID())
		sout = append(sout, fmt.Sprintf("@%s", returnLabel))
		sout = append(sout, fmt.Sprintf("D=A"))
		sout = append(sout, fmt.Sprintf("@LT_PROCEDURE"))
		sout = append(sout, fmt.Sprintf("0;JMP"))
		sout = append(sout, fmt.Sprintf("(%s)", returnLabel))
	case GT:
		returnLabel := fmt.Sprintf("RETURN_%d", translator.getID())
		sout = append(sout, fmt.Sprintf("@%s", returnLabel))
		sout = append(sout, fmt.Sprintf("D=A"))
		sout = append(sout, fmt.Sprintf("@GT_PROCEDURE"))
		sout = append(sout, fmt.Sprintf("0;JMP"))
		sout = append(sout, fmt.Sprintf("(%s)", returnLabel))
	case AND:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("AM=M-1"))
		sout = append(sout, fmt.Sprintf("D=M"))
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=D&M"))
	case OR:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("AM=M-1"))
		sout = append(sout, fmt.Sprintf("D=M"))
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=D|M"))
	case NOT:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("A=M-1"))
		sout = append(sout, fmt.Sprintf("M=-M"))
		sout = append(sout, fmt.Sprintf("M=M-1"))
	default:
		return "", fmt.Errorf("Unknown single operation: %s", s.orig)
	}
	return strings.Join(*format(&sout), "\n"), nil
}

func (translator *Translator) translatePP(p *PP) (string, error) {
	var segment string
	switch p.segmentType {
	case LOCAL:
		segment = "LCL"
	case ARGUMENT:
		segment = "ARG"
	case THIS:
		segment = "THIS"
	case THAT:
		segment = "THAT"
	}

	var sout []string
	i := p.dst

	switch p.segmentType {
	case LOCAL, ARGUMENT, THIS, THAT:
		switch p.commandType {
		// addr = segmentPointer + i, SP--, *addr = *SP
		case POP:
			sout = append(sout, fmt.Sprintf("@%s", segment))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("D=D+A"))
			sout = append(sout, fmt.Sprintf("@R13"))
			sout = append(sout, fmt.Sprintf("M=D"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M-1"))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@R13"))
			sout = append(sout, fmt.Sprintf("A=M"))
			sout = append(sout, fmt.Sprintf("M=D"))

		// addr = segmentPointer + i, *SP = *addr, SP++
		case PUSH:
			sout = append(sout, fmt.Sprintf("@%s", segment))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("A=D+A"))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("A=M"))
			sout = append(sout, fmt.Sprintf("M=D"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("M=M+1"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)

		}
	case CONSTANT:
		switch p.commandType {
		case POP:
			return "", fmt.Errorf("Cannot pop constant")
		// *SP = i; SP++
		case PUSH:
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("D=A"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("A=M"))
			sout = append(sout, fmt.Sprintf("M=D"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("M=M+1"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)
		}
	case STATIC:
		varName := fmt.Sprintf("%s.%d", translator.filename, i)
		switch p.commandType {
		case POP:
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M-1"))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@%s", varName))
			sout = append(sout, fmt.Sprintf("M=D"))
		case PUSH:
			sout = append(sout, fmt.Sprintf("@%s", varName))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M+1"))
			sout = append(sout, fmt.Sprintf("M=D"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)
		}
	case TEMP:
		if i > 7 {
			return "", fmt.Errorf("Temp address is above 7: %s", p.orig)
		}
		i += TEMPADDR
		switch p.commandType {
		// addr = 5 + i; *SP = *addr; SP++
		case POP:
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M-1"))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("M=D"))
		// addr = 5 + i; SP--; *addr = *SP
		case PUSH:
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M+1"))
			sout = append(sout, fmt.Sprintf("M=D"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)
		}
	case POINTER:
		if i != 0 && i != 1 {
			return "", fmt.Errorf("Pointer segment -- only 0 or 1 allowed: %s", p.orig)
		}
		if i == 0 {
			i = THISADDR
		} else {
			i = THATADDR
		}
		// *SP = THIS/THAT; SP++
		switch p.commandType {
		case POP:
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M-1"))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("M=D"))
		// SP--; THIS/THAT = *SP
		case PUSH:
			sout = append(sout, fmt.Sprintf("@%d", i))
			sout = append(sout, fmt.Sprintf("D=M"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("AM=M+1"))
			sout = append(sout, fmt.Sprintf("M=D"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)
		}

	default:
		return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)

	}
	return strings.Join(*format(&sout), "\n"), nil
}
