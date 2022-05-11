use multimap::MultiMap;

pub fn dijkstra<'a>(
    graph: MultiMap<&'a str, (&'a str, usize)>,
    costs: MultiMap<&'a str, usize>,
    mut parents: MultiMap<&'a str, &'a str>,
) -> Vec<&'a str> {
    println!("graph: {:?}", graph);
    println!("costs: {:?}", costs);
    let mut processed: Vec<&str> = vec![];
    let mut node = find_lowest_cost_node(costs.clone());
    let mut cnt = 1;

    while node != "none" {
        println!("\n\nstep: {}", cnt);
        println!("node: {}", node);
        let cost = costs.get(node).unwrap();
        println!("cost: {}", cost);
        let neighbors: &Vec<(&str, usize)> = graph.get_vec(node).unwrap();
        println!("naver: {:?}", neighbors);
        let neighbors_keys: Vec<usize> = neighbors.iter().map(|x: &(&str, usize)| x.1).collect();
        println!("naverkey: {:?}", neighbors_keys);
        let mut costs_vec: Vec<usize> = costs.iter().map(|x| *x.1).collect();
        for i in 0..neighbors_keys.len() {
            println!("{}", i);
            println!("in for: costs_vec: {:?}", costs_vec);
            let current_nodes: Vec<&str> = costs.iter().map(|x| *x.0).collect();
            println!("in for: current_nodes: {:?}", current_nodes);
            let new_cost = cost + neighbors_keys[i];
            println!("in for: new_cost: {}", new_cost);
            println!("in for: costs_vec[{}]: {}", i, costs_vec[i]);
            if costs_vec[i] > new_cost {
                costs_vec[i] = new_cost;
                parents.insert(current_nodes[i], node);
            }
            println!("new costs_vec: {:?}", costs_vec);
        }
        processed.push(node);

        node = find_lowest_cost_node(costs.clone());
        println!("next node: {}", node);
        cnt += 1;
        if cnt >= 10 {
            break;
        }
    }
    processed
}

fn find_lowest_cost_node(costs: MultiMap<&str, usize>) -> &str {
    let mut lowest_cost: usize = 100_000_000; //Placeholder large value
    let mut lowest_cost_node = "none";
    let processed: Vec<&str> = vec![];

    for node in &costs {
        let cost = costs.get(node.0).unwrap();
        if cost < &lowest_cost && !processed.contains(&node.0) {
            lowest_cost = *cost;
            lowest_cost_node = node.0;
        }
    }
    lowest_cost_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut graph: MultiMap<&str, (&str, usize)> = MultiMap::new();
        graph.insert("start", ("a", 2));
        graph.insert("start", ("b", 6));
        graph.insert("b", ("a", 3));
        graph.insert("a", ("end", 1));
        graph.insert("b", ("end", 5));

        let mut costs: MultiMap<&str, usize> = MultiMap::new();
        costs.insert("a", 6);
        costs.insert("b", 2);
        costs.insert("end", 100_000_000);

        let mut parents: MultiMap<&str, &str> = MultiMap::new();
        parents.insert("a", "start");
        parents.insert("b", "start");
        parents.insert("end", "none");

        let answer = dijkstra(graph, costs, parents);
        let expected: Vec<&str> = vec![];
        assert_eq!(answer, expected);
    }
}
