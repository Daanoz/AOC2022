import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

const HUMAN = 'humn'
const ROOT = 'root'


class Monkey {
    public name = ''
    private value?: number
    private leftName?: string
    public left?: Monkey
    private operation?: string
    private rightName?: string
    public right?: Monkey
    private isHumanInTree?: boolean

    constructor(row: string) {
        const [name, statement] = row.split(': ')
        this.name = name
        if (statement.match(/^(\d+)$/)) {
            this.value = parseInt(statement)
        } else {
            this.leftName = statement.substring(0, 4)
            this.operation = statement.substring(5, 6)
            this.rightName = statement.substring(7, 11)
        }
    }

    public setMonkeys(monkeyMap: Map<string, Monkey>) {
        if (this.leftName) { this.left = monkeyMap.get(this.leftName) }
        if (this.rightName) { this.right = monkeyMap.get(this.rightName) }
    }

    public getValue(): number {
        if (this.value !== undefined) {
            return this.value
        }
        const left = this.left!.getValue()
        const right = this.right!.getValue()
        switch (this.operation) {
            case '+' : this.value = left + right; break
            case '-' : this.value = left - right; break
            case '*' : this.value = left * right; break
            case '/' : this.value = left / right; break
            default: throw new Error('Unknown operation: ' + this.operation)
        }
        return this.value
    }

    public hasHuman(): boolean {
        if (this.name === HUMAN) { return true }
        if (this.isHumanInTree !== undefined) { return this.isHumanInTree }
        if (this.left === undefined || this.right === undefined) { return false }
        this.isHumanInTree = this.left!.hasHuman() || this.right!.hasHuman()
        return this.isHumanInTree
    }

    public findHumanValue(desiredOutcome = 0): number {
        if (this.name === HUMAN) { return desiredOutcome }
        if (!this.left) { return -1 }
        if (this.left!.hasHuman()) {
            const right = this.right!.getValue()
            if (this.name === ROOT) { return this.left!.findHumanValue(right) }
            let targetValue: number
            switch (this.operation) {
                case '+' : targetValue = (desiredOutcome - right); break
                case '-' : targetValue = (desiredOutcome + right); break 
                case '*' : targetValue = (desiredOutcome / right); break
                case '/' : targetValue = (desiredOutcome * right); break
                default: throw new Error('Unknown operation: ' + this.operation)
            }
            return this.left!.findHumanValue(targetValue)
        } else {
            const left = this.left!.getValue()
            if (this.name === ROOT) { return this.right!.findHumanValue(left) }
            let targetValue: number
            switch (this.operation) {
                case '+' : targetValue = (desiredOutcome - left); break
                case '-' : targetValue = (left - desiredOutcome); break 
                case '*' : targetValue = (desiredOutcome / left); break
                case '/' : targetValue = (left / desiredOutcome); break
                default: throw new Error('Unknown operation: ' + this.operation)
            }
            return this.right!.findHumanValue(targetValue)
        }
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const monkeyMap = new Map(this.getInputAsRows().map(m => new Monkey(m)).map(m => [m.name, m]))
        Array.from(monkeyMap.values()).forEach(m => m.setMonkeys(monkeyMap))
        const rootMonkey = monkeyMap.get(ROOT)!

        result.a = rootMonkey.getValue()
        result.b = rootMonkey.findHumanValue()
        return result
    }

}

Runner(PuzzleSolution)