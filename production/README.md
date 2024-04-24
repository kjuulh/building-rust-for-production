# Profile

## Stats

- Unoptimized:
  - Size: 5.4 MB
  - Buildtime: 21 sec
- Optimized:
  - Size: 2.9 MB
  - Buildtime: 39 sec

- Bench

```bash
$ hyperfine  './target/debug/sample-app bench' './target/release/sample-app bench' './target/release-optimized/sample-app bench'
Benchmark 1: ./target/debug/sample-app bench
  Time (mean ± σ):      7.606 s ±  0.085 s    [User: 7.538 s, System: 0.006 s]
  Range (min … max):    7.565 s …  7.842 s    10 runs

  Warning: The first benchmarking run for this command was significantly slower than the rest (7.842 s). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.

Benchmark 2: ./target/release/sample-app bench
  Time (mean ± σ):     946.9 ms ±   4.7 ms    [User: 936.2 ms, System: 3.2 ms]
  Range (min … max):   942.6 ms … 955.0 ms    10 runs

Benchmark 3: ./target/release-optimized/sample-app bench
  Time (mean ± σ):     953.7 ms ±   2.6 ms    [User: 937.3 ms, System: 3.2 ms]
  Range (min … max):   949.0 ms … 957.7 ms    10 runs

Summary
  ./target/release/sample-app bench ran
    1.01 ± 0.01 times faster than ./target/release-optimized/sample-app bench
    8.03 ± 0.10 times faster than ./target/debug/sample-app bench
```


### Bonus upx

- Unoptimized:
  - Size: 2.9MB

- Optimized:
  - Size: 1.3MB
