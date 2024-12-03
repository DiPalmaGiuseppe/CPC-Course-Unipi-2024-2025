use std::{cmp, collections::HashSet};

pub struct MinMaxSegmentTree {
    tree: Vec<u32>,
    lazy: Vec<Option<u32>>,
    range_len: usize,
}

impl MinMaxSegmentTree {
    // Initialize the segment tree and build it from the input vector
    pub fn new(vec: &Vec<u32>) -> Self {
        let tree_len = vec.len() * 2 - 1;
        let lazy = vec![None; tree_len];
        let mut tree = vec![0; tree_len];

        Self::build_segment_tree(&mut tree, vec, 0, 0, vec.len() - 1);

        Self {
            tree,
            lazy,
            range_len: vec.len(),
        }
    }

    // Build the segment tree recursively to store the maximum value in each range
    fn build_segment_tree(
        tree: &mut Vec<u32>,
        vec: &Vec<u32>,
        index: usize,
        left_range: usize,
        right_range: usize,
    ) {
        if left_range == right_range {
            tree[index] = vec[left_range];
        } else {
            let mid = left_range + (right_range - left_range) / 2;
            let right_index = index + 2 * (mid - left_range + 1);
            let left_index = index + 1;

            Self::build_segment_tree(tree, vec, left_index, left_range, mid);
            Self::build_segment_tree(tree, vec, right_index, mid + 1, right_range);
            tree[index] = cmp::max(tree[left_index], tree[right_index]);
        }
    }

    // Public method to get the maximum value in a given range [i, j]
    pub fn max(&mut self, i: usize, j: usize) -> u32 {
        self.rec_max(0, 0, self.range_len - 1, i, j)
    }

    // Recursive function to retrieve the maximum value within range [i, j]
    fn rec_max(
        &mut self,
        index: usize,
        left_range: usize,
        right_range: usize,
        i: usize,
        j: usize,
    ) -> u32 {
        if i > j {
            return u32::MIN;
        }

        if i <= left_range && j >= right_range {
            return self.tree[index];
        }

        let mid = left_range + (right_range - left_range) / 2;
        let right_index = index + 2 * (mid - left_range + 1);
        let left_index = index + 1;

        self.push(index, mid, left_range);

        cmp::max(
            self.rec_max(left_index, left_range, mid, i, j.min(mid)),
            self.rec_max(right_index, mid + 1, right_range, i.max(mid + 1), j),
        )
    }

    // Public method to update range [i, j] with a minimum value t
    pub fn update(&mut self, i: usize, j: usize, t: u32) {
        self.rec_update(0, 0, self.range_len - 1, i, j, t)
    }

    // Recursive function to update a range with a minimum value, using lazy propagation
    fn rec_update(
        &mut self,
        index: usize,
        left_range: usize,
        right_range: usize,
        i: usize,
        j: usize,
        t: u32,
    ) {
        if i <= j {
            if i <= left_range && j >= right_range {
                self.tree[index] = self.tree[index].min(t);
                self.lazy[index] = self.lazy[index].map_or(Some(t), |x| Some(x.min(t)));
            } else {
                let mid = left_range + (right_range - left_range) / 2;
                let right_index = index + 2 * (mid - left_range + 1);
                let left_index = index + 1;

                self.push(index, mid, left_range);

                self.rec_update(left_index, left_range, mid, i, j.min(mid), t);
                self.rec_update(right_index, mid + 1, right_range, i.max(mid + 1), j, t);
                self.tree[index] = cmp::max(self.tree[left_index], self.tree[right_index]);
            }
        }
    }

    // Push lazy updates to children and reset the lazy value at the current node
    fn push(&mut self, index: usize, mid: usize, left_range: usize) {
        if let Some(lazy_value) = self.lazy[index] {
            let right_index = index + 2 * (mid - left_range + 1);
            let left_index = index + 1;

            self.tree[left_index] = self.tree[left_index].min(lazy_value);
            self.lazy[left_index] =
                self.lazy[left_index].map_or(Some(lazy_value), |x| Some(x.min(lazy_value)));

            self.tree[right_index] = self.tree[right_index].min(lazy_value);
            self.lazy[right_index] =
                self.lazy[right_index].map_or(Some(lazy_value), |x| Some(x.min(lazy_value)));

            self.lazy[index] = None;
        }
    }
}

pub struct IsThereSegmentTree {
    tree: Vec<HashSet<u32>>,
    range_len: usize,
}

impl IsThereSegmentTree {
    // Initialize the segment tree and build it from the input vector
    pub fn new(segments: &Vec<(u32, u32)>, num_segments: usize) -> Self {
        // create a vector of points with their value and their type
        let mut points: Vec<(u32, Point)> = Vec::new();
        for (start, end) in segments {
            points.push((*start, Point::Start));
            points.push((*end + 1, Point::End));
        }

        // sort the vector with counting_sort, exploiting the fact
        // the maximum value of end point is known
        points = counting_sort(points, num_segments);

        let mut values = vec![0; num_segments];
        let mut active_segments: u32 = 0;
        let mut current_position: u32 = 0;

        // apply the sweep line method to count the number of active segments at each position
        for (position, event) in points {
            while current_position < position {
                values[current_position as usize] = active_segments;
                current_position += 1;
            }

            match event {
                Point::Start => active_segments += 1,
                Point::End => active_segments -= 1,
            }
        }

        let tree_len = values.len() * 2 - 1;
        let mut tree = vec![HashSet::new(); tree_len];

        Self::build_segment_tree(&mut tree, &values, 0, 0, values.len() - 1);

        Self {
            tree,
            range_len: values.len(),
        }
    }

    // Build the segment tree to store sets of values for range queries
    fn build_segment_tree(
        tree: &mut Vec<HashSet<u32>>,
        vec: &Vec<u32>,
        index: usize,
        left_range: usize,
        right_range: usize,
    ) {
        if left_range == right_range {
            tree[index].insert(vec[left_range]);
        } else {
            let mid = left_range + (right_range - left_range) / 2;
            let right_index = index + 2 * (mid - left_range + 1);
            let left_index = index + 1;

            Self::build_segment_tree(tree, vec, left_index, left_range, mid);
            Self::build_segment_tree(tree, vec, right_index, mid + 1, right_range);
            tree[index] = tree[left_index]
                .union(&tree[right_index])
                .cloned()
                .collect();
        }
    }

    // Check if a value exists in the given range [i, j]
    pub fn is_there(&self, i: usize, j: usize, k: u32) -> bool {
        self.rec_is_there(0, 0, self.range_len - 1, i, j, k)
    }

    // Recursive function to check if a value is in the set of a given range
    fn rec_is_there(
        &self,
        index: usize,
        left_range: usize,
        right_range: usize,
        i: usize,
        j: usize,
        k: u32,
    ) -> bool {
        if i > j {
            return false;
        }

        if i <= left_range && j >= right_range {
            return self.tree[index].contains(&k);
        }

        let mid = left_range + (right_range - left_range) / 2;
        let right_index = index + 2 * (mid - left_range + 1);
        let left_index = index + 1;

        self.rec_is_there(left_index, left_range, mid, i, j.min(mid), k)
            || self.rec_is_there(right_index, mid + 1, right_range, i.max(mid + 1), j, k)
    }
}

pub enum Point {
    Start,
    End,
}

// Counting sort to order elements by position, ensuring 'Start' precedes 'End' for ties
pub fn counting_sort(arr: Vec<(u32, Point)>, max: usize) -> Vec<(u32, Point)> {
    let mut count = vec![(0, 0); max + 1];

    for (pos, t) in arr {
        match t {
            Point::Start => count[pos as usize].0 += 1,
            Point::End => count[pos as usize].1 += 1,
        };
    }

    let mut sorted = Vec::new();
    for (i, &(start_ct, end_ct)) in count.iter().enumerate() {
        for _ in 0..start_ct {
            sorted.push((i as u32, Point::Start));
        }
        for _ in 0..end_ct {
            sorted.push((i as u32, Point::End));
        }
    }

    sorted
}
