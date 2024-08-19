/* use std::{fs, io};

use althread_with_pest::run;

macro_rules! create_eq_test {
    ($name:ident, $folder: literal) => {
        #[test]
        fn $name() -> io::Result<()> {
            let input = fs::read_to_string(format!("{}/in.alt", $folder))?;
            let mut output = Vec::new();
            let expected: String = fs::read_to_string(format!("{}/expected.alt", $folder))?;
            run(&input, &mut output).unwrap();
            assert_eq!(output, expected.as_bytes());

            Ok(())
        }
    };
}

macro_rules! create_error_test {
    ($name:ident, $folder: literal) => {
        #[test]
        fn $name() -> io::Result<()> {
            let input = fs::read_to_string(format!("{}/in.alt", $folder))?;
            let mut output = Vec::new();
            let result = run(&input, &mut output);
            assert!(result.is_err());

            Ok(())
        }
    };
}

create_eq_test!(addition, "tests/files/addition");
create_eq_test!(subtraction, "tests/files/subtraction");
create_eq_test!(multiplication, "tests/files/multiplication");
create_eq_test!(division, "tests/files/division");
create_eq_test!(division_float, "tests/files/division_float");
create_error_test!(division_by_zero, "tests/files/division_by_zero");
create_eq_test!(modulus, "tests/files/modulus");
create_eq_test!(complexe_expression, "tests/files/complexe_expression");
create_eq_test!(precedence, "tests/files/precedence");
create_eq_test!(if_stmt, "tests/files/if");
create_eq_test!(if_else_stmt, "tests/files/if_else");
create_eq_test!(while_stmt, "tests/files/while");
create_eq_test!(scope, "tests/files/scope");
 */
