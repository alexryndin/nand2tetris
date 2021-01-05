// This is an example implementation of LT VM command
// which handles Add16 overflow
// The point is that standard implementation
// doesn't take overflow into account and since
// ALU doesn't have overflow bit we have to handle
// this ourselves
(LT_PROCEDURE)
	@1
	D=M
	@LT_PROCEDURE_CHECK_A_LT
	D;JGE // if b >= 0
(LT_PROCEDURE_CHECK_A_GT)
	@0    // b < 0 here
	D=M
	@LT_PROCEDURE_EXIT_0
	D;JGE // if a >= 0
	@LT_PROCEDURE_MAIN
	0;JMP // if b < 0 and a < 0 too
(LT_PROCEDURE_CHECK_A_LT)
	@0    // b >= 0 here
	D=M
	@LT_PROCEDURE_EXIT_1
	D;JLT // if a < 0
(LT_PROCEDURE_MAIN)
	@1
	D=D-M
	@LT_PROCEDURE_EXIT_1
	D;JLT
(LT_PROCEDURE_EXIT_0)
	@2
	M=0
	@LT_PROCEDURE_END
	0;JMP
(LT_PROCEDURE_EXIT_1)
	@2
	M=-1
(LT_PROCEDURE_END)
	@LT_PROCEDURE_END
	0;JMP

