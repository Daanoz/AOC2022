import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

class Digit {
    public value: number
    public previous?: Digit = undefined
    public next?: Digit = undefined

    constructor(value: number) {
        this.value = value
    }

    public mix(totalCount: number) {
        if (this.value === 0) {
            return
        }
        // detach current
        this.next!.previous = this.previous
        this.previous!.next = this.next

        const steps = this.value % (totalCount - 1)
        const target = steps < 0 ? this.previous!.traverse(steps + 1) : this.next!.traverse(steps)

        // insert at new position
        this.previous = target.previous
        this.previous!.next = this
        this.next = target
        target.previous = this
    }

    public traverse(steps: number): Digit {
        if (steps === 0) {
            return this
        } else if (steps < 0) {
            return this.previous!.traverse(steps + 1)
        } else if (steps > 0) {
            return this.next!.traverse(steps - 1)
        }
        throw new Error('Unable to traverse ' + steps)
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const originalSequence = this.getInputAsRows()
            .map(v => new Digit(Number(v)))
        const count = originalSequence.length
        originalSequence.forEach((d, i) => {
            d.previous = originalSequence[(count + (i - 1)) % count]
            d.next = originalSequence[(count + (i + 1)) % count]
        })
        originalSequence.forEach(d => d.mix(count))
        let _1000thDigit = originalSequence.find(d => d.value === 0)!.traverse(1000) 
        let _2000thDigit = _1000thDigit.traverse(1000) 
        let _3000thDigit = _2000thDigit.traverse(1000) 

        result.a = _1000thDigit.value + _2000thDigit.value + _3000thDigit.value

        const decipherKey = 811589153
        originalSequence.forEach((d, i) => {
            d.value *= decipherKey
            d.previous = originalSequence[(count + (i - 1)) % count]
            d.next = originalSequence[(count + (i + 1)) % count]
        })
        for (let i = 0; i < 10; i++) {
            originalSequence.forEach(d => d.mix(count))
        }
        _1000thDigit = originalSequence.find(d => d.value === 0)!.traverse(1000) 
        _2000thDigit = _1000thDigit.traverse(1000) 
        _3000thDigit = _2000thDigit.traverse(1000) 

        result.b = _1000thDigit.value + _2000thDigit.value + _3000thDigit.value

        return result
    }

}

Runner(PuzzleSolution)