import { Puzzle, Runner, BasePuzzle, Result } from '../shared'

const tuningFrequencyMultiplier = 4000000

type SensorBeaconCombo = {
    sensor: [number, number],
    beacon: [number, number],
    distance: number
}

export class PuzzleSolution extends BasePuzzle implements Puzzle {
    public run(yRow = 2000000, scanRange = tuningFrequencyMultiplier): Result {
        const result: Result = {}

        const coords = this.getInputAsRows().map(row => this.mapToCoords(row))

        result.a = this.findEmptySpotsOnRow(coords, yRow)
        result.b = this.findTuningFrequency(coords, scanRange)

        return result
    }

    private findTuningFrequency(coords: SensorBeaconCombo[], limit: number): number {
        for (let i = 0; i < coords.length; i++) {
            const coord = coords[i]
            const delta = (coord.distance + 1)
            for (let d = -delta; d <= +delta; d++) {
                const y = coord.sensor[1] + d
                const dx = delta - Math.abs(d)
                const left: [number, number] = [coord.sensor[0] - dx, y]
                const right: [number, number] = [coord.sensor[0] + dx, y]
                if (this.isInLimit(left, limit)) {
                    if (!this.isInRange(coords, left[0], left[1])) {
                        return (left[0] * tuningFrequencyMultiplier) + y
                    }
                }
                if (this.isInLimit(right, limit) && left[0] !== right[0]) {
                    if (!this.isInRange(coords, right[0], right[1])) {
                        return (right[0] * tuningFrequencyMultiplier) + y
                    }
                }
            }
        }
        return 0
    }

    private isInLimit(coord: [number, number], limit: number): boolean {
        return (coord[0] >= 0 && coord[0] <= limit && coord[1] >= 0 && coord[1] <= limit)
    }

    private isInRange(coords: SensorBeaconCombo[], x: number, y: number): boolean {
        return !!coords.find(coord => {
            const distanceToSensor = Math.abs(coord.sensor[0] - x) + Math.abs(coord.sensor[1] - y)
            return distanceToSensor <= coord.distance
        })
    }

    private findEmptySpotsOnRow(coords: SensorBeaconCombo[], yRow: number): number {
        const horizontalLimits = this.findHorizontalLimits(coords)
        const beaconsOnYRow = coords
            .filter((c) => c.beacon[1] === yRow)
            .reduce((positions, c) => positions.includes(c.beacon[0]) ? positions : positions.concat(c.beacon[0]), [] as number[])
            .length

        let empty = 0
        for (let x = horizontalLimits[0]; x <= horizontalLimits[1]; x++) {
            const inRange = this.isInRange(coords, x, yRow)
            if (inRange) {
                empty++
            }
        }
        return empty - beaconsOnYRow
    }

    private mapToCoords(row: string): SensorBeaconCombo {
        const [x1, y1, x2, y2] = row.match(/Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)/)!.slice(1, 5).map(Number)
        const distance = Math.abs(x1 - x2) + Math.abs(y1 - y2)
        return {
            sensor: [x1, y1],
            beacon: [x2, y2], 
            distance
        }
    }

    private findHorizontalLimits(coords: SensorBeaconCombo[]): [number, number] {
        return coords.reduce((limits, c) => {
            return [Math.min(limits[0], c.sensor[0] - c.distance), Math.max(limits[1], c.sensor[0] + c.distance)]
        }, [Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY])
    }
}

Runner(PuzzleSolution)