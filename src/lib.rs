pub mod circuit_nodes;
pub mod modular;
pub mod circuit;
pub mod compile;

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use crate::modular::FiniteModular;
    use crate::circuit::CircuitBuilder;
    use crate::circuit_nodes::{CircuitNode, GateOp, InputNode, InputNodeVisibility, ArithmeticGateNode};
    use crate::compile::{compile, ConstraintSystem};

    #[test]
    fn test_modular_arithmetic() {
        // Тестируем операции: сложение, вычитание, умножение и обратный элемент по модулю.
        let a = U256::from(10);
        let b = U256::from(3);
        // Сравниваем результат сложения: 10 + 3 = 13.
        let res_add = a.mod_add(b);
        assert_eq!(res_add, U256::from(13), "addition_test");

        // Вычитание: 10 - 3 = 7.
        let res_sub = a.mod_sub(b);
        assert_eq!(res_sub, U256::from(7), "substraction_test");

        // Умножение: 10 * 3 = 30.
        let res_mul = a.mod_mul(b);
        assert_eq!(res_mul, U256::from(30), "multiplication_test");

        // Обратный элемент: проверяем, что обратный к 3 существует и 3 * inv = 1 (mod MOD).
        if let Some(inv) = b.mod_inv() {
            let prod = b.mod_mul(inv);
            assert_eq!(prod, U256::from(1), "invmod_test");
        } else {
            panic!("Обратный элемент для 3 должен существовать");
        }
    }

    #[test]
    fn test_circuit_builder_and_compile() {
        // Создаём простую схему, состоящую из двух входных узлов и одного арифметического узла (сложение).
        let mut builder = CircuitBuilder::new();

        // Создаем входные узлы.
        let input1 = CircuitNode::InputNode(
            InputNode::new(
                1, 
                InputNodeVisibility::Public, 
                U256::from_str_radix("2", 10).unwrap(), 
                true
            )
        );
        let input2 = CircuitNode::InputNode(
            InputNode::new(
                2, 
                InputNodeVisibility::Public, 
                U256::from_str_radix("3", 10).unwrap(), 
                true
            )
        );

        // Создаем арифметический узел для сложения: 2 + 3 = 5.
        // Здесь в качестве входов используем клоны входных узлов (для простоты теста).
        let binding = [input1.clone(), input2.clone()];
        let arithmetic_node = CircuitNode::ArithmeticNode(ArithmeticGateNode::new(
            GateOp::Add, &binding, U256::from_str_radix("5", 10).unwrap()));

        // Добавляем узлы в схему.
        let _id1 = builder.add_node(input1).expect("Не удалось добавить InputNode 1");
        let _id2 = builder.add_node(input2).expect("Не удалось добавить InputNode 2");
        let _id3 = builder.add_node(arithmetic_node).expect("Не удалось добавить ArithmeticNode");

        // Получаем построенную схему.
        let circuit = builder.build();

        // Компилируем схему в систему ограничений.
        let cs: ConstraintSystem = compile(&circuit);

        // Проверяем, что система ограничений содержит хотя бы одно ограничение (от арифметического узла).
        assert!(!cs.constraints.is_empty(), "Ограничения не сформировались");

        // Количество переменных должно соответствовать количеству добавленных узлов (здесь 3).
        assert_eq!(cs.num_variables, 3, "Количество переменных не соответствует ожидаемому");
    }
}
