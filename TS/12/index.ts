import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

const BOTTOM_CHAR = 'a'.charCodeAt(0)

class GridCell {
    private height: number
    private startCell = false
    private exitCell = false

    constructor(cell: string) {
        if (cell === 'S') {
            this.startCell = true
            cell = 'a'
        } else if (cell === 'E') {
            this.exitCell = true
            cell = 'z'
        }
        this.height = cell.charCodeAt(0) - BOTTOM_CHAR
    }

    public getHeight(): number {
        return this.height
    }
    public isStart(): boolean {
        return this.startCell
    }
    public isExit(): boolean {
        return this.exitCell
    }
    public toString(): string {
        return String.fromCharCode(this.height + BOTTOM_CHAR)
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private grid: EndlessGrid<GridCell> = new EndlessGrid()
    private longestDistance: number = Number.POSITIVE_INFINITY

    public run(): Result {
        const result: Result = {}

        this.grid = this.getInputAsGrid({
            splitByCol: '',
            splitByRow: '\n',
            cellParser: cell => new GridCell(cell)
        })
        const start = this.grid.findIndex(c => c.isStart())!
        result.a = this.fillPaths([start], [this.grid.getByIndex(start)!], 1)
        result.b = this.findShortestPath()
        return result
    }

    private findShortestPath() {
        const startPositions = this.grid
            .filterIndex(c => c.getHeight() === 0)
            .filter(index => !!this.grid.getNeighbors(index[0], index[1]).find(n => n?.getHeight() === 1))
        let shortestDistance = Number.POSITIVE_INFINITY
        for (let i = 0; i < startPositions.length; i++) {
            const coord = startPositions[i]
            const distance = this.fillPaths([coord], [this.grid.getByIndex(coord)!], 1)
            shortestDistance = Math.min(shortestDistance, distance)
        }
        return shortestDistance
    }

    private fillPaths(currentPositions: [number, number][], visited: GridCell[], currentDistance: number): number {
        if (currentDistance > this.longestDistance) {
            return Number.POSITIVE_INFINITY
        }
        const cellsToCheck = currentPositions.reduce((list, pos) => {
            const currentCell = this.grid.getByIndex(pos)!
            const neighborIndexes = this.grid.getNeighborsIndexes(pos[0], pos[1], {
                includeDiagonals: false,
                onlyDefined: true
            })
            const neighbors = neighborIndexes
                .map(index => ({
                    position: index,
                    cell: this.grid.getByIndex(index)!
                }))
                .filter(neighbor => neighbor.cell.getHeight() <= currentCell.getHeight() + 1)
                .filter(neighbor => !visited.includes(neighbor.cell))
            visited.push(...neighbors.map(n => n.cell))
            return list.concat(neighbors)
        }, [] as {
            position: [number, number],
            cell: GridCell
        }[])
        const end = cellsToCheck.find(c => c.cell.isExit())
        if (end) {
            this.longestDistance = Math.min(this.longestDistance, currentDistance)
            return currentDistance
        }
        return this.fillPaths(cellsToCheck.map(c => c.position), visited, currentDistance + 1)
    }
}

Runner(PuzzleSolution)