#[cfg(test)]
mod tests {
    use std::error::Error;

    use styxc_main::compile_to_mem;

    #[test]
    fn test_basic_assign() -> Result<(), Box<dyn Error>> {
        let input = r#"mut x = 0
x = 1"#;
        compile_to_mem(input.into())?;
        Ok(())
    }
}
