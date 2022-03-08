use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: u32,
    }

    let mut table = HashMap::<(u32, u32), u64>::new();

    let ans = (1..10u32).into_iter()
                        .map(|num| count(n, num, &mut table))
                        .sum::<u64>();

    println!("{}", ans);
}

fn count(digit: u32, num: u32, table: &mut HashMap<(u32, u32), u64>) -> u64 {
    if num == 0 || num == 10 {
        return 0;
    }

    if digit == 1 {
         return match num {
            0 | 10 => 0,
            _ => 1,
        };
    }

    return match table.get(&(digit, num)) {
        Some(&x) => x,
        None => {
            let count = count(digit - 1, num + 1, table) + count(digit - 1, num, table) + count(digit - 1, num - 1, table);
            table.insert((digit, num), count);
            count
        }
    }
}
