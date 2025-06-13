#[cfg(test)]
mod integration_tests {
    use crate::interpreter::errors::reference_error;
    use crate::lexer::tokenize;
    use crate::parser::Parser;
    use crate::interpreter::interpreter::{eval_expression, eval_statement, eval_statements};
    use crate::ast::{Expression, ExpressionResult, Statement};
    use crate::environment::Environment;

    fn eval_statement_at_index(statements: &Vec<Statement>, env: &mut Environment, index: usize) {
        let statement = match &statements[index] {
            Statement::Let(identifier, expression) => {
                        Statement::Let(identifier.to_string(), expression.clone())
                    }
            Statement::ExpressionStatement(expression) => {
                        Statement::ExpressionStatement(expression.clone())
                    },
            Statement::FunctionDeclaration(identifier, arguments, block) => todo!(),
            Statement::ReturnStatement(expression) => todo!(),
            Statement::ConditionalStatement(condition, block) => todo!()
        };
        eval_statement(statement, env);
    }

    #[test]
    fn line_without_semicolon() {
        let input = "3 + 5";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(8.0));
    }

    #[test]
    fn math_with_parentheses() {
        let input = "(3 + 2) * (3 - 1);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(10.0));
    }

    #[test]
    fn negation_of_parentheses() {
        let input = "-(3+2);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(-5.0));
    }

    #[test]
    fn testing_less_than() {
        let input = "1 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_less_than_with_math_true() {
        let input = "1 < 1 + 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_less_than_with_math_false() {
        let input = "1 + 2 < 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_and_true_true() {
        let input = "true && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_and_true_false() {
        let input = "true && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_and_false_true() {
        let input = "false && true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_and_false_false() {
        let input = "false && false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_or_true_true() {
        let input = "true || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_or_true_false() {
        let input = "true || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_or_false_true() {
        let input = "false || true;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_or_false_false() {
        let input = "false || false;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_more_complicated_logic() {
        let input = "let x = 3;  x > 2 && true";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        assert_eq!(
            env.get_variable("x").unwrap_or(ExpressionResult::Number(-255.0)),
            ExpressionResult::Number(3.0)
        );
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_logic_with_not_expect_false() {
        let input = "let x = 5; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_logic_with_not_expect_true() {
        let input = "let x = 1; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_logic_with_not_not() {
        let input = "let x = 1; !!(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_logic_with_decrement_prefix() {
        let input = "let x = 3; --x; x == 3;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statement_at_index(&statements, &mut env, 0);
        assert_eq!(env.get_variable("x").unwrap(), ExpressionResult::Number(3.0));
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(2.0));
        assert_eq!(env.get_variable("x").unwrap(), ExpressionResult::Number(2.0));

        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_reassignment() {
        let input = "let x = 3; x = 4;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();

        eval_statements(statements, &mut env);

        let stored_value = env.get_variable("x").unwrap();
        assert_eq!(stored_value, ExpressionResult::Number(4.0));
    }

    #[test]
    fn testing_reference_error() {
        let input = "x = 6;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression, &mut env);
        assert!(result.is_err(), "{}", reference_error("x"));
    }

    #[test]
    fn testing_storing_boolean_in_variables() {
        let input = "let x = true;  let y = false;  x || y;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        let result = eval_expression(expression, &mut env);
        assert_eq!(result.unwrap(), ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_storing_strings_in_variables_and_concatenating() {
        let input = "let x = \"apple\";  let y = \"sauce\";  x + y;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements, &mut env);
        let stored_value = env.get_variable("x").unwrap();
        assert_eq!(stored_value, ExpressionResult::String("apple".to_string()));
        let stored_value = env.get_variable("y").unwrap();
        assert_eq!(stored_value, ExpressionResult::String("sauce".to_string()));

        let result = eval_expression(expression, &mut env);
        assert_eq!(
            result.unwrap(),
            ExpressionResult::String("applesauce".to_string())
        );
    }

    #[test]
    fn testing_string_and_non_string_concatenation() {
        let input = "let x = \"apple\";  let y = 5; let z = false;  x + y; x + z;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[3] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let second_expression = match &statements[4] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        eval_statements(statements, &mut env);
        let stored_value = env.get_variable("x").unwrap();
        assert_eq!(stored_value, ExpressionResult::String("apple".to_string()));
        let stored_value = env.get_variable("y").unwrap();
        assert_eq!(stored_value, ExpressionResult::Number(5.0));
        let stored_value = env.get_variable("z").unwrap();
        assert_eq!(stored_value, ExpressionResult::Boolean(false));

        let result = eval_expression(expression, &mut env);
        assert_eq!(
            result.unwrap(),
            ExpressionResult::String("apple5".to_string())
        );
        let result = eval_expression(second_expression, &mut env);
        assert_eq!(
            result.unwrap(),
            ExpressionResult::String("applefalse".to_string())
        );
    }

    #[test]
    fn testing_string_coersion_via_prefix() {
        let input = "let x = \"apple\";  let y = \"5\";";
        let second_input = "+x; +y; -x; -y;";
        let tokens = tokenize(&(input.to_owned() + second_input));
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        
        let mut env = Environment::new();
        let expression = match &statements[2] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let second_expression = match &statements[3] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let third_expression = match &statements[4] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let fourth_expression = match &statements[5] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        eval_statements(statements, &mut env);
        let stored_value = env.get_variable("x").unwrap();
        assert_eq!(stored_value, ExpressionResult::String("apple".to_string()));
        let stored_value = env.get_variable("y").unwrap();
        assert_eq!(stored_value, ExpressionResult::String("5".to_string()));

        let result = eval_expression(expression, &mut env);
        assert!(result.is_err(), "NaN");
        let result = eval_expression(second_expression, &mut env);
        assert_eq!(
            result.unwrap(),
            ExpressionResult::Number(5.0)
        );
        let result = eval_expression(third_expression, &mut env);
        assert!(result.is_err(), "NaN");
        let result = eval_expression(fourth_expression, &mut env);
        assert_eq!(
            result.unwrap(),
            ExpressionResult::Number(-5.0)
        );
    }

    #[test]
    fn prefix_increment_on_variable_true_returns_number_two() {
        let input = "let x = true; ++x";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let second_expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statement(statements[0].clone(), &mut env);
        assert_eq!(
            env.get_variable("x").unwrap(), ExpressionResult::Boolean(true)
        );
        let result = eval_expression(second_expression, &mut env);
        assert_eq!(
            result.unwrap(), ExpressionResult::Number(2.0)
        );
        assert_eq!(
            env.get_variable("x").unwrap(), ExpressionResult::Number(2.0)
        );
    }

    #[test]
    fn prefix_plus_on_true_returns_number_one() {
        let input = "+true";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression, &mut env);
        assert_eq!(
            result.unwrap(), ExpressionResult::Number(1.0)
        );
    }

    #[test]
    fn function_and_call() {
        let input = "
            function return_3() { return 3; }
            return_3();
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements.clone(), &mut env);
        let result = eval_expression(expression, &mut env);

        assert_eq!(
            statements.len(), 2
        );
        assert!(
            env.has_function("return_3".into())
        );
        assert_eq!(
            result.unwrap(), ExpressionResult::Number(3.0)
        );
    }

    #[test]
    fn function_and_call_with_argument() {
        let input = "
            function add_three(a) { return a + 3; }
            add_three(4);
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements.clone(), &mut env);
        let result = eval_expression(expression, &mut env);

        assert_eq!(
            statements.len(), 2
        );
        assert!(
            env.has_function("add_three".into())
        );
        assert_eq!(
            result.unwrap(), ExpressionResult::Number(7.0)
        );
    }

    #[test]
    fn function_and_call_with_argument_and_variable_reassignment() {
        let input = "
            function add_three(a) { return a + 3; }
            let x = add_three(4);
            x = add_three(x);
            x;
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();

        let second_function_call = match &statements[2] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        eval_statements(statements.clone(), &mut env);
        let x = env.get_variable("x".into());

        assert_eq!(
            statements.len(), 4
        );
        assert!(
            env.has_function("add_three".into())
        );
        assert_eq!(
            env.get_variable("x".into()).unwrap(), ExpressionResult::Number(10.0)
        );
        let result = eval_expression(second_function_call, &mut env);

        assert_eq!(
            result.unwrap(), ExpressionResult::Number(13.0)
        );
    }

    #[test]
    fn function_and_call_with_multiple_arguments() {
        let input = "
            function add(a, b) { return a + b; }
            add(8, 4);
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[1] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        eval_statements(statements.clone(), &mut env);
        let result = eval_expression(expression, &mut env);

        assert_eq!(
            statements.len(), 2
        );
        assert!(
            env.has_function("add".into())
        );
        assert_eq!(
            result.unwrap(), ExpressionResult::Number(12.0)
        );
    }

    #[test]
    fn if_statement_true() {
        let input = "
            if (2 > 1) {
                4 + 3;
            }
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let expression = match &statements[0] {
            Statement::ConditionalStatement(expression, block) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        let mut env = Environment::new();
        assert_eq!(statements.len(), 1);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            true, true
        );
    }

    #[test]
    fn if_statement_false() {
        let input = "
            if (1 > 2) {
                4 + 3;
            }
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let expression = match &statements[0] {
            Statement::ConditionalStatement(expression, block) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        let mut env = Environment::new();
        assert_eq!(statements.len(), 1);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            true, true
        );
    }
}
