import { PuzzleRenderer } from './puzzle.renderer'

const updateRange = (input: [number, number], val: number) : [number, number] => {
    return [Math.min(input[0], val), Math.max(input[1], val)]
}

export interface GridCell {
    toString(): string
}

export class EndlessGrid<T extends string | GridCell> {
    private grid: Map<number, Map<number, T>> = new Map();
    private xRange: [number, number] = [0, 0];
    private yRange: [number, number] = [0, 0];

    constructor() {}

    public set(x: number, y: number, value: T | undefined): void {
        if (value === undefined) {
            if (this.grid.has(y) && this.grid.get(y)?.has(x)) {
                this.grid.get(y)?.delete(x)
            }
            return
        }
        if (!this.grid.has(y)) { this.grid.set(y, new Map()) }
        this.grid.get(y)?.set(x, value)
        this.xRange = updateRange(this.xRange, x)
        this.yRange = updateRange(this.yRange, y)
    }

    public update(x: number, y: number, updateFn: (value: T) => T, defaultValue?: T): void {
        if (!defaultValue && !this.has(x, y)) {
            throw new Error('Unable to update undefined value')
        }
        this.set(x, y, updateFn(this.get(x, y, defaultValue)!))
    }

    public get(x: number, y: number): T | undefined;
    public get(x: number, y: number, defaultValue: T | undefined): T;
    public get(x: number, y: number, defaultValue: string): string;
    public get(x: number, y: number, defaultValue?: T | string): T | undefined | string {
        if (!this.grid.has(y)) {
            return defaultValue
        }
        const cell = this.grid.get(y)?.get(x)
        return cell === undefined ? defaultValue : cell
    }

    public getNeighborsIndexes(x: number, y: number, opts?: { includeDiagonals?: boolean, onlyDefined?: boolean }): [number, number][] {
        let coordinates: [number, number][] = [ [x, y - 1], [x + 1, y], [x, y + 1], [x - 1, y] ]
        if (opts?.includeDiagonals) {
            coordinates = [ 
                [x, y - 1], [x + 1, y - 1], [x + 1, y], [x + 1, y + 1], 
                [x, y + 1], [x - 1, y + 1], [x - 1, y], [x - 1, y - 1] 
            ]
        }
        if (!opts?.onlyDefined) {
            return coordinates
        } else {
            return coordinates.filter(coord => this.has(coord[0], coord[1]))
        }
    }

    public getNeighbors<U extends (T | undefined)>(x: number, y: number, defaultValue?: U, opts?: { includeDiagonals?: boolean, onlyDefined?: boolean }): (T | U)[] {
        return this.getNeighborsIndexes(x, y, opts)
            .map(pos => this.get(pos[0], pos[1], defaultValue))
    }

    public getHeight(): number { return Math.abs(this.yRange[1] - this.yRange[0]) + 1 }
    public getWidth():  number { return Math.abs(this.xRange[1] - this.xRange[0]) + 1 }
    public getXRange(): [number, number] { return [...this.xRange] }
    public getYRange(): [number, number] { return [...this.yRange] }

    public getRow(y: number): T[] {
        const results: T[] = []
        if (this.grid.has(y)) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    results.push(this.get(x, y)!)
                }
            }
        }
        return results
    }

    public getColumn(x: number): T[] {
        const results: T[] = []
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            if (this.has(x, y)) {
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                results.push(this.get(x, y)!)
            }
        }
        return results
    }

    public has(x: number, y: number): boolean {
        return this.grid.get(y)?.has(x) ?? false
    }

    public count(): number {
        return Array.from(this.grid.keys()).reduce((totalSum, y) =>
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            totalSum + Array.from(this.grid.get(y)!.values()).length
        , 0)
    }

    public countBy(callbackfn: (value: T, index: [number, number]) => boolean): number {
        return this.filter(callbackfn).length
    }


    public forEach(callbackfn: (value: T | undefined, index: [number, number]) => void): void;
    public forEach(callbackfn: (value: T, index: [number, number]) => void, defaultValue: T): void;
    public forEach(callbackfn: 
        ((value: T, index: [number, number]) => void) | ((value: T | undefined, index: [number, number]) => void),
        defaultValue?: T
    ): void {
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                callbackfn(this.get(x, y, defaultValue as T)!, [x, y])
            }
        }
    }

    public reduce<U>(callbackfn: (previousValue: U, currentValue: T, currentIndex: [number, number]) => U, initialValue: U): U {
        let value = initialValue
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    value = callbackfn(value, this.get(x, y)!, [x, y])
                }
            }
        }
        return value
    }

    public clone(callbackfn?: (value: T, index: [number, number]) => T): EndlessGrid<T> {
        return this.map<T>(callbackfn || (a => a))
    }

    public map<T2 extends string | GridCell>(callbackfn: (value: T, index: [number, number]) => T2): EndlessGrid<T2> {
        const mappedGrid = new EndlessGrid<T2>()
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    mappedGrid.set(x, y, callbackfn(this.get(x, y)!, [x, y]))
                }
            }
        }
        return mappedGrid
    }

    public filterRow(y: number, callbackfn: (value: T, index: [number, number]) => boolean): T[] {
        const results: T[] = []
        if (this.grid.has(y)) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    const cell = this.get(x, y)!
                    if (callbackfn(cell, [x, y])) {
                        results.push(cell)
                    }
                }
            }
        }
        return results
    }

    public filterColumn(x: number, callbackfn: (value: T, index: [number, number]) => boolean): T[] {
        const results: T[] = []
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            if (this.has(x, y)) {
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                const cell = this.get(x, y)!
                if (callbackfn(cell, [x, y])) {
                    results.push(cell)
                }
            }
        }
        return results
    }

    public filter(callbackfn: (value: T, index: [number, number]) => boolean): T[] {
        const results: T[] = []
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    const cell = this.get(x, y)!
                    if (callbackfn(cell, [x, y])) {
                        results.push(cell)
                    }
                }
            }
        }
        return results
    }

    public filterIndex(callbackfn: (value: T, index: [number, number]) => boolean): [number, number][] {
        const results: [number, number][] = []
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    const cell = this.get(x, y)!
                    if (callbackfn(cell, [x, y])) {
                        results.push([x, y])
                    }
                }
            }
        }
        return results
    }

    public findIndex(callbackfn: (value: T, index: [number, number]) => boolean): [number, number] | undefined {
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                    const cell = this.get(x, y)!
                    if (callbackfn(cell, [x, y])) {
                        return [x, y]
                    }
                }
            }
        }
        return undefined
    }

    public find(callbackfn: (value: T, index: [number, number]) => boolean): T | undefined {
        const index = this.findIndex(callbackfn)
        if (index) { 
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            return this.get(index[0], index[1])! 
        }
        return undefined
    }

    public slice(x1: number, y1: number, x2: number, y2: number): EndlessGrid<T> {
        const slice = new EndlessGrid<T>()
        for (let x = Math.min(x1, x2); x < Math.max(x1, x2); x++) {
            for (let y = Math.min(y1, y2); y < Math.max(y1, y2); y++) {
                slice.set(x, y, this.get(x, y))
            }
        }
        return slice
    }

    public toString(opts?: {
        defaultValue?: string,
        upsideDown?: boolean,
        reversed?: boolean,
        cellToString?: (cell: T | undefined) => string
    }): string {
        const {
            defaultValue = ' ',
            upsideDown = false,
            reversed = false,
            cellToString
        } = opts || {}
        let body = ''
        for(let y = this.yRange[1]; y >= (this.yRange[0]); y--) {
            let row = ''
            for(let x = this.xRange[0]; x <= (this.xRange[1]); x++) {
                if (cellToString) {
                    row += cellToString(this.get(
                        reversed ? this.xRange[0] - (this.xRange[0] - x) : x, 
                        upsideDown ? this.yRange[1] - (this.yRange[1] - y) : y, 
                    ))
                } else {
                    const cell = this.get(
                        reversed ? this.xRange[0] - (this.xRange[0] - x) : x, 
                        upsideDown ? this.yRange[1] - (this.yRange[1] - y) : y, 
                        defaultValue
                    )
                    if (typeof cell === 'string') {
                        row += cell
                    } else {
                        row += (cell as GridCell).toString()
                    }
                }
            }
            body += row + '\n'
        }
        return body
    }
    
    public renderGrid(render: PuzzleRenderer, size = 5): void {
        const pxWidth = this.getWidth() * size
        const pxHeight = this.getHeight() * size
        render.beginPath()
        render.moveTo(0, 0)
        render.lineTo(pxWidth, 0)
        let pxY = size
        console.log(this.yRange, this.xRange)
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            render.moveTo(0, pxY)
            render.lineTo(pxWidth, pxY)
            pxY += size
        }
        render.moveTo(0, 0)
        render.lineTo(0, pxHeight)
        let pxX = size
        for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
            render.moveTo(pxX, 0)
            render.lineTo(pxX, pxHeight)
            pxX += size
        }
    }    

    public renderCells(render: PuzzleRenderer): void {
        const xOffset = this.xRange[0] * -1
        const yOffset = this.yRange[0] * -1
        for(let y = this.yRange[1]; y >= this.yRange[0]; y--) {
            for(let x = this.xRange[0]; x <= this.xRange[1]; x++) {
                if (this.has(x, y)) {
                    render.fillRect(
                        x + xOffset, 
                        y + yOffset, 
                        1, 
                        1 
                    )
                }
            }
        }
    }
}