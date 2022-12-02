import { Puzzle, Runner, BasePuzzle, Result } from '../shared';

const LOSS_SCORE = 0
const DRAW_SCORE = 3
const WIN_SCORE = 6

const ROCK = 1
const PAPER = 2
const SCISSORS = 3

export class PuzzleSolution extends BasePuzzle implements Puzzle {
  private opponentShapeMap = new Map<string, number>([
    ['A', ROCK],
    ['B', PAPER],
    ['C', SCISSORS],
  ]);

  public run(): Result {
    const result: Result = {}

    result.a = this.playRounds(new Map<string, number>([
      ['X', ROCK],
      ['Y', PAPER],
      ['Z', SCISSORS],
    ]))
    result.b = this.playRounds(new Map<string, number>([
      ['X', LOSS_SCORE],
      ['Y', DRAW_SCORE],
      ['Z', WIN_SCORE],
    ]), true)

    return result
  }

  private playRounds(map: Map<string, number>, asStrategy = false): number {
    return this.getInputAsRows().reduce(
      (totalScore, round) => totalScore +
        (asStrategy ? this.playStrategicRound(map, round) : this.playRound(map, round)),
      0
    )
  }

  private playRound(shapeMap: Map<string, number>, round: string): number {
    const [opponent, response] = round.split(' ')
    const opponentPlay = this.opponentShapeMap.get(opponent)!
    const responsePlay = shapeMap.get(response)!
    if (opponentPlay === responsePlay) {
        return responsePlay + DRAW_SCORE
    }
    if ((opponentPlay + 1 === responsePlay) || (opponentPlay === SCISSORS && responsePlay === ROCK)) {
        return responsePlay + WIN_SCORE
    }
    return responsePlay + LOSS_SCORE
  }

  private playStrategicRound(strategy: Map<string, number>, round: string): number {
    const [opponent, outcome] = round.split(' ')
    const opponentPlay = this.opponentShapeMap.get(opponent)!
    const outcomeResult = strategy.get(outcome)!
    if (outcomeResult === DRAW_SCORE) {
        const responsePlay = opponentPlay
        return responsePlay + DRAW_SCORE
    }
    if (outcomeResult === WIN_SCORE) {
        const responsePlay = (opponentPlay + 1) === 4 ? 1 : (opponentPlay + 1)
        return responsePlay + WIN_SCORE
    }
    const responsePlay = (opponentPlay - 1) === 0 ? 3 : (opponentPlay - 1)
    return responsePlay + LOSS_SCORE
  }
}

Runner(PuzzleSolution)
