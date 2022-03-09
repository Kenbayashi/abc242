use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(u64, u64); q],
    }

    tk.into_iter()
      .map(|(t, k)| get_ans(t, k - 1, &s))
      .for_each(|c| println!("{}", c));
}

fn get_ans(t: u64, k: u64, s: &Vec<char>) -> char {
    if t == 0 {
        return s[k as usize];
    }

    if k == 0 {
        return advance(s[0], t);
    }

    advance(get_ans(t - 1, k / 2, s), k % 2 + 1)
}

fn advance(c: char, count: u64) -> char {
    let d = (count % 3) as u8;
    char::from(b'A' + (c as u8 - b'A' + d) % 3)
}
