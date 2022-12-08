use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct TreeNode<'a> {
  pub dir: &'a str,
  pub value: u32,
  pub children: HashMap<&'a str, Rc<RefCell<TreeNode<'a>>>>,
  pub parent: Option<Rc<RefCell<TreeNode<'a>>>>,
}

impl<'a> TreeNode<'a> {
  pub fn new(dir: &'a str) -> TreeNode<'a> {
    TreeNode {
      dir: dir,
      value: 0,
      children: HashMap::new(),
      parent: None,
    }
  }

  // We want to find the smallest dir that is still larger than the amount we need to free
  pub fn find_smallest<'b>(&self, mut smallest: &'b mut u32, to_allocate: u32) {
    if self.value < *smallest && self.value >= to_allocate {
      *smallest = self.value;
    }
    for child in self.children.values() {
      child.borrow_mut().find_smallest(&mut smallest, to_allocate);
    }
  }

  pub fn add_child(&mut self, dir: &'a str, new_node: Rc<RefCell<TreeNode<'a>>>) {
    self.children.insert(dir, new_node);
  }

  // For efficiency reasons we go through the children to the bottom
  // Then we update the parents with their new total sum and go back up
  pub fn sum_bottom_up_filter<'b>(&mut self, mut val: &'b mut u32) -> &'b mut u32 {
    if !self.children.is_empty() {
      for child in self.children.values() {
        child.borrow_mut().sum_bottom_up_filter(&mut val);
        self.value += child.borrow_mut().value
      }
    }

    if self.value <= 100000 { *val += self.value; }
    return val;
  }

  // We don't filter on this one
  pub fn sum_bottom_up<'b>(&mut self, mut val: &'b mut u32) {
    if !self.children.is_empty() {
      for child in self.children.values() {
        child.borrow_mut().sum_bottom_up(&mut val);
        self.value += child.borrow_mut().value
      }
    }

    *val += self.value;
  }
}
