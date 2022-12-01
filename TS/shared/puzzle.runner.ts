import { program } from 'commander'
import fs from 'fs'
import path from 'path'
import https from 'https'

import { Puzzle } from './puzzle.interface'
import { PuzzleServer } from './puzzle.server'

type Constructor<T> = new (renderServer?: PuzzleServer) => T;

export type PuzzleResult = {
    a: (string | number | Promise<string | number>),
    b: (string | number | Promise<string | number>)
}

export const Runner = (PuzzleClass: Constructor<Puzzle>): void => {
    if (process.env.NODE_ENV === 'test') { return } // do not execute for tests

    program
        .option('-i, --input <input>', 'input file')
    program.parse(process.argv)

    const puzzleDir = process.cwd()
    let server: PuzzleServer | undefined = undefined
    if (process.env.isWatching === '1') {
        server = new PuzzleServer()
    }

    getInputData(puzzleDir, program.opts().input).then(inputData => {
        const inst = new PuzzleClass(server)
        inst.setInput(inputData)

        const start = (new Date()).getTime()
        const result = inst.run()

        const resultPromises: Promise<void | boolean>[] = []
        if (result) {
            if (result.a) {
                resultPromises.push(asPromise(result.a).then(res => console.log('Part A', res)))
            }
            if (result.b) {
                resultPromises.push(asPromise(result.b).then(res => console.log('Part B', res)))
            }
        }
        const complete = () => {
            const duration = (new Date()).getTime() - start
            console.log('\nTotal time taken: ' + duration + 'ms')
            console.log(inst.getBenchmarks()
                .map(benchMark => benchMark.label + ': ' + benchMark.time + 'ms')
                .join(', ')
            )
        }
        return Promise.all(resultPromises)
            .then(complete)
            .finally(() => {
                // server.close();
            })
    }).catch(error => console.error(error))
}

const asPromise = (val: string | number | Promise<string | number | void>) => {
    if (val && val instanceof Promise) {
        return val
    } else {
        return Promise.resolve(val)
    }
}

const getInputData = (puzzleDir: string, inputFile?: string): Promise<string> => {
    const filename = inputFile ? inputFile : 'input'
    if (fs.existsSync(path.join(puzzleDir, filename))) {
        return Promise.resolve(fs.readFileSync(path.join(puzzleDir, filename), {encoding: 'utf-8'}))
    }
    if (filename !== 'input') {
        return Promise.reject('File not found!')
    }
    const DAY = puzzleDir.split('/').pop()
    if (!DAY || !DAY.match(/\d\d/)) {
        return Promise.reject('Unable determine current day!')
    }
    if (!process.env.AOC_SESSION) {
        return Promise.reject('AOC_SESSION not set in env')
    }
    const YEAR = '2022'
    return new Promise<string>((resolve, reject) => {
        const options = {
            hostname: 'adventofcode.com',
            port: 443,
            path: `/${YEAR}/day/${parseInt(DAY)}/input`,
            headers: {
                cookie: 'session=' + process.env.AOC_SESSION
            }
        }
        const req = https.request(options, (res) => {
            res.setEncoding('utf8')
            let data = ''
            if (res.statusCode !== 200) {
                reject('Input data request failed: ' + res.statusCode)
                return
            }
            res.on('data', (chunk) => {
                data += chunk
            })
            res.on('end', () => {
                console.log('Downloaded input data from adventofcode.com')
                resolve(data)
            })
        })
        req.on('error', (e) => {
            console.error(`problem with fetching input data: ${e.message}`)
            reject('Input data request failed')
            return
        })
        req.end()
    }).then(data => {
        fs.writeFileSync(path.join(puzzleDir, filename), data, {encoding: 'utf-8'})
        return data
    })
}
