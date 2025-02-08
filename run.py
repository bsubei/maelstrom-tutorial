#!/bin/python3

import argparse
import logging
import sys
import subprocess
from tqdm import tqdm

from enum import Enum

logger = logging.getLogger(__name__)

CARGO_BUILD = ["cargo", "build"]
MAELSTROM_PATH = "./maelstrom/maelstrom"
TARGET_PATH = "./target/debug/maelstrom-tutorial"


class Topology(Enum):
    LINE = "line"
    TREE4 = "tree4"

    def __str__(self):
        return self.value


class Nemesis(Enum):
    PARTITION = "partition"

    def __str__(self):
        return self.value

def check_positive_int(value):
    ivalue = int(value)
    if ivalue <= 0:
        raise argparse.ArgumentTypeError(f"{value} is an invalid positive int value")
    return ivalue

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    # Common arguments go here.
    parser.add_argument("-v", "--verbose", action="store_true", help="If enabled, log at debug level.")
    parser.add_argument("-t", "--topology", type=Topology, choices=list(Topology), default=Topology.LINE, help="The topology flag to pass to maelstrom.")
    parser.add_argument("-n", "--nemesis", type=Nemesis, choices=list(Nemesis), required=False, help="The nemesis flag to pass to maelstrom.")
    parser.add_argument("--loop", type=check_positive_int, help="If specified, will run this many times in a loop looking for non-zero exit codes.")

    # Define the subcommands here.
    subparser = parser.add_subparsers(required=True, dest="subcommand")

    _ = subparser.add_parser('echo')

    _ = subparser.add_parser('broadcast')

    return parser.parse_args()


def main(args: argparse.Namespace) -> int:
    # Build first, and exit early if failed.
    retval = subprocess.run(CARGO_BUILD)
    if retval.returncode != 0:
        return retval.returncode

    run_cmd = [MAELSTROM_PATH , "test", "-w", args.subcommand, "--bin", TARGET_PATH, "--time-limit", "20", "--topology", args.topology.value, "--log-stderr"]
    if args.nemesis:
        run_cmd = run_cmd + ["--nemesis", args.nemesis.value]

    if args.loop is not None:
        logger.info(f"Running in a loop {args.loop} times... Hiding output unless error is encountered...")
        for i in tqdm(range(args.loop)):
            retval = subprocess.run(run_cmd, capture_output=True)
            if retval.returncode != 0:
                logger.error(f"Encountered non-zero returncode: {retval.returncode}")
                logger.error(f"Stdout:\n{retval.stdout}\n\n")
                logger.error(f"Stderr:\n{retval.stderr}")
                return retval.returncode

        logger.info(f"Successfully ran with no errors {args.loop} times!")
    else:
        retval = subprocess.run(run_cmd)

    return retval.returncode

if __name__ == "__main__":
    args = parse_args()
    logging.basicConfig(format='[%(asctime)s] {%(pathname)s:%(lineno)d} %(levelname)s - %(message)s', level=logging.DEBUG if args.verbose else logging.INFO)
    sys.exit(main(args))