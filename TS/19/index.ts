import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

type BuildCosts = [number, number, number, number]
type OreStash = [number, number, number, number]
type BotCount = [number, number, number, number]
type QueueItem = {
    bots: BotCount,
    ores: OreStash,
    time: number
}

class BluePrint {
    public id: number
    public botCosts: [BuildCosts, BuildCosts, BuildCosts, BuildCosts]
    public maxCost: BuildCosts = [0, 0, 0, 0]

    constructor(id: number, oreCost: BuildCosts, clayCost: BuildCosts, obsidianCost: BuildCosts, geodeCost: BuildCosts) {
        this.id = id
        this.botCosts = [oreCost, clayCost, obsidianCost, geodeCost]
        this.maxCost = this.maxCost.map((_, costIndex) => Math.max(...this.botCosts.map(c => c[costIndex]))) as BuildCosts
    }

    public static parse(line: string): BluePrint {
        const matches = line.match(/Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian./)!
        return new BluePrint(
            parseInt(matches[1]),
            [ parseInt(matches[2]), 0, 0, 0],
            [ parseInt(matches[3]), 0, 0, 0],
            [ parseInt(matches[4]), parseInt(matches[5]), 0, 0],
            [ parseInt(matches[6]), 0, parseInt(matches[7]), 0],
        )
    }
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    private maxGeodeIncrement: Record<number, number> = { 0: 0 }
    public run(): Result {
        const result: Result = {}
        const blueprints = this.getInputAsRows().map(BluePrint.parse)
        for (let m = 1; m <= 32; m++) {
            this.maxGeodeIncrement[m] = this.maxGeodeIncrement[m - 1] + m
        }
        result.a = blueprints.reduce((sum, bp) => {
            const output = this.findBestOutcomeForBlueprint(bp)
            return sum + (output * bp.id)
        }, 0)

        result.b = blueprints.slice(0, 3).reduce((sum, bp) => {
            const output = this.findBestOutcomeForBlueprint(bp, 32)
            return sum * output
        }, 1)

        return result
    }

    private findBestOutcomeForBlueprint(bp: BluePrint, maxTime = 24) {
        const queue = [{
            ores: [0, 0, 0, 0] as OreStash,
            bots: [1, 0, 0, 0] as BotCount,
            time: 0
        }]
        const visits: Map<number, Map<number, boolean>>[] = new Array(maxTime + 1).fill(undefined).map(() => new Map())   
        let maxGeodeCount = 0
        while (queue.length > 0) {
            const current = queue.pop()!
            if (current.time >= maxTime) {
                maxGeodeCount = Math.max(maxGeodeCount, current.ores[3])
                continue
            }
            const timeLeft = maxTime - current.time
            const bestCaseMax = this.maxGeodeIncrement[timeLeft] + current.ores[3] + (current.bots[3] * timeLeft)
            if (maxGeodeCount > bestCaseMax) {
                continue
            }

            const botState = 0 +
                (current.bots[0] << (5 * 0)) + 
                (current.bots[1] << (5 * 1)) + 
                (current.bots[2] << (5 * 2)) + 
                (current.bots[3] << (5 * 3))
            const oreState = 0 +
                (current.ores[0] << (7 * 0)) + 
                (current.ores[1] << (7 * 1)) + 
                (current.ores[2] << (7 * 2)) + 
                (current.ores[3] << (7 * 3))
            const botCache = visits[current.time].get(botState)
            if (!botCache) {
                visits[current.time].set(botState, new Map([[oreState, true]]))
            } else {
                if (botCache?.has(oreState)) { continue }
                visits[current.time].get(botState)!.set(oreState, true)
            }
       

            this.findBotPurchaseOptions(current.ores, current.bots, current.time, bp).forEach((option) => {
                queue.push(option)
            })
        }
        return maxGeodeCount
    }

    private findBotPurchaseOptions(ores: OreStash, bots: BotCount, time: number, bp: BluePrint): QueueItem[] {
        const options: QueueItem[] = []
        for (let bot = 3; bot >= 0; bot--) {
            if (bot !== 3 && (bp.maxCost[bot] <= bots[bot])) { // we don't need more of these!
                continue
            }
            const cost = bp.botCosts[bot]            
            if (ores.every((o, i) => o >= cost[i])) {
                const newBots = [...bots] as BotCount
                newBots[bot]++
                options.push({
                    bots: newBots,
                    time: time + 1,
                    ores: ores.map((o, i) => (o + bots[i]) - cost[i]) as OreStash
                })
                if (bot === 3) { // we can buy a geode bot, ignore all other options.
                    return options
                }
            }
        }
        if (ores[0] <= bp.maxCost[0]) {
            options.unshift({
                bots: bots,
                time: time + 1,
                ores: ores.map((o, i) => o + bots[i]) as OreStash
            })
        }
        return options
    }
}

Runner(PuzzleSolution)