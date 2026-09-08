use std::path::Path;

use nx_common::Readable;
use nx_qb::qb::parser::Node;

pub fn load_qb(filepath: impl AsRef<Path>) -> Result<Node, String> {
    let qb = nx_qb::Qb::read_file(filepath, &mut ()).map_err(|e| format!("{:?}", e))?;
    let node = Node::try_from(&qb).map_err(|e| format!("{:?}", e))?;
    Ok(node)
}

pub fn read_nodearray(node: Node) -> Result<Vec<(String, String)>, String> {
    match node {
        Node::File(nodes) => {
            let e = 0;
            todo!()
        }
        _ => Err(format!("unexpected node: {:?}", node)),
    }
}
