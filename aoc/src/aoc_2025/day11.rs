use std::{collections::HashMap, rc::Rc};

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Clone)]
struct NodeId(String);

#[derive(Debug)]
struct NodeInfo {
    node: NodeId,
    connections: Vec<NodeId>,
}

fn parse_line(l: &str) -> NodeInfo {
    let [node_name, connections_part, ..] = l.split(':').collect_vec()[..] else {
        unreachable!();
    };
    let connections = connections_part
        .split_whitespace()
        .map(|i| NodeId(i.to_string()))
        .collect_vec();
    NodeInfo {
        node: NodeId(node_name.to_string()),
        connections,
    }
}

fn part_a(input: &Input) -> u64 {
    let lines = input.lines().map(|i| parse_line(&i));
    let map: HashMap<NodeId, Vec<NodeId>> =
        lines.fold(HashMap::<NodeId, Vec<NodeId>>::new(), |mut acc, node| {
            for c in node.connections {
                let ent = acc.entry(c).or_default();
                ent.push(node.node.clone());
            }
            acc
        });
    num_paths(&map, NodeId("you".to_string()), &NodeId("out".to_string()))
}

// fn get_paths_rec(map: &HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: &NodeId, memo: &mut ) -> Vec<Vec<NodeId>> {
//     let mut stack = vec![vec![from]];
//     let mut completed = vec![];
//     while let Some(path) = stack.pop() {
//         let next = path.last().unwrap();
//         let next_locations = map.get(next).cloned().unwrap_or(vec![]);
//         for loc in next_locations {
//             if &loc == to {
//                 completed.push(path.clone());
//             } else {
//                 let mut new_path = path.clone();
//                 new_path.push(loc);
//                 stack.push(new_path);
//             }
//         }
//     }
//     completed
// }
//
fn get_paths(
    reverse_map: &HashMap<NodeId, Vec<NodeId>>,
    from: NodeId,
    to: &NodeId,
) -> Vec<Vec<NodeId>> {
    let mut routes_map: HashMap<NodeId, Vec<Vec<NodeId>>> = HashMap::new();
    let mut stack: Vec<(NodeId, Vec<NodeId>)> = vec![(to.clone(), vec![])];
    while let Some(i) = stack.pop() {
        let mut path = i.1.clone();
        path.push(i.0.clone());
        for c in reverse_map.get(&i.0).cloned().unwrap_or_else(|| vec![]) {
            routes_map.entry(c.clone()).or_default().push(path.clone());
            stack.push((c, path.clone()));
        }
    }
    routes_map.get(&from).cloned().unwrap_or_default()
}

fn num_paths(map: &HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: &NodeId) -> u64 {
    get_paths(map, from, to).len() as u64
}

fn part_b(input: &Input) -> u64 {
    let lines = input.lines().map(|i| parse_line(&i));
    let map: HashMap<NodeId, Vec<NodeId>> =
        lines.fold(HashMap::<NodeId, Vec<NodeId>>::new(), |mut acc, node| {
            for c in &node.connections {
                let ent = acc.entry(c.clone()).or_default();
                ent.push(node.node.clone());
            }
            acc
        });
    get_paths(&map, NodeId("svr".to_string()), &NodeId("out".to_string()))
        .into_iter()
        .filter(|i| {
            i.contains(&NodeId("dac".to_string())) && i.contains(&NodeId("fft".to_string()))
        })
        .count() as u64
}

// fn rec_num_paths(
//     map: &HashMap<NodeId, Vec<NodeId>>,
//     from: (NodeId, [bool; 2]),
//     to: &NodeId,
//     memo: &mut HashMap<NodeId, Vec<Vec<NodeId>>>,
// ) -> u64 {
//     const REQS: [&str; 2] = ["dac", "fft"];
//     if let Some(v) = memo.get(&from.0) {
//         return v.3;
//     }
//     let next_locations = map.get(&from.0).cloned().unwrap_or(vec![]);
//
//     let mut total = 0;
//     for loc in next_locations {
//         println!("going to {loc:?}");
//         if &loc == to {
//             println!("reached an output");
//             if from.1[0] && from.1[1] {
//                 //
//             }
//         } else {
//             let r1 = from.1[0] || loc.0 == REQS[0];
//             let r2 = from.1[1] || loc.0 == REQS[1];
//             rec_num_paths(map, (loc.clone(), [r1, r2]), to, memo);
//             let &(new_count_r1, new_count_r2, new_count_total, new_count_both) =
//                 memo.get(&loc).unwrap();
//             count_r1 += new_count_r1;
//             count_r2 += new_count_r2;
//             count_total += new_count_total;
//             count_both += new_count_both;
//         }
//     }
//     memo.insert(from.0, (count_r1, count_r2, count_total, count_both));
//     count_both
// }

// fn num_paths_b(map: &HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: &NodeId) -> u64 {
//     let mut memo = HashMap::new();
//     rec_num_paths(map, (from, [false; 2]), to, &mut memo)
// }

aoc_helpers::mk_aoc_test!(
    &"svr: aaa bbb
you:svr
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    8,
    2
);
