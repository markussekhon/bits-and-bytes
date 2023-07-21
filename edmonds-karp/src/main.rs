use std::cmp::min;
use std::collections::VecDeque;

fn bfs(residual_graph: &Vec<Vec<i32>>, parent: &mut Vec<usize>, source: usize, sink: usize) -> bool {
    let size = residual_graph.len();
    let mut visited = vec![false; size];

    let mut queue = VecDeque::new();
    queue.push_back(source);
    visited[source] = true;

    while let Some(current_node) = queue.pop_front() {
        for ind in 0..size {
            if residual_graph[current_node][ind] > 0 && !visited[ind] {
                queue.push_back(ind);
                visited[ind] = true;
                parent[ind] = current_node;
                if ind == sink {
                    return true;
                }
            }
        }
    }
    false
}

fn edmonds_karp(mut graph: Vec<Vec<i32>>, source: usize, sink: usize) -> i32 {
    let size = graph.len();
    let mut parent = vec![0; size];
    let mut max_flow = 0;

    while bfs(&graph, &mut parent, source, sink) {
        let mut path_flow = i32::MAX;
        let mut s = sink;
        let mut path = Vec::new();
        while s != source {
            path_flow = min(path_flow, graph[parent[s]][s]);
            s = parent[s];
            path.push(s);
        }

        path.reverse();
        println!("Path found: {:?}", path);

        max_flow += path_flow;
        println!("Flow of the path: {}", path_flow);

        let mut v = sink;
        while v != source {
            let u = parent[v];
            graph[u][v] -= path_flow;
            graph[v][u] += path_flow;
            v = parent[v];
        }

        println!("Updated matrix: {:?}", graph);
    }

    max_flow
}

fn main() {
    let graph = vec![
        vec![0, 16, 13, 0, 0, 0], 
        vec![0, 0, 10, 12, 0, 0],
        vec![0, 4, 0, 0, 14, 0],
        vec![0, 0, 9, 0, 0, 20],
        vec![0, 0, 0, 7, 0, 4],
        vec![0, 0, 0, 0, 0, 0]
    ];

    println!("The maximum possible flow is {}", edmonds_karp(graph, 0, 5));
}
