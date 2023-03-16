// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

// declare variables
    @resultNegative
    M=0

    // n1
    @R0
    D=M
    @n1
    M=D
    @CHECKED_N1_NEGATIVE
    D; JGE

    @n1
    M=!M
    M=M+1
    @resultNegative
    M=!M
(CHECKED_N1_NEGATIVE)

    // n2
    @R1
    D=M
    @n2
    M=D
    @CHECKED_N2_NEGATIVE
    D; JGE

    @n2
    M=!M
    M=M+1
    @resultNegative
    M=!M
(CHECKED_N2_NEGATIVE)

    // n2 > n1
    @n1
    D=M
    @n2
    D=M-D
    @STOP_N_DECLARE
    D; JLT

    // switch n1, n2
    @n1
    D=M
    @tmp
    M=D

    @n2
    D=M
    @n1
    M=D

    @tmp
    D=M
    @n2
    M=D
(STOP_N_DECLARE)

    @i
    M=0

    @sum
    M=0

(LOOP)
    // break
    @i
    D=M
    @n2
    D=M-D
    @STOP
    D; JEQ

    @n1
    D=M
    @sum
    M=D+M

    @i
    M=M+1

    @LOOP
    0; JMP

(STOP)

    @resultNegative
    D=M
    @CHECKED_NEGATIVE
    D; JEQ

    @sum
    M=!M
    M=M+1

(CHECKED_NEGATIVE)
    @sum
    D=M
    @R2
    M=D

(END)
    @END
    0; JMP
