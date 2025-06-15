#[cfg(test)]
mod integration_tests {
    use crate::interpreter::errors::{InterpreterError, InterpreterErrorKind};
    use crate::interpreter::process_statements;
    use crate::lexer::tokenize;
    use crate::parser::{Parser, separate_out_statements_and_parser_errors};
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
            Statement::FunctionDeclaration(_identifier, _arguments, _block) => todo!(),
            Statement::ReturnStatement(_expression) => todo!(),
            Statement::ConditionalStatement(_condition, _block, _next_conditional) => todo!(),
            Statement::While(_statement) => todo!(),
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(10.0));
    }

    #[test]
    fn math_with_exponents() {
        let input = "2 ** 2 ** 3";
        // if we do this left to right, would end up with 4 ** 3 => 64
        // if we do this right to left, would end up with 2 ** 8 => 256
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(256.0));
    }

    
    #[test]
    fn math_with_exponents_testing_precedence() {
        let input = "2 ** 2 ** 3 - 50 * 2";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(156.0));
    }

    #[test]
    fn negation_of_parentheses() {
        let input = "-(3+2);";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_less_than_or_equal_true() {
        let input = "1 <= 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_greater_than_or_equal_true() {
        let input = "2 >= 1;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_not_equal_true() {
        let input = "2 != 1;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
            _ => &Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_not_equal_false() {
        let input = "2 != 2;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut env = Environment::new();
        let expression = match &statements[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
            Ok(Statement::ExpressionStatement(expression)) => expression,
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
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements, &mut env);
        assert_eq!(
            env.get_variable("x").unwrap_or(ExpressionResult::Number(-255.0)),
            ExpressionResult::Number(3.0)
        );
        assert_eq!(
            0,
            errors.len()
        );
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_logic_with_not_expect_false() {
        let input = "let x = 5; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_logic_with_not_expect_true() {
        let input = "let x = 1; !(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_logic_with_not_not() {
        let input = "let x = 1; !!(x > 3)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements, &mut env);
        let result = eval_expression(expression.clone(), &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_logic_with_decrement_prefix() {
        let input = "let x = 3; --x; x == 3;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let third_expression = match &results[2] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statement_at_index(&statements, &mut env, 0);
        assert_eq!(env.get_variable("x").unwrap(), ExpressionResult::Number(3.0));
        let result = eval_expression(expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Number(2.0));
        assert_eq!(env.get_variable("x").unwrap(), ExpressionResult::Number(2.0));
        let result = eval_expression(third_expression, &mut env).unwrap();
        assert_eq!(result, ExpressionResult::Boolean(false));
    }

    #[test]
    fn testing_reassignment() {
        let input = "let x = 3; x = 4;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements, &mut env);

        let stored_value = env.get_variable("x").unwrap();
        assert_eq!(stored_value, ExpressionResult::Number(4.0));
    }

    #[test]
    fn testing_reference_error() {
        let input = "x = 6;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[0] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let result = eval_expression(expression, &mut env);
        assert!(result.is_err(), "{}", InterpreterError{kind: InterpreterErrorKind::ReferenceError("x".into())}.to_string());
    }

    #[test]
    fn testing_storing_boolean_in_variables() {
        let input = "let x = true;  let y = false;  x || y;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[2] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements, &mut env);
        let result = eval_expression(expression, &mut env);
        assert_eq!(result.unwrap(), ExpressionResult::Boolean(true));
    }

    #[test]
    fn testing_storing_strings_in_variables_and_concatenating() {
        let input = "let x = \"apple\";  let y = \"sauce\";  x + y;";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[2] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                        let (statements, errors) = separate_out_statements_and_parser_errors(results);

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
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[3] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let second_expression = match &results[4] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
                        let (statements, errors) = separate_out_statements_and_parser_errors(results);

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
        let results = parser.parse();
        
        let mut env = Environment::new();
        let expression = match &results[2] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let second_expression = match &results[3] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let third_expression = match &results[4] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let fourth_expression = match &results[5] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

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
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        if let Ok(statement) = &statements[0] {
            eval_statement(statement.clone(), &mut env);
        }
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
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
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
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        process_statements(statements.clone(), &mut env);
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
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        process_statements(statements.clone(), &mut env);
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
        let results = parser.parse();
        let mut env = Environment::new();

        let second_function_call = match &results[2] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        process_statements(statements.clone(), &mut env);
        let x = env.get_variable("x".into());

        assert_eq!(
            statements.len(), 4
        );
        assert!(
            env.has_function("add_three".into())
        );
        assert_eq!(
            x.unwrap(), ExpressionResult::Number(10.0)
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
        let results = parser.parse();
        let mut env = Environment::new();
        let expression = match &results[1] {
            Ok(Statement::ExpressionStatement(expression)) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        process_statements(statements.clone(), &mut env);
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
            let x = 3;
            if (2 > 1) {
                x = 4 + 3;
            }
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();

        let mut env: Environment = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        assert_eq!(statements.len(), 2);

        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(7.0))
        );
    }

    #[test]
    fn if_statement_false() {
        let input = "
            let x = 3;
            if (1 > 2) {
                x = 4 + 3;
            }
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();

        let mut env = Environment::new();
        assert_eq!(results.len(), 2);
        let (statements, errors) = separate_out_statements_and_parser_errors(results);

        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(3.0))
        );
    }

    #[test]
    fn it_does_not_propogate_block_scope_variables_upwards() {
        let input = "
            let x = 3;
            if (x > 2) {
                let y = 5;
                ++x;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(4.0))
        );
        assert_eq!(
            env.get_variable("y".into()),
            None
        )
    }

    #[test]
    fn it_handles_less_than_or_equals() {
        let input = "
            let x = 0;
            let y = 0;
            if (x <= 0) {
                x = -1;
            }

            if (x <= 0) {
                y = 1;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(-1.0))
        );

        assert_eq!(
            env.get_variable("y".into()),
            Some(ExpressionResult::Number(1.0))
        );

    }

    #[test]
    fn it_handles_greater_than_or_equals() {
        let input = "
            let x = 1;
            let y = 0;
            if (x >= 1) {
                x = 3;
            }
            if (x >= 3) {
                y = 2;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(3.0))
        );
        assert_eq!(
            env.get_variable("y".into()),
            Some(ExpressionResult::Number(2.0))
        );
    }

    #[test]
    fn it_handles_star_equals() {
        let input = "
            let x = 2;
            x *= 3;
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(6.0))
        );
    }
    #[test]
    fn it_handles_slash_equals() {
        let input = "
            let x = 6;
            x /= 3;
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(2.0))
        );
    } 
    
    #[test]
    fn it_handles_plus_equals() {
        let input = "
            let x = 2;
            x += 3;
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(5.0))
        );
    }
    
    #[test]
    fn it_handles_minus_equals() {
        let input = "
            let x = 2;
            x -= 3;
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(-1.0))
        );
    }

    #[test]
    fn it_handles_else() {
        let input = "
            let x = 2;
            if (x > 3) {
                x = 3;
            } else {
                x = 1;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(1.0))
        );
    }

    #[test]
    fn it_handles_else_if() {
        let input = "
            let x = 2;
            if (x > 3) {
                x = 3;
            } else if (x <= 2 && x > -5) {
                x = 1;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(1.0))
        );
    }

    #[test]
    fn it_handles_else_if_else() {
        let input = "
            let x = 5;
            if (x > 6) {
                x = 3;
            } else if (x <= 2 && x > -5) {
                x = 1;
            } else {
                x = 4
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(4.0))
        );
    }

    #[test]
    fn it_handles_while() {
        let input = "
            let x = 0;
            while (x < 5) {
                ++x;
            }
        ";

        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        eval_statements(statements.clone(), &mut env);
        assert_eq!(
            env.get_variable("x".into()),
            Some(ExpressionResult::Number(5.0))
        );
    }

    #[test]
    fn it_throws_error_when_calling_undefined_function() {
        let input = "
            callFunction();
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        let function_call = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        let expected_error = eval_expression(function_call, &mut env);

        assert_eq!(
            Err("Function callFunction not defined".into()),
            expected_error
        )
    }

    #[test]
    fn it_hoists_function() {
        let input = "
            callFunction();
            function callFunction() {
                return 4;
            }
        ";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let results = parser.parse();
        let mut env = Environment::new();
        let (statements, errors) = separate_out_statements_and_parser_errors(results);
        let function_call = match &statements[0] {
            Statement::ExpressionStatement(expression) => expression.clone(),
            _ => Expression::NumberLiteral(-255.0),
        };

        process_statements(statements, &mut env);

        let expected_result = eval_expression(function_call, &mut env);

        assert_eq!(
            ExpressionResult::Number(4.0),
            expected_result.unwrap()
        );

        assert_eq!(
            errors.len(),
            0
        );
    }
}
