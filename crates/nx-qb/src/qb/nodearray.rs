use std::path::Path;

use nx_common::Readable;

use crate::Qb;
use crate::qb::nodearray::Error::UnexpectedNode;
use crate::qb::parser;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("unexpected node: {0:?}")]
    UnexpectedNode(parser::Node),

    #[error("unknown node value: {0:?}")]
    HasNoValue(parser::Node),
}

pub fn load_qb(filepath: impl AsRef<Path>) -> Result<Qb, String> {
    let qb = Qb::read_file(filepath, &mut ()).map_err(|e| format!("{:?}", e))?;
    Ok(qb)
}

pub fn parse_qb(qb: &Qb) -> Result<parser::Node, String> {
    Ok(parser::Node::try_from(qb).map_err(|e| format!("{:?}", e))?)
}

pub enum NodeValue {
    String(String),
    Vec3(f32, f32, f32),
    Vec2(f32, f32),
}
type Node = Vec<(String, NodeValue)>;
type NodeArray = Vec<Node>;

pub fn read_nodearray(node: &parser::Node) -> Result<Vec<(String, String)>, Error> {
    let mut nodearray: NodeArray = Vec::new();
    match node {
        parser::Node::File(nodes) => {
            let node = extract_assignment(nodes, "LA_NodeArray")?;
            Ok(vec![])
        }
        _ => panic!("yeah figure this one out ellie huh"),
    }
}

/// assumes global or symbol
fn get_node_name(node: &parser::Node) -> Result<String, Error> {
    match node {
        parser::Node::Global(name) => Ok(name.clone()),
        parser::Node::Symbol(name) => Ok(name.clone()),
        _ => Err(UnexpectedNode(node.clone())),
    }
}

fn get_node_value(node: &parser::Node) -> Result<NodeValue, Error> {
    match node {
        parser::Node::Global(name) | parser::Node::Symbol(name) => {
            Ok(NodeValue::String(name.clone()))
        }
        parser::Node::Array(nodes) => todo!(),
        parser::Node::Token(token) => todo!(),
        parser::Node::Parenthesized(_)
        | parser::Node::Structure(_)
        | parser::Node::If(_)
        | parser::Node::File(_)
        | parser::Node::Assignment(_, _)
        | parser::Node::Script { .. } => Err(Error::HasNoValue(node.clone())),
    }
}

fn extract_assignment(nodes: &Vec<parser::Node>, name: &str) -> Result<NodeValue, Error> {
    for node in nodes.iter() {
        match node {
            parser::Node::Assignment(lhs, rhs) => {
                let name = get_node_name(lhs)?;
                let value = get_node_value(rhs)?;
            }
            parser::Node::File(nodes) => todo!(),
            parser::Node::Script { name, nodes } => todo!(),
            parser::Node::If(nodes) => todo!(),
            parser::Node::Structure(nodes) => todo!(),
            parser::Node::Array(nodes) => todo!(),
            parser::Node::Parenthesized(nodes) => todo!(),
            parser::Node::Global(_) => todo!(),
            parser::Node::Symbol(_) => todo!(),
            parser::Node::Token(token) => todo!(),
        }
    }

    todo!()
}
