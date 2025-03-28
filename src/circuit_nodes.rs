use alloy_primitives::U256;

#[derive(Debug, Clone, PartialEq)]
pub enum GateOp {
    Add, Sub, Mul, InvMod
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitNode<'a> {
    ArithmeticNode(ArithmeticGateNode<'a>),
    InputNode(InputNode),
    OutputNode(OutputNode)
}

impl<'a> CircuitNode<'a> {

    pub fn metadata(&self) -> InputNodeMetadata {
        match self {
            CircuitNode::ArithmeticNode(arithmetic_gate_node) => arithmetic_gate_node.metadata,
            CircuitNode::InputNode(input_node) => input_node.metadata,
            CircuitNode::OutputNode(output_node) => output_node.metadata,
        }
    }
}

//                  -----ArithmeticGateNode
#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticGateNode<'a> {
    pub gate_op: GateOp,
    inputs: &'a [CircuitNode<'a>],
    output: U256,
    metadata: InputNodeMetadata
}

impl<'a> ArithmeticGateNode<'a> {
    pub fn new(
        gate_op: GateOp, 
        inputs: &'a [CircuitNode<'a>], 
        output: U256
    ) -> Self {
        Self { gate_op, inputs, output, metadata: Default::default() }
    }
}

//                  -----InputNode
#[derive(Debug, Clone, PartialEq)]
pub struct InputNode {
    metadata: InputNodeMetadata,
    data: U256
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct InputNodeMetadata {
    visibility: InputNodeVisibility,
    id: u32,
    is_input: bool
}

impl InputNodeMetadata {
    pub fn new(visibility: InputNodeVisibility, id: u32, is_input: bool) -> Self {
        Self { visibility, id, is_input}
    }
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum InputNodeVisibility {
    #[default] Public = 0, Private = 1
}

impl InputNode {
    pub fn new(id: u32, visibility: InputNodeVisibility, data: U256, is_input: bool) -> Self {
        Self { metadata: InputNodeMetadata::new(visibility, id, is_input), data }
    }
}

//                  -----OutputNode
#[derive(Debug, Clone, PartialEq)]
pub struct OutputNode {
    metadata: InputNodeMetadata,
    source_output: u32,
    output: U256
}

pub fn wrap_arithmetic_result(arith: &ArithmeticGateNode) -> InputNode {
    InputNode::new(0, InputNodeVisibility::Private, arith.output, true)
}


pub trait InputsReader: Clone + PartialEq {
    fn inputs(&self) -> Vec<u32>;
}

impl<'a> InputsReader for ArithmeticGateNode<'a> {
    fn inputs(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        vec.extend_from_slice(self.inputs);
        vec.iter().map(|input| {
            input.metadata().id
        }).collect()
    }
}

impl InputsReader for InputNode {
    fn inputs(&self) -> Vec<u32> {
        vec![]
    }
}

impl InputsReader for OutputNode {
    fn inputs(&self) -> Vec<u32> {
        vec![self.source_output]
    }
}

impl<'a> InputsReader for CircuitNode<'a> {
    fn inputs(&self) -> Vec<u32> {
        match self {
            CircuitNode::ArithmeticNode(ref node) => node.inputs(),
            CircuitNode::InputNode(ref node) => node.inputs(),
            CircuitNode::OutputNode(ref node) => node.inputs(),
        }
    }
}
