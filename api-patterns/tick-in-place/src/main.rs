const NODE_COUNT: usize = 32;

struct Node {
    position: i32,
}

impl Node {
    fn new() -> Self {
        Node { position: 0 }
    }

    fn tick(&mut self) {
        self.position = self.position + 1;
    }
}

struct Scenario {
    count: usize,
    nodes: [Node; NODE_COUNT],
}

impl Scenario {
    fn new(count: usize) -> Self {
        let nodes = [0; NODE_COUNT].map(|_| Node::new());
        Scenario { count, nodes }
    }

    fn tick(&mut self) {
        for index in 0..self.count {
            self.nodes[index].tick();
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_node_tick() {
    let mut node = Node::new();
    assert_eq!(0, node.position);
    node.tick();
    assert_eq!(1, node.position);
}

#[test]
fn test_many_nodes_tick() {
    let mut nodes = [0; NODE_COUNT].map(|_| Node::new());
    assert_eq!(0, nodes.get(0).unwrap().position);
    nodes[0].tick();
    assert_eq!(1, nodes.get(0).unwrap().position);
}

#[test]
fn test_scenario_tick() {
    let mut scenario = Scenario::new(2);
    assert_eq!(0, scenario.nodes.get(0).unwrap().position);
    scenario.tick();
    assert_eq!(1, scenario.nodes.get(0).unwrap().position);
}
