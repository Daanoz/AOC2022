import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

const EMPTY = '.'
enum Direction {
    Right = '>', 
    Down = 'v', 
    Left = '<', 
    Up = '^'
}
type Coord = [number, number]
type Range = [number, number]
class Blizzard {
    public direction: Direction
    public pos: Coord
    constructor(direction: Direction, pos: Coord) {
        this.direction = direction
        this.pos = pos
    }
}

function coordToHash(coord: Coord): number {
    return coord[0] * 1000 + coord[1]
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private blizzardsPerTurn: Map<number, Blizzard[]> = new Map()
    private grid: EndlessGrid<string> = new EndlessGrid()
    private yBounds: Range = [0, 0]
    private xBounds: Range = [0, 0]
    private startPos: Coord = [0, 0]
    private endPos: Coord = [0, 0]
    public run(): Result {
        const result: Result = {}

        const blizzards: Blizzard[] = []
        this.grid = this.getInputAsGrid<string>({ 
            splitByCol: ''
         }).map((cell, pos) => {
            switch(cell) {
                case '^': blizzards.push(new Blizzard(Direction.Up, pos)); return EMPTY 
                case '<': blizzards.push(new Blizzard(Direction.Left, pos)); return EMPTY 
                case 'v': blizzards.push(new Blizzard(Direction.Down, pos)); return EMPTY 
                case '>': blizzards.push(new Blizzard(Direction.Right, pos)); return EMPTY 
                default: return cell
            }
        })
        this.yBounds  = [this.grid.getYRange()[1] - 1, this.grid.getYRange()[0] + 1]
        this.xBounds  = [this.grid.getXRange()[0] + 1, this.grid.getXRange()[1] - 1]
        this.startPos = [this.xBounds[0], this.yBounds[0] + 1]
        this.endPos   = [this.xBounds[1], this.yBounds[1] - 1]
        this.blizzardsPerTurn.set(0, blizzards)

        result.a = this.runBFS()
        
        this.endPos   = [this.xBounds[0], this.yBounds[0] + 1]
        this.startPos = [this.xBounds[1], this.yBounds[1] - 1]
        const goBack = this.runBFS(result.a)

        this.startPos = [this.xBounds[0], this.yBounds[0] + 1]
        this.endPos   = [this.xBounds[1], this.yBounds[1] - 1]
        result.b = this.runBFS(goBack)

        return result
    }

    private runBFS(offset = 0): number {
        let positions: Coord[] = [this.startPos]


        for (let i = 1 + offset; i < 5000; i++ ) {
            const blizzards = new Map(this.getBlizzardsForTurn(i).map(b => [coordToHash(b.pos), b]))
            const nextPositionMap = new Map<number, Coord>()
            for (let p = 0; p < positions.length; p++) {
                const pos = positions[p]
                const moves: Coord[] = [pos]
                if (pos[0] === this.endPos[0] && (pos[1] === (this.endPos[1] + 1) || pos[1] === (this.endPos[1] - 1))) {
                    return i
                }
                const isInYBounds = pos[1] <= this.yBounds[0] && pos[1] >= this.yBounds[1]
                if (pos[0] > this.xBounds[0] && isInYBounds) { moves.push([pos[0] - 1, pos[1]]) }
                if (pos[0] < this.xBounds[1] && isInYBounds) { moves.push([pos[0] + 1, pos[1]]) }
                if (pos[1] > this.yBounds[1]) { moves.push([pos[0], pos[1] - 1]) }
                if (pos[1] < this.yBounds[0]) { moves.push([pos[0], pos[1] + 1]) }
 
                moves
                    .map(m => ({ hash: coordToHash(m), move: m }))
                    .forEach(m => {
                        if (!blizzards.has(m.hash) && !nextPositionMap.has(m.hash)) {
                            nextPositionMap.set(m.hash, m.move)
                        }
                    })
            }
            positions = Array.from(nextPositionMap.values())
        }
        return -1
    }

    private getBlizzardsForTurn(turn: number): Blizzard[] {
        const blizzardsFromMap = this.blizzardsPerTurn.get(turn)
        if (blizzardsFromMap !== undefined) {
            return blizzardsFromMap
        }
        const previousBlizzards = this.getBlizzardsForTurn(turn - 1)
        const blizzards = Array.from(previousBlizzards.values()).map(b => {
            switch(b.direction) {
                case Direction.Up:    return new Blizzard(b.direction, [b.pos[0], b.pos[1] >= this.yBounds[0] ? this.yBounds[1] : b.pos[1] + 1])
                case Direction.Right: return new Blizzard(b.direction, [b.pos[0] >= this.xBounds[1] ? this.xBounds[0] : b.pos[0] + 1, b.pos[1]]) 
                case Direction.Down:  return new Blizzard(b.direction, [b.pos[0], b.pos[1] <= this.yBounds[1] ? this.yBounds[0] : b.pos[1] - 1]) 
                case Direction.Left:  return new Blizzard(b.direction, [b.pos[0] <= this.xBounds[0] ? this.xBounds[1] : b.pos[0] - 1, b.pos[1]]) 
            }
        })
        this.blizzardsPerTurn.set(turn, blizzards)
        return blizzards
    }
}

Runner(PuzzleSolution)