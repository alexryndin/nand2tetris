// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(START)
	@KBD
	D=M
	@color
	M=-1
	@FILL
	D; JNE
	@color
	M=0
	@FILL
	0; JMP
(FILL)
	@SCREEN
	D=A
	@addr
	M=D
(LOOP1)
	@addr
	D=M
	@KBD
	D=D-A
	@START
	D; JEQ
	@color
	D=M
	@addr
	A=M
	M=D
	@addr
	M=M+1
	@LOOP1
	0; JMP


