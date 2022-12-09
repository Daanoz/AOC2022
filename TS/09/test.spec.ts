import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    describe('part A', () => {
        beforeEach(() => {
            solution = new PuzzleSolution()
            solution.setInput(`
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`.trim())
        })
        test('should get the positions for a length of 2', () => {
            expect(solution.run().a).toBe(13)
        })
    })
    describe('part B', () => {
        beforeEach(() => {
            solution = new PuzzleSolution()
            solution.setInput(`
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`.trim())
        })
        test('should get the positions for a length of 10', () => {
            expect(solution.run().b).toBe(36)
        })
    })
})