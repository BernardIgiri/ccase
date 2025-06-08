use convert_case::pattern;
use strum::{Display, EnumIter, EnumString, IntoStaticStr, VariantNames};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    VariantNames,
    EnumIter,
    Display,
    IntoStaticStr,
)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum UserPattern {
    Alternating,
    Camel,
    Capital,
    Lowercase,
    Noop,
    Sentence,
    Toggle,
    Uppercase,
}

impl UserPattern {
    pub fn apply(self) -> fn(&[&str]) -> Vec<String> {
        match self {
            UserPattern::Alternating => pattern::alternating,
            UserPattern::Camel => pattern::camel,
            UserPattern::Capital => pattern::capital,
            UserPattern::Lowercase => pattern::lowercase,
            UserPattern::Noop => pattern::noop,
            UserPattern::Sentence => pattern::sentence,
            UserPattern::Toggle => pattern::toggle,
            UserPattern::Uppercase => pattern::uppercase,
        }
    }
    pub fn example(&self) -> &'static str {
        match self {
            UserPattern::Lowercase => "lower, lower, ...",
            UserPattern::Uppercase => "UPPER, UPPER, ...",
            UserPattern::Capital => "Capital, Capital, ...",
            UserPattern::Sentence => "Capital, lower, lower, ...",
            UserPattern::Camel => "lower, Capital, Capital, ...",
            UserPattern::Alternating => "aLtErNaTiNg, aLtErNaTiNg, ...",
            UserPattern::Toggle => "tOGGLE, tOGGLE, ...",
            UserPattern::Noop => "unchanged, unchanged, ...",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unknown_pattern_parse() {
        assert!("sent".parse::<UserPattern>().is_err());
    }
}
