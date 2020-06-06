use logos::{Lexer, Logos};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Token {
    #[error]
    Error,
    #[token("Show")]
    Show,
    #[token("Hide")]
    Hide,
    #[token("Continue")]
    Continue,
    #[token("#", ignore_comments)]
    Hash,
    #[regex(" | |", logos::skip)]
    Skip,
    #[token("\n")]
    EndLine,

    // Conditions
    #[token("AreaLevel")]
    AreaLevel,
    #[token("ItemLevel")]
    ItemLevel,
    #[token("DropLevel")]
    DropLevel,
    #[token("Quality")]
    Quality,
    #[token("Rarity")]
    Rarity,
    #[token("Class")]
    Class,
    #[token("BaseType")]
    BaseType,
    #[token("Prophecy")]
    Prophecy,
    #[token("LinkedSockets")]
    LinkedSockets,
    #[token("SocketGroup")]
    SocketGroup,
    #[token("Sockets")]
    Sockets,
    #[token("Height")]
    Height,
    #[token("Width")]
    Width,
    #[token("HasExplicitMod")]
    HasExplicitMod,
    #[token("AnyEnchantment")]
    AnyEnchantment,
    #[token("HasEnchantment")]
    HasEnchantment,
    #[token("StackSize")]
    StackSize,
    #[token("GemLevel")]
    GemLevel,
    #[token("Identified")]
    Identified,
    #[token("Corrupted")]
    Corrupted,
    #[token("CorruptedMods")]
    CorruptedMods,
    #[token("Mirrored")]
    Mirrored,
    #[token("ElderItem")]
    ElderItem,
    #[token("ShaperItem")]
    ShaperItem,
    #[token("HasInfluence")]
    HasInfluence,
    #[token("FracturedItem")]
    FracturedItem,
    #[token("SynthesisedItem")]
    SynthesisedItem,
    #[token("ShapedMap")]
    ShapedMap,
    #[token("MapTier")]
    MapTier,

    // Actions
    #[token("SetBorderColor")]
    SetBorderColor,
    #[token("SetTextColor")]
    SetTextColor,
    #[token("SetBackgroundColor")]
    SetBackgroundColor,
    #[token("SetFontSize")]
    SetFontSize,
    #[token("PlayAlertSound")]
    PlayAlertSound,
    #[token("PlayAlertSoundPositional")]
    PlayAlertSoundPositional,
    #[token("DisableDropSound")]
    DisableDropSound,
    #[token("CustomAlertSound")]
    CustomAlertSound,
    #[token("MinimapIcon")]
    MinimapIcon,
    #[token("PlayEffect")]
    PlayEffect,

    // Values
    #[regex("[0-9]+", |s| s.slice().to_string())]
    Numbers(String),
    #[regex("\"([^\"]*)\"", |s| s.slice().to_string())]
    Quotes(String),
    #[regex("true|false|True|False", |s| s.slice().to_string())]
    Boolean(String),
    #[regex("[a-zA-Z]+", |s| s.slice().to_string())]
    Text(String),
}
pub enum KeywordType {
    Conditions,
    Actions,
    Block,
    Operations,
    Values(String),
}

impl Token {
    pub fn keyword_type(&self) -> Option<KeywordType> {
        match self {
            Token::Error => None,
            Token::Hash => None,
            Token::Skip => None,
            Token::EndLine => None,
            // blocks
            Token::Show => Some(KeywordType::Block),
            Token::Hide => Some(KeywordType::Block),
            Token::Continue => Some(KeywordType::Block),
            //contisitons
            Token::AreaLevel => Some(KeywordType::Conditions),
            Token::ItemLevel => Some(KeywordType::Conditions),
            Token::DropLevel => Some(KeywordType::Conditions),
            Token::Quality => Some(KeywordType::Conditions),
            Token::Rarity => Some(KeywordType::Conditions),
            Token::Class => Some(KeywordType::Conditions),
            Token::BaseType => Some(KeywordType::Conditions),
            Token::Prophecy => Some(KeywordType::Conditions),
            Token::LinkedSockets => Some(KeywordType::Conditions),
            Token::SocketGroup => Some(KeywordType::Conditions),
            Token::Sockets => Some(KeywordType::Conditions),
            Token::Height => Some(KeywordType::Conditions),
            Token::Width => Some(KeywordType::Conditions),
            Token::HasExplicitMod => Some(KeywordType::Conditions),
            Token::AnyEnchantment => Some(KeywordType::Conditions),
            Token::HasEnchantment => Some(KeywordType::Conditions),
            Token::StackSize => Some(KeywordType::Conditions),
            Token::GemLevel => Some(KeywordType::Conditions),
            Token::Identified => Some(KeywordType::Conditions),
            Token::Corrupted => Some(KeywordType::Conditions),
            Token::CorruptedMods => Some(KeywordType::Conditions),
            Token::Mirrored => Some(KeywordType::Conditions),
            Token::ElderItem => Some(KeywordType::Conditions),
            Token::ShaperItem => Some(KeywordType::Conditions),
            Token::HasInfluence => Some(KeywordType::Conditions),
            Token::FracturedItem => Some(KeywordType::Conditions),
            Token::SynthesisedItem => Some(KeywordType::Conditions),
            Token::ShapedMap => Some(KeywordType::Conditions),
            Token::MapTier => Some(KeywordType::Conditions),
            //actions
            Token::SetBorderColor => Some(KeywordType::Actions),
            Token::SetTextColor => Some(KeywordType::Actions),
            Token::SetBackgroundColor => Some(KeywordType::Actions),
            Token::SetFontSize => Some(KeywordType::Actions),
            Token::PlayAlertSound => Some(KeywordType::Actions),
            Token::PlayAlertSoundPositional => Some(KeywordType::Actions),
            Token::DisableDropSound => Some(KeywordType::Actions),
            Token::CustomAlertSound => Some(KeywordType::Actions),
            Token::MinimapIcon => Some(KeywordType::Actions),
            Token::PlayEffect => Some(KeywordType::Actions),
            // values
            Token::Numbers(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Quotes(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Boolean(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Text(s) => Some(KeywordType::Values(s.to_owned())),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::Error
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct FilterBlock {
    pub block: Option<Token>,
    pub keywords: Vec<TokenAndSpan>,
    pub bspan: Option<std::ops::Range<usize>>,
}
impl FilterBlock {
    pub fn clear(&mut self) -> Self {
        FilterBlock::default()
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct TokenAndSpan {
    pub token: Token,
    pub span: Option<std::ops::Range<usize>>,
    pub value: Vec<ValueAndSpan>,
}
#[derive(PartialEq, Debug, Default, Clone)]
pub struct ValueAndSpan {
    pub token: Token,
    pub span: Option<std::ops::Range<usize>>,
    pub value: String,
}

fn match_filter(
    vec: &mut Vec<FilterBlock>,
    token: Token,
    span: std::ops::Range<usize>,
    block: &mut FilterBlock,
) {
    if let Some(key) = token.keyword_type() {
        match key {
            KeywordType::Block => {
                new_block(vec, token, span.clone(), block);
            }
            KeywordType::Conditions => add_keyword(token, span, block),
            KeywordType::Actions => add_keyword(token, span, block),
            KeywordType::Operations => {}
            KeywordType::Values(s) => {
                add_values(token, span.clone(), block, s);
            }
        }
    }
}

pub fn parse(filter_file: &str) -> Vec<FilterBlock> {
    let mut vec: Vec<FilterBlock> = vec![];
    let mut block = FilterBlock::default();
    let mut lex = Token::lexer(filter_file).spanned();
    while let Some((token, span)) = lex.next() {
        match_filter(&mut vec, token.clone(), span.clone(), &mut block);
    }
    vec.push(block.clone());
    vec
}

fn new_block(
    vec: &mut Vec<FilterBlock>,
    token: Token,
    span: std::ops::Range<usize>,
    block: &mut FilterBlock,
) {
    vec.push(block.clone());
    block.clear();
    block.block = Some(token.clone());
    block.bspan = Some(span)
}

fn add_keyword(token: Token, span: std::ops::Range<usize>, block: &mut FilterBlock) {
    block.keywords.push(TokenAndSpan {
        token: token.clone(),
        span: Some(span),
        ..Default::default()
    })
}

fn add_values(token: Token, span: std::ops::Range<usize>, block: &mut FilterBlock, string: String) {
    if let Some(last_key) = block.keywords.last_mut() {
        last_key.value.push(ValueAndSpan {
            token: token.clone(),
            span: Some(span),
            value: string,
        });
        // println!("STRING PLEASE: {:#?}", last_key.value);
    };
}

pub fn ignore_comments(lex: &mut Lexer<Token>) {
    if lex.slice() == "#" {
        loop {
            // let result = lex.next();
            match lex.next() {
                Some(Token::EndLine) => break,
                _ => {}
            }
        }
    }
}
