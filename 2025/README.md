This uses entrypoints feature to decouple runner from solutions, so the solutions may be in different python projects, and they work as long as they are installed to the same venv as the runner.

To run:

```
python -m venv .venv
source .venv/bin/activate
pip install --editable .
pytest
aoc2025 --day=2
```

## Note about day 1:

Default implementation of day 1 uses [rust](https://www.reddit.com/r/adventofcode/comments/1pb3y8p/comment/nrryygj/), and requires this:

```
cd day1rs
pip install .
cd ..
```

To run pure python day 1, use `aoc2025 --day=1 --variant=py`
