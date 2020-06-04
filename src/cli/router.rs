use super::Cmd;
use sequence_trie::SequenceTrie;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum RouteSegment {
    Literal(&'static str),
    Variable(&'static str),
}

pub struct Router<'a, 'b> {
    pub(crate) trie: SequenceTrie<RouteSegment, Cmd<'a, 'b>>,
}

impl<'a, 'b> Router<'a, 'b> {
    pub fn new() -> Router<'a, 'b> {
        Router {
            trie: SequenceTrie::<RouteSegment, Cmd<'a, 'b>>::new(),
        }
    }
    pub fn add(&mut self, cmd: Cmd<'a, 'b>) {
        let route_segments = cmd.route();
        self.trie.insert_owned(route_segments, cmd);
    }
}
