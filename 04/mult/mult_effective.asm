// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// This is a bit more efficient version of mult,
// which doesn't pass tests due to number of ticks limitation

	@R2
	M=0

	@16
	D=A
	@i
	M=D

(LOOP)
	// i == 0 ? ENDLOOP
	@i
	D=M
	@ENDLOOP
	D; JEQ

	@R2
	D=M
	M=D+M

	// R0 & 0x8000
	@R0
	D=M
	@32767
	A=A+1  // 0x8000
	D=D&A
	@NOTIF
	D; JEQ // if not (a & 0x8000)
	@R1    // else
	D=M
	@R2
	M=D+M
(NOTIF)
	@R0
	D=M
	M=D+M

	@i
	M=M-1
	@LOOP
	0; JMP


(ENDLOOP)

(END)
	@END
	0; JMP
