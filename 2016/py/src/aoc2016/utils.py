import re

NUM = re.compile(r'(\d+)')

def numbers(line: str):
    return [int(n) for n in NUM.findall(line)]
