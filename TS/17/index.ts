import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

const EMPTY = ' '
const BLOCK = '#'
const WALL = '█'

type Cell = typeof WALL | typeof EMPTY | typeof BLOCK

function findLastIndex<T>(list: T[], predicate: (val: T, index: number) => boolean) {
    for (let i = list.length - 1; i >= 0; i--) {
        if (predicate(list[i], i)) {
            return i
        }
    }
    return -1
}

abstract class Block {
    protected shape: boolean[][] = []

    public canGoDown(pos: [number, number], grid: EndlessGrid<Cell>): boolean {
        const shapeHeight = this.shape.length
        const bottomRow = this.shape[shapeHeight - 1]
        const indexes = bottomRow.map((_, index) => shapeHeight - (findLastIndex(this.shape, (row) => row[index]) + 1))
        return indexes.find((yI, xI) => {
            return grid.get((pos[0] + xI), (pos[1] + yI) - 1, EMPTY) !== EMPTY
        }) === undefined   
    }
    public canGoLeft(pos: [number, number], grid: EndlessGrid<Cell>): boolean {
        const shapeHeight = this.shape.length
        return this.shape.find((row, yI) => {
            const leftEdge = row.findIndex(c => c)
            return grid.get((pos[0] + leftEdge) - 1, pos[1] + (shapeHeight - (yI + 1)), EMPTY) !== EMPTY
        }) === undefined 
    }
    public canGoRight(pos: [number, number], grid: EndlessGrid<Cell>): boolean {
        const shapeHeight = this.shape.length
        return this.shape.find((row, yI) => {
            const rightEdge = findLastIndex(row, c => c)
            return grid.get((pos[0] + rightEdge) + 1, pos[1] + (shapeHeight - (yI + 1)), EMPTY) !== EMPTY
        }) === undefined     
    }
    public applyJet(jet: string, pos: [number, number], grid: EndlessGrid<Cell>): [number, number] {
        if (jet === '<') {
            if (this.canGoLeft(pos, grid)) {
                return [pos[0] - 1, pos[1]]
            }
        } else {
            if (this.canGoRight(pos, grid)) {
                return [pos[0] + 1, pos[1]]
            }
        }
        return pos
    }
    public addToGrid(pos: [number, number], grid: EndlessGrid<Cell>) {
        const shapeHeight = this.shape.length
        this.shape.forEach((row, yI) => {
            row.forEach((cell, xI) => {
                if (cell) {
                    grid.set(pos[0] + xI, pos[1] + (shapeHeight - (yI + 1)), BLOCK)
                }
            })
        })
    }
}
class Minus extends Block {
    constructor() {
        super()
        this.shape = [
            [ true, true, true, true]
        ]
    }
}
class Plus extends Block {
    constructor() {
        super()
        this.shape = [
            [ false, true, false ],
            [ true, true, true ],
            [ false, true, false ],
        ]
    }
}
class Corner extends Block {
    constructor() {
        super()
        this.shape = [
            [ false, false, true ],
            [ false, false, true ],
            [ true, true, true ],
        ]
    }
}
class Pipe extends Block {
    constructor() {
        super()
        this.shape = [
            [ true ],
            [ true ],
            [ true ],
            [ true ]
        ]
    }
}
class Square extends Block {
    constructor() {
        super()
        this.shape = [
            [ true, true ],
            [ true, true ]
        ]
    }
}

const SHAPES: Block[] = [ new Minus(), new Plus(), new Corner(), new Pipe(), new Square() ]

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private grid: EndlessGrid<Cell> = new EndlessGrid()
    private width = 7
    private jets: string[] = []
    private jetIndex = 0
    private cycles: Map<number, {
        blockId: number,
        height: number
    }> = new Map()

    public run(): Result {
        const result: Result = {}
        for (let x = 0; x < this.width; x++) {
            this.grid.set(x, 0, WALL)
        }
        this.createEdges()
        this.jets = this.getInputAsRows('')
     
        const targetA = 2022

        for (let i = 0; i < targetA; i++) {
            this.dropBlock(i)
        }

        result.a = this.findBottom()
    
        const targetB = 1000000000000

        for (let blockId = targetA; blockId < 50000; blockId++) {
            const cycleId = this.getCycleId(blockId)
            if (!this.cycles.has(cycleId)) {
                this.dropBlock(blockId)
                continue
            }
            const previous = this.cycles.get(cycleId)!
            const cycleSize = blockId - previous.blockId
            if (blockId % cycleSize !== (targetB) % cycleSize) {
                this.dropBlock(blockId)
                continue
            }
            const currentBottom = this.findBottom()
            const cycleHeight = currentBottom - previous.height
            const cycleCount = (targetB - blockId) / cycleSize
            result.b = currentBottom + (cycleCount * cycleHeight)
            break
        }

        return result
    }

    private getCycleId(blockId: number) {
        return (this.jetIndex << SHAPES.length) + blockId % SHAPES.length
    }

    private dropBlock(blockId: number) {
        const cycleId = this.getCycleId(blockId)
        const bottom = this.findBottom()
        this.cycles.set(cycleId, {
            blockId,
            height: bottom
        })

        const block = SHAPES[blockId % SHAPES.length]
        let position: [number, number] = [2, bottom + 4]
        while (true) {
            position = block.applyJet(this.jets[this.jetIndex], position, this.grid)
            this.jetIndex = (this.jetIndex + 1) % this.jets.length
            if (!block.canGoDown(position, this.grid)) {
                break
            }
            position = [position[0], position[1] - 1]
        }
        block.addToGrid(position, this.grid)
        this.createEdges()
    }

    private createEdges() {
        const bottom = this.findBottom()
        for (let y = 0; y < 5; y++) {
            this.grid.set(-1, bottom + y, WALL)
            this.grid.set(this.width, bottom + y, WALL)
        }
    }

    private findBottom(): number {
        const range = this.grid.getYRange()
        for (let y = range[1]; y > range[0]; y--) {
            if (this.grid.getRow(y).find(c => c === BLOCK)) {
                return y
            }
        }
        return 0
    }
}

Runner(PuzzleSolution)