import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
1
2
-3
3
-2
0
4`.trim())
    })
    test('Decipher the coordinates', () => {
        const result = solution.run()
        expect(result.a).toBe(3)
        expect(result.b).toBe(1623178306)
    })
})