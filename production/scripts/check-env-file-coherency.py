#!/bin/env python3

from pathlib import Path
import os
import sys

def get_key(line: str):
    return line.strip().split('=')[0]

if __name__ == "__main__":
    dir_path = Path(__file__).parent.parent

    for d in os.listdir(dir_path):
        p = dir_path / d
        if os.path.isdir(d):
            dot_env_path = p / '.env'
            sample_path = p / 'sample.env'

            dot_env = os.path.exists(dot_env_path)
            sample = os.path.exists(sample_path)

            if not dot_env and not sample:
                continue

            if dot_env and sample:
                with open(dot_env_path) as de:
                    with open(sample_path) as s:
                        de_lines = de.readlines()
                        s_lines = s.readlines()

                        if len(de_lines) != len(s_lines):
                            print(f'`{p}`: .env and sample.env should have the same number of lines', file=sys.stderr)
                            exit(1)

                        for [a, b] in zip(de_lines, s_lines):
                            a = get_key(a)
                            b = get_key(b)
                            if a != b:
                                print(f'`{p}`: key do not match between sample and real: `{a}` vs `{b}`', file=sys.stderr)
                                exit(1)

            else:
                print(f'`{p}`: each .env should have an associated sample.env', file=sys.stderr)
                exit(1)