import { Puzzle, Runner, BasePuzzle, Result } from '../shared/'

function uniqueOnly(item: string, index: number, list: string[]) {
    return list.indexOf(item) === index
}

const CHAR_CODE_A = 'A'.charCodeAt(0)
const CHAR_CODE_a = 'a'.charCodeAt(0)
const ALPHABET_SIZE = 26
const ONE_INDEXED = 1
function getItemPriority(item: string) {
    const charValue = item.charCodeAt(0)
    if (charValue >= CHAR_CODE_a) {
        return charValue - CHAR_CODE_a + ONE_INDEXED
    }
    return charValue - CHAR_CODE_A + ALPHABET_SIZE + ONE_INDEXED
}
export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}

        const rows = this.getInputAsRows()
        const bags = rows.map(row => row.split(''))
        const bagsWithCompartments = bags.map(bag => [bag.slice(0, bag.length / 2), bag.slice(bag.length / 2)])

        const duplicates = bagsWithCompartments.reduce((list, bag) => {
            const duplicateItems = bag[0]
                .filter(item => bag[1].indexOf(item) >= 0)
                .filter(uniqueOnly)
            return list.concat(...duplicateItems)
        }, [] as string[])

        result.a = duplicates
            .map(getItemPriority)
            .reduce((sum, item) => sum + item, 0)

        const groups = new Array(rows.length / 3)
            .fill([])
            .map((_, index) => bags.slice(index * 3, (index * 3) + 3))

        const groupBadges = groups
            .map(group => group[0].find(item => group[1].includes(item) && group[2].includes(item))!)

        result.b = groupBadges
            .map(getItemPriority)
            .reduce((sum, item) => sum + item, 0)

        return result
    }
}

Runner(PuzzleSolution)