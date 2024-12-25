import { expect, test } from "bun:test";

type Input = { lefties: number[]; righties: number[] };

function parseInput(input: string): Input {
  const lefties: number[] = [];
  const righties: number[] = [];

  input.split("\n").forEach((l) => {
    const splits = l.split(" ");
    const left = Number(splits.at(0));
    const right = Number(splits.at(-1));
    lefties.push(left);
    righties.push(right);
  });

  return {
    lefties,
    righties,
  };
}

function numberSort(a: number, b: number): number {
  return a - b;
}

function partOne(input: Input): number {
  const sortedLefties = input.lefties.toSorted(numberSort);
  const sortedRighties = input.righties.toSorted(numberSort);

  const differences = input.lefties.map((_, i) => {
    const left = sortedLefties[i];
    const right = sortedRighties[i];

    return Math.abs(left - right);
  });

  return differences.reduce((a, b) => a + b, 0);
}

function partTwo(input: Input): number {
  const occurrences = new Map<number, number>();

  input.righties.forEach((l) => {
    const occurrence = occurrences.get(l);

    occurrences.set(l, 1 + (occurrence ?? 0));
  });

  const similarities = input.lefties.map((l) => l * (occurrences.get(l) ?? 0));

  return similarities.reduce((a, b) => a + b, 0);
}

const input = parseInput(await Bun.file("input/day01.txt").text());
console.log(partOne(input));
console.log(partTwo(input));

const SAMPLE_INPUT = {
  lefties: [3, 4, 2, 1, 3, 3],
  righties: [4, 3, 5, 3, 9, 3],
} as Input;

test("parsing", () => {
  const parsedInput = parseInput(`3   4
4   3
2   5
1   3
3   9
3   3`);

  expect(parsedInput).toEqual(SAMPLE_INPUT);
});

test("part one", () => {
  const result = partOne(SAMPLE_INPUT);

  expect(result).toBe(11);
});

test("part two", () => {
  const result = partTwo(SAMPLE_INPUT);

  expect(result).toBe(31);
});
