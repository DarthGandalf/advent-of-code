import { ints } from './util'

export enum Result {
  Good,
  TooFar,
  TooNear,
  TooFastX,
  TooFastY,
}

export class Area {
  private mx: number = 0;
  private my: number = 0;

  constructor(
    private x1: number,
    private x2: number,
    private y1: number,
    private y2: number,
  ) {
    this.mx = (x1 + x2) / 2;
    this.my = (y1 + y2) / 2;
  }

  attempt(vx: number, vy: number): Result {
    let x = 0;
    let y = 0;
    while (true) {
      let nx = x + vx;
      let ny = y + vy;
      if (nx >= this.x1 && nx <= this.x2 && ny >= this.y1 && ny <= this.y2) {
        return Result.Good;
      }
      if (x < this.x1 && nx > this.x2) {
        return Result.TooFastX;
      }
      if (ny < this.y1) {
        if (y > this.y2) {
          return Result.TooFastY;
        }
        let tx = this.mx - x;
        let ty = this.my - y;
        if (vx * ty < vy * tx) {
          return Result.TooFar;
        } else {
          return Result.TooNear;
        }
      }
      x = nx;
      y = ny;
      if (vx > 0) vx--;
      vy--;
    }
  }
}

export function solution(input: string): number[] {
  const [x1, x2, y1, y2] = ints(input);

  const area = new Area(x1, x2, y1, y2)

  let my = -1;
  let count = 0;
  for (let y = -Math.abs(y1); y < Math.abs(y1); ++y) {
    for (let x = 0; x < x2 * x2; ++x) {
      if (area.attempt(x, y) == Result.Good) {
        if (y > my) my = y;
        count++;
      }
    }
  }
  return [my * (my + 1) / 2, count];
}
