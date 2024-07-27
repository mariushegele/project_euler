type NodeIndex = usize;
type RowIndex = usize;
type NodeWeight = u64;

pub struct Triangle {
    nodes: Vec<NodeWeight>,
    row_indices: Vec<RowIndex>,
    // neighbours: Vec<Vec<usize>>,
}

impl Triangle {
    pub fn from_string(triangle: String) -> Self {
        let triangle_fmt = triangle.trim().replace("\n", " ");
        let nodes: Vec<u64> = triangle_fmt.split(' ').map(|n| n.parse().unwrap()).collect();

        Self::new(nodes)
    }

    pub fn new(nodes: Vec<NodeWeight>) -> Self {
        let mut row_indices: Vec<RowIndex> = vec![0; nodes.len()];
        let mut row_index: RowIndex = 0;
        let mut node: NodeIndex = 0;
        while node < nodes.len() {
            let num_els_in_row = row_index + 1;
            for _ in 0..num_els_in_row {
                row_indices[node] = row_index;
                node += 1;
            }

            row_index += 1;
        }
        Self {
            nodes,
            row_indices
        }
    }

    pub fn into_dag(self) -> DAG {
        let mut children: Vec<Vec<NodeIndex>> = vec![vec![]; self.nodes.len()];
        for node_index in 0..self.nodes.len() {
            for child in &self.children(node_index) {
                children[node_index].push(*child);
            }
        }
        DAG::new(self.nodes, children)
    }

    fn children(&self, node: NodeIndex) -> Vec<NodeIndex> {
        let row_index = self.row_indices[node];
        (node + row_index + 1..=(node + row_index + 2).min(self.nodes.len() - 1)).collect()
    }

}


pub struct DAG {
    nodes: Vec<NodeWeight>,
    children: Vec<Vec<NodeIndex>>,

    // invariant: child > node for every child in children[node]
}
    
impl DAG {
    pub fn new(nodes: Vec<NodeWeight>, children: Vec<Vec<NodeIndex>>) -> Self {
        for node in 0..nodes.len() {
            for child in &children[node] {
                assert!(*child > node, "Graph is not a directed acyclic graph: child {child} of {node} is larger");
            }
        }

        Self {
            nodes,
            children,
        }
    }


    pub fn reversed(&self) -> Self {
        let mut reversed_nodes = self.nodes.clone();
        reversed_nodes.reverse();

        let mut reversed_children: Vec<Vec<NodeIndex>> = vec![vec![]; self.nodes.len()];
        for node in 0..self.nodes.len() {
            let node_index_in_reversed_graph: NodeIndex = self.nodes.len() - 1 - node;
            for child in &self.children[node] {
                let child_index_in_reversed_graph: NodeIndex = self.nodes.len() - 1 - *child;
                reversed_children[child_index_in_reversed_graph].push(node_index_in_reversed_graph);
            }
        }

        Self::new(reversed_nodes, reversed_children)
    }

    pub fn max_path_sum(&self) -> u64 {
        let reversed_dag = self.reversed();
        let mut max_path_sums: Vec<NodeWeight> = vec![0; self.nodes.len()];

        for rnode in 0..reversed_dag.nodes.len() {
            let node_weight = reversed_dag.nodes[rnode];
            for rchild in &reversed_dag.children[rnode] {
                max_path_sums[*rchild] = max_path_sums[*rchild].max(node_weight + max_path_sums[rnode]);
            }
        }

        *max_path_sums.last().unwrap() + *self.nodes.first().unwrap()
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    fn get_triangle() -> Triangle {
        let triangle = "
3
7 4
2 4 6
8 5 9 3
".to_string();

        Triangle::from_string(triangle)
    }

    #[test]
    fn test_row_indices() {
        let triangle = get_triangle();
        assert_eq!(triangle.row_indices, vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3]);
    }

    #[test]
    fn test_triangle_neighbours() {
        let triangle = get_triangle();

        // g  r ri     c1  c2
        // 0  0  0 -> (1, 2)
        // 1  1  0 -> (3, 4)
        // 2  1  1 -> (4, 5)
        // 3  2  0 -> (6, 7)
        // 4  2  1 -> (7, 8)
        // 5  2  2 -> (8, 9)


/* 
   0
  1 2
 3 4 5
6 7 8 9
*/

        assert_eq!(triangle.children(0), vec![1, 2]);
        assert_eq!(triangle.children(1), vec![3, 4]);
        assert_eq!(triangle.children(2), vec![4, 5]);
        assert_eq!(triangle.children(3), vec![6, 7]);
        assert_eq!(triangle.children(4), vec![7, 8]);
        assert_eq!(triangle.children(5), vec![8, 9]);
        assert_eq!(triangle.children(6), vec![]);
        assert_eq!(triangle.children(7), vec![]);
        assert_eq!(triangle.children(8), vec![]);
        assert_eq!(triangle.children(9), vec![]);

    }

    #[test]
    fn test_max_path_sum() {
        let triangle = get_triangle();
        let dag = triangle.into_dag();
        assert_eq!(dag.max_path_sum(), 23);
    }

}