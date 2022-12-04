import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

function rangeToStartEnd(range: string): [number, number] {
    return range.split('-').map(val => parseInt(val)) as [number, number]
}

function rangeToSection(range: string): string {
    const [start, end] = rangeToStartEnd(range)
    return `,${new Array((end - start) + 1).fill('0').map((_, index) => `${index + start}`).join(',')},`
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const fullyOverlappingSections = this.getInputAsRows()
            .map(item => item.split(',').map(rangeToSection))
            .filter(item => item[0].includes(item[1]) || item[1].includes(item[0]))

        const overlappingSections = this.getInputAsRows()
            .map(item => item.split(',').map(rangeToStartEnd))
            .filter(item => 
                (item[0][0] <= item[1][0]) && (item[0][1] >= item[1][0]) || // left boundary
                (item[0][0] <= item[1][1]) && (item[0][1] >= item[1][1]) || // right boundary
                (item[0][0] >= item[1][0]) && (item[0][1] <= item[1][1]) || // 0 in 1
                (item[0][0] <= item[1][0]) && (item[0][1] >= item[1][1])    // 1 in 0
            )

        result.a = fullyOverlappingSections.length
        result.b = overlappingSections.length

        return result
    }

}

Runner(PuzzleSolution)