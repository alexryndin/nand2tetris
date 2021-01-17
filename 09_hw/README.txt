Hi!

This programm is a very simple file system and shell simulation, providing screen scrolling.

Promgram features several classes:
Node -- file system class, providing a tree data structure
Prog -- shell and screen handling
KKeyboard -- additional Keyboard Functions
SString -- additional String functions
AArray -- Vector-like array featuring data reallocations when needed
Main -- main class

Program provides several shell-like cmds like:
ls -- list files in current dir
mkdir -- create new dir
cd -- change dir
touch -- new file
put -- put string in a file

Note how promt goes down as you type new commands :)

Following bugs should be considered as a features:
* No rm cmd
* Input appears in the next line (below prompt)
* Absence of tests :(


