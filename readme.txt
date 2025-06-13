This is a very simple javascript toy engine written in rust for my own educational purposes.

## Current features:
Basic math operators: +, -, *, /, ( )
Basic logic operators: &&, ||, !
Basic comparators: >, <, ==
Increment and decrement prefixes (--x, ++x)
Working with booleans, strings, and numbers
Adding strings and booleans and number coersion
Defining functions with the function keyword, with return statements returning values
Calling defined functions
If statements (without else/else if)

## To do:
Add <= and >=
Add conditional else and else if
Fill out more of the operators that are missing from the operator precedence chart

run this with cargo run to run the application, and at the empty line, type something simple like "3 + 5 * 6;", should get 33 printed below that.
Or type "let x = 3 * 5; x + 3;", should see "> 18" as a response.

See integration tests in interpreter.rs for more examples of simple code that can be run

## Tests
run with cargo test