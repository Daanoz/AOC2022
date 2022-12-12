import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi`.trim())
    })
    test('example', () => {
        const result = solution.run()
        expect(result.a).toBe(31)
        expect(result.b).toBe(29)
    })
    test('complexer scenario', () => {
        solution.setInput(`
Saaabqponm
abbbcryxxl
accccszExk
acccctuvwj
abdddefghi`.trim())
        const result = solution.run()
        expect(result.a).toBe(33)
        expect(result.b).toBe(30)
})
})