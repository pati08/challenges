use std::{collections::HashMap, rc::Rc};

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Clone)]
struct NodeId(Rc<str>);

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
        .map(|i| NodeId(i.into()))
        .collect_vec();
    NodeInfo {
        node: NodeId(node_name.into()),
        connections,
    }
}

fn part_a(input: &Input) -> u64 {
    let lines = input.lines().map(|i| parse_line(&i));
    let map: HashMap<NodeId, Vec<NodeId>> =
        lines.fold(HashMap::<NodeId, Vec<NodeId>>::new(), |mut acc, node| {
            let ent = acc.entry(node.node).or_default();
            for c in node.connections {
                ent.push(c);
            }
            acc
        });
    num_paths(&map, &NodeId("you".into()), &NodeId("out".into()), &[])
}

fn rec_num_paths(
    map: &HashMap<NodeId, Vec<NodeId>>,
    from: &NodeId,
    to: &NodeId,
    reqs: &[NodeId],
    memo: &mut HashMap<(NodeId, Vec<NodeId>), u64>,
) -> u64 {
    if let Some(v) = memo.get(&(from.clone(), reqs.to_vec())) {
        return *v;
    }
    if from == to {
        return u64::from(reqs.is_empty());
    }
    let mut total = 0;
    for i in map.get(from).unwrap() {
        let new_reqs = reqs.iter().filter(|j| *j != i).cloned().collect_vec();
        total += rec_num_paths(map, i, to, &new_reqs[..], memo);
    }
    memo.insert((from.clone(), reqs.to_vec()), total);
    total
}

fn num_paths(
    map: &HashMap<NodeId, Vec<NodeId>>,
    from: &NodeId,
    to: &NodeId,
    reqs: &[NodeId],
) -> u64 {
    let mut memo = HashMap::new();
    rec_num_paths(map, from, to, reqs, &mut memo)
}

fn part_b(input: &Input) -> u64 {
    let lines = input.lines().map(|i| parse_line(&i));
    let map: HashMap<NodeId, Vec<NodeId>> =
        lines.fold(HashMap::<NodeId, Vec<NodeId>>::new(), |mut acc, node| {
            let ent = acc.entry(node.node).or_default();
            for c in node.connections {
                ent.push(c);
            }
            acc
        });

    num_paths(
        &map,
        &NodeId("svr".into()),
        &NodeId("out".into()),
        &vec![NodeId("dac".into()), NodeId("fft".into())][..],
    )
}

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
