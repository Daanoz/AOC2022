import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
30373
25512
65332
33549
35390`.trim())
    })
    describe('part A', () => {
        test('to count visible trees', () => {
            expect(solution.run().a).toBe(21)
        })
    })
    describe('part B', () => {
        test('to find the best scene score', () => {
            expect(solution.run().b).toBe(8)
        })
    })
})