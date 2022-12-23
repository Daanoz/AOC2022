import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

class Elf {
    public toString() { return '#' }
}
const EMPTY = ' '
type GridCell = Elf | typeof EMPTY

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}
        let grid = this.getInputAsGrid<GridCell>({
            splitByCol: '',
            cellParser: (v) => (v === '#') ? new Elf() : EMPTY
        })

        for (let i = 0; i < 10; i++) {
            grid = this.makeMoves(grid, i) || grid
        }

        result.a = grid.countBy(c => c === EMPTY, EMPTY)

        for (let i = 10; i < 100000; i++) {
            const nextGrid = this.makeMoves(grid, i)
            if (nextGrid === undefined) {
                result.b = (i + 1)
                break
            }
            grid = nextGrid
        }

        return result
    }

    private makeMoves(grid: EndlessGrid<GridCell>, roundIndex: number): EndlessGrid<GridCell> | undefined {
        const next = new EndlessGrid<GridCell>()
        const moves = new Map<number, [number, number, GridCell]>()
        grid.filterCellAndIndex(c => c instanceof Elf).forEach((p) => {
            const currentElf = p[2]
            const neighbors = [
                [ grid.get(p[0] - 1, p[1] + 1, EMPTY), grid.get(p[0] + 0, p[1] + 1, EMPTY), grid.get(p[0] + 1, p[1] + 1, EMPTY) ],
                [ grid.get(p[0] - 1, p[1] + 0, EMPTY), EMPTY                              , grid.get(p[0] + 1, p[1] + 0, EMPTY) ],
                [ grid.get(p[0] - 1, p[1] - 1, EMPTY), grid.get(p[0] + 0, p[1] - 1, EMPTY), grid.get(p[0] + 1, p[1] - 1, EMPTY) ]
            ]
            const hasNeighbor = neighbors.find(row => row.find(c => c !== EMPTY))
            if (!hasNeighbor) {
                next.set(p[0], p[1], currentElf)
                return
            }
            let nextP: [number, number] = [p[0], p[1]]
     
            const directionalMoves: ([number, number] | undefined)[] = [
                neighbors[0].every(c => c === EMPTY) ? [p[0], p[1] + 1] : undefined,
                neighbors[2].every(c => c === EMPTY) ? [p[0], p[1] - 1] : undefined,
                neighbors.every(c => c[0] === EMPTY) ? [p[0] - 1, p[1]] : undefined,
                neighbors.every(c => c[2] === EMPTY) ? [p[0] + 1, p[1]] : undefined,
            ]
            for (let dm = roundIndex; dm < (roundIndex + 4); dm++) {
                if (directionalMoves[dm % 4]) {
                    nextP = directionalMoves[dm % 4]!
                    break
                }
            }

            const moveKey = nextP[0] * 1000 + nextP[1]
            const existingMove = moves.get(moveKey)
            if (!existingMove) {
                next.set(nextP[0], nextP[1], currentElf)
                moves.set(moveKey, p)
                return
            }
            const currentCell = next.getByIndex(nextP, EMPTY)
            if (currentCell !== EMPTY) {
                // has clashed, reset position
                next.set(nextP[0], nextP[1], EMPTY)
                next.set(existingMove[0], existingMove[1], existingMove[2])
            }
            next.set(p[0], p[1], currentElf)
        })
        if (moves.size < 1) {
            return undefined
        }
        return next
    }
}

Runner(PuzzleSolution)