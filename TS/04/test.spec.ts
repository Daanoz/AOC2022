import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`)
    })
    describe('part A', () => {
        test('should find 2 pairs', () => {
            expect(solution.run().a).toBe(2)
        })
    })
    describe('part B', () => {
        test('should find 4 pairs', () => {
            expect(solution.run().b).toBe(4)
        })
    })
})