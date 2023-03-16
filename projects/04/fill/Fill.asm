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
@pressed
M=0

@keep_screen
M=1

(LOOP)
    // check press
    @KBD
    D=M
    @NO_PRESS
    D; JEQ

        // filled previous?
        @pressed
        D=M
        @FILL_SCREEN_END
        D; JGT
            // set fill screen
            @keep_screen
            M=0
        (FILL_SCREEN_END)

        // set press
        @pressed
        M=1

        @PRESSING_CHECK_END
        0; JMP

    (NO_PRESS)
        // removed previous?
        @pressed
        D=M
        @REMOVE_SCREEN_END
        D; JEQ
            // remove screen
            @keep_screen
            M=0

        (REMOVE_SCREEN_END)
        @pressed
        M=0

(PRESSING_CHECK_END)

// keep screen?
@keep_screen
D=M-1
@SET_SCREEN_END
D; JEQ
    // set screen
    @fill_value
    M=0
    @pressed
    D=M
    @FILL_VALUE_END
    D; JEQ

        @fill_value
        M=-1

    (FILL_VALUE_END)

    // addr
    @SCREEN
    D=A
    @addr
    M=D

    // end addr
    @8192
    D=D+A
    @end_addr
    M=D

    (SET_SCREEN_LOOP)
        @end_addr
        D=M
        @addr
        D=D-M
        @SET_SCREEN_END
        D; JEQ

        //
        @fill_value
        D=M
        @addr
        A=M
        M=D

        // addr = addr + 1
        @addr
        M=M+1

        @SET_SCREEN_LOOP
        0; JMP
        

(SET_SCREEN_END)
@keep_screen
M=1

@LOOP
0; JMP
