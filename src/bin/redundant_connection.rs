// In this problem, a tree is an undirected graph that is connected and has no cycles.
// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.

// Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
  pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    // we build the graph
    for edge in edges.iter() {
      let a = edge[0];
      let b = edge[1];
      adj.entry(a).or_insert(Vec::new()).push(b);
      adj.entry(b).or_insert(Vec::new()).push(a);
    }
    let mut visited = HashSet::new();

    fn dfs(curr: i32, adj: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> i32 {
        if visited.contains(&curr) {
            return 0
        }

      visited.insert(curr);
      let mut count = 1; // count current node

      if let Some(neighbors) = adj.get(&curr) {
        for &next in neighbors {
          count = count + dfs(next, adj, visited)
        }
      }
      count
    }

    // We remove edges from last to first
    for edge in edges.iter().rev() {
      let a = edge[0];
      let b = edge[1];

      // Remove edge temporarily
      adj.get_mut(&a).unwrap().retain(|&x| x != b);
      adj.get_mut(&b).unwrap().retain(|&x| x != a);

      visited.clear();
      // We check if we can still visit n nodes after removing edge
      if dfs(1, &adj, &mut visited) == edges.len() as i32 {
        return edge.clone();
        break;
      }
      // add edge back
      adj.get_mut(&a).unwrap().push(b);
      adj.get_mut(&b).unwrap().push(a);
    }
    vec![]
  } 
}