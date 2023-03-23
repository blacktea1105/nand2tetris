#!/usr/bin/python
# -*- coding: UTF-8 -*-

import os
import sys

from Parser import *;
from CodeWriter import CodeWriter;

if __name__ == "__main__":
    fileArg = sys.argv[1]
    filepath = os.path.realpath(fileArg)
    filename = os.path.basename(filepath)
    dirname = os.path.dirname(filepath)
    (filenameWithoutExt, ext) = os.path.splitext(filename)

    parser = Parser(filepath)
    codeWriter = CodeWriter(dirname, filenameWithoutExt)

    while parser.advance():
        commandType = parser.getCommandType()

        if commandType in [C_PUSH, C_POP]:
            codeWriter.writePushPop(commandType, parser.getArg1(), parser.getArg2())
        elif commandType == C_ARITHMETIC:
            codeWriter.writeArithmetic(parser.getArg1())

    parser.close()
    codeWriter.close()
