use loess::Lowess;

fn main() {
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

    let test = Lowess::new(&xvals, &yvals, 0.25, 0, 0.0);

    println!("{:?}", test);
}
