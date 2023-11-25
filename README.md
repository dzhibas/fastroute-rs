# fastroute-rs

Rust regex router which builds all routes into one single regex and in one shot tries to match route

Bench results

```
Compare Routers/fastroute-rs-regex
                        time:   [27.168 µs 27.236 µs 27.307 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

Compare Routers/fastroute-rs-regext_lite
                        time:   [2.2492 ms 2.2556 ms 2.2625 ms]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

Compare Routers/regex   time:   [945.08 ns 946.56 ns 948.09 ns]
                        change: [-0.5462% -0.0708% +0.3600%] (p = 0.77 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Compare Routers/actix   time:   [17.038 µs 17.074 µs 17.110 µs]
                        change: [+0.8709% +1.2171% +1.5783%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

Compare Routers/matchit time:   [191.22 ns 191.45 ns 191.70 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```
