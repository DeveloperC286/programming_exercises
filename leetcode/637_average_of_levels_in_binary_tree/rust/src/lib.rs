use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn get_levels_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<(f64, usize)> {
    fn set_level_values(
        node: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        levels_values: &mut HashMap<i32, (f64, usize)>,
    ) {
        if let Some(node) = node {
            let value = node.borrow().val;
            let (level_sum, level_count) = levels_values.entry(depth).or_insert((0.0, 0));
            *level_sum += f64::from(value);
            *level_count += 1;

            let children_depth = depth + 1;

            set_level_values(node.borrow().left.clone(), children_depth, levels_values);
            set_level_values(node.borrow().right.clone(), children_depth, levels_values);
        }
    }

    let mut levels_values: HashMap<i32, (f64, usize)> = HashMap::new();
    set_level_values(root, 0, &mut levels_values);

    let mut each_levels_values = vec![];

    for level in 0..levels_values.len() {
        each_levels_values.push(levels_values.get(&(level as i32)).unwrap().clone());
    }

    each_levels_values
}

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    get_levels_values(root)
        .into_iter()
        .map(|(level_sum, level_count)| level_sum / level_count as f64)
        .collect()
}
