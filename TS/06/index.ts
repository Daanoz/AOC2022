import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

function isStartMarker(str: string[], count: number): boolean {
    if (str.length < count) {
        return false
    }
    return !str.find((value, index, list) => list.indexOf(value) !== index)
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const characters = this.getInput()
            .trim()
            .split('')

        result.a = characters.findIndex((_, index, chars) => isStartMarker(chars.slice(index - 4, index), 4))
        result.b = characters.findIndex((_, index, chars) => isStartMarker(chars.slice(index - 14, index), 14))
        return result
    }

}

Runner(PuzzleSolution)