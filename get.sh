#!/bin/bash

cd "$(dirname "$0")"

if [ -r .env ]; then
  . .env
fi

export TZ=EST
thisyear="$(date +%Y)"
thismonth="$(date +%m)"
thisday="$(date +%d)"

mkdir -p input

year=2022
for day in {1..25}; do
  if [ "$year" -eq "$thisyear" -a "$thismonth" -ne 12 ]; then
      exit 0
  fi
  if [ "$year" -eq "$thisyear" -a "$thismonth" -eq 12 -a "$day" -gt "$thisday" ]; then
      exit 0
  fi
  filename="input/day$(printf "%02d" $day)".input
  if [ -r "$filename" ]; then
    continue  # make sure we don't fetch the same file twice!
  fi
  curl -sS -o "$filename" -b "$AOC_SESSION" https://adventofcode.com/"$year"/day/"$day"/input
done
