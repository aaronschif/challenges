#!/usr/bin/env python3

import argparse
from pathlib import Path
from subprocess import Popen, PIPE, TimeoutExpired
from time import time

parser = argparse.ArgumentParser()
parser.add_argument('cmd')
parser.add_argument('path')
parser.add_argument('--timeout', nargs='?', default=5, type=float)

args = parser.parse_args()

def run_test(cmd, stdin, timeout):
    proc = Popen(cmd, stdin=PIPE, stdout=PIPE)
    try:
        outs, errs = proc.communicate(stdin, timeout=timeout)
        goal = test_path.read_bytes()
    except TimeoutExpired:
        proc.terminate()
        outs = None
    return outs

try:
    for path in Path(args.path).glob('*.in'):
        test_path = path.with_suffix('.out')
        assert test_path.exists()

        stdin_size = path.stat().st_size//1024

        total_time = -time()
        if stdin_size <= 50000:
            our_output = run_test(args.cmd, path.read_bytes(), args.timeout)
            correct_output = test_path.read_bytes()
            correctness = our_output == correct_output
        else:
            correctness = None

        total_time += time()

        if correctness:
            reason = ""
        elif correctness is None:
            reason = "skipped"
        elif total_time > args.timeout:
            reason = "timeout"
        elif 0 is not 0:
            reason = "returncode"
        else:
            reason = "diff ({} != {})".format(our_output, correct_output)


        print("{} {:8} {:>20} [{:6}k]   {:6.2f}s".format("\033[92m✓\033[0m" if correctness else "\033[91m✗\033[0m", reason, path.name, stdin_size, total_time))
        # if not correct and outs:
except KeyboardInterrupt:
    pass
