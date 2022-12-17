import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput('>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>')
    })
    test('should do tetris', () => {
        const result = solution.run()
        expect(result.a).toBe(3068)
        expect(result.b).toBe(1514285714288)
    })
})