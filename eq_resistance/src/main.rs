use std::collections::HashMap;
use std::io;

struct Circuit {
    circuit: Node,
    elements: HashMap<String, f64>,
}

impl Circuit {
    fn add_elem(&mut self, name: String, res: f64) {
        self.elements.insert(name, res);
    }

    fn parse_circuit(circuit: String, elements: &HashMap<String, f64>) -> Self {
        // let curr
        let depth = 0;
        let elements: Vec<NodeItem> = vec![];
        let mut mode = NodeMode::Par;
        let mut buffer = String::new();
        for item in circuit.split(' ').collect() {
            match item {
                '[' => {
                    mode = NodeMode::Par;
                    depth += 1;
                }
                '(' => {
                    mode = NodeMode::Ser;
                    depth += 1;
                }
                _ => {
                    buffer.push(item);
                }
            }
        }

        Circuit {
            circuit: (),
            elements: (),
        }
    }
}

enum NodeMode {
    Ser,
    Par,
}

enum NodeItem {
    Resist(f64),
    Elem(Box<Node>),
}

struct Node {
    mode: NodeMode,
    items: Vec<NodeItem>,
}

impl Node {
    fn new(mode: NodeMode, items: Vec<NodeItem>) -> Node {
        Node { mode, items }
    }

    fn parse_circuit(circuit: &Vec<String>, elements: &HashMap<&str, f64>) -> Node {
        let depth = 0;
        let mut items: Vec<NodeItem> = vec![];
        let mut buffer: Vec<String> = vec![];
        let mut mode = NodeMode::Par;

        for string in circuit {
            match string.as_str() {
                "]" | ")" if depth > 1 => {
                    items.push(NodeItem::Elem(Box::new(Node::parse_circuit(&buffer, elements))));
                    buffer.clear()
                },
                "[" if depth == 0 => {
                    mode = NodeMode::Par;
                    depth += 1;
                }
                "(" if depth == 0 => {
                    mode = NodeMode::Ser;
                    depth += 1;
                }
                _ => {

                    buffer.push(string.to_string());
                }
            }
        }
        Node {
            mode,
            items,
        }
    }
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let name = inputs[0].trim().to_string();
        let r = parse_input!(inputs[1], i32);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let circuit = input_line.trim_matches('\n').to_string();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("Equivalent Resistance");
}
