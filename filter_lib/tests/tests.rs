#[cfg(test)]
mod tests {
    use filter_lib::logos_parsing;
    use filter_lib::logos_parsing::{Block, FilterBlock, TESTFILTER};
    #[test]
    fn test_new_filter_block() {
        let x = logos_parsing::create();
        assert!(x == logos_parsing::FilterBlock { block: Block::Show })
    }
    #[test]
    fn test_filter_exits() {
        assert!(!TESTFILTER.is_empty())
    }
}
