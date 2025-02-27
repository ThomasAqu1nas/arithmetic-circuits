use std::{collections::BTreeMap, error::Error};

use dag::{DagNode, DAG};
use crate::circuit_nodes::{CircuitNode, InputsReader};

#[derive(Clone, Debug, PartialEq)]
pub struct Circuit<T: Clone + PartialEq> {
    dag: DAG<T>
}

impl<T: Clone + PartialEq> Circuit<T> {
    pub fn dag(&self) -> &DAG<T> {
        &self.dag
    }

    pub fn inner(&self) -> &BTreeMap<u32, DagNode<T>> {
        self.dag.inner()
    }

    pub fn new() -> Self {
        Self { dag: DAG::new() }
    }

    pub fn builder() -> CircuitBuilder<T> {
        CircuitBuilder { next_node_id: 1, circuit: Circuit::new() }
    }
}

pub struct CircuitBuilder<T: Clone + PartialEq> {
    next_node_id: u32,
    circuit: Circuit<T>,
}

impl<'a> CircuitBuilder<CircuitNode<'a>> {
    pub fn new() -> Self {
        Self { next_node_id: 1, circuit: Circuit::new() }
    }

    pub fn add_node(&mut self, node: CircuitNode<'a>) -> Result<u32, Box<dyn Error>> {
        match node {
            CircuitNode::ArithmeticNode(arithmetic_gate_node) => {
                let id = self.next_node_id;
                self.next_node_id += 1;
                self.circuit.dag.add_node(id, DagNode::new(
                    arithmetic_gate_node.inputs(), CircuitNode::ArithmeticNode(arithmetic_gate_node)
                ))?;
                Ok(id)
            },
            CircuitNode::InputNode(input_node) => {
                let id = self.next_node_id;
                self.next_node_id += 1;
                self.circuit.dag.add_node(id, DagNode::new(
                    input_node.inputs(), CircuitNode::InputNode(input_node)
                ))?; 
                Ok(id)
            },
            CircuitNode::OutputNode(output_node) => {
                let id = self.next_node_id;
                self.next_node_id += 1;
                self.circuit.dag.add_node(id, DagNode::new(
                    output_node.inputs(), CircuitNode::OutputNode(output_node)
                ))?; 
                Ok(id)
            },
        }
    }

    pub fn build(&self) -> Circuit<CircuitNode<'a>> {
        self.circuit.clone()
    } 
}
