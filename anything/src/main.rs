mod dijkstra;

use crate::dijkstra::dijkstra;
use multimap::MultiMap;
fn main() {
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
    println!("{:?}", answer);
}
