use lazy_static::lazy_static;
use regex::Regex;

// enum FileTransformations {
//     SnakeCase,
// }

pub fn to_snake_case(s: &str) -> String {
    lazy_static! {
        static ref SNAKE_CASE_RE: Regex = Regex::new(r"([A-Z])").unwrap();
    }

    // Check if it has spaces
    if s.contains(" ") {
        return s.replace(" ", "_").to_lowercase();
    }

    let result = SNAKE_CASE_RE.replace_all(&s, r"_$L$1");

    if result.starts_with("_") {
        return result[1..result.len()].to_lowercase();
    }

    result.to_lowercase()
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestCase {
        normal: String,
        expected: String,
    }

    impl TestCase {
        fn check_test_case(&self) {
            let snake_case = to_snake_case(&self.normal);

            assert_eq!(
                &snake_case, &self.expected,
                "{snake_case} expected to be snake case"
            )
        }

        fn new(normal: String, expected: String) -> TestCase {
            TestCase {
                normal: normal,
                expected: expected,
            }
        }
    }

    #[test]
    fn space_case_to_snake_case() {
        let t = TestCase::new(
            String::from("Hello my name is John"),
            String::from("hello_my_name_is_john"),
        );

        t.check_test_case();
    }

    #[test]
    fn cammel_case_to_snake_case() {
        let t = TestCase::new(
            String::from("Hello my name is mar"),
            String::from("hello_my_name_is_mar"),
        );

        t.check_test_case();
    }

    #[test]
    fn check_several_cases() {
        let tests = vec![
            TestCase::new(String::from("hello"), String::from("hello")),
            TestCase::new(String::from("my Homework"), String::from("my_homework")),
            TestCase::new(
                String::from("project10 2022"),
                String::from("project10_2022"),
            ),
            TestCase::new(
                String::from("myFunctionBackup"),
                String::from("my_function_backup"),
            ),
        ];

        for t in &tests {
            t.check_test_case();
        }
    }
}
