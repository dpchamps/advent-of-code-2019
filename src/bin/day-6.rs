use petgraph::{Graph, graph::NodeIndex};
use adventofcode::read_input_file;
use std::collections::{HashMap, HashSet};
use adventofcode::int_code_computer::Opcode::ProgramEnd;
use std::ops::Index;
use petgraph::visit::NodeIndexable;
use petgraph::graph::Node;
use std::hash::Hash;

type OrbitData = (String, String);


fn count_orbits(graph : &Graph<String, String>, node_index : NodeIndex) -> i32{
    let mut count = 0;
    let mut idx = node_index;

    loop {
        let neighbors : Vec<NodeIndex> = graph.neighbors(idx).collect();
        if neighbors.len() > 1{
            panic!("Something terrible happened");
        }
        if neighbors.len() == 0{
            break;
        }

        count += 1;
        idx = neighbors[0];
    }

    count
}

fn sum_orbits(graph : &Graph<String, String>) -> i32{
    let mut orbits = 0;

    for node_index in graph.node_indices(){
        let o = count_orbits(&graph, node_index);

        orbits += o;
    }

    orbits
}

fn orbits_between(graph : &Graph<String, String>, a : NodeIndex, b : NodeIndex) -> i32 {
    let mut a_idx =  graph.neighbors(a).next().unwrap();
    let mut b_idx = graph.neighbors(b).next().unwrap();

    let mut transfers_between = 0;

    let mut a_steps = 0;
    let mut b_steps = 0;

    let mut map : HashMap<NodeIndex, i32> = HashMap::new();



    loop {
        let mut was_move = false;

        if let Some(a_parent) = graph.neighbors(a_idx).next() {
            a_steps += 1;
            was_move = true;
            a_idx = a_parent;

            if let Some(steps) = map.get(&a_parent){
                transfers_between = steps + a_steps;
                break;
            }else{
                map.insert(a_parent, a_steps);
            }
        }

        if let Some(b_parent) = graph.neighbors(b_idx).next(){
            b_steps += 1;
            was_move = true;
            b_idx = b_parent;

            if let Some(steps) = map.get(&b_parent){
                transfers_between = steps + b_steps;
                break;
            }else{
                map.insert(b_parent, b_steps);
            }
        }

        if !was_move {
            panic!("Couldn't help ya :/");
        }
    };

    transfers_between
}

fn find_node(graph : &Graph<String, String>, weight : &str) -> Result<NodeIndex, &'static str>{
    for index in graph.node_indices(){
        let node = graph.index(index);

        if node == weight {
            return Ok(index)
        }
    }

    Err("No node found")
}

fn build_graph(input : String) -> Graph<String, String> {
    let mut edge_map : HashMap<String, NodeIndex> = HashMap::new();
    let mut graph: Graph<String, String> = Graph::new();

    let orbit_list : Vec<OrbitData> = input.split("\n")
        .map(|x|{
            let orbit_data : Vec<&str> = x.split(")").collect();

            (orbit_data[0].to_string(), orbit_data[1].to_string())
        }).collect();

    for (subject, orbiter) in orbit_list.iter(){
        let orbiter_node = if let Some(existing_orbiter_node) = edge_map.get(orbiter){
            *existing_orbiter_node
        }else{
            let idx = graph.add_node(orbiter.to_string());
            edge_map.insert(orbiter.clone(), idx);

            idx
        };

        let subject_node = if let Some(existing_subject_node) = edge_map.get(subject){
            *existing_subject_node
        }else{
            let idx = graph.add_node(subject.to_string());
            edge_map.insert(subject.clone(), idx);

            idx
        };


        graph.add_edge(orbiter_node, subject_node, Default::default());
    }

    graph
}

fn part_1(input : String){
    let graph = build_graph(input);
    let orbits = sum_orbits(&graph);

    println!("Total direct and indirect orbits : {}", orbits)

}

fn part_2(input : String){
    let graph = build_graph(input);

    let santa = find_node(&graph, "SAN").unwrap();
    let you = find_node(&graph, "YOU").unwrap();
    let distance = orbits_between(&graph, you, santa);

    println!("Distance between you and santa: {}", distance)
}

fn main(){
    let input = read_input_file("day-6-part-1-input");

    part_1(input.clone());
    part_2(input.clone());
}


#[cfg(test)]
mod day_6_tests{
    use crate::*;

    #[test]
    fn count(){
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".to_string();
        let graph = build_graph(input);
        let orbits = sum_orbits(&graph);

        assert_eq!(orbits, 42);
    }

    #[test]
    fn distance(){
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN".to_string();
        let graph = build_graph(input);
        let santa = find_node(&graph, "SAN").unwrap();
        let you = find_node(&graph, "YOU").unwrap();
        let distance = orbits_between(&graph, you, santa);

        assert_eq!(distance, 4);
    }
}