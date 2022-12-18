use std::iter::once;
use std::{str::Lines, mem::ManuallyDrop};
use std::fmt::Debug;

use crate::problem::Problem;

#[derive(PartialEq)]
enum Item {
    Item(u32),
    List(ManuallyDrop<Vec<Item>>)
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Item(item) => write!(f, "{}", item),
            Item::List(list) => write!(f, "{:?}", **list),
        }
    }
}

pub struct Day13 {}

impl Problem for Day13 {
    fn part_one(&self, input: &str) -> String {
        let pairs = input
            .replace('\r', "")
            .split("\n\n")
            .map(|pair| convert_to_lists(pair.lines()))
            .collect::<Vec<(ManuallyDrop<Vec<Item>>, ManuallyDrop<Vec<Item>>)>>();

        let mut count: usize = 0;

        for (j, (left, right)) in pairs.iter().enumerate() {
            iterate(&(left, right), &mut count, j+1, &mut false);
        }

        return format!("{}", count);
    }

    fn part_two(&self, input: &str) -> String {
        let pairs = input
            .replace('\r', "")
            .split("\n\n")
            .map(|pair| convert_to_lists(pair.lines()))
            .flat_map(|p| once(p.0).chain(once(p.1)))
            .collect::<Vec<ManuallyDrop<Vec<Item>>>>();

        let mut count1: usize = 0;
        let mut count2: usize = 0;

        // First
        let mut v1 = Vec::new();
        v1.push(Item::Item(2));
        let m1 = ManuallyDrop::new(v1);
        let mut v2 = Vec::new();
        v2.push(Item::List(m1));
        let m2 = ManuallyDrop::new(v2);

        // Second
        let mut v3 = Vec::new();
        v3.push(Item::Item(6));
        let m3 = ManuallyDrop::new(v3);
        let mut v4 = Vec::new();
        v4.push(Item::List(m3));
        let m4 = ManuallyDrop::new(v4);

        for packet in pairs.iter() {
            iterate(&(packet, &m2), &mut count1, 1, &mut false);
            iterate(&(packet, &m4), &mut count2, 1, &mut false);
        }

        return format!("{}", (count1+1) * (count2+2));
    }
}

fn iterate((left, right): &(&ManuallyDrop<Vec<Item>>, &ManuallyDrop<Vec<Item>>), count: &mut usize, j: usize, to_skip: &mut bool) {
    for (i, l) in left.iter().enumerate() {
        // Right is shorter than left, so wrong order
        if i == right.len() { *to_skip = true; return; }

        let r = &right[i];

        match l {
            Item::Item(l_item) => {
                match r {
                    Item::Item(r_item) => {
                        if l_item == r_item {
                            continue;
                        }
                        if l_item < r_item {
                            if !*to_skip {
                                *count += j;
                            }
                        }
                        *to_skip = true;
                        return;
                    },
                    Item::List(r_list) => {
                        let mut v = Vec::new();
                        v.push(Item::Item(*l_item));
                        iterate(&(&ManuallyDrop::new(v), r_list), count, j, to_skip);
                    },
                }
            },
            Item::List(l_list) => {
                match r {
                    Item::Item(r_item) => {
                        let mut v = Vec::new();
                        v.push(Item::Item(*r_item));
                        iterate(&(l_list, &ManuallyDrop::new(v)), count, j, to_skip);
                    },
                    Item::List(r_list) => {
                        iterate(&(l_list, r_list), count, j, to_skip);
                    },
                }
            },
        }
        if *to_skip { return; }
    }

    // Checking if got to the end of the left list but it's smaller than the right
    if !*to_skip && left.len() < right.len() {
        *count += j;
        *to_skip = true;
    }
}

fn convert_to_lists(mut pair: Lines) -> (ManuallyDrop<Vec<Item>>, ManuallyDrop<Vec<Item>>) {
    let mut l_chr = pair.next().unwrap().chars().collect::<Vec<char>>();
    let mut r_chr = pair.next().unwrap().chars().collect::<Vec<char>>();

    let left = do_convert(&mut l_chr, &mut 0).unwrap();
    let right = do_convert(&mut r_chr, &mut 0).unwrap();

    match left {
        Item::List(l_list) => {
            match right {
                Item::List(r_list) => return (l_list, r_list),
                _ => panic!("Nope")
            }
        },
        _ => panic!("Nope")
    }
}

fn do_convert(chars: &mut Vec<char>, i: &mut usize) -> Option<Item> {
    loop {
        let mut chr = chars[*i];
        *i += 1;

        if chr.is_digit(10) {
            // Checking for double digit
            if chars[*i].is_digit(10) {
                return Some(Item::Item((chr.to_digit(10).unwrap() * 10) + chars[*i].to_digit(10).unwrap()))
            }
            return Some(Item::Item(chr.to_digit(10).unwrap()));
        }

        if chr == ',' { continue; }

        if chr == ']' { return None };

        if chr == '[' {
            let mut temp = ManuallyDrop::new(vec![]);

            while chr != ']' {
                match do_convert(chars, i) {
                    Some(res) => temp.push(res),
                    None => break,
                };
                chr = chars[*i];
                *i += 1;
            }

            return Some(Item::List(temp));
        }
    }
}
