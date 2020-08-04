
#[allow(non_camel_case_types)]
#[derive(Debug)]

pub struct node {
    value: isize,
    left: Option<Box<node>>,
    right: Option<Box<node>>,
}

impl node{
    ///
    /// Initialize a tree
    /// 
    pub fn initialise(val: isize) -> node {
        return node{
            value: val,
            left: None,
            right: None,
        }
    }

    ///
    /// Adds a new `input` to the existing binary tree in O(log(n)).
    ///
    pub fn insert(&mut self, input: isize) {
        //
        // `Greater than or equals` numbers of the right side
        //
        if input >= self.value {
            match self.right {
                //
                // Insert if there's no node to the right of current node
                //  
                None => self.right = Some(Box::new(
                    node{
                        value: input,
                        left: None,
                        right: None,
                    }
                )),
                //
                // If we have an existing right node, call `insert` on that node
                //
                Some(ref mut next) => next.insert(input)
            }
        }
        else {
            match self.left {
                //
                // Insert if there's no node to the left of current node
                //  
                None => self.left = Some(Box::new(
                    node{
                        value: input,
                        left: None,
                        right: None,
                    }
                )),
                //
                // If we have an existing left node, call `insert` on that node
                //
                Some(ref mut next) => next.insert(input)
            }
        }
    }

    ///
    /// Find k-th largest element given a binary search tree
    /// 
    pub fn find_kth_largest(&self, k: usize) -> isize {
        return self.descending()[k-1]
    }

    ///
    /// Sorts values in a binary tree in descending order
    /// 
    pub fn descending(&self) -> Vec<isize> {

        let mut values: Vec<isize> = Vec::new();

        if let Some(ref right) = self.right {
            let right_vec = right.descending();
            values.extend(right_vec);
        }
        values.push(self.value);
        if let Some(ref left) = self.left {
            let left_vec = left.descending();
            values.extend(left_vec);
        }
        values
    }

    ///
    /// Sorts values in a binary tree in ascending order
    /// 
    pub fn ascending(&self) -> Vec<isize> {

        let mut values: Vec<isize> = Vec::new();

        if let Some(ref left) = self.left {
            let left_vec = left.ascending();
            values.extend(left_vec);
        }
        values.push(self.value);
        if let Some(ref right) = self.right {
            let right_vec = right.ascending();
            values.extend(right_vec);
        }
        
        values
    }
}

mod test {
    use super::node;

    #[test]
    pub fn construct_bst() {
        //         8
        //        / \
        //       /   \
        //      /     \
        //     3       10
        //    / \       \
        //   1   6       14 
        //      / \     /  \
        //     4   7   13  16
        let mut sample_tree: node = node::initialise(8);
        sample_tree.insert(3);
        sample_tree.insert(10);
        sample_tree.insert(1);
        sample_tree.insert(6);
        sample_tree.insert(14);
        sample_tree.insert(13);
        sample_tree.insert(4);
        sample_tree.insert(7);
        sample_tree.insert(16);

        println!("sample tree d: {:?}", sample_tree.descending());
        println!("sample tree a: {:?}", sample_tree.ascending());

        println!("3rd: {:?}", sample_tree.find_kth_largest(3));
        println!("8th: {:?}", sample_tree.find_kth_largest(8));
    }
}
