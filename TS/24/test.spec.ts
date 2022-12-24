import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#`.trim())
    })
    test('Should dodge blizzards', () => {
        const result = solution.run()
        expect(result.a).toBe(18)
        expect(result.b).toBe(54)
    })
})