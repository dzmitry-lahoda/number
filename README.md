# number

Composable exact numbers for Rust tests, model code, fuzzing, and invariant checks.

`Number` is a small convenience wrapper around Malachite rationals. It is meant for code where
readability and exactness matter more than speed: second implementations, reference formulas,
property tests, simulation checks, and inverse calculations.

The goal is to write formulas close to the way they appear on paper, without littering test code
with casts, widening, rounding, or temporary integer types.

## Black-Scholes Shape

For production pricing you should use a numeric library with the right floating-point and
statistical behavior. For tests, it is often useful to encode the structure of a famous formula
exactly and plug in rational approximations.

Black-Scholes call option price:

```text
C = S * N(d1) - K * e^(-rT) * N(d2)
```

As an exact reference-style formula:

```rust
use number::num;

let spot = 100;
let strike = 95;
let discount_factor = num!(0.97);
let normal_d1 = num!(3 / 5);
let normal_d2 = num!(11 / 20);

let call = spot * normal_d1 - strike * discount_factor * normal_d2;

assert_eq!(call, num!(1471 / 200));
assert_eq!(format!("{:?}", call), "7.355");
```
