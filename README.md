# fastroute-rs

Rust regex router which builds all routes into one single regex and in one shot tries to match route

Bench results

```
Compare Routers/fastroute-rs
                        time:   [27.293 µs 27.364 µs 27.435 µs]
                        change: [-1.1415% -0.7290% -0.3240%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Compare Routers/regex   time:   [946.91 ns 951.36 ns 956.77 ns]
                        change: [-0.3647% +0.1183% +0.6081%] (p = 0.64 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
Compare Routers/actix   time:   [16.840 µs 16.863 µs 16.889 µs]
                        change: [-3.3037% -2.8641% -2.4374%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe
```
