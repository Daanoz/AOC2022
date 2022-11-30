# AOC TS solutions

## Installation

```sh
yarn
```

## Usage

```sh
# Default startup
yarn start

# Start puzzle in watch mode
yarn start -- --mode watch

# Run puzzle for specific day
yarn start -- --day 1 --mode run

# Alter the file to read from the puzzle dir (defaults to "input")
yarn start -- --input test
```

## Automatic downloading of input

Define an environment variable called `AOC_SESSION` using the cookie session token from adventofcode.com

## Visualizing complex puzzles

To enable complex visualizations, run a puzzle in watch mode, and open the vizualizer using `npm run browser`. This only works in a very limited set of puzzles...
