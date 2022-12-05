import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const [stackInput, moves] = this.getInput().split('\n\n').map(l => l.split('\n'))

        result.a = this.getPartA(stackInput, moves)
        result.b = this.getPartB(stackInput, moves)

        return result
    }

    private getPartA(stackInput: string[], moves: string[]): string {
        const stacks = this.parseStacks(stackInput)
        moves.forEach(move => this.makeMove(move, stacks))
        return Array.from(stacks.values()).map(stack => stack[stack.length - 1]).join('')
    }

    private getPartB(stackInput: string[], moves: string[]): string {
        const stacks = this.parseStacks(stackInput)
        moves.forEach(move => this.makeMove(move, stacks, true))
        return Array.from(stacks.values()).map(stack => stack[stack.length - 1]).join('')
    }

    private makeMove(move: string, stacks: Map<string, string[]>, isCrateMover9001 = false) {
        if (!move) {
            return
        }
        const step = move.match(/move (\d+) from (\d+) to (\d+)/)
        if (!step) {
            throw new Error('Unknown move: ' + move)
        }
        const [countStr, src, target] = step.slice(1)
        const count = parseInt(countStr)
        if (isCrateMover9001) {
            stacks.get(target)?.push(...stacks.get(src)!.splice(-count))
        } else {
            for (let c = 0; c < count; c++) {
                stacks.get(target)?.push(stacks.get(src)!.pop()!)
            }
        }
    }

    private parseStacks(stackInput: string[]): Map<string, string[]> {
        const stacks = new Map()
        const reversedInput = [...stackInput].reverse()
        reversedInput.shift()?.match(/ \d  ?/g)?.forEach(column => {
            stacks.set(column.trim(), [])
        })
        reversedInput.forEach(line => {
            const matches = line.match(/(\[[A-Z]\]|   ) ?/g)
            matches?.forEach((cell, index) => {
                if (cell.trim() !== '') {
                    stacks.get(`${index + 1}`).push(cell.match(/\[(.)\] ?/)![1])
                }
            })
        })
        return stacks
    }

}

Runner(PuzzleSolution)