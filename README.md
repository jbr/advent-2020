# [advent-2020](https://adventofcode.com/2020)

In general, I'm trying to strike a good balance between readability and cpu/memory usage, generally trying to stick to idiomatic rust and only pulling in libraries where necessary/appropriate

## [01: repair report](https://github.com/jbr/advent-2020/blob/main/report-repair/src/main.rs)

I wasn't sure if it was legit to pre-parse the input data, so the input is treated as a string. Given that constraint I think it's fairly efficient.

We parse numbers from the input string as we iterate over the input, and will only parse as many lines as needed in order to find our three digits. We store a hashmap of sums to products. For each new digit that we visit, we iterate over all of the previous digits and store `2020 - a - b -> a * b` in our our lookup table. We also check if each number exists in that lookup table. If it does, that means that we have found our three numbers, and multiplying our new number by the existing product yields the final answer.

## [02: password-philosophy](https://github.com/jbr/advent-2020/blob/main/password-philosophy/src/main.rs)

I tried a few ways of doing this without regex, and they were all worse than just using a regex. I probably should have tried using a parser-combinator or similar tool, since I know regexes aren't really the rusty way to do write parsers due to the runtime cost of regex pattern compilation. All told, not my favorite problem or solution.

## [03: toboggan-trajectory](https://github.com/jbr/advent-2020/blob/main/toboggan-trajectory/src/main.rs)

Although I barely got this one in within the 24h time limit, I ended up enjoying writing it a lot. Iterator is one of my favorite rust types, and sold me on the language early on. The promise of functional expression with zero runtime computational cost is really powerful, and I don't have enough opportunities to use them.  This ended up being quite straightforward to type out, and I'm pretty happy with the end product. It was tempting to express the entire thing without a single let binding, but I decided this was slightly more readable.

Note: It seems like it's probably possible to do this in a single pass through the input, which would be more efficient at a small readability cost. For each row in the input, you'd iterate over the list of slopes and accumulate the trees for each of those slopes, instead of iterating over the slopes and then iterating over the full input for each of them.

<!-- ## [04: X](https://github.com/jbr/advent-2020/blob/main/X/src/main.rs) -->
