# LOcal rEgreSSion in Rust

A port of https://github.com/hroest/CppLowess/blob/master/include/CppLowess/Lowess.h.

There are some discrepancies in the results to the tests in the repo above. I'm trying to find the bug(s) if it is one. It still produces a smoothed line however.

## Usage

```
use loess::Lowess;

let xvals: Vec<f64> = vec![
    1, 2, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 10, 12, 14, 50,
    ]
    .iter()
    .map(|a| *a as f64)
    .collect();

let yvals: Vec<f64> = vec![
    18, 2, 15, 6, 10, 4, 16, 11, 7, 3, 14, 17, 20, 12, 9, 13, 1, 8, 5, 19,
    ]
    .iter()
    .map(|a| *a as f64)
    .collect();

// takes &[f64] for x and y
// tune the fit with the three parameters (not actually sure what they are yet)

let test = Lowess::new(&xvals, &yvals, 0.25, 0, 0.0);

println!("{:?}", test);

```

You can run this example by running `cargo run --example example1`.