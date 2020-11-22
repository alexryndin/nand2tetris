// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

	@R2
	M=0

(LOOP)
	@R0
	D=M
	@ENDLOOP
	D; JEQ

	@R1
	D=M
	@R2
	M=D+M
	@R0
	M=M-1
	@LOOP
	0; JMP
(ENDLOOP)

(END)
	@END
	0; JMP