package template

const INIT = `
	@256
	D=A
	@SP
	M=D
	@LCL
	MD=-A
	@ARG
	MD=D-1
	@THIS
	MD=D-1
	@THAT
	MD=D-1
	@INIT_RETURN
	D=A
	@SP
	A=M
	M=D
	@Sys.init
	D=A
	@R13
	M=D
	@R14
	M=0
	@CALL_PROCEDURE
	0;JMP
(INIT_RETURN)
	@INIT_RETURN
	0;JMP
`
