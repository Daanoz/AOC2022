import {PuzzleSolution} from './index'

describe('Puzzle TEMPLATE', () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`)
    })
    describe('part A', () => {
        test('should find the highest elf', () => {
            expect(solution.run().a).toBe(24000)
        })
    })
    describe('part B', () => {
        test('should find the highest three elfs', () => {
            expect(solution.run().b).toBe(45000)
        })
    })
})