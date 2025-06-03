This will be a very simple javascript toy engine written in rust for my own educational purposes.

Current features:
Basic math operators: +, -, *, /, and math_with_parentheses
Basic logic operators: &&, ||, !
Basic comparators: >, <, ==

To do:
Add <= and >=
Add conditionals
Add functions
Fill out more of the operators that are missing from the operator precedence chart

run this with cargo run to run the application, and at the empty line, type something simple like "3 + 5 * 6;", should get 33 printed below that.
Or type "let x = 3 * 5; x + 3;", should see "> 18" as a response.

run the tests with cargo test