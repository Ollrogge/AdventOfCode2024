#!/bin/bash

if [[ -z $1 ]]; then
    echo "Please pass challenge name"
    exit 0
fi

if [[ -e $1 ]]; then
    echo "Directory already exists"
    exit 0
fi

echo "Todo: Add script to automatically download input.txt"

cargo new --bin $1

code=$(cat <<END
fn parse_input(input: &str) {

}

fn part1(input: &str) {

}

fn part2(input: &str) {

}

fn main() {
  let input = include_str!("../input.txt");

  part1(input);
  part2(input);
}
END
)

echo "$code" > ./$1/src/main.rs

if [[ -e "./get_input.sh" ]]; then
    ./get_input.sh $1
fi


