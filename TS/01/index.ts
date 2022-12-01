import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const elves = this.getInput().split('\n\n')
        const amountPerElf = elves.map(elf => elf.split('\n').map(v => parseInt(v)).reduce((sum, val) => sum + val, 0))
        result.a = Math.max(...amountPerElf)
        amountPerElf.sort((a, b) => b - a)
        result.b = amountPerElf.slice(0, 3).reduce((sum, val) => sum + val, 0)
        return result
    }

}

Runner(PuzzleSolution)