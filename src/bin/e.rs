use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        t: usize,
        tuples: [(usize, Bytes); t],
    }

    let offset = 'A' as u8;

    for (n, s) in tuples.into_iter() {  
        let s = s.into_iter().map(|c| c - offset).collect::<Vec<u8>>();

        let count = |n: usize, s: &Vec<u8>| {
            (0..=(n - 1) / 2).into_iter()
                            .fold(0, |acc, num| (acc * 26 + (s[num] as u64)) % 998244353)
        };

        let issue = |n: usize, s: &Vec<u8>| {
            let mut s_copy = s.clone();
            (0..=(n - 1) / 2).into_iter()
                             .for_each(|num| s_copy[n - num - 1] = s_copy[num]);

            s_copy <= *s
        };

        let ans = count(n, &s) + if issue(n, &s) {1} else {0};

        println!("{}", ans);
    }
}
