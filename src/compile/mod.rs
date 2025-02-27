use std::collections::HashMap;

use dag::TopologicalSort;
use alloy_primitives::U256;

use crate::{circuit::Circuit, circuit_nodes::{CircuitNode, GateOp, InputsReader}};

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    // Каждое ограничение представлено списком коэффициентов для переменных.
    a: Vec<(usize, U256)>,
    b: Vec<(usize, U256)>,
    c: Vec<(usize, U256)>,
}

#[derive(Debug, Clone)]
pub struct ConstraintSystem {
    pub constraints: Vec<Constraint>,
    pub num_variables: usize,
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
            num_variables: 0,
        }
    }
}

pub fn compile(circuit: &Circuit<CircuitNode>) -> ConstraintSystem {
    let mut cs = ConstraintSystem::new();
    let mut var_indices: HashMap<u32, usize> = HashMap::new();
    
    // 1. Получаем топологически отсортированный список узлов
    // (предполагается, что метод sort() реализован)
    let sorted_ids = circuit.dag().sort()
        .expect("DAG должен быть ациклическим");
    
    // 2. Назначаем переменную для каждого узла:
    for id in sorted_ids.iter() {
        // Для каждого узла создаем переменную:
        // переменная = значение узла.
        // Здесь индекс переменной просто равен порядку добавления.
        var_indices.insert(*id, cs.num_variables);
        cs.num_variables += 1;
    }
    
    // 3. Идем по узлам в топологическом порядке и генерируем ограничения:
    // Для каждого арифметического узла (CircuitNode::ArithmeticNode) генерируем ограничение
    for (&id, dag_node) in circuit.inner() {
        match &dag_node.value() {
            CircuitNode::ArithmeticNode(arith) => {
                // Получаем индексы входных переменных:
                // Предполагается, что arith.input_ids содержит идентификаторы входных узлов.
                let mut input_vars = vec![];
                for &src in &arith.inputs() {
                    let index = var_indices.get(&src)
                        .expect("Источник должен быть назначен переменной");
                    input_vars.push(*index);
                }
                // Получаем индекс переменной для результата:
                let out_index = *var_indices.get(&id)
                    .expect("Результат должен иметь индекс переменной");
                
                // В зависимости от операции генерируем соответствующее ограничение:
                match arith.gate_op {
                    GateOp::Mul => {
                        // Ограничение: w[input_vars[0]] * w[input_vars[1]] - w[out_index] = 0
                        let constraint = Constraint {
                            a: vec![(input_vars[0], U256::from(1))],
                            b: vec![(input_vars[1], U256::from(1))],
                            c: vec![(out_index, U256::from(1))],
                        };
                        cs.constraints.push(constraint);
                    }
                    GateOp::Add => {
                        // Ограничение: (w[input_vars[0]] + w[input_vars[1]]) * 1 - w[out_index] = 0
                        let constraint = Constraint {
                            a: vec![
                                (input_vars[0], U256::from(1)),
                                (input_vars[1], U256::from(1)),
                            ],
                            b: vec![(0, U256::from(1))],  // Допустим, фиксируем константу 1 в переменной с индексом 0.
                            c: vec![(out_index, U256::from(1))],
                        };
                        cs.constraints.push(constraint);
                    }
                    GateOp::Sub => {
                        // Ограничение: (w[input_vars[0]] - w[input_vars[1]]) * 1 - w[out_index] = 0
                        let constraint = Constraint {
                            a: vec![
                                (input_vars[0], U256::from(1)),
                                (input_vars[1], U256::ZERO - U256::from(1)), // вычитание
                            ],
                            b: vec![(0, U256::from(1))],
                            c: vec![(out_index, U256::from(1))],
                        };
                        cs.constraints.push(constraint);
                    }
                    GateOp::InvMod => {
                        // Ограничение: w[input_vars[0]] * w[out_index] - 1 = 0
                        let constraint = Constraint {
                            a: vec![(input_vars[0], U256::from(1))],
                            b: vec![(out_index, U256::from(1))],
                            c: vec![(0, U256::from(1))], // опять же, фиксируем константу 1
                        };
                        cs.constraints.push(constraint);
                    }
                }
            }
            _ => {}
        }
    }
    
    cs
}