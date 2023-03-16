// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// read kbd
// if kbd then fill
// else clear

//    @preKey
//    M=0
//    @curKey
//    M=0

//(LOOP)
//    @curKey
//    D=M
//    @preKey
//    M=D

    // load key
//    @KBD
//    D=M
//    @curKey
//    M=D

@n
M=0

@8192
D=A
@screenLen
M=D

(MAIL_LOOP)
    @drawIndex
    M=0

    @KBD
    D=M

    @FILL
    D; JNE

    // clear
    @n
    M=0
    @DRAW
    0; JMP

(FILL)
    // fill
    @n
    M=-1
//    @DRAW
//    0; JMP

(DRAW)
    @screenLen
    D=M
    @drawIndex
    D=M-D
    @MAIL_LOOP
    D; JGE

    // current screen address
    @SCREEN
    D=A
    @drawIndex
    D=D+M
    @curAddr
    M=D

    // draw
    @n
    D=M
    @curAddr
    A=M
    M=D

    @drawIndex
    M=M+1
    @DRAW
    0; JMP