import { expect, test } from "bun:test";

type Report = number[];
type Input = Report[];

function parseInput(input: string): Input {
  return input.split("\n").map((l) => {
    return l.split(" ").map((n) => Number(n));
  });
}

function isSafe(report: Report): boolean {
  let expectedSign = Math.sign(report[1] - report[0]);

  return report
    .slice(1)
    .map((value, index) => {
      return value - report[index];
    })
    .every((d) => {
      const absoluteDifference = Math.abs(d);
      return (
        absoluteDifference >= 1 &&
        absoluteDifference <= 3 &&
        expectedSign === Math.sign(d)
      );
    });
}

function partOne(input: Input): number {
  return input.map((r) => isSafe(r)).filter((r) => r === true).length;
}

function partTwo(input: Input): number {
  return input
    .map(
      (r) =>
        isSafe(r) ||
        r.reduce((previousValue, _, index) => {
          const withoutCurrentIndex = r.filter((_, i) => i !== index);
          return previousValue || isSafe(withoutCurrentIndex);
        }, false)
    )
    .filter((r) => r === true).length;
}

let input = parseInput(await Bun.file("input/day02.txt").text());
console.log(partOne(input));
console.log(partTwo(input));

const SAMPLE_INPUT = [
  [7, 6, 4, 2, 1],
  [1, 2, 7, 8, 9],
  [9, 7, 6, 2, 1],
  [1, 3, 2, 4, 5],
  [8, 6, 4, 4, 1],
  [1, 3, 6, 7, 9],
] as Input;

test("parsing", () => {
  let parsedInput = parseInput(`7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`);

  expect(parsedInput).toEqual(SAMPLE_INPUT);
});

test("part one", () => {
  const result = partOne(SAMPLE_INPUT);

  expect(result).toBe(2);
});

test("part two", () => {
  const result = partTwo(SAMPLE_INPUT);

  expect(result).toBe(4);
});
