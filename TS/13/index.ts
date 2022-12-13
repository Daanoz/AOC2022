import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

type Msg = (number | Msg)[]
type MsgSet = [Msg, Msg]

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const pairs: MsgSet[] = this.getInputAsRows('\n\n')
            .map(set => set.split('\n').map(msg => JSON.parse(msg)) as MsgSet)

        result.a = pairs.reduce((sum, pair, i) => {
            const o = this.isValidPair(pair)
            return o ? sum + (i + 1) : sum
        }, 0)

        const divider1 = [[2]]
        const divider2 = [[6]]

        const ordered = pairs.flat()
            .concat([divider1, divider2])
            .sort((a, b) => this.isValidPair([a, b]) ? -1 : 1)
        result.b = (ordered.indexOf(divider1) + 1) * (ordered.indexOf(divider2) + 1)

        return result
    }

    private isValidPair(pair: MsgSet): boolean {
        let [left, right] = pair
        if (!Array.isArray(left) && !Array.isArray(right)) {
            return left <= right
        }
        if (!Array.isArray(left)) { left = [left] }
        if (!Array.isArray(right)) { right = [right] }
        for (let i = 0; i < left.length; i++) {
            if (right[i] === undefined) {
                return false
            }
            if (left[i] === right[i]) {
                continue
            }
            return this.isValidPair([
                left[i] as Msg,
                right[i] as Msg
            ])
        }
        return true
    }
}

Runner(PuzzleSolution)