use logos::Lexer;
use logos::Logos;
// use logos_derive::Logos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
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
    #[regex("\"([^\"]*)\"")]
    Quotes,
    #[regex("true|false|True|False")]
    Boolean,
    #[regex("[a-zA-Z]+")]
    Text,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct FilterBlock<'a> {
    pub block: Option<Token>,
    pub conditions: Vec<TokenAndSpan<'a>>,
    pub actions: Vec<TokenAndSpan<'a>>,
}
impl<'a> FilterBlock<'a> {
    pub fn clear(&mut self) -> Self {
        FilterBlock::default()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TokenAndSpan<'a> {
    pub token: Token,
    pub span: Span,
    pub value: &'a str,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

pub fn parse<'a>(filter_file: &'a str) -> Vec<FilterBlock<'a>> {
    let mut vec: Vec<FilterBlock> = vec![];
    let mut block = FilterBlock::default();

    let mut lex = Token::lexer(filter_file);

    loop {
        match lex.next() {
            Some(Token::Error) => {}
            Some(Token::Show) => {
                if let Some(_) = block.block {
                    vec.push(block.clone());
                    block.clear();
                }
                block.block = Some(Token::Show)
            }
            Some(Token::Hide) => {
                if let Some(_) = block.block {
                    vec.push(block.clone());
                    block.clear();
                }
                block.block = Some(Token::Hide)
            }
            Some(Token::Continue) => {
                if let Some(_) = block.block {
                    vec.push(block.clone());
                    block.clear();
                }
                block.block = Some(Token::Continue)
            }
            Some(Token::AreaLevel) => {}
            Some(Token::ItemLevel) => {}
            Some(Token::DropLevel) => {}
            Some(Token::Quality) => {}
            Some(Token::Rarity) => {}
            Some(Token::Class) => {}
            Some(Token::BaseType) => {}
            Some(Token::Prophecy) => {}
            Some(Token::LinkedSockets) => {}
            Some(Token::SocketGroup) => {}
            Some(Token::Sockets) => {}
            Some(Token::Height) => {}
            Some(Token::Width) => {}
            Some(Token::HasExplicitMod) => {}
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
            Some(Token::Quotes) => {}
            Some(Token::Hash) => {}
            Some(Token::Skip) => {}
            Some(Token::EndLine) => {}
            Some(Token::Boolean) => {}
            Some(Token::Text) => {}
            Some(_) => {}
            None => {
                if let Some(b) = block.block {
                    vec.push(block.clone());
                }
                break;
            }
        }
    }
    vec
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
