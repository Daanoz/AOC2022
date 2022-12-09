import { Puzzle, Runner, BasePuzzle, Result, EndlessGrid } from '../shared'

type TreeCell = {
    visible: boolean
    height: number
}

function setVisibleTrees(trees: TreeCell[]): void {
    trees.reduce((currentHeight, tree) => {
        if (tree.height > currentHeight) {
            tree.visible = true
            return tree.height
        } 
        return currentHeight
    }, -1)
}

function setVisibleTreesFromGrid(trees: TreeCell[]): void {
    const highest = Math.max(...trees.map(c => c.height))
    const highestFromLeft = trees.findIndex(c => c.height === highest)
    const treesFromRight = [...trees].reverse()
    const highestFromRight = treesFromRight.findIndex(c => c.height === highest)
    setVisibleTrees(trees.slice(0, highestFromLeft + 1))
    setVisibleTrees(treesFromRight.slice(0, highestFromRight + 1))
}

function getVisibleTreesFromHeight(height: number, trees: number[]): number {
    const firstBlockingTree = trees.findIndex(t => t >= height)
    return firstBlockingTree < 0 ? trees.length : firstBlockingTree + 1
}

function getSceneScore(treeGrid: EndlessGrid<TreeCell>, height: number, position: [number, number]): number {
    const treesToTheLeft = treeGrid.getRow(position[1]).slice(0, position[0]).reverse().map(c => c.height)
    const treesToTheRight = treeGrid.getRow(position[1]).slice(position[0] + 1).map(c => c.height)
    const treesToTheTop = treeGrid.getColumn(position[0]).slice(0, Math.abs(position[1])).reverse().map(c => c.height)
    const treesToTheBottom = treeGrid.getColumn(position[0]).slice(Math.abs(position[1]) + 1).map(c => c.height)
    const score = getVisibleTreesFromHeight(height, treesToTheLeft) *
        getVisibleTreesFromHeight(height, treesToTheRight) *
        getVisibleTreesFromHeight(height, treesToTheTop) *
        getVisibleTreesFromHeight(height, treesToTheBottom)
    return score
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(): Result {
        const result: Result = {}
        let c = 0
        const treeGrid = this.getInputAsGrid<TreeCell>({
            splitByCol: '',
            cellParser: (val) => ({
                id: c++,
                visible: false,
                height: parseInt(val)
            })
        })
        treeGrid.mapRows((row) => setVisibleTreesFromGrid(row as TreeCell[]))
        treeGrid.mapColumns((column) => setVisibleTreesFromGrid(column as TreeCell[]))

        result.a = treeGrid.countBy(c => c.visible)

        result.b = treeGrid.reduce((highestSceneScore, cell, position) => {
            const score = getSceneScore(treeGrid, cell.height, position)
            return score > highestSceneScore ? score : highestSceneScore
        }, 0)

        return result
    }
}

Runner(PuzzleSolution)