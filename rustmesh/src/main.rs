use std::{borrow::BorrowMut, cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Clone)]
struct MockLink {
    queue: Rc<RefCell<VecDeque<Vec<u8>>>>,
}

struct Node {
    tx: MockLink,
    rx: MockLink,
}

impl MockLink {
    fn send(&self, data: &[u8]) {
        self.queue.borrow_mut().push_back(data.to_vec());
    }

    fn recv(&self) -> Option<Vec<u8>> {
        self.queue.borrow_mut().pop_front()
    }
}

fn main() {
    let link = Rc::new(RefCell::new(MockLink::new()));
    let mut node_a = Node {
        tx: link.clone(),
        rx: link.clone(),
    };
    let mut node_b = Node {
        tx: link.clone(),
        rx: link.clone(),
    };
    node_a.tx.send(b"sabinonweb is back with the mesh!");

    if let Some(data) = node_b.rx.recv() {
        println!("Data is {:?}", data);
    }
}
