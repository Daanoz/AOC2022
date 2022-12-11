import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

class Monkey {
    private startingItems: number[]
    private operation: (number: number) => number
    private divisibleBy: number
    private valid: number
    private invalid: number
    private actions = 0

    constructor(startingItems: number[], operation: (number: number) => number, divisibleBy: number, valid: number, invalid: number) {
        this.startingItems = startingItems
        this.operation = operation
        this.divisibleBy = divisibleBy
        this.valid = valid
        this.invalid = invalid
    }

    public doTurn(monkeys: Monkey[], doDivide: boolean, overflowModulo: number) {
        const items = [...this.startingItems]
        this.startingItems = []
        items.forEach(item => {
            item %= overflowModulo
            let newLevel = 0
            if (doDivide) {
                newLevel = Math.floor(this.operation(item) / 3)
            } else {
                newLevel = this.operation(item)
            }
            const isValid = newLevel % this.divisibleBy === 0
            if (isValid) {
                monkeys[this.valid].catch(newLevel)
            } else {
                monkeys[this.invalid].catch(newLevel)
            }
            this.actions++
        })
    }

    public catch(item: number) {
        this.startingItems.push(item)
    }

    public getDivisibleBy(): number {
        return this.divisibleBy
    }

    public getAmountOfActions(): number {
        return this.actions
    }

    static asMonkey(input: string): Monkey {
        const startingItems = input.match(/Starting items: ([\d, ]*)/)![1].split(', ').map(v => parseInt(v))
        const operationMatch = input.match(/Operation: new = old (\*|\+) (.+)/)!
        const operation = operationMatch[1]
        const operationArgument = operationMatch[2]
        const test = parseInt(input.match(/Test: divisible by (\d+)/)![1])
        const valid = parseInt(input.match(/If true: throw to monkey (\d+)/)![1])
        const invalid = parseInt(input.match(/If false: throw to monkey (\d+)/)![1])
        return new Monkey(
            startingItems,
            Monkey.stringOpToOp(operation, operationArgument),
            test,
            valid,
            invalid
        )
    }

    static stringOpToOp(operation: string, operationArgument: string): (val: number) => number {
        if (operationArgument === 'old') {
            if (operation === '+') {
                return ((val: number) => val + val)
            } else {
                return ((val: number) => val * val)
            }
        } else {
            const arg = parseInt(operationArgument)
            if (operation === '+') {
                return ((val: number) => val + arg)
            } else {
                return ((val: number) => val * arg)
            }
        }
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        let monkeys = this.getInputAsRows('\n\n').map(Monkey.asMonkey)
        const overflowModulo = monkeys.reduce((mod, monkey) => mod * monkey.getDivisibleBy(), 1)

        for (let i = 0; i < 20; i++) {
            monkeys.forEach(monkey => monkey.doTurn(monkeys, true, overflowModulo))
        }
        let actionCounts = monkeys.map(m => m.getAmountOfActions()).sort((a, b) => b - a)
        result.a = actionCounts[0] * actionCounts[1]

        monkeys = this.getInputAsRows('\n\n').map(Monkey.asMonkey)
        for (let i = 0; i < 10000; i++) {
            monkeys.forEach(monkey => monkey.doTurn(monkeys, false, overflowModulo))
        }
        actionCounts = monkeys.map(m => m.getAmountOfActions()).sort((a, b) => b - a)
        result.b = actionCounts[0] * actionCounts[1]

        return result
    }

}

Runner(PuzzleSolution)