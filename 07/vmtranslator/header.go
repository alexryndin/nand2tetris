package main

const HEADER = `
// end of execution
(END)
	@END
	0;JMP
(EQ_PROCEDURE)
	@R13
	M=D
	@SP
	AM=M-1
	D=M
	A=A-1
	D=D-M
	M=-1
	@EQ_PROCEDURE_END
	D;JEQ
	@SP
	A=M-1
	M=0
(EQ_PROCEDURE_END)
	@R13
	A=M
	0;JMP

(LT_PROCEDURE)
	@R13
	M=D
	@SP
	AM=M-1
	D=M
	@SP
	M=M-1
	@R14
	M=D
	@LT_PROCEDURE_CHECK_A_LT
	D;JGE // if b >= 0
(LT_PROCEDURE_CHECK_A_GT)
	@SP    // b < 0 here
	A=M
	D=M
	@LT_PROCEDURE_EXIT_0
	D;JGE // if a >= 0
	@LT_PROCEDURE_MAIN
	0;JMP // if b < 0 and a < 0 too
(LT_PROCEDURE_CHECK_A_LT)
	@SP    // b >= 0 here
	A=M
	D=M
	@LT_PROCEDURE_EXIT_1
	D;JLT // if a < 0
(LT_PROCEDURE_MAIN)
	@R14
	D=D-M
	@LT_PROCEDURE_EXIT_1
	D;JLT
(LT_PROCEDURE_EXIT_0)
	@SP
	A=M
	M=0
	@LT_PROCEDURE_END
	0;JMP
(LT_PROCEDURE_EXIT_1)
	@SP
	A=M
	M=-1
(LT_PROCEDURE_END)
	@SP
	M=M+1
	@R13
	A=M
	0;JMP

(GT_PROCEDURE)
	@R13
	M=D
	@SP
	AM=M-1
	D=M
	@SP
	M=M-1
	@R14
	M=D
	@GT_PROCEDURE_CHECK_A_GT
	D;JLT // if b < 0
(GT_PROCEDURE_CHECK_A_LT)
	@SP    // b >= 0 here
	A=M
	D=M
	@GT_PROCEDURE_EXIT_0
	D;JLT // if a < 0
	@GT_PROCEDURE_MAIN
	0;JMP // if b >= 0 and a >= 0 too
(GT_PROCEDURE_CHECK_A_GT)
	@SP    // b < 0 here
	A=M
	D=M
	@GT_PROCEDURE_EXIT_1
	D;JGE // if a >= 0
(GT_PROCEDURE_MAIN)
	@R14
	D=D-M
	@GT_PROCEDURE_EXIT_1
	D;JGT
(GT_PROCEDURE_EXIT_0)
	@SP
	A=M
	M=0
	@GT_PROCEDURE_END
	0;JMP
(GT_PROCEDURE_EXIT_1)
	@SP
	A=M
	M=-1
(GT_PROCEDURE_END)
	@SP
	M=M+1
	@R13
	A=M
	0;JMP
`