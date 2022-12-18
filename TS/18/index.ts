import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

type Coord = [number, number, number]

function removeDuplicates(list: Coord[]): Coord[] {
    return list.filter((itemA, index) =>
        index ===
        list.findIndex((itemB) => itemA[0] === itemB[0] && itemA[1] === itemB[1] && itemA[2] === itemB[2])   
    )
} 

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private coords: Coord[] = []
    private map: Map<number, Map<number, number[]>> = new Map()
    private xRange: [number, number] = [0, 0]
    private closedCoords: string[] = []
    private openCoords: string[] = []

    public run(): Result {
        const result: Result = {}

        this.coords = this.getInputAsRows().map(row => row.split(',').map(Number) as Coord)
        this.coords.forEach(c => {
            if (!this.map.has(c[0])) { this.map.set(c[0], new Map()) }
            if (!this.map.get(c[0])!.has(c[1])) { this.map.get(c[0])!.set(c[1], []) }
            this.map.get(c[0])!.get(c[1])!.push(c[2])
        })
        this.xRange = [Math.min(...this.coords.map(c => c[0])), Math.max(...this.coords.map(c => c[0]))]

        const sides: Coord[] = []
        this.coords.forEach(c => {
            const neighbors = this.findNeighborsOf(c)
            sides.push(...neighbors.filter(n => !this.containsBlock(n)))
        })
        result.a = sides.length

        let sidesToTest = removeDuplicates(sides)
        sidesToTest = sidesToTest.filter(side => !this.isOutside(side))
        const closedSides = sidesToTest.filter(stt => !this.isOpenToOutside(stt))
        const closedSidesHashed = closedSides.map(cs => cs.join(','))
        result.b = sides.filter(s => !closedSidesHashed.includes(s.join(','))).length

        return result
    }

    private findNeighborsOf(c: Coord): Coord[] {
        const neighbors: Coord[] = [-1, 1].map(offset => [c[0] + offset, c[1], c[2]])
        neighbors.push(...[-1, 1].map(offset => [c[0], c[1] + offset, c[2]]) as Coord[])
        neighbors.push(...[-1, 1].map(offset => [c[0], c[1], c[2] + offset]) as Coord[])
        return neighbors
    }

    private containsBlock(c: Coord): boolean {
        return this.map.get(c[0])?.get(c[1])?.includes(c[2]) ?? false
    }

    private isOutside(c: Coord): boolean {
        if (c[0] < this.xRange[0]) return true
        if (c[0] > this.xRange[1]) return true
        if (c[1] < Math.min(...this.map.get(c[0])?.keys() ?? [Number.NEGATIVE_INFINITY])) return true
        if (c[1] > Math.max(...this.map.get(c[0])?.keys() ?? [Number.POSITIVE_INFINITY])) return true
        if (c[2] < Math.min(...this.map.get(c[0])?.get(c[1]) ?? [Number.NEGATIVE_INFINITY])) return true
        if (c[2] > Math.max(...this.map.get(c[0])?.get(c[1]) ?? [Number.POSITIVE_INFINITY])) return true
        return false
    }

    private isOpenToOutside(coord: Coord): boolean {
        if (this.closedCoords.includes(coord.join(','))) {
            return false
        }
        const visited: string[] = [coord.join(',')]
        const queue = this.findNeighborsOf(coord).filter(c => !this.containsBlock(c))
        while (queue.length > 0) {
            const current = queue.shift()!
            const hash = current.join(',')
            if (visited.includes(hash)) {
                continue
            }
            if (this.openCoords.includes(hash)) {
                this.openCoords.push(...visited)
                return true
            }
            visited.push(hash)
            if (this.isOutside(current)) {
                this.openCoords.push(...visited)
                return true
            }
            queue.push(
                ...this.findNeighborsOf(current)
                    .filter(n => !visited.includes(n.join(',')))
                    .filter(n => !this.containsBlock(n))
            )
        }
        this.closedCoords.push(...visited)
        return false
    }
}

Runner(PuzzleSolution)