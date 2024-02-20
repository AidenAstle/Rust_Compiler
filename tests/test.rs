extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, start_interpreter};
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected);
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));

// TESTS FOR CUT 1 : Conditionals
test!(true_numberic_conditional, r#"2 == 2"#, Ok(Value::Bool(true)));
test!(false_numberic_conditional, r#"6+11 == 8"#, Ok(Value::Bool(false)));
test!(double_conditional, r#"3 > 2 == false"#, Ok(Value::Bool(false)));
test!(true_bool_conditional, r#"true != false"#, Ok(Value::Bool(true)));
test!(false_bool_conditional, r#"true == false"#, Ok(Value::Bool(false)));
test!(conditional_identifier, r#"let x = 8 >= 1;"#, Ok(Value::Bool(true)));
test!(conditional_return, r#"fn main(){return 5 != 5;}"#, Ok(Value::Bool(false)));
test!(identifier_conditional_return, r#"fn main(){
  let x = true;
  let y = foo();
  return x == y;
}
fn foo() {
  return false;
}"#, Ok(Value::Bool(false)));
test!(program_conditional_error, r#"fn main(){
  let x = foo(1);
  let y = bar(5,5);
  return x != y;
}
fn foo(a) {
  return a <= 0;
}
fn bar(a,b) {
  return a == b;
}"#, Ok(Value::Bool(true)));
//TESTS FOR CUT 2 : If Statements
test!(if_statement, r#"if (true){ return 1; } else { return 2; }"#, Ok(Value::Number(1)));
test!(conditional_if, r#"if (1 > 3){return true;} else {return false;}"#, Ok(Value::Bool(false)));
test!(else_if, r#"if (1 > 3){return true;} else if (true){return false;} else {return true;}"#, Ok(Value::Bool(false)));
test!(mismatched_returns, r#"if (true){ return 1; } else { return false; }"#, Err("Mismatched return types"));
test!(if_function, r#"fn main(){
  if (true){ return 1; } else { return 0; }
}"#, Ok(Value::Number(1)));
test!(program_if, r#"fn main(){
  let x = foo(1);
  let y = bar(5,5);
  return x == y;
}
fn foo(a) {
  if (a > 0){ return true;} else {return false;}
}
fn bar(a,b) {
  if (a > b){return false;} else if (a < b){return false;} else {return true;}
}"#, Ok(Value::Bool(true)));
test!(multiple_else_if, 
  r#"if (false){
    return 1;
  } else if (false){
    return 5;
  } else if (true){
    return 10;
  } else {
    return 11;
  }"#, 
  Ok(Value::Number(10)));
  