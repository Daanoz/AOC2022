import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`)
    })
    describe('part A', () => {
        test('Get the priority of duplicate items', () => {
            expect(solution.run().a).toBe(157)
        })
    })
    describe('part B', () => {
        test('Get the priority of the badges', () => {
            expect(solution.run().b).toBe(70)
        })
    })
})