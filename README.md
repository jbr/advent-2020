# [advent-2020](https://adventofcode.com/2020)

## [01: repair report](https://github.com/jbr/advent-2020/blob/main/report-repair/src/main.rs)

I wasn't sure if it was legit to pre-parse the input data, so the input is treated as a string. Given that constraint I think it's fairly efficient.

We parse numbers from the input string as we iterate over the input, and will only parse as many lines as needed in order to find our three digits. We store a hashmap of sums to products. For each new digit that we visit, we iterate over all of the previous digits and store `2020 - a - b -> a * b` in our our lookup table. We also check if each number exists in that lookup table. If it does, that means that we have found our three numbers, and multiplying our new number by the existing product yields the final answer.
