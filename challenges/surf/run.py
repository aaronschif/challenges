#!/usr/bin/env python3

import argparse
from pathlib import Path
from subprocess import Popen, PIPE, TimeoutExpired
from time import time

parser = argparse.ArgumentParser()
parser.add_argument('cmd')
parser.add_argument('path')
parser.add_argument('--timeout', nargs='?', default=5, type=int)

args = parser.parse_args()

try:
    for path in Path(args.path).glob('*.in'):
        test_path = path.with_suffix('.out')
        assert test_path.exists()

        total_time = -time()
        proc = Popen(args.cmd, stdin=PIPE, stdout=PIPE)
        try:
            outs, errs = proc.communicate(path.read_bytes(), timeout=args.timeout)
        except TimeoutExpired:
            proc.terminate()
            outs = None

        correct = outs == test_path.read_bytes()

        total_time += time()
        reason = ""

        if correct:
            pass
        elif total_time > args.timeout:
            reason = "timeout"
        elif proc.returncode is not 0:
            reason = "returncode"
        else:
            reason = "diff"


        print("{} {:8} {:>20} [{:6}k]   {:6.2f}s".format("\033[92m✓\033[0m" if correct else "\033[91m✗\033[0m", reason, path.name, path.stat().st_size//1024, total_time))
        # if not correct and outs:
except KeyboardInterrupt:
    pass
