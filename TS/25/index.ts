import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

function snafuToDecimal(val: string): number {
    const digits = val.length
    return val.split('').reduce((total, ds, i) => {
        let d: number
        if (ds === '-') { d = -1 }
        else if (ds === '=') { d = -2 }
        else { d = parseInt(ds) }
        return total + (Math.pow(5, (digits - i) - 1) * d)
    }, 0)
}

function decimalToSnafu(val: number): string {
    const digits = Math.ceil(Math.log(val) / Math.log(5))
    let out = ''
    let remainder = val
    for (let i = digits; i >= 0; i--) {
        const lowestNextDigit = snafuToDecimal(new Array(i).fill('=').join(''))
        const snafuPower = Math.floor((remainder - lowestNextDigit) / Math.pow(5, i))
        if (snafuPower !== 0 || out !== '') {
            if (snafuPower == -2) { out += '=' }
            else if (snafuPower == -1) { out += '-' }
            else { out += `${snafuPower}` }
            remainder -= (snafuPower * Math.pow(5, i))
        }
    }
    return out
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const numbers = this.getInputAsRows().map(v => snafuToDecimal(v))
        result.a = decimalToSnafu(numbers.reduce((sum, v) => sum + v, 0))

        return result
    }

}

Runner(PuzzleSolution)