#!/usr/bin/python
# -*- coding: UTF-8 -*-

import os

LOCAL_SEG = 'local'
ARGUMENT_SEG = 'argument'
THIS_SEG = 'this'
THAT_SEG = 'that'
CONSTANT_SEG = 'constant'
STATIC_SEG = 'static'
TEMP_SEG = 'temp'
POINTER_SEG = 'pointer'

VARIABLE_LABEL_MAP = {
    LOCAL_SEG: 'LCL',
    ARGUMENT_SEG: 'ARG',
    THIS_SEG: 'THIS',
    THAT_SEG: 'THAT',
}

POINTER_LABEL_MAP = {
    '0': 'THIS',
    '1': 'THAT',
}

SP = 'SP'
TEMP_MEM_BASE = 5
ADDR_VAR_LABEL = 'addr'

LOGIC_EQ_LABEL = 'LOGIC_EQ'
LOGIC_GT_LABEL = 'LOGIC_GT'
LOGIC_LT_LABEL = 'LOGIC_LT'
LOGIC_JUMP_VARIABLE = 'logicJump'

class CodeWriter:
    def __init__(self, dirName, filenameWithoutExt):
        self.filenameWithoutExt = filenameWithoutExt
        filename = filenameWithoutExt + '.asm'
        fileparh = os.path.join(dirName, filename)
        self.file = open(fileparh, 'w')

        self.logicLabelCount = 0

    def writeInitLogical(self):
        # compare 'D'

        # comment
        self._writeLine('// logic function ---')
        self._writeLine('@LOGIC_INIT_END')
        self._writeLine('0; JMP')

        self.writeInitLogicalEQ()
        self.writeInitLogicalGT()
        self.writeInitLogicalLT()
        self._writeLine('(LOGIC_INIT_END)')

    def writeInitLogicalEQ(self):
        # comnent
        self._writeLine('// EQ')

        self._writeLine(f'({LOGIC_EQ_LABEL})')
        self._writeLine('@LOGIC_EQ_EQ')
        self._writeLine('D; JEQ')

        # not eq
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=0')
        self._writeLine('@LOGIC_EQ_END')
        self._writeLine('0; JMP')

        # eq
        self._writeLine('(LOGIC_EQ_EQ)')
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=1')

        # return
        self._writeLine('(LOGIC_EQ_END)')
        self._writeLine(f'@{LOGIC_JUMP_VARIABLE}')
        self._writeLine('A=M')
        self._writeLine('0; JMP')

        # new line
        self._writeLine('')

    def writeInitLogicalGT(self):
        # comnent
        self._writeLine('// GT')

        self._writeLine(f'({LOGIC_GT_LABEL})')
        self._writeLine('@LOGIC_GT_GT')
        self._writeLine('D; JGT')

        # not gt
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=0')
        self._writeLine('@LOGIC_GT_END')
        self._writeLine('0; JMP')

        # gt
        self._writeLine('(LOGIC_GT_GT)')
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=1')

        # return
        self._writeLine('(LOGIC_GT_END)')
        self._writeLine(f'@{LOGIC_JUMP_VARIABLE}')
        self._writeLine('A=M')
        self._writeLine('0; JMP')

        # new line
        self._writeLine('')

    def writeInitLogicalLT(self):
        # comnent
        self._writeLine('// LT')

        self._writeLine(f'({LOGIC_LT_LABEL})')
        self._writeLine('@LOGIC_LT_LT')
        self._writeLine('D; JLT')

        # not lt
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=0')
        self._writeLine('@LOGIC_LT_END')
        self._writeLine('0; JMP')

        # lt
        self._writeLine('(LOGIC_LT_LT)')
        self._writeLine(f'@{SP}')
        self._writeLine('A=M-1')
        self._writeLine('M=1')

        # return
        self._writeLine('(LOGIC_LT_END)')
        self._writeLine(f'@{LOGIC_JUMP_VARIABLE}')
        self._writeLine('A=M')
        self._writeLine('0; JMP')

        # new line
        self._writeLine('')

    def writeArithmetic(self, cmd):
        # comment
        self._writeLine('// {0}'.format(cmd))

        if cmd in ['add', 'sub', 'and', 'or']:
            # SP--; v = *SP;
            self._writeLine(f'@{SP}')
            self._writeLine('M=M-1')
            self._writeLine('A=M')
            self._writeLine('D=M')

            self._writeLine(f'@{SP}')
            self._writeLine('A=M-1')

            if cmd == 'add':
                self._writeLine('M=D+M')
            elif cmd == 'sub':
                self._writeLine('M=M-D')
            elif cmd == 'and':
                self._writeLine('M=M&D')
            elif cmd == 'or':
                self._writeLine('M=M|D')
        elif cmd in ['eq', 'gt', 'lt']:
            logicTrueLabel = f'LOGIC_TRUE_{self.logicLabelCount}'
            logicEndLabel = f'LOGIC_END_{self.logicLabelCount}'
            self.logicLabelCount = self.logicLabelCount + 1

            self._writeLine(f'@{SP}')
            self._writeLine('M=M-1')
            self._writeLine('A=M')
            self._writeLine('D=M')

            self._writeLine('A=A-1')
            self._writeLine('D=M-D')

            self._writeLine(f'@{logicTrueLabel}')
            if cmd == 'eq':
                self._writeLine('D; JEQ')
            elif cmd == 'gt':
                self._writeLine('D; JGT')
            elif cmd == 'lt':
                self._writeLine('D; JLT')

            # false
            self._writeLine(f'@{SP}')
            self._writeLine('A=M-1')
            self._writeLine('M=0')
            self._writeLine(f'@{logicEndLabel}')
            self._writeLine('0; JMP')

            # true
            self._writeLine(f'({logicTrueLabel})')
            self._writeLine(f'@{SP}')
            self._writeLine('A=M-1')
            self._writeLine('M=-1')

            # end
            self._writeLine(f'({logicEndLabel})')

        elif cmd in ['neg', 'not']:
            self._writeLine(f'@{SP}')
            self._writeLine('A=M-1')

            if cmd == 'neg':
                self._writeLine('M=-M')
            elif cmd == 'not':
                self._writeLine('M=!M')
        else:
            pass

        # new line
        self._writeLine('')

    def writePushPop(self, cmd, segment, index):
        # comment
        self._writeLine('// {0} {1} {2}'.format(cmd, segment, index))

        if cmd == 'pop':
            if segment in [LOCAL_SEG, ARGUMENT_SEG, THIS_SEG, THAT_SEG]:
                # addr = segmentPointer + i, SP--, *addr = *SP
                self._writeLine(f'@{index}')
                self._writeLine('D=A')
                self._writeLine(f'@{VARIABLE_LABEL_MAP[segment]}')
                self._writeLine('D=D+M')
                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M-1')
                self._writeLine('A=M')
                self._writeLine('D=M')

                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('A=M')
                self._writeLine('M=D')
            elif segment == STATIC_SEG:
                self._writeLine(f'@{SP}')
                self._writeLine('M=M-1')
                self._writeLine('A=M')
                self._writeLine('D=M')

                self._writeLine(f'@{self.filenameWithoutExt}.{index}')
                self._writeLine('M=D')
            elif segment == TEMP_SEG:
                # addr = 5 + i, SP--, *addr = *SP
                self._writeLine(f'@{index}')
                self._writeLine('D=A')
                self._writeLine(f'@{TEMP_MEM_BASE}')
                self._writeLine('D=D+A')
                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M-1')
                self._writeLine('A=M')
                self._writeLine('D=M')

                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('A=M')
                self._writeLine('M=D')
            elif segment == POINTER_SEG:
                # SP--, THIS/THAT = *SP
                self._writeLine(f'@{SP}')
                self._writeLine('M=M-1')
                self._writeLine('A=M')
                self._writeLine('D=M')

                self._writeLine(f'@{POINTER_LABEL_MAP[index]}')
                self._writeLine('M=D')
            else:
                pass
        elif cmd == 'push':
            if segment in [LOCAL_SEG, ARGUMENT_SEG, THIS_SEG, THAT_SEG]:
                # addr = segmentPointer + i, *SP = *addr, SP++
                self._writeLine(f'@{index}')
                self._writeLine('D=A')
                self._writeLine(f'@{VARIABLE_LABEL_MAP[segment]}')
                self._writeLine('D=D+M')
                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('M=D')

                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('A=M')
                self._writeLine('D=M')
                self._writeLine(f'@{SP}')
                self._writeLine('A=M')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M+1')
            elif segment == CONSTANT_SEG:
                # *SP = i, SP++
                self._writeLine(f'@{index}')
                self._writeLine('D=A')
                self._writeLine(f'@{SP}')
                self._writeLine('A=M')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M+1')
            elif segment == STATIC_SEG:
                self._writeLine(f'@{self.filenameWithoutExt}.{index}')
                self._writeLine('D=M')
                self._writeLine(f'@{SP}')
                self._writeLine('A=M')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M+1')
            elif segment == TEMP_SEG:
                # addr = 5 + i, *SP = *addr, SP++
                self._writeLine(f'@{index}')
                self._writeLine('D=A')
                self._writeLine(f'@{TEMP_MEM_BASE}')
                self._writeLine('D=D+A')
                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('M=D')

                self._writeLine(f'@{ADDR_VAR_LABEL}')
                self._writeLine('A=M')
                self._writeLine('D=M')
                self._writeLine(f'@{SP}')
                self._writeLine('A=M')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M+1')
            elif segment == POINTER_SEG:
                # *SP = THIS/THAT, SP++
                self._writeLine(f'@{POINTER_LABEL_MAP[index]}')
                self._writeLine('D=M')
                self._writeLine(f'@{SP}')
                self._writeLine('A=M')
                self._writeLine('M=D')

                self._writeLine(f'@{SP}')
                self._writeLine('M=M+1')
            else:
                pass

        # empty new line
        self._writeLine('')

    def _writeLine(self, line):
        self.file.write(line + '\n')

    def close(self):
        self.file.close()
