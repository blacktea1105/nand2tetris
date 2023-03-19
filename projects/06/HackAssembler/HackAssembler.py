#!/usr/bin/python
# -*- coding: UTF-8 -*-

import os
import sys
import re

NO_LINE_TYPE = -1
EMPTY_LINE_TYPE = 1
ACOMMAND_LINE_TYPE = 2
CCOMMAND_LINE_TYPE = 3
LABEL_REF = 4
LABEL_DECLARATION = 5

class Parser:
    def __init__(self, filepath):
        self.filepath = filepath
        self.file = open(filepath, 'r')

        self.nextLine = False  # False => 還沒抓取下一行；None => 以嘗試抓取下一行，但已經 EoF
        self.curLine = ''
        self.curLineType = NO_LINE_TYPE

        self.dest = ''
        self.comp = ''
        self.jmp = ''
        self.address = ''
        self.labelName = ''

    def hasMoreCommands(self):
        if self.nextLine is False:
            nextLine = self.file.readline()
            if nextLine != '':
                self.nextLine = nextLine
            else:
                self.nextLine = None

        return self.nextLine is not None

    def advance(self):
        if not self.hasMoreCommands():
            return

        self.curLine = self.nextLine
        self.nextLine = False
        self.parseCurrentLine()

    def parseCurrentLine(self):
        line = self.curLine
        self.curLineType = NO_LINE_TYPE

        self.dest = ''
        self.comp = ''
        self.jmp = ''
        self.address = ''
        self.labelName = ''

        # 去除空白、結尾 '\n'
        line = re.sub(r'\s', '', line)

        # 去除 '//' 以及 '//' 後面的字串
        line = re.sub(r'//.*$', '', line)

        if line == '':
            # 空白行
            self.curLineType = EMPTY_LINE_TYPE
        elif line[0] == '@':
            content = line[1:]

            # 正整數 (沒判斷兩位數以上開頭為 0)
            if re.match(r'^\d+$', content):
                # A command
                self.curLineType = ACOMMAND_LINE_TYPE
                self.address = content
            else:
                # label declaration
                self.curLineType = LABEL_REF
                self.labelName = content
        else:
            matches = re.match(r'^\((?P<labelName>.+?)\)$', line)
            if matches and matches['labelName']:
                self.curLineType = LABEL_DECLARATION
                self.labelName = matches['labelName']
            else:
                # C command
                self.curLineType = CCOMMAND_LINE_TYPE
                lineParts = line.split('=')
                # 字串有 '='
                if len(lineParts) > 1:
                    self.dest = lineParts[0]
                    line = lineParts[1]

                lineParts = line.split(';')
                self.comp = lineParts[0]
                if len(lineParts) > 1:
                    self.jmp = lineParts[1]

    def isCurLineEmpty(self):
        return self.getCurLineType() == EMPTY_LINE_TYPE

    def isCurLineLabelRef(self):
        return self.getCurLineType() == LABEL_REF

    def isCurLineLabelDeclaration(self):
        return self.getCurLineType() == LABEL_DECLARATION

    def isCurLineACommand(self):
        return self.getCurLineType() == ACOMMAND_LINE_TYPE

    def isCurLineCCommand(self):
        return self.getCurLineType() == CCOMMAND_LINE_TYPE

    def getCurLineType(self):
        return self.curLineType

    def release(self):
        self.file.close()

    def reseek(self):
        self.nextLine = False
        self.file.seek(0, 0)

class Code:
    def __init__(self):
        self.destMap = {
            '': '000',
            'M': '001',
            'D': '010',
            'MD': '011',
            'A': '100',
            'AM': '101',
            'AD': '110',
            'AMD': '111',
        }

        self.compMap = {
            '0': '0101010',
            '1': '0111111',
            '-1': '0111010',
            'D': '0001100',
            'A': '0110000',
            'M': '1110000',
            '!D': '0001101',
            '!A': '0110001',
            '!M': '1110001',
            '-D': '0001111',
            '-A': '0110011',
            '-M': '1110011',
            'D+1': '0011111',
            'A+1': '0110111',
            'M+1': '1110111',
            'D-1': '0001110',
            'A-1': '0110010',
            'M-1': '1110010',
            'D+A': '0000010',
            'D+M': '1000010',
            'D-A': '0010011',
            'D-M': '1010011',
            'A-D': '0000111',
            'M-D': '1000111',
            'D&A': '0000000',
            'D&M': '1000000',
            'D|A': '0010101',
            'D|M': '1010101',
        }

        self.jmpMap = {
            '': '000',
            'JGT': '001',
            'JEQ': '010',
            'JGE': '011',
            'JLT': '100',
            'JNE': '101',
            'JLE': '110',
            'JMP': '111',
        }

        self.maxAddress = (2**15) - 1

    def getDest(self, dest):
        return self.destMap[dest]

    def getComp(self, comp):
        return self.compMap[comp]

    def getJmp(self, jmp):
        return self.jmpMap[jmp]

    def getAddress(self, address):
        addr = int(address) & self.maxAddress
        return "{:015b}".format(addr)

class Writer:
    def __init__(self, dirName, filenameWithoutExt):
        filename = filenameWithoutExt + '.hack'
        fileparh = os.path.join(dirName, filename)
        self.file = open(fileparh, 'w')

    def writeLine(self, line):
        self.file.write(line + '\n')

    def release(self):
        self.file.close()

class SymbolTable:
    def __init__(self):
        self.nextVariableAddress = 16

        self.map = {
            'R0': '0',
            'R1': '1',
            'R2': '2',
            'R3': '3',
            'R4': '4',
            'R5': '5',
            'R6': '6',
            'R7': '7',
            'R8': '8',
            'R9': '9',
            'R10': '10',
            'R11': '11',
            'R12': '12',
            'R13': '13',
            'R14': '14',
            'R15': '15',
            'SCREEN': '16384',
            'KBD': 24576,
            'SP': 0,
            'LCL': 1,
            'ARG': 2,
            'THIS': 3,
            'THAT': 4,
        }

    def exists(self, labelName):
        return labelName in self.map

    def getOrCreateVariable(self, labelName):
        if not self.exists(labelName):
            self.map[labelName] = self.nextVariableAddress
            self.nextVariableAddress = self.nextVariableAddress + 1

        return self.map[labelName]

    def declareLabelIfNotExist(self, labelName, value):
        if not self.exists(labelName):
            self.map[labelName] = value

if __name__ == "__main__":
    fileArg = sys.argv[1]
    filepath = os.path.realpath(fileArg)
    filename = os.path.basename(filepath)
    dirname = os.path.dirname(filepath)
    (filenameWithoutExt, ext) = os.path.splitext(filename)

    parser = Parser(filepath)
    writer = Writer(dirname, filenameWithoutExt)
    symbolTable = SymbolTable()
    code = Code()

    # parse label declarations
    totalMachineCodeLines = 0
    while parser.hasMoreCommands():
        parser.advance()

        if not parser.isCurLineEmpty():
            if parser.isCurLineLabelDeclaration():
                lineIndex = totalMachineCodeLines  # line - 1(index) + 1(next line index)
                symbolTable.declareLabelIfNotExist(parser.labelName, lineIndex)
            else:
                totalMachineCodeLines = totalMachineCodeLines + 1

    parser.reseek()

    # parse asm
    while parser.hasMoreCommands():
        parser.advance()

        if not parser.isCurLineEmpty() and not parser.isCurLineLabelDeclaration():
            machineCode = ''

            if parser.isCurLineLabelRef():
                address = symbolTable.getOrCreateVariable(parser.labelName)
                address = code.getAddress(address)
                machineCode = '0' + address

            elif parser.isCurLineACommand():
                address = code.getAddress(parser.address)
                machineCode = '0' + address

            elif parser.isCurLineCCommand():
                dest = code.getDest(parser.dest)
                comp = code.getComp(parser.comp)
                jmp = code.getJmp(parser.jmp)
                machineCode = '111' + comp + dest + jmp

            writer.writeLine(machineCode)

    parser.release()
    writer.release()
