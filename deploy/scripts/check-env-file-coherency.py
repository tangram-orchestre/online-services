#!/bin/env python3
"""Checks that every `.env` file has a `sample.env` next to it with the same keys."""

from pathlib import Path
import sys


def get_keys(path: Path):
    return [line.strip().split('=')[0] for line in path.read_text().splitlines() if line.strip()]


if __name__ == "__main__":
    deploy_dir = Path(__file__).resolve().parent.parent
    errors = []

    dirs = {p.parent for p in deploy_dir.rglob('.env')} | {p.parent for p in deploy_dir.rglob('sample.env')}
    for d in sorted(dirs):
        dot_env = d / '.env'
        sample = d / 'sample.env'
        name = d.relative_to(deploy_dir)

        if not sample.exists():
            errors.append(f'`{name}`: each .env should have an associated sample.env')
        elif dot_env.exists() and get_keys(dot_env) != get_keys(sample):
            errors.append(f'`{name}`: keys do not match between .env {get_keys(dot_env)} and sample.env {get_keys(sample)}')

    for error in errors:
        print(error, file=sys.stderr)
    sys.exit(1 if errors else 0)
