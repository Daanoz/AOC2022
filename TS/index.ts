import { terminal as term } from 'terminal-kit'
import { program } from 'commander'
import { from, Observable, Subject, timer, of, merge } from 'rxjs'
import { switchMap, map, takeUntil, debounce, catchError } from 'rxjs/operators'
import { ChildProcess, fork } from 'child_process'
import path from 'path'
import fs from 'fs'

const logo = [`
 .--.  .--.  .--. .---.  .--. .---. .---. 
: .; :: ,. :: .--'\`--. :: ,. :\`--. :\`--. :
:    :: :: :: :     ,',': :: :  ,','  ,','
: :: :: :; :: :__ .'.'_ : :; :.'.'_ .'.'_ 
:_;:_;\`.__.'\`.__.':____;\`.__.':____;:____;                                                                                                                                       
`].join('\n')
term(logo +' '.repeat(28) + 'By Daan Sieben\n')

program
  .option('-i, --input <input>', 'input file')
  .option('-d, --day <day>', 'day to run')
  .option('-m, --mode <mode>', 'mode (run or watch)')
program.parse(process.argv)

const destroy$ = new Subject()
term.on('key' , (name: string) => {
	if ( name === 'CTRL_C' ) { 
        if (cmd) {
            cmd.kill()
        }
        destroy$.next(true)
    }
})

const choosePuzzleMode = (): Observable<string> => {
    const options = [
        {label: 'Run mode', key: 'run' },
        {label: 'Watch mode (re-run on file change)', key: 'watch' },
    ]
    const mode = program.opts().mode
    if (mode) {
        if (options.find(o => o.key === mode)) {
            return of(mode)
        }
    }

    term.white.bold( 'In which mode do you want to run the puzzle?.\n')
    return from(term.singleColumnMenu(
        options.map(o => o.label),
        {cancelable: true, exitOnUnexpectedKey: true}
    ).promise.then((response?: {selectedIndex: number}) => {
        if (!response) { return Promise.reject() }
        if (!options[response.selectedIndex]) { return Promise.reject() }
        return options[response.selectedIndex].key
    }))
}

const choosePuzzle = (): Observable<string> => {
    const puzzleList = fs.readdirSync(__dirname, {withFileTypes: true})
        .filter(item => item.isDirectory)
        .filter(item => item.name.match(/\d\d/))
        .map(item => item.name)
    const day = program.opts().day
    if (day) {
        if (puzzleList.indexOf(day) >= 0) { return of(day) }
        if (day.length < 2 &&
            puzzleList.indexOf('0' + day) >= 0) {
            return of('0' + day)
        }
    }

    term.white.bold('Which puzzle do you want to run?.\n')
    return from(term.gridMenu(
        puzzleList,
        {exitOnUnexpectedKey: true}
    ).promise.then((response?: {selectedText: string}) => {
        if (!response) { return Promise.reject() }
        return response.selectedText
    }))
}

const watchDir$ = (path: string) => {
    return new Observable<boolean>(subscriber => {
        const watcher = fs.watch(
            path,
            {persistent: true, recursive: true},
            ( ) => subscriber.next(true)
        )
        return () => watcher.close()
    }).pipe(debounce(() => timer(100)))
}

let cmd: ChildProcess
const run = (puzzle: string, mode: string): Observable<boolean> => {
    term('\n\n')
    const puzzleDir = path.join(__dirname, puzzle)
    const run$ = new Observable<boolean>(subscriber => {
        term.bold('\nExecuting puzzle %s\n\n', puzzle)
        const args: string[] = []
        const input = program.opts().input
        if (input) { args.push('--input', input) }
        if (cmd) {
            cmd.kill()
        }
        cmd = fork(
            'index.ts', args,
            {
                execArgv: ['-r', 'ts-node/register'],
                env: {
                    ...process.env,
                    isWatching: (mode === 'watch') ? '1' : '0'
                },
                cwd: puzzleDir
            }
        )
        if (!cmd) { return subscriber.error('Execution failed') }
        cmd.on('close', (code) => {
            if (code !== 0) {
                subscriber.error('Error code: ' + code)
                return
            }
            subscriber.next(true)
            subscriber.complete()
        })
        return () => cmd.kill()
    })
    if (mode === 'watch') {
        return merge(
            of(true),
            watchDir$(puzzleDir)
        ).pipe(
            switchMap(
                () => run$.pipe(
                    catchError(() => of(true))
                )
            )
        )
    } else {
        return run$
    }
}

choosePuzzle().pipe(
    switchMap(puzzle => choosePuzzleMode().pipe(map(mode => ({
        puzzle, mode
    })))),
    switchMap((conf: {puzzle: string, mode: string}) => run(conf.puzzle, conf.mode)),
    takeUntil(destroy$)
).subscribe(
    () => {},
    err => {
        if (err) { term.red('%s\n', err) }
        process.exit(0)
    },
    () => process.exit(0)
)