import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.`.trim())
    })
    test('Finds best blueprints', () => {
        const result = solution.run()
        expect(result.a).toBe(33)
        expect(result.b).toBe(56 * 62)
    })
})