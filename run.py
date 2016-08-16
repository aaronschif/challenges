#!/usr/bin/env python3

import argparse
from pathlib import Path
from subprocess import Popen, PIPE, TimeoutExpired
from time import time
from enum import Enum
from difflib import Differ, context_diff
import re

parser = argparse.ArgumentParser()
parser.add_argument('path')
parser.add_argument('cmd')
parser.add_argument('limit', nargs='*', type=int)
parser.add_argument('--timeout', nargs='?', default=5, type=float)
parser.add_argument('--size', nargs='?', default=0, type=float)

args = parser.parse_args()

def run_cmd(cmd, stdin, timeout):
    proc = Popen(cmd.split(), stdin=PIPE, stdout=PIPE)
    try:
        outs, errs = proc.communicate(stdin, timeout=timeout)
        goal = test_path.read_bytes()
    except TimeoutExpired:
        proc.terminate()
        outs = None
    return outs, proc.returncode


class Result(Enum):
    SUCCESS = 0
    TIMEOUT = 1
    DIFFRENT = 2
    RETURN = 3
    SKIP = 4


def run_test(in_path, expect_path) -> (Result, "time", "diff"):
    if args.size and in_path.stat().st_size/1024 > args.size:
        return Result.SKIP, 0, None

    total_time = -time()
    our_output, code = run_cmd(args.cmd, in_path.read_bytes(), args.timeout if args.timeout else None)
    total_time += time()

    if our_output is None:
        return Result.TIMEOUT, total_time, None

    if code != 0:
        return Result.RETURN, total_time, None

    correct_output = test_path.read_bytes()
    if our_output != correct_output:
        diff = context_diff(
            correct_output.decode('utf-8').splitlines(keepends=True),
            our_output.decode('utf-8').splitlines(keepends=True),
            tofile='have stdout', fromfile='correct stdout')
        return Result.DIFFRENT, total_time, diff

    return Result.SUCCESS, total_time, None


try:
    for n, path in enumerate(Path(args.path).glob('*.in')):
        test_path = path.with_suffix('.out')
        assert test_path.exists()

        if args.limit and n not in args.limit:
            result, total_time, diff = Result.SKIP, 0, None
        else:
            result, total_time, diff = run_test(path, test_path)

        if result is Result.SUCCESS:
            fmt_icon = "\033[92m✓\033[0m"
        elif result is Result.SKIP:
            fmt_icon = "\033[1;30m●\033[0m"
        else:
            fmt_icon = "\033[91m✗\033[0m"
        fmt_stdin_size = path.stat().st_size/1024

        print("{:4}: {} {:8} {:>20} [{:6.0f}k]   {:6.2f}s".format(n, fmt_icon, result.name, path.name, fmt_stdin_size, total_time))
        if diff:
            for line in diff:
                print(" "*8, line.rstrip('\n'))
        # if not correct and outs:
except KeyboardInterrupt:
    pass
