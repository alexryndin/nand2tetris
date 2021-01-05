package main

import (
	tmpl "./template"
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"text/template"
)

const (
	THISADDR = 3
	THATADDR = 4
	TEMPADDR = 5
)

type Translator struct {
	parser      *Parser
	srcFileName string
	curFunction string
	id          uint64
	fd          *os.File
}

func newTranslator(parser *Parser, dstFileName string, bootstrap bool) (*Translator, error) {
	t, err := newEmptyTranslator(dstFileName, bootstrap)
	if err != nil {
		return nil, err
	}
	t.init(parser)
	return t, nil
}

func newEmptyTranslator(dstFileName string, bootstrap bool) (*Translator, error) {
	file, err := os.Create(dstFileName)
	if err != nil {
		return nil, err
	}
	t := &Translator{fd: file}
	if bootstrap {
		out := strings.TrimPrefix(tmpl.INIT, "\n")
		if _, err := file.Write([]byte(out)); err != nil {
			return nil, err
		}
		t.curFunction = "Sys.init"
	}
	return t, nil
}

func (t *Translator) init(parser *Parser) {
	srcFileName := filepath.Base(parser.fileName)
	srcFileName = strings.TrimSuffix(srcFileName, filepath.Ext(srcFileName))
	t.srcFileName = srcFileName
	t.parser = parser
}

func (t *Translator) close() {
	t.fd.Close()
}

func (t *Translator) GetID() uint64 {
	return t.getID()
}

func (t *Translator) GetCurFnName() string {
	return t.curFunction
}

func (t *Translator) getID() uint64 {
	t.id++
	return t.id
}

func (t *Translator) translate(bootstrap bool) error {
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
		case *FC:
			if translated, err := t.translateFC(ct); err == nil {
				sout = append(sout, ct.orig, "\n", translated)
			} else {
				return err
			}
		case *Func:
			if translated, err := t.translateFunc(ct); err == nil {
				sout = append(sout, ct.orig, "\n", translated)
			} else {
				return err
			}
		default:
			return fmt.Errorf("Unknown command to translate '%T'", ct)
		}
		sout = append(sout, "\n")

	}
	sout = append(sout, strings.TrimSpace(tmpl.HEADER))
	sout = append(sout, "\n")
	out := strings.Join(sout, "")
	if _, err := t.fd.Write([]byte(out)); err != nil {
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
	case RET:
		sout = append(sout, fmt.Sprintf("@RETURN_PROCEDURE"))
		sout = append(sout, fmt.Sprintf("0;JMP"))
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
	case STATIC, TEMP, POINTER:
		var varName string
		switch p.segmentType {
		case STATIC:
			varName = fmt.Sprintf("%s.%d", translator.srcFileName, i)
		case TEMP:
			if i > 7 {
				return "", fmt.Errorf("Temp address is above 7: %s", p.orig)
			}
			varName = fmt.Sprintf("%d", i+TEMPADDR)
		case POINTER:
			if i != 0 && i != 1 {
				return "", fmt.Errorf("Pointer segment -- only 0 or 1 allowed: %s", p.orig)
			}
			if i == 0 {
				varName = fmt.Sprintf("%d", THISADDR)
			} else {
				varName = fmt.Sprintf("%d", THATADDR)
			}
		}
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
			sout = append(sout, fmt.Sprintf("A=M"))
			sout = append(sout, fmt.Sprintf("M=D"))
			sout = append(sout, fmt.Sprintf("@SP"))
			sout = append(sout, fmt.Sprintf("M=M+1"))
		default:
			return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)
		}
	default:
		return "", fmt.Errorf("Unknown push-pop operation: %s", p.orig)

	}
	return strings.Join(*format(&sout), "\n"), nil
}

func (t *Translator) translateFC(fc *FC) (string, error) {
	var sout []string
	switch fc.commandType {
	case LABEL:
		sout = append(sout, fmt.Sprintf("(%s$%s)", t.curFunction, fc.label))
	case GOTO:
		sout = append(sout, fmt.Sprintf("@%s$%s", t.curFunction, fc.label))
		sout = append(sout, fmt.Sprintf("0;JMP"))
	case IFGOTO:
		sout = append(sout, fmt.Sprintf("@SP"))
		sout = append(sout, fmt.Sprintf("AM=M-1"))
		sout = append(sout, fmt.Sprintf("D=M"))
		sout = append(sout, fmt.Sprintf("@%s$%s", t.curFunction, fc.label))
		sout = append(sout, fmt.Sprintf("D;JNE"))
	default:
		return "", fmt.Errorf("Unknown flow control operation: %s", fc.orig)
	}
	return strings.Join(*format(&sout), "\n"), nil
}

func (t *Translator) translateFunc(fn *Func) (string, error) {
	switch fn.commandType {
	case CALL:
		tmpl, err := template.New("call").Parse(strings.TrimSpace(tmpl.CALL))
		if err != nil {
			return "", err
		}
		var out bytes.Buffer
		err = tmpl.Execute(&out, struct {
			T  *Translator
			Fn *Func
		}{t, fn})
		if err != nil {
			return "", err
		}
		return out.String(), nil
	case FUNC:
		t.curFunction = fn.name
		tmpl, err := template.New("func").Parse(strings.TrimSpace(tmpl.FUNC))
		if err != nil {
			return "", err
		}
		var out bytes.Buffer
		err = tmpl.Execute(&out, struct {
			T  *Translator
			Fn *Func
		}{t, fn})
		if err != nil {
			return "", err
		}
		return out.String(), nil
	default:
		return "", fmt.Errorf("Unknown function operation: %s", fn.orig)
	}
}
