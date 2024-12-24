This repository contains my solutions to [Advent of Code 2024](https://adventofcode.com/2024) written in Rust.

# Running a solution
A solution can be run from the `main.rs` binary, which will prompt the user to select a day from AoC (1-25).

# Solution structure
Each solution is its own type named `DayXX` corresponding to AoC puzzle the solution is for. Each `DayXX` implements a Solution interface (defined in the solution.rs module) with methods for printing the solution to Part 1 and Part 2 of that day. 
