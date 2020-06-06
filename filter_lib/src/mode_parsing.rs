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
    #[regex("[0-9]+")]
    Numbers,
    #[regex("\"([^\"]*)\"", |s| s.slice().to_string())]
    Quotes(String),
    #[regex("true|false|True|False")]
    Boolean,
    #[regex("[a-zA-Z]+")]
    Text,
}
pub enum KeywordType {
    Conditions,
    Actions,
    Block,
    Operations,
    Values,
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
            Token::Numbers => Some(KeywordType::Values),
            Token::Quotes(_) => Some(KeywordType::Values),
            Token::Boolean => Some(KeywordType::Values),
            Token::Text => Some(KeywordType::Values),
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

pub fn parse(filter_file: &str) -> Vec<FilterBlock> {
    let mut vec: Vec<FilterBlock> = vec![];
    let mut block = FilterBlock::default();
    let mut lex = Token::lexer(filter_file).spanned();
    while let Some((token, span)) = lex.next() {
        match Some(token.clone()) {
            Some(Token::Error) => {}
            Some(Token::Show) => {
                new_block(&mut vec, token, span, &mut block);
            }
            Some(Token::Hide) => {
                new_block(&mut vec, token, span, &mut block);
            }
            Some(Token::Continue) => {
                new_block(&mut vec, token, span, &mut block);
            }
            Some(Token::AreaLevel) => {}
            Some(Token::ItemLevel) => {}
            Some(Token::DropLevel) => {}
            Some(Token::Quality) => {}
            Some(Token::Rarity) => {}
            Some(Token::Class) => add_keyword(token, span, &mut block),
            Some(Token::BaseType) => {}
            Some(Token::Prophecy) => {}
            Some(Token::LinkedSockets) => {}
            Some(Token::SocketGroup) => {}
            Some(Token::Sockets) => {}
            Some(Token::Height) => {}
            Some(Token::Width) => {}
            Some(Token::HasExplicitMod) => add_keyword(token, span, &mut block),
            Some(Token::AnyEnchantment) => {}
            Some(Token::HasEnchantment) => {}
            Some(Token::StackSize) => {}
            Some(Token::GemLevel) => {}
            Some(Token::Identified) => {}
            Some(Token::Corrupted) => {}
            Some(Token::CorruptedMods) => {}
            Some(Token::Mirrored) => {}
            Some(Token::ElderItem) => {}
            Some(Token::ShaperItem) => {}
            Some(Token::HasInfluence) => {}
            Some(Token::FracturedItem) => {}
            Some(Token::SynthesisedItem) => {}
            Some(Token::ShapedMap) => {}
            Some(Token::MapTier) => {}
            Some(Token::SetBorderColor) => {}
            Some(Token::SetTextColor) => {}
            Some(Token::SetBackgroundColor) => {}
            Some(Token::SetFontSize) => {}
            Some(Token::PlayAlertSound) => {}
            Some(Token::PlayAlertSoundPositional) => {}
            Some(Token::DisableDropSound) => {}
            Some(Token::CustomAlertSound) => {}
            Some(Token::MinimapIcon) => {}
            Some(Token::PlayEffect) => {}
            Some(Token::Numbers) => {}
            Some(Token::Quotes(string)) => {
                add_values(token, span, &mut block, string);
            }
            Some(Token::Hash) => {}
            Some(Token::Skip) => {}
            Some(Token::EndLine) => {}
            Some(Token::Boolean) => {}
            Some(Token::Text) => {}
            None => {}
        }
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
