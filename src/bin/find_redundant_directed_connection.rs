// In this problem, a rooted tree is a directed graph such that, there is exactly one node (the root) for which all other nodes are descendants of this node, plus every node has exactly one parent, except for the root node which has no parents.

// The given input is a directed graph that started as a rooted tree with n nodes (with distinct values from 1 to n), with one additional directed edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed.

// The resulting graph is given as a 2D-array of edges. Each element of edges is a pair [ui, vi] that represents a directed edge connecting nodes ui and vi, where ui is a parent of child vi.

// Return an edge that can be removed so that the resulting graph is a rooted tree of n nodes. If there are multiple answers, return the answer that occurs last in the given 2D-array.

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![-1; n + 1];
        let mut candidate1 = vec![-1; 2];
        let mut candidate2 = vec![-1; 2];
        
        // Create a mutable copy of edges
        let mut edge_copy = edges.clone();
        
        // First pass: find nodes with two parents
        for (i, edge) in edges.iter().enumerate() {
            let u = edge[0];
            let v = edge[1];
            
            if parent[v as usize] == -1 {
                parent[v as usize] = u;
            } else {
                candidate1 = vec![parent[v as usize], v];
                candidate2 = vec![u, v];
                // Remove the second edge temporarily in our copy
                edge_copy[i][0] = -1;
                edge_copy[i][1] = -1;
            }
        }
        
        // Reset parent array for UnionFind
        parent = (0..=n as i32).collect();
        
        // Function to find root with path compression
        fn find(x: i32, parent: &mut Vec<i32>) -> i32 {
            if parent[x as usize] != x {
                parent[x as usize] = find(parent[x as usize], parent);
            }
            parent[x as usize]
        }
        
        // Check if remaining edges form a valid tree
        for edge in edge_copy.iter() {
            if edge[0] == -1 {
                continue;
            }
            
            let u = edge[0];
            let v = edge[1];
            
            let pu = find(u, &mut parent);
            let pv = find(v, &mut parent);
            
            if pu == pv {
                if candidate1[0] == -1 {
                    return vec![u, v];
                }
                return candidate1;
            }
            parent[pv as usize] = pu;
        }
        
        candidate2
    }
}