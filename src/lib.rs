pub mod instruction;
pub mod interpreter;
pub mod optimizer;
pub mod parser;

#[cfg(test)]
mod test {
    #[test]
    fn hello_world() {
        let input = "inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aa outputa inc aaaaaaa outputa outputa inc aaa outputa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaa outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc a outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaa outputa inc aaa outputa decc aaaaaa outputa decc aaaaaaaa outputa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaa outputa endprogram";
        let instructions = crate::parser::Parser::parse(input);
        assert!(instructions.is_ok());
        let instructions = instructions.unwrap();
        let output = std::sync::Arc::new(std::cell::RefCell::new(String::new()));
        let output_clone = output.clone();
        let mut interpreter = crate::interpreter::Interpreter::new(instructions).with_output(
            Box::new(move |s: String| {
                output_clone.borrow_mut().push_str(s.as_str());
            }),
        );
        interpreter.run();

        let result = output.borrow().to_string();
        let expected = "Hello World!";

        assert_eq!(result, expected);
    }

    #[test]
    fn hello_world_fixed_memory() {
        let input = "inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aa outputa inc aaaaaaa outputa outputa inc aaa outputa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaa outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaaaaa inc a outputa inc aaaaaaaaa inc aaaaaaaaa inc aaaaaa outputa inc aaa outputa decc aaaaaa outputa decc aaaaaaaa outputa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaaaaaaa decc aaaa outputa endprogram";
        let instructions = crate::parser::Parser::parse(input);
        assert!(instructions.is_ok());
        let instructions = instructions.unwrap();
        let output = std::sync::Arc::new(std::cell::RefCell::new(String::new()));
        let output_clone = output.clone();
        let mut interpreter = crate::interpreter::Interpreter::new_fixed_size::<1>(instructions)
            .with_output(Box::new(move |s: String| {
                output_clone.borrow_mut().push_str(s.as_str());
            }));
        interpreter.run();

        let result = output.borrow().to_string();
        let expected = "Hello World!";

        assert_eq!(result, expected);
    }

    #[test]
    /// this only runs with fixed memory since it wraps around
    fn hello_world_shortest_fixed_memory() {
        let input = "inc a a decc aa fwdfw a decc a a fwdfw aa inc a fwdfw a decc aaaaa bakbak aa ei bakbak a decc aa bakbak a decc aaa ei fwdfw a decc a outputa fwdfw aaa inc a outputa fwdfw aa outputa outputa inc aaa a outputa fwdfw a ei bakbak aaaa outputa inc a inc aa outputa decc aaaaaa outputa bakbak aa decc a outputa fwdfw aaaa inc a outputa";
        let instructions = crate::parser::Parser::parse(input);
        let instructions = instructions.unwrap();
        let output = std::sync::Arc::new(std::cell::RefCell::new(String::new()));
        let output_clone = output.clone();
        let mut interpreter = crate::interpreter::Interpreter::new_fixed_size::<100>(instructions)
            .with_output(Box::new(move |s: String| {
                output_clone.borrow_mut().push_str(s.as_str());
            }));
        interpreter.run();

        let result = output.borrow().to_string();
        let expected = "Hello, World!";

        assert_eq!(result, expected);
    }
}
