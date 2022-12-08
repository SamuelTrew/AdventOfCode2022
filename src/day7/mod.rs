use std::cell::RefCell;
use std::rc::Rc;
use std::str::Lines;

use crate::problem::Problem;
use tree::{TreeNode};

mod tree;

pub struct Day7 {}

impl Problem for Day7 {
    fn part_one(&self, input: &str) -> String {
        let mut cmds = input.lines();
        cmds.nth(1);

        let root = Rc::new(RefCell::new(TreeNode::new("/")));
        build_tree(&root, cmds);

        let res = &mut 0;
        root.borrow_mut().sum_bottom_up_filter(res);

        return format!("{}", res);
    }

    fn part_two(&self, input: &str) -> String {
        const TOTAL_SPACE: u32 = 70_000_000;
        const SPACE_NEEDED: u32 = 30_000_000;
        let mut cmds = input.lines();
        cmds.nth(1);

        let root = Rc::new(RefCell::new(TreeNode::new("/")));
        build_tree(&root, cmds);

        // This modifies the Tree so each dir knows about its children
        root.borrow_mut().sum_bottom_up(&mut 0);
        let space_used = root.borrow_mut().value;
        let to_allocate = SPACE_NEEDED - (TOTAL_SPACE - space_used);

        let mut smallest = space_used; // arbitrary initial value
        root.borrow_mut().find_smallest(&mut smallest, to_allocate);

        return format!("{}", smallest);
    }
}

fn build_tree<'a>(root: &Rc<RefCell<TreeNode<'a>>>, mut cmds: Lines<'a>) {
    let mut curr_node = Rc::clone(&root);

    while let Some(cmd) = cmds.next() {
        let mut parts = cmd.split(" ");
        match parts.next().unwrap() {
            "$" => {
                let unix = parts.next().unwrap();
                if unix == "ls" { continue; }

                // Either set as parent or chosen child
                curr_node = match parts.next().unwrap() {
                    ".." => curr_node.borrow_mut().parent.as_ref().unwrap().to_owned(),
                    dir => curr_node.borrow_mut().children.get(dir).unwrap().to_owned(),
                };
            },
            // Create a new dir inside of the current directory (assumes this doesn't repeat)
            "dir" => {
                let new_dir = parts.next().unwrap();
                let child = Rc::new(RefCell::new(TreeNode::new(new_dir)));
                curr_node.borrow_mut().add_child(new_dir, Rc::clone(&child));
                child.borrow_mut().parent = Some(Rc::clone(&curr_node));
            },
            // Just add value to the curr_dir as it's a file
            val => curr_node.borrow_mut().value += val.parse::<u32>().unwrap(),
        }
    }
}
