from aocd import get_data
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--day', type=int)
args = parser.parse_args()

print(get_data(day=args.day))
