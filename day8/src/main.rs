static INPUT: &str = include_str!("input");

fn main() {
    println!("Day 8, Part 1: {}", part1(&INPUT));
    println!("Day 8, Part 2: {}", part2(&INPUT));
}

fn part1(input: &str) -> usize {
    let mut tree = nodes(input);
    sum_metadata(&mut tree)
}

fn part2(input: &str) -> usize {
    let mut tree = nodes(input);
    value_of_root(&mut tree)
}

fn nodes<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.split_whitespace().map(|line| line.parse()).filter_map(Result::ok)
}

fn sum_metadata<T: Iterator<Item = usize>>(tree: &mut T) -> usize {
    let childs = tree.next().unwrap();
    let entries = tree.next().unwrap();
    (0..childs).map(|_| sum_metadata(tree)).sum::<usize>() + tree.take(entries).sum::<usize>()
}

fn value_of_root<T: Iterator<Item = usize>>(tree: &mut T) -> usize {
    let childs = tree.next().unwrap();
    let entries = tree.next().unwrap();
    if childs == 0 {
        return tree.take(entries).sum();
    }
    let value_of_child: Vec<_> = (0..childs).map(|_| value_of_root(tree)).collect();
    tree.take(entries)
        .filter(|&i| 0 < i && i <= childs)
        .map(|i| value_of_child[i - 1])
        .sum()
}
