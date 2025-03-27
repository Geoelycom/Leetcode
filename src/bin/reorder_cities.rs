use std::collections::HashSet;
use std::collections::HashMap;

    pub fn min_reorder(_n: i32, connections: Vec<Vec<i32>>) -> i32 {    
        // Create adjacency list with edges in both directions
        let mut adj: HashMap<i32, Vec<(i32, bool)>> = HashMap::new();
        
        // Build the graph
        for conn in connections.iter() {
            let a = conn[0];
            let b = conn[1];
            
            // true means original edge direction
            adj.entry(a).or_insert(Vec::new()).push((b, true));
            
            // false means reverse edge direction
            adj.entry(b).or_insert(Vec::new()).push((a, false));
        }
        
        let mut visited = HashSet::new();
        
        // DFS function to count reorientations
        fn dfs(city: i32, adj: &HashMap<i32, Vec<(i32, bool)>>, visited: &mut HashSet<i32>) -> i32 {
            let mut count = 0;
            visited.insert(city);
            
            if let Some(neighbors) = adj.get(&city) {
                for &(next_city, is_original) in neighbors {
                    if !visited.contains(&next_city) {
                        if is_original {
                            count = count + 1;
                        }
                        count += dfs(next_city, adj, visited);
                    }
                }
            }
            count
        }
        
        dfs(0, &adj, &mut visited)
    }   

fn main() {

}