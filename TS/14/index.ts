import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

const WALL = '█'
const SAND_SRC = '+'
const SAND = '.'
type Cell = typeof WALL | typeof SAND_SRC | typeof SAND

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private grid: EndlessGrid<Cell> = new EndlessGrid()
    private sandSrc = [500, 0]
    private floorBottom = 0

    public run(): Result {
        const result: Result = {}

        this.getInputAsRows().map(row => this.addToGrid(row))
        this.grid.set(this.sandSrc[0], this.sandSrc[1], SAND_SRC)
        this.floorBottom = this.grid.getYRange()[1] + 2

        let sandTurns = 0
        while (this.dropSand([this.sandSrc[0], this.sandSrc[1]])) {
            sandTurns++
        }
        result.a = sandTurns
        sandTurns++ // already registered when finding ABYSS
        while (this.grid.get(this.sandSrc[0], this.sandSrc[1]) !== SAND) {
            this.dropSand([this.sandSrc[0], this.sandSrc[1]])
            sandTurns++
        }
        result.b = sandTurns

        // console.log(this.grid.toString({upsideDown: true}))

        return result
    }

    private dropSand(sand: [number, number]): boolean {
        const [x, y] = sand
        if (y + 1 >= this.floorBottom) {
            this.grid.set(x, y, SAND)
            return false // FALLING INTO ABYSS
        }
        const cellBelow = this.grid.get(x, y + 1)
        if (cellBelow === undefined) {
            return this.dropSand([x, y + 1])
        }
        const cellBottomLeft = this.grid.get(x - 1, y + 1)
        if (cellBottomLeft === undefined) {
            return this.dropSand([x - 1, y + 1])
        }
        const cellBottomRight = this.grid.get(x + 1, y + 1)
        if (cellBottomRight === undefined) {
            return this.dropSand([x + 1, y + 1])
        }
        this.grid.set(x, y, SAND)
        return true // RESTED
    }

    private addToGrid(row: string) {
        const coords = row.split(' -> ').map(c => c.split(',').map(Number) as [number, number])

        for (let i = 1; i < coords.length; i++) {
            const start = coords[i - 1]
            const end = coords[i]
            if (start[0] === end[0]) {
                for (let y = start[1]; y != end[1]; y += start[1] < end[1] ? 1 : -1) {
                    this.grid.set(start[0], y, WALL)
                }
            } else {
                for (let x = start[0]; x != end[0]; x += start[0] < end[0] ? 1 : -1) {
                    this.grid.set(x, start[1], WALL)
                }
            }
            this.grid.set(end[0], end[1], WALL)
        }
    }   
}

Runner(PuzzleSolution)