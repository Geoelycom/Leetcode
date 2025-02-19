// Understanding And Implementing Graphs in Rust

// Graphs are a collection of nodes and edges. The nodes are sometimes called vertices. The edges are the lines that connect the nodes. Graphs can be directed or undirected. In a directed graph, the edges have a direction. In an undirected graph, the edges do not have a direction. Graphs can be cyclic or acyclic. In a cyclic graph, there is a path from a node back to itself. In an acyclic graph, there is no path from a node back to itself.

// Examples code
// In the following code, we create a graph with three nodes and two edges. The graph is undirected and acyclic. The graph is represented as an adjacency list. The adjacency list is a map where the keys are the nodes and the values are the nodes that are connected to the key node.

use std::collections::HashMap; // Import the HashMap type from the collections module

fn main() { // Define the main function
    let mut graph = HashMap::new(); // Create a new empty HashMap called graph
   graph.insert(1, vec![2]); // Insert the key 1 with the value [2] into the graph
   graph.insert(2, vec![1, 3]); // Insert the key 2 with the value [1, 3] into the graph
   graph.insert(3, vec![2]); // Insert the key 3 with the value [2] into the graph
   println!("{:?}", graph); // Print the graph
}
// The output of the code is:
// {1: [2], 2: [1, 3], 3: [2]}

// In the following code, we create a graph with three nodes and two edges. The graph is directed and cyclic. The graph is represented as an adjacency list.
// The adjacency list is a map where the keys are the nodes and the values are the nodes that are connected to the key node.
// The graph is cyclic because there is a path from node 1 back to itself.
// The graph is directed because the edges have a direction.
// The graph is represented as an adjacency list where the keys are the nodes and the values are the nodes that are connected to the key node.
// The graph is cyclic because there is a path from node 1 back to itself.

// EXPLAINING EDGES
// For edge 1-2:
graph.entry(1).or_insert(Vec::new()).push(2);  // 1 can reach 2
graph.entry(2).or_insert(Vec::new()).push(1);  // 2 can reach 1

// For edge 1-3:
graph.entry(1).or_insert(Vec::new()).push(3);  // 1 can reach 3
graph.entry(3).or_insert(Vec::new()).push(1);  // 3 can reach 1

// For edge 2-4:
graph.entry(2).or_insert(Vec::new()).push(4);  // 2 can reach 4
graph.entry(4).or_insert(Vec::new()).push(2);  // 4 can reach 2



// GRAPHS EXAMPLES AND CODES

// Undirected Graph (two-way connections, like Facebook friends)
fn create_undirected_graph() {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // Let's add edges: 1-2, 1-3, 2-4
    // When we add 1-2, we also add 2-1 (that's what makes it undirected)
    
    // Add edge 1-2
    graph.entry(1).or_insert(Vec::new()).push(2);
    graph.entry(2).or_insert(Vec::new()).push(1);  // Two-way connection!
    
    // Add edge 1-3
    graph.entry(1).or_insert(Vec::new()).push(3);
    graph.entry(3).or_insert(Vec::new()).push(1);  // Two-way connection!
    
    // Add edge 2-4
    graph.entry(2).or_insert(Vec::new()).push(4);
    graph.entry(4).or_insert(Vec::new()).push(2);  // Two-way connection!
}

// Directed Graph (one-way connections, like Twitter follows)
fn create_directed_graph() {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // Let's add edges: 1->2, 1->3, 2->4
    // When we add 1->2, we DON'T add 2->1 (that's what makes it directed)
    
    // Add edge 1->2
    graph.entry(1).or_insert(Vec::new()).push(2);
    
    // Add edge 1->3
    graph.entry(1).or_insert(Vec::new()).push(3);
    
    // Add edge 2->4
    graph.entry(2).or_insert(Vec::new()).push(4);
}

// How to use Different Graphs
fn main() {
  // Undirected Graph (like Facebook)
  let mut facebook_graph: HashMap<i32, Vec<i32>> = HashMap::new();
  
  // If 1 and 2 are friends, they both connect to each other
  facebook_graph.entry(1).or_insert(Vec::new()).push(2);
  facebook_graph.entry(2).or_insert(Vec::new()).push(1);
  
  // Directed Graph (like Twitter)
  let mut twitter_graph: HashMap<i32, Vec<i32>> = HashMap::new();
  
  // If 1 follows 2, only 1 points to 2
  twitter_graph.entry(1).or_insert(Vec::new()).push(2);
  // 2 doesn't automatically follow 1 back!
}

// CHECK IF NODES ARE CONNECTED
fn is_connected(graph: &HashMap<i32, Vec<i32>>, from: i32, to: i32) -> bool {
  if let Some(neighbors) = graph.get(&from) {
      neighbors.contains(&to)
  } else {
      false
  }
}

// Example usage:
// In undirected graph (Facebook):
// is_connected(facebook_graph, 1, 2) == true
// is_connected(facebook_graph, 2, 1) == true

// In directed graph (Twitter):
// is_connected(twitter_graph, 1, 2) == true
// is_connected(twitter_graph, 2, 1) == false


// 1.0 Function adding a new edge in an undirected graph
fn add_undirected_edge(graph: &mut Hashmap<i32, Vec<i32>>, from: i32, to: i32) {
  graph.entry(from).or_insert(Vec::new().push(to));
  graph.entry(to).or_insert(Vec::new().push(from));
}

// 1.1 Function adding a new edge in a directed graph
fn add_directed_edge(graph: &mut Hashmap<i32, Vec<i32>>, from: i32, to: i32){
  graph.entry(from).or_insert(Vec::new().push(to));
}

// 2. Finding Neighbors(nodes directly connected to a node)
fn get_neighbors(graph: &HashMap<i32, Vec<i32>>, node: i32) -> Vec<i32> {
  graph.get(&node).cloned().unwrap_or_vec![]
}

// 3. Checking if Edge Exists
fn has_edge(graph: &HashMap<i32, Vec<i32>>, from: i32, to: i32) -> bool {
  if let Some(neighbors) = graph.get(&from) {
    neighbors.contains(&to)
  } else {
    false
  }
}

// 4. Remove Edges
fn remove_undirected_edge(graph: &mut HashMap<i32, Vec<i32>>, from: i32, to: i32 ) {
  if let Some(neighbors) = graph.get_mut(&from) {
    edges.retain(|&x| x != to);
  }
  if let Some(neighbors) = graph.get_mut(&to) {
    edges.retain(|&x| x != from);
  }
}

// 5. Count Number of Edges
fn count_edges(graph: &HashMap<i32, Vec<i32>>, directed: bool) -> i32 {
  let total = graph.values().map(|edges| edges.len()).sum::<usize>();
  if directed{
    total as i32
  } else {
    (total / 2) as i32 // Each edge is counted twice in undirected graph
  }
}

// 6. Find Degree of a Node
fn get_degree(graph: &HashMap<i32, Vec<i32>>, node: i32) -> i32 {
  graph.get(&node).map_or(0, |edges| edges.len() as i32)
}

// 7.  Find Isolated Nodes (nodes with no connections):
fn find_isolated_nodes(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
  graph.iter()
      .filter(|(_, edges)| edges.is_empty())
      .map(|(&node, _)| node)
      .collect()
}

