use super::Cmd;
use radix_trie::{Trie, TrieKey};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RouteSegment {
    Literal(&'static str),
    Variable(&'static str),
}

impl TrieKey for RouteSegment {
    fn encode_bytes(&self) -> Vec<u8> {
        match *self {
            RouteSegment::Literal(s) => s.as_bytes().to_vec(),
            RouteSegment::Variable(s) => s.as_bytes().to_vec(),
        }
    }
}

pub struct Router<'a> {
    trie: Trie<RouteSegment, Option<Cmd<'a>>>,
}

impl<'a> Router<'a> {
    pub fn new() -> Router<'a> {
        Router {
            trie: Trie::<RouteSegment, Option<Cmd<'a>>>::new(),
        }
    }
    pub fn add(&mut self, cmd: Cmd<'a>) {
        let route_segments = cmd.route();
        let len = route_segments.len();
        for i in 0..len - 1 {
            let segment = route_segments.get(i).unwrap();
            self.trie.insert(*segment, None);
        }
        let last_segment = route_segments.get(len - 1).unwrap();
        self.trie.insert(*last_segment, Some(cmd));
    }
}
