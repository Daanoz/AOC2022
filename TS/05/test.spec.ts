import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`)
    })
    describe('part A', () => {
        test('rearrange the stack', () => {
            expect(solution.run().a).toBe('CMZ')
        })
    })
    describe('part B', () => {
        test('rearrange the stack with multiple at the same time', () => {
            expect(solution.run().b).toBe('MCD')
        })
    })
})