import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

const OPEN = '.'
const WALL = '#'
const VOID = ' '

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3
}

type Coord = [number, number]

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}
        const grid = this.getInputAsGrid({
            content: () => this.getInputAsTable({ 
                splitByCol: '',
                skipTrimInput: true 
            }).slice(0, -2),
            reversed: false
        })
        const cursor = {
            pos: [
                grid.getRow(0).findIndex((value) => value === OPEN)!,
                0
            ] as Coord,
            dir: Direction.Right
        }
        const moves = Array.from(this.getInputAsRows().pop()!.matchAll(/(\d+|[LR])/g)).map(match => match[0])

        result.a = this.makeFlatMoves({ ...cursor }, grid, moves)
        result.b = this.makeCubeMoves({ ...cursor }, grid, moves)

        return result
    }

    private makeFlatMoves(cursor: { pos: Coord, dir: Direction }, grid: EndlessGrid<string>, moves: string[]): number {
        const path: Coord[] = []
        moves.forEach(move => {
            if (move.match(/^\d+$/)) {
                for (let s = 0; s < parseInt(move); s++) {
                    let nextPos: Coord = [
                        cursor.dir % 2 !== 0 ? cursor.pos[0] : cursor.pos[0] + (cursor.dir === Direction.Left ? -1 : 1 ),
                        cursor.dir % 2 === 0 ? cursor.pos[1] : cursor.pos[1] + (cursor.dir === Direction.Up ? -1 : 1 )
                    ]
                    let nextCell = grid.getByIndex(nextPos, VOID)
                    if (nextCell === OPEN) {
                        cursor.pos = nextPos
                    } else if (nextCell === VOID) {
                        const column = grid.getColumn(nextPos[0], true)
                        const row = grid.getRow(nextPos[1])
                        if (cursor.dir === Direction.Left)       { nextPos = [row.length - (row.reverse().findIndex(c => c !== VOID) + 1), nextPos[1]] }
                        else if (cursor.dir === Direction.Right) { nextPos = [row.findIndex(c => c !== VOID), nextPos[1]] }
                        else if (cursor.dir === Direction.Up)    { nextPos = [nextPos[0], column.length - (column.reverse().findIndex(c => c !== VOID) + 1) ] }
                        else if (cursor.dir === Direction.Down)  { nextPos = [nextPos[0], column.findIndex(c => c !== VOID)] }
                        nextCell = grid.getByIndex(nextPos, VOID)
                        if (nextCell === OPEN) {
                            cursor.pos = nextPos
                        } else if (nextCell === VOID) {
                            throw new Error('Fell into void @ ' + nextPos + ' after ' + cursor.dir)
                        } else {
                            break // we hit a wall
                        }
                    } else {  
                        break // we hit a wall
                    }
                    path.push(cursor.pos)
                }
            } else {
                if (move === 'L') {
                    cursor.dir = ((cursor.dir + 4) - 1) % 4
                } else {
                    cursor.dir = ((cursor.dir + 4) + 1) % 4
                }
            }
        })
        return ((cursor.pos[1] + 1) * 1000) + ((cursor.pos[0] + 1) * 4) + cursor.dir
    }

    private makeCubeMoves(cursor: { pos: Coord, dir: Direction }, grid: EndlessGrid<string>, moves: string[]): number {
        const path: Coord[] = []
        if (grid.getHeight() < 50) { return 0 } // No support for example
        const tileSize = 50
        moves.forEach(move => {
            if (move.match(/^\d+$/)) {
                for (let s = 0; s < parseInt(move); s++) {
                    let nextPos: Coord = [
                        cursor.dir % 2 !== 0 ? cursor.pos[0] : cursor.pos[0] + (cursor.dir === Direction.Left ? -1 : 1 ),
                        cursor.dir % 2 === 0 ? cursor.pos[1] : cursor.pos[1] + (cursor.dir === Direction.Up ? -1 : 1 )
                    ]
                    let nextDir = cursor.dir
                    let nextCell = grid.getByIndex(nextPos, VOID)
                    if (nextCell === OPEN) {
                        cursor.pos = nextPos
                    } else if (nextCell === VOID) {


                        //     13
                        //     2
                        //    46
                        //    5
                        // There should be a better way...

                        if (nextDir === Direction.Left) { 
                            if (cursor.pos[1] < tileSize) {
                                nextPos = [0, (tileSize * 3 - cursor.pos[1]) - 1]
                                nextDir = Direction.Right
                            } else if (cursor.pos[1] < tileSize * 2) {
                                nextPos = [cursor.pos[1] - tileSize, tileSize * 2]
                                nextDir = Direction.Down
                            } else if (cursor.pos[1] < tileSize * 3) {
                                nextPos = [tileSize, (tileSize - (cursor.pos[1] - (tileSize * 2))) - 1]
                                nextDir = Direction.Right
                            } else if (cursor.pos[1] < tileSize * 4) {
                                nextPos = [tileSize + (cursor.pos[1] - (tileSize * 3)), 0]
                                nextDir = Direction.Down
                            }
                        } else if (cursor.dir === Direction.Right) { 
                            if (cursor.pos[1] < tileSize) {
                                nextPos = [(tileSize * 2) - 1, ((tileSize * 3) - cursor.pos[1]) - 1]
                                nextDir = Direction.Left
                            } else if (cursor.pos[1] < tileSize * 2) {
                                nextPos = [(tileSize * 2) + (cursor.pos[1] - tileSize), tileSize - 1]
                                nextDir = Direction.Up
                            } else if (cursor.pos[1] < tileSize * 3) {
                                nextPos = [(tileSize * 3) - 1, (tileSize - (cursor.pos[1] - (tileSize * 2))) - 1]
                                nextDir = Direction.Left
                            } else if (cursor.pos[1] < tileSize * 4) {
                                nextPos = [tileSize + (cursor.pos[1] - (tileSize * 3)), (tileSize * 3) - 1]
                                nextDir = Direction.Up
                            }                        
                        } else if (cursor.dir === Direction.Up) { 
                            if (cursor.pos[0] < tileSize) {
                                nextPos = [tileSize, tileSize + cursor.pos[0]]
                                nextDir = Direction.Right
                            } else if (cursor.pos[0] < tileSize * 2) {
                                nextPos = [0, (tileSize * 3) + (cursor.pos[0] - tileSize)]
                                nextDir = Direction.Right
                            } else if (cursor.pos[0] < tileSize * 3) {
                                nextPos = [cursor.pos[0] - (tileSize * 2), (tileSize * 4) - 1]
                                nextDir = Direction.Up
                            }                         
                        } else if (cursor.dir === Direction.Down) {                   
                            if (cursor.pos[0] < tileSize) {
                                nextPos = [(tileSize * 2) + (cursor.pos[0]), 0]
                                nextDir = Direction.Down
                            } else if (cursor.pos[0] < tileSize * 2) {
                                nextPos = [(tileSize) - 1, (tileSize * 3) + (cursor.pos[0] - tileSize)]
                                nextDir = Direction.Left
                            } else if (cursor.pos[0] < tileSize * 3) {
                                nextPos = [(tileSize * 2) - 1, (tileSize + (cursor.pos[0] - (tileSize * 2)))]
                                nextDir = Direction.Left
                            }     
                        }
                        nextCell = grid.getByIndex(nextPos, VOID)
                        if (nextCell === OPEN) {
                            cursor.pos = nextPos
                            cursor.dir = nextDir
                        } else if (nextCell === VOID) {
                            throw new Error('Fell into void @ ' + nextPos)
                        } else {
                            break // we hit a wall
                        }
                    } else {  
                        break // we hit a wall
                    }
                    path.push(cursor.pos)
                }
            } else {
                if (move === 'L') {
                    cursor.dir = ((cursor.dir + 4) - 1) % 4
                } else {
                    cursor.dir = ((cursor.dir + 4) + 1) % 4
                }
            }
        })
        return ((cursor.pos[1] + 1) * 1000) + ((cursor.pos[0] + 1) * 4) + cursor.dir
    }
}

Runner(PuzzleSolution)