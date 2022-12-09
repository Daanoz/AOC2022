import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

class Dir {
    public name = ''
    public parent: Dir
    public files: File[] = []
    public dirs: Dir[] = []

    constructor(name: string, parent?: Dir) {
        this.name = name
        this.parent = parent || this
    }

    public getDirSize(): number {
        return this.files.reduce((sum, f) => sum + f.size, 0) +
            this.dirs.reduce((sum, d) => sum + d.getDirSize(), 0)
    }
}
class File {
    public name = ''
    public size = 0

    constructor(name: string, size: number) {
        this.name = name
        this.size = size
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private root: Dir = new Dir('')
    private pwd: Dir = this.root
    private terminalOutput: string[] = []

    public run(): Result {
        const result: Result = {}

        this.terminalOutput = this.getInputAsRows()
        while (this.terminalOutput.length > 0) {
            this.executeCommand()
        }
        
        result.a = this.sumDirsWithSizeOfAtMost(100000, this.root)
        const spaceFree = (70000000 - this.root.getDirSize())
        const spaceRequired = 30000000 - spaceFree
        result.b = this.findDirWithClosestSizeTo(spaceRequired, this.root)

        return result
    }

    private sumDirsWithSizeOfAtMost(size: number, dir: Dir): number {
        let sum = 0
        const dirSize = dir.getDirSize()
        if (dirSize <= size) {
            sum += dirSize
        }
        sum += dir.dirs.reduce((subSum, d) => subSum + this.sumDirsWithSizeOfAtMost(size, d), 0)
        return sum
    }

    private findDirWithClosestSizeTo(size: number, dir: Dir): number {
        const dirSize = dir.getDirSize()
        if (dirSize < size) {
           return Number.POSITIVE_INFINITY
        }
        return Math.min(
            dirSize,
            ...dir.dirs.map(d => this.findDirWithClosestSizeTo(size, d))
        )
    }

    private executeCommand() {
        const terminalLine = this.terminalOutput.shift()!
        const commandStr = terminalLine.substring(2)
        const commandParts = commandStr.split(' ')
        const command = commandParts[0]
        switch (command) {
            case 'cd': this.changeDirectory(commandParts[1]); break
            case 'ls': this.readDirectory(); break
        }
    }

    private changeDirectory(dirname: string) {
        switch (dirname) {
            case '/': this.pwd = this.root; break
            case '..': this.pwd = this.pwd.parent; break
            default: this.pwd = this.pwd.dirs.find(dir => dir.name === dirname)!; break
        }
    }

    private readDirectory() {
        while(this.terminalOutput.length > 0 && this.terminalOutput[0][0] !== '$') {
            const terminalLine = this.terminalOutput.shift()!
            const [prop, name] = terminalLine.split(' ')
            if (prop === 'dir') {
                this.pwd.dirs.push(new Dir(
                    name,
                    this.pwd
                ))
            } else {
                this.pwd.files.push(new File(
                    name,
                    parseInt(prop)
                ))
            }
        }
    }
}

Runner(PuzzleSolution)