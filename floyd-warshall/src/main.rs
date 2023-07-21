fn floyd_warshall(dist: &mut Vec<Vec<i32>>) {
    let n = dist.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] == std::i32::MAX || dist[k][j] == std::i32::MAX {
                    // Skip if there's no valid path.
                    continue;
                }

                // If the current distance from i to j is greater than the distance from i to k plus the distance from k to j,
                // then update the distance from i to j.
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
        // Print the distance matrix at the end of each iteration over k.
        println!("After iteration {}: ", k + 1);
        for row in dist.iter() {
            println!("{:?}", row);
        }
        println!("");

    }
}

fn main() {
    let mut dist = vec![
        vec![0, 3, 8, std::i32::MAX, -4],
        vec![std::i32::MAX, 0, std::i32::MAX, 1, 7],
        vec![std::i32::MAX, 4, 0, std::i32::MAX, std::i32::MAX],
        vec![2, std::i32::MAX, -5, 0, std::i32::MAX],
        vec![std::i32::MAX, std::i32::MAX, std::i32::MAX, 6, 0]
    ];

    floyd_warshall(&mut dist);

    // Print the final distance matrix.
    for row in &dist {
        println!("{:?}", row);
    }
}
