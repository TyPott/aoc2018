static INPUT: &str = include_str!("input");

fn main() {
    println!("Day 5, Part 1: {}", part1(INPUT.trim().chars()));
    println!("Day 5, Part 2: {}", part2(INPUT.trim().chars()).unwrap());
}

fn part1<T: Iterator<Item = char>>(s: T) -> usize {
    s.fold(Vec::new(), |mut cache, c| {
        match cache.last() {
            None => cache.push(c),
            Some(&l) => if l != c && l.eq_ignore_ascii_case(&c) {
                cache.pop();
            } else {
                cache.push(c);
            },
        }
        cache
    }).len()
}

fn part2<T: Iterator<Item = char> + Clone>(s: T) -> Option<usize> {
    Some((b'A'..=b'Z')
        .map(|b| b as char)
        .map(|c| part1(s.clone().filter(|l| !l.eq_ignore_ascii_case(&c))))
        .min()?)
}
