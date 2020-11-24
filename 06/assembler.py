#!/usr/bin/python

import sys
import os


class ParseError(Exception):
    pass


def remove_inline_comments(text):
    ret = []
    in_comment_block = False
    p = 0
    while True:
        if (op := text.find('/*', p)) > 0:
            in_comment_block = True
            if op != p:
                ret.append(text[p:op])
            p = op + 2
        else:
            ret.append(text[p:])
            break

        if (op := text.find('*/', p)) > 0:
            p = op + 2
            in_comment_block = False
            continue
        else:
            break

    if in_comment_block:
        exit(2)

    return ''.join(ret)


def remove_comments(contents):
    ret = []
    for l in contents:
        lstrip = l.strip()
        if lstrip.startswith('//'):
            continue
        if (com := lstrip.find('//')) > 0:
            ret.append(l[0:com])
            continue

        ret.append(l)

    return ret


class AIns:
    def __init__(self, token, symbols):
        if token.isdecimal():
            self.value = int(token)
            if self.value > 2**15-1:
                raise ParseError("A instruction value is too high")

        elif token in symbols.symbols:
            self.value = symbols.symbols[token]
        else:
            symbols.add(token)
            self.value = symbols.symbols[token]

    def get_binary(self):
        return "0{:015b}".format(self.value)


class CIns:

    comp = {
        '0': '101010',
        '1': '111111',
        '-1': '111010',
        'D': '001100',
        'A': '110000',
        'M': '110000',
        '!D': '001101',
        '!A': '110001',
        '!M': '110001',
        '-D': '001111',
        '-A': '110011',
        '-M': '110011',
        'D+1': '011111',
        'A+1': '110111',
        'M+1': '110111',
        'D-1': '001110',
        'A-1': '110010',
        'M-1': '110010',
        'D+A': '000010',
        'D+M': '000010',
        'D-A': '010011',
        'D-M': '010011',
        'A-D': '000111',
        'M-D': '000111',
        'D&A': '000000',
        'D&M': '000000',
        'D|A': '010101',
        'D|M': '010101',
    }

    jmp = {
        'JGT': '001',
        'JEQ': '010',
        'JGE': '011',
        'JLT': '100',
        'JNE': '101',
        'JLE': '110',
        'JMP': '111',
    }

    def __init__(self, token):
        self.raw_instruction = token
        token = token.replace(' ', '')
        self.dest = ''
        self.comp = ''
        self.jmp = ''
        if '=' in token:
            self.dest, token = token.split('=', 1)
        if ';' in token:
            self.comp, self.jmp = token.split(';', 1)
        else:
            self.comp = token

    def get_binary(self):
        head = '111'
        a='0'
        comp = '000000'
        dst = ['0', '0', '0']
        jmp = '000'

        if self.dest:
            if len(self.dest) > 3:
                raise ParseError('Wrong dest length')
            if 'A' in self.dest:
                dst[0] = '1'
            if 'D' in self.dest:
                dst[1] = '1'
            if 'M' in self.dest:
                dst[2] = '1'

        if self.jmp:
            try:
                jmp = CIns.jmp[self.jmp]
            except KeyError:
                raise ParseError('Wrong jmp instruction')

        try:
            comp = CIns.comp[self.comp]
        except KeyError:
            raise ParseError("Wrong comp instruction")

        if 'M' in self.comp:
            a = '1'

        ret = "{}{}{}{}{}".format(head, a, comp, ''.join(dst), jmp)
        if len(ret) > 16:
            raise ParseError("CInstruction binary contruction error, command was '{}'".format(self.raw_instruction))
        return ret


def parse(contents, symbols):
    ret = []
    for l in contents:
        ls = l.strip()
        if ls.startswith('@'):
            ret.append(AIns(ls[1:], symbols))
        else:
            ret.append(CIns(ls))

    return ret


class Symbols:
    def __init__(self):
        self.memptr = 16
        self.symbols = {
            'R0': 0,
            'R1': 1,
            'R2': 2,
            'R3': 3,
            'R4': 4,
            'R5': 5,
            'R6': 6,
            'R7': 7,
            'R8': 8,
            'R9': 9,
            'R10': 10,
            'R11': 11,
            'R12': 12,
            'R13': 13,
            'R14': 14,
            'R15': 15,
            'SCREEN': 16384,
            'KBD': 24576,
            'SP': 0,
            'LCL': 1,
            'ARG': 2,
            'THIS': 3,
            'THAT': 4,
        }

    def fill_with_labels(self, contents):
        ret = []
        pos = 0
        for l in contents:
            ls = l.strip()
            if ls.startswith('(') and ls.endswith(')'):
                label = ls[1:-1]
                if label in self.symbols:
                    raise ParseError('Label redefinition')
                else:
                    self.symbols[label] = pos
            else:
                ret.append(l)
                pos += 1

        return ret

    def add(self, symbol):
        if symbol in self.symbols:
            raise ParseError('Variable redefinition')
        self.symbols[symbol] = self.memptr
        self.memptr += 1


def main():
    if len(sys.argv) < 1:
        exit(1)

    filename = sys.argv[1]

    contents = []
    with open(filename) as f:
        text = f.read()

    contents = (remove_inline_comments(text)).split('\n')

    contents = filter(None, remove_comments(contents))

    symbols = Symbols()
    contents = symbols.fill_with_labels(contents)

    parsed = parse(contents, symbols)

    out_filename = "{}.hack".format(os.path.splitext(filename)[0])

    with open(out_filename, 'w') as f:
        for i in parsed:
            try:
                f.write("{}\n".format(i.get_binary()))
            except ParseError as e:
                print(e)
                exit(1)


if __name__ == "__main__":
    main()
