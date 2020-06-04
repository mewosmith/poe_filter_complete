#[cfg(test)]
mod tests {
    use filter_lib::logos_parsing;
    #[test]
    fn test_new_filter_block() {
        let x = logos_parsing::parse();
        println!("{:#?}", x)
    }

}
