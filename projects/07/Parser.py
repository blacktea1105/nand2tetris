#!/usr/bin/python
# -*- coding: UTF-8 -*-

import os
import re

C_EMPTY = 'empty'
C_ARITHMETIC = 'arithmetic'
C_PUSH = 'push'
C_POP = 'pop'
C_LABEL = 'label'
C_GOTO = 'goto'
C_IF = 'if'
C_FUNCTION = 'function'
C_RETURN = 'return'
C_CALL = 'call'

CMD_TYPE_MAP = {
    'add': C_ARITHMETIC,
    'sub': C_ARITHMETIC,
    'neg': C_ARITHMETIC,
    'eq': C_ARITHMETIC,
    'gt': C_ARITHMETIC,
    'lt': C_ARITHMETIC,
    'and': C_ARITHMETIC,
    'or': C_ARITHMETIC,
    'not': C_ARITHMETIC,
    'pop': C_POP,
    'push': C_PUSH,
}

class Parser:
    def __init__(self, filepath):
        self.filepath = filepath
        self.file = open(filepath, 'r')
        self.filesize = os.path.getsize(filepath)

        self.curLine = ''
        self.commandType = C_EMPTY
        self.arg1 = None
        self.arg2 = None

    def hasMoreCommands(self):
        raise BaseException('qq')
        # seek problem
        # https://stackoverflow.com/questions/15934950/python-file-tell-giving-strange-numbers
        # return self.file.tell() < self.filesize

    def advance(self):
        self.curLine = self.file.readline()
        if self.curLine == '':
            return False

        line = self.curLine

        # 去除 '//' 以及 '//' 後面的字串
        line = re.sub(r'//.*$', '', line)
        # 去除開頭空白
        line = re.sub(r'^\s*?', '', line)
        # 去除結尾空白、'\n'
        line = re.sub(r'\s*$', '', line)

        self.commandType = C_EMPTY
        self.arg1 = None
        self.arg2 = None

        if line != '':
            m = re.match(r'^(\S+)(?:\s+(\S+))?(?:\s+(\S+))?$', line)
            if m:
                mGroups = m.groups()
                self.commandType = CMD_TYPE_MAP[mGroups[0]]

                if self.commandType == C_ARITHMETIC:
                    self.arg1 = mGroups[0]

                else:
                    self.arg1 = mGroups[1]
                    self.arg2 = mGroups[2]
        return True

    def getCommandType(self):
        return self.commandType

    def getArg1(self):
        return self.arg1

    def getArg2(self):
        return self.arg2

    def close(self):
        self.file.close()
