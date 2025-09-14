use crate::util::parse::*;

pub struct Input<'a> {
    tree: Vec<Node<'a>>,
    layers: Vec<usize>,
    descendants: Vec<usize>,
}

#[derive(Clone, Copy)]
struct Node<'a> {
    code: &'a str,
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node<'_> {
    fn from(line: &str) -> Node<'_> {
        let code = &line[..7];
        let id = (&line[10..]).unsigned();
        Node { code, id, left: None, right: None }
    }
}

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let nodes: Vec<_> = prefix.lines().map(Node::from).collect();
    let descendants = suffix.iter_unsigned().collect();

    let mut tree = Vec::new();
    tree.push(nodes[0]);

    let mut layers = Vec::new();
    layers.push(nodes[0].id);

    for &node in &nodes[1..] {
        let mut index = 0;
        let mut layer = 1;

        #[expect(clippy::collapsible_else_if)]
        loop {
            if layers.len() == layer {
                layers.push(0);
            }

            if node.id < tree[index].id {
                if let Some(next) = tree[index].left {
                    index = next;
                    layer += 1;
                } else {
                    tree[index].left = Some(tree.len());
                    layers[layer] += node.id;
                    break;
                }
            } else {
                if let Some(next) = tree[index].right {
                    index = next;
                    layer += 1;
                } else {
                    tree[index].right = Some(tree.len());
                    layers[layer] += node.id;
                    break;
                }
            }
        }

        tree.push(node);
    }

    Input { tree, layers, descendants }
}

pub fn part1(input: &Input<'_>) -> usize {
    let Input { layers, .. } = input;
    layers.iter().max().unwrap() * layers.iter().filter(|&&layer| layer > 0).count()
}

pub fn part2(input: &Input<'_>) -> String {
    let Input { tree, .. } = input;
    path(tree, 500_000).join("-")
}

pub fn part3(input: &Input<'_>) -> String {
    let Input { tree, descendants, .. } = input;

    let first = path(tree, descendants[0]);
    let second = path(tree, descendants[1]);

    let lca = first.iter().zip(second.iter()).take_while(|(a, b)| a == b).count();
    first[lca - 1].to_string()
}

fn path<'a>(tree: &[Node<'a>], id: usize) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut index = Some(0);

    while let Some(next) = index {
        path.push(tree[next].code);

        if id < tree[next].id {
            index = tree[next].left;
        } else {
            index = tree[next].right;
        }
    }

    path
}
