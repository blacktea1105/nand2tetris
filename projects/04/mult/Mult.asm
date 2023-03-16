// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
@R2
M=0

// result is negative?
@R0
D=M
@a
M=D

@R0_NEGATIVE
D; JLT
    @postive
    M=1
    @r0pos
    M=1

    @R0_POSITIVE_CHECK_END
    0; JMP
(R0_NEGATIVE)
    @postive
    M=0

    @a
    M=-M
(R0_POSITIVE_CHECK_END)

@R1
D=M
@b
M=D
@R1_NEGATIVE
D; JLT
    D=1
    @postive
    M=D&M

    @R1_POSITIVE_CHECK_END
    0; JMP
(R1_NEGATIVE)
    D=0
    @postive
    M=D&M

    @b
    M=-M
(R1_POSITIVE_CHECK_END)

// 
// |a| >= |b|
@b
D=M
@a
D=D-M
@SWITCH_A_B_END
D; JLT
    @a
    D=M
    @temp
    M=D

    @b
    D=M
    @a
    M=D

    @temp
    D=M
    @a
    M=D

(SWITCH_A_B_END)

@i
M=0
@sum
M=0
(LOOP)
    // end loop branch
    @i
    D=M
    @b
    D=D-M
    @LOOP_END
    D; JEQ

    @a
    D=M
    @sum
    M=D+M

    @i
    M=M+1

    @LOOP
    0; JMP

(LOOP_END)

@postive
D=M-1
@SIGN_END
D; JEQ
    @sum
    M=-M

(SIGN_END)
@sum
D=M
@R2
M=D

(END)
    @END
    0; JMP
