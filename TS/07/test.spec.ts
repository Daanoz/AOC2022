import {PuzzleSolution} from './index'
import path from 'path'

describe(`Puzzle ${__dirname.split(path.sep).pop()}`, () => {
    let solution: PuzzleSolution
    beforeEach(() => {
        solution = new PuzzleSolution()
        solution.setInput(`
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`.trim())
    })
    describe('part A', () => {
        test('Dirs of at most 100000 to be summed', () => {
            expect(solution.run().a).toBe(95437)
        })
    })
    describe('part B', () => {
        test('Smallest deletable dir', () => {
            expect(solution.run().b).toBe(24933642)
        })
    })
})