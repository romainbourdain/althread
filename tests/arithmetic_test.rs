use std::{fs, io};

use althread_with_pest::run_file;

macro_rules! create_test {
    ($name:ident, $folder: literal) => {
        #[test]
        fn $name() -> io::Result<()> {
            let mut output = Vec::new();
            let expected: String = fs::read_to_string(format!("{}/expected.alt", $folder))?;
            run_file(format!("{}/in.alt", $folder).as_str(), &mut output).unwrap();
            assert_eq!(output, expected.as_bytes());

            Ok(())
        }
    };
}

create_test!(addition, "tests/files/addition");
create_test!(subtraction, "tests/files/subtraction");
create_test!(multiplication, "tests/files/multiplication");
create_test!(division, "tests/files/division");
create_test!(division_float, "tests/files/division_float");
