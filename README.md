# Optimistic Read In Rust

## Benchmark result

```sh
$hyperfine --shell=none ./target/release/optimistic_lock ./target/release/rwlock

Benchmark 1: ./target/release/optimistic_lock
  Time (mean ± σ):      34.6 ms ±   7.3 ms    [User: 143.2 ms, System: 2.2 ms]
  Range (min … max):    23.1 ms …  55.1 ms    54 runs
 
Benchmark 2: ./target/release/rwlock
  Time (mean ± σ):      71.0 ms ±   1.9 ms    [User: 414.7 ms, System: 373.2 ms]
  Range (min … max):    66.7 ms …  74.0 ms    41 runs
 
Summary
  './target/release/optimistic_lock' ran
    2.05 ± 0.44 times faster than './target/release/rwlock'
```

