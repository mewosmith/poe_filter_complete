#[cfg(test)]
mod tests {
    use filter_lib::logos_parsing;
    use filter_lib::mode_parsing;
    #[test]
    fn test_new_filter_block() {
        let x = logos_parsing::parse();
        println!("{:#?}", x)
    }

    #[test]
    fn iterating_modes() {
        let s = include_str!("../src/test_filters/small.filter");
        let moded = mode_parsing::ModeBridge {
            mode: mode_parsing::Modes::new(s),
        };

        let results: Vec<mode_parsing::Tokens> = moded.collect();
        // println!("{:?}", results)
    }
}
