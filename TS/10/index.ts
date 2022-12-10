import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

const SCREEN_WIDTH = 40

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        let x = 1
        let cycle = 0

        const intervalNumbers: number[] = []
        const screenBuffer: boolean[] = []
        const incrementCycle = () => {
            screenBuffer.push([x - 1, x, x + 1].includes((cycle % 40)))
            cycle++
            if (cycle % SCREEN_WIDTH === (SCREEN_WIDTH / 2)) {
                intervalNumbers.push(cycle * x)
            }
        }

        this.getInputAsRows().forEach(row => {
            const parts = row.split(' ')
            incrementCycle()
            switch (parts[0]) {
                case 'noop': break
                case 'addx': {
                    incrementCycle()
                    x += parseInt(parts[1])
                } break
            }
        })

        result.a = intervalNumbers.reduce((sum, n) => sum + n, 0)
        
        const screenLines = new Array(screenBuffer.length / SCREEN_WIDTH)
            .fill('')
            .map((_, i) => 
                screenBuffer
                    .slice(i * SCREEN_WIDTH, ((i + 1) * SCREEN_WIDTH) - 1)
                    .map(p => p ? '#' : ' ')
                    .join('')
            )
        result.b = screenLines.join('\n')

        return result
    }

}

Runner(PuzzleSolution)