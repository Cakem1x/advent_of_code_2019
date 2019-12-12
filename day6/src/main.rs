use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("loading relation string from file.");
    let input_string = read_to_string("input.txt").unwrap();
    let relations = string_to_relations(&input_string[..input_string.len()-1]);
    let graph = generate_graph_from_relations(relations);

    let you_orbit_object_to_com = path_to_com(&graph, graph.get("YOU").unwrap());
    let santa_orbit_object_to_com = path_to_com(&graph, graph.get("SAN").unwrap());
    println!("Path YOU to COM: {:?}", you_orbit_object_to_com);
    println!("Path SAN to COM: {:?}", santa_orbit_object_to_com);

    let mut closest_common_object = "COM";
    let mut dist_along_you_orbit = you_orbit_object_to_com.len() - 1;
    for (n, object_name) in you_orbit_object_to_com.iter().enumerate() {
        if santa_orbit_object_to_com.contains(object_name) {
            println!("Found common object: {}. Nr transfers necessary to reach it: {}", object_name, n);
            closest_common_object = object_name;
            dist_along_you_orbit = n;
            break;
        }
    }
    let mut dist_along_santa_orbit = 0;
    for object_name in santa_orbit_object_to_com.iter() {
        if object_name == &closest_common_object {
            break;
        }
        dist_along_santa_orbit += 1;
    }
    println!("distance along you path: {}\ndistance along santa path: {}\nnumber of transfers needed: {}", dist_along_you_orbit, dist_along_santa_orbit, dist_along_you_orbit+dist_along_santa_orbit);
}

fn string_to_relations(relations_string: &str) -> Vec<(&str, &str)> {
    let mut relations = Vec::new();
    for relation_string in relations_string.split('\n') {
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
    //let mut orbit_count = 0;
    //let mut result = graph.get(object_name);
    //while result.is_some() {
    //    let object_name = result.unwrap();
    //    result = graph.get(object_name);
    //    orbit_count += 1;
    //}
    //return orbit_count;
    return path_to_com(&graph, object_name).len() - 1;
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

fn path_to_com<'a>(graph: &'a Graph, object_name: &'a str) -> Vec<&'a str> {
    let mut path = [object_name].to_vec();
    let mut result = graph.get(object_name);
    while result.is_some() {
        let object_name = result.unwrap();
        path.push(object_name);
        result = graph.get(object_name);
    }
    return path;
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
