fn main() {
    proconio::input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    }

    if x <= a {
        println!("{}", 1f64);
    } else if x <= b {
        println!("{}", c / (b - a));
    } else {
        println!("{}", 0f64);
    }
}
