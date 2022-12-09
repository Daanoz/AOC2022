import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'


export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private grid = new EndlessGrid<boolean>()
    private head = [0, 0]
    private rope: [number, number][] = []

    public run(): Result {
        const result: Result = {}

        result.a = this.simulateRope(2)
        result.b = this.simulateRope(10)

        return result
    }

    private simulateRope(length: number): number {
        this.head = [0, 0]
        this.rope = new Array(length - 1).fill(null).map(() => [0, 0])
        this.grid = new EndlessGrid<boolean>()
        this.grid.set(0, 0, true)
        this.getInputAsRows().forEach(move => this.makeMove(move))
        return this.grid.countBy(c => c)
    }

    private makeMove(move: string) {
        const [direction, distanceStr] = move.split(' ')
        const distance = parseInt(distanceStr)
        for (let i = 0; i < distance; i++) {
            switch (direction) {
                case 'U': this.head = [this.head[0], this.head[1] + 1]; break
                case 'D': this.head = [this.head[0], this.head[1] - 1]; break
                case 'L': this.head = [this.head[0] - 1, this.head[1]]; break
                case 'R': this.head = [this.head[0] + 1, this.head[1]]; break
            }
            let prev = this.head
            this.rope.forEach((current, i) => {
                if (prev[0] === current[0] && prev[1] === current[1]) {
                    prev = this.rope[i] 
                    return
                }
                if (Math.abs(prev[0] - current[0]) >= 2 && Math.abs(prev[1] - current[1]) >= 2) {
                    const deltaX = (prev[0] < current[0] ? -1 : 1)
                    const deltaY = (prev[1] < current[1] ? -1 : 1)
                    this.rope[i] = [current[0] + deltaX, current[1] + deltaY]
                } else if (Math.abs(prev[1] - current[1]) >= 2) {
                    const delta = (prev[1] < current[1] ? -1 : 1)
                    this.rope[i] = [prev[0], current[1] + delta]
                } else if (Math.abs(prev[0] - current[0]) >= 2) {
                    const delta = (prev[0] < current[0] ? -1 : 1)
                    this.rope[i] = [current[0] + delta, prev[1]]
                }
                prev = this.rope[i]
            })
            const tail = this.rope[this.rope.length - 1]
            this.grid.set(tail[0], tail[1], true)
        }
    }
}

Runner(PuzzleSolution)