import {PuzzleSolution} from './index'

describe('Puzzle 02', () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput('A Y\nB X\nC Z\n')
    })
    describe('part A', () => {
        test('to score 15', () => {
            expect(solution.run().a).toBe(15)
        })
    })
    describe('part B', () => {
        test('to score 12', () => {
            expect(solution.run().b).toBe(12)
        })
    })
})