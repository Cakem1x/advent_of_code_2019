use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("loading relation string from file.");
    let input_string = read_to_string("input.txt").unwrap();
    let relations = string_to_relations(&input_string[..input_string.len()-1]);
    let graph = generate_graph_from_relations(relations);
    println!("Total number of orbits: {}", count_orbits(&graph));
}

fn string_to_relations(relations_string: &str) -> Vec<(&str, &str)> {
    let mut relations = Vec::new();
    for relation_string in relations_string.split('\n') {
        println!("Got relation_string {:?}", relation_string);
        let relation: Vec<&str> = relation_string.split(')').collect();
        assert_eq!(relation.len(), 2);
        relations.push((relation[0], relation[1]));
    }
    return relations;
}

/// graph['A'] = ['B'] means that 'A' orbits around 'B'.
type Graph = HashMap<String, String>;

fn generate_graph_from_relations(relations: Vec<(&str, &str)>) -> Graph {
    let mut graph = Graph::new();
    for relation in relations {
        graph.insert(String::from(relation.1), String::from(relation.0));
    }
    return graph;
}

fn count_orbits_of_object(graph: &Graph, object_name: &str) -> usize {
    let mut orbit_count = 0;
    let mut result = graph.get(object_name);
    while result.is_some() {
        let object_name = result.unwrap();
        result = graph.get(object_name);
        orbit_count += 1;
    }
    return orbit_count;
}

fn count_orbits(graph: &Graph) -> usize {
    let mut orbit_count = 0;
    for key in graph.keys() {
        let n = count_orbits_of_object(&graph, key);
        println!("{} orbits around {} objects.", key, n);
        orbit_count += n;
    }
    return orbit_count;
}

#[test]
fn string_to_relations_creates_correct_relations_vector() {
    let input_string = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let relations = string_to_relations(&input_string);
    println!("{:?}", relations);
    assert_eq!(relations.len(), 11);
    assert_eq!(relations[0].0, "COM");
    assert_eq!(relations[0].1, "B");
    assert_eq!(relations[1].0, "B");
    assert_eq!(relations[1].1, "C");
    assert_eq!(relations[2].0, "C");
    assert_eq!(relations[2].1, "D");
    assert_eq!(relations[3].0, "D");
    assert_eq!(relations[3].1, "E");
    assert_eq!(relations[4].0, "E");
    assert_eq!(relations[4].1, "F");
    assert_eq!(relations[5].0, "B");
    assert_eq!(relations[5].1, "G");
    assert_eq!(relations[6].0, "G");
    assert_eq!(relations[6].1, "H");
    assert_eq!(relations[7].0, "D");
    assert_eq!(relations[7].1, "I");
    assert_eq!(relations[8].0, "E");
    assert_eq!(relations[8].1, "J");
    assert_eq!(relations[9].0, "J");
    assert_eq!(relations[9].1, "K");
    assert_eq!(relations[10].0, "K");
    assert_eq!(relations[10].1, "L");
}

#[test]
fn generated_graph_contains_all_nodes() {
    let input_string = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let relations = string_to_relations(&input_string);
    let graph = generate_graph_from_relations(relations);
    assert_eq!(graph.len(), 11);
}

#[test]
fn generated_graph_expresses_orbits() {
    let input_string = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let relations = string_to_relations(&input_string);
    let graph = generate_graph_from_relations(relations);
    assert_eq!(graph.get("COM"), None);
    assert_eq!(graph.get("B").unwrap(), "COM");
    assert_eq!(graph.get("K").unwrap(), "J");
    assert_eq!(graph.get("L").unwrap(), "K");
}

#[test]
fn count_orbits_of_object_returns_correct_value() {
    let input_string = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let relations = string_to_relations(&input_string);
    let graph = generate_graph_from_relations(relations);
    assert_eq!(count_orbits_of_object(&graph, "COM"), 0);
    assert_eq!(count_orbits_of_object(&graph, "D"), 3);
    assert_eq!(count_orbits_of_object(&graph, "L"), 7);
}

#[test]
fn count_orbits_returns_correct_value() {
    let input_string = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let relations = string_to_relations(&input_string);
    let graph = generate_graph_from_relations(relations);
    assert_eq!(count_orbits(&graph), 42);
}
