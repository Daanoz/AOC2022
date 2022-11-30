export interface Result {
    a?: string | number | Promise<string | number | void>,
    b?: string | number | Promise<string | number | void>,
    meta?: unknown
}

export interface Puzzle {
    setInput(data: string): void;
    run(): Result | undefined;
    getBenchmarks(): {label: string, time: number}[];
}