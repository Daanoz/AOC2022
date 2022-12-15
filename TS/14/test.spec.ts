import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9`.trim())
    })
    describe('part A', () => {
        test('something', () => {
            const result = solution.run()
            expect(result.a).toBe(24)
            expect(result.b).toBe(93)
        })
    })
    describe('part B', () => {
    })
})