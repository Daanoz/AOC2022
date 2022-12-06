import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
    })
    describe('part A', () => {
        test.each([
            { input: 'bvwbjplbgvbhsrlpgdmjqwftvncz', output: 5 },
            { input: 'nppdvjthqldpwncqszvftbrmjlhg', output: 6 },
            { input: 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', output: 10 },
            { input: 'zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', output: 11 },
        ])('should find the first package header', ({ input, output }) => {
            solution = new PuzzleSolution()
            solution.setInput(input)
            expect(solution.run().a).toBe(output)
        })
    })
    describe('part B', () => {
        test.each([
            { input: 'mjqjpqmgbljsphdztnvjfqwrcgsmlb', output: 19 },
            { input: 'bvwbjplbgvbhsrlpgdmjqwftvncz', output: 23 },
            { input: 'nppdvjthqldpwncqszvftbrmjlhg', output: 23 },
            { input: 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', output: 29 },
            { input: 'zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', output: 26 },
        ])('should find the first message header', ({ input, output }) => {
            solution = new PuzzleSolution()
            solution.setInput(input)
            expect(solution.run().b).toBe(output)
        })
    })
})