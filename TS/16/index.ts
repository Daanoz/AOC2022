import path from 'path'
import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

class Node {
    public name = ''
    public neighbors: Node[] = []
    public flowRate = 0
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private nodeMap: Map<string, Node> = new Map()
    private startNode: Node = new Node()
    private closedNodes: Node[] = []
    private openNodes: Node[] = []

    private memo: Map<number, Map<string, Map<string, number>>> = new Map()
    private travelDistances: Map<Node, Map<Node, number>> = new Map()
    private paths: {
        road: Node[],
        pressure: number
    }[] = []

    public run(): Result {
        const result: Result = {}

        this.getInputAsRows().map(r => this.mapToNode(r))
        this.startNode = this.nodeMap.get('AA')!
        this.closedNodes = Array.from(this.nodeMap.values()).filter(node => node.flowRate > 0)
        this.openNodes = Array.from(this.nodeMap.values()).filter(node => node.flowRate === 0)
        this.timed('calculateTravelDistances', () => this.calculateTravelDistances())

        const useDFS = true // DFS is slightly quicker (~15%)
        if (useDFS) {
            result.a = this.timed('depthFirstSearch', () => 
                this.depthFirstSearch(this.startNode, this.openNodes, 0)
            )
        } else {
            result.a = this.timed('calculateAllPossiblePaths(30)', () => {
                this.calculateAllPossiblePaths(30)
                return this.paths.reduce((max, p) => Math.max(max, p.pressure), 0)
            })
        }

        this.paths = []
        this.timed('calculateAllPossiblePaths(26)', () => this.calculateAllPossiblePaths(26))
        result.b = this.timed('findHighestScoreWith2', () => this.findHighestScoreWith2())
        return result
    }

    private calculateTravelDistances(): void {
        const startingNodes = [this.startNode, ...this.closedNodes]
        startingNodes.forEach((node) => {
            const distanceMap = new Map([[ node, 0 ]])
            this.travelDistances.set(node, distanceMap)
            const visited: Node[] = []
            const queue: Node[] = [node]
            while (queue.length > 0) {
                const current = queue.shift()!
                if (visited.includes(current)) {
                    continue
                }
                visited.push(current)
		        const currentDistance = distanceMap.get(current)!

                const validTargets = current.neighbors.filter(n => !visited.includes(n))
                validTargets.forEach(t => 
                    distanceMap.set(t, Math.min(currentDistance + 1, distanceMap.get(t) ?? Number.POSITIVE_INFINITY))
                )
                queue.push(...validTargets)
            }
            distanceMap.delete(node)
        })
    }

    private findHighestScoreWith2(): number {
        this.paths.sort((a, b) => b.pressure - a.pressure)
        const singleMax = this.paths[0].pressure
        let max = Number.NEGATIVE_INFINITY
        this.paths.forEach((pathA, index) => {
            if (pathA.pressure + singleMax < max) {
                return // this trick safes more than 9 seconds, or 90% of this function
            }
            const firstPathNotReachingMax = this.paths.findIndex(p => pathA.pressure + p.pressure < max)
            this.paths.slice(index + 1, firstPathNotReachingMax).forEach((pathB) => {
                const combinedPressure = pathA.pressure + pathB.pressure
                if (combinedPressure > max) {
                    if (pathA.road.every(s => !pathB.road.includes(s))) {
                        max = combinedPressure
                    }
                }
            })
        })
        return max
    }

    private calculateAllPossiblePaths(timeLimit: number) {
        const queue = [{
            node: this.startNode,
            next: this.closedNodes,
            time: timeLimit,
            road: [] as Node[],
            pressure: 0
        }]
        while(queue.length > 0) {
            const current = queue.shift()!
            const distanceMap = this.travelDistances.get(current.node)!
            const nextVisits = current.next.filter(next => current.time - distanceMap.get(next)! > 1)
            if (nextVisits.length < 1) {
                this.paths.push({ road: current.road, pressure: current.pressure })
                continue
            }
            nextVisits.forEach(next => {
                const timeLeft = current.time - distanceMap.get(next)! - 1
                const nextPressure = current.pressure + timeLeft * next.flowRate
                queue.push({
                    node: next,
                    next: current.next.filter(v => v !== next),
                    time: timeLeft,
                    road: [...current.road, next],
                    pressure: nextPressure
                })
                this.paths.push({
                    road: [...current.road, next],
                    pressure: nextPressure
                })
            })
        }
    }

    private writeToMemo(minutes: number, node: string, openNodesId: string, pressure: number) {
        if (!this.memo.has(minutes)) {
            this.memo.set(minutes, new Map())
        }
        if (!this.memo.get(minutes)?.has(node)) {
            this.memo.get(minutes)?.set(node, new Map())
        }
        this.memo.get(minutes)?.get(node)?.set(openNodesId, pressure)
    }

    private depthFirstSearch(nodeA: Node, openNodes: Node[], minutes: number): number {
        const timeLimit = 30
        minutes += 1
        if (minutes >= timeLimit || (openNodes.length === this.nodeMap.size)) {
            return 0
        }
        const openNodesId = openNodes.map(on => on.name).sort().join('|')
        const currentNodeId = [nodeA].map(on => on.name).sort().join('|')
        const cachedMins = minutes
        const cached = this.memo.get(cachedMins)?.get(currentNodeId)?.get(openNodesId)
        if (cached !== undefined) {
            return cached
        }

        const move = (n: Node, openNodes: Node[]) => ({
            addedPressure: 0,
            openNodes,
            opts: n.neighbors
        })
        const open = (n: Node, openNodes: Node[]) => ({
            addedPressure: (n.flowRate * (timeLimit - minutes)),
            openNodes: [...openNodes, n],
            opts: [n]
        })

        const options = [ move ]
        if (!openNodes.includes(nodeA)) {
            options.push( open )
        }
        const max = Math.max(...options.map(option => {
            const actionA = option(nodeA, openNodes)
            return actionA.addedPressure +
                Math.max(
                    ...actionA.opts.map(moveFromA => 
                        this.depthFirstSearch(moveFromA, actionA.openNodes, minutes)
                    )
                )
        }))
        this.writeToMemo(cachedMins, currentNodeId, openNodesId, max)
        return max                 
    }  

    private mapToNode(row: string): Node {
        const match = row.match(/Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)/)!
        const name = match[1]
        const flowRate = match[2]
        const node = this.getNode(name)
        const neighbors = match[3].split(', ').map(n => this.getNode(n))
        node.flowRate = parseInt(flowRate)
        node.neighbors.push(...neighbors)
        return node
    }

    private getNode(name: string): Node {
        let node = this.nodeMap.get(name)
        if (!node) {
            node = new Node()
            node.name = name
            this.nodeMap.set(name, node)
        }
        return node
    }
}

Runner(PuzzleSolution)