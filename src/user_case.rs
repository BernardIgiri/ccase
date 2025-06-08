use convert_case::Case;
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
pub enum UserCase {
    Snake,
    Constant,
    UpperSnake,
    Ada,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Pascal,
    UpperCamel,
    Camel,
    Lower,
    Upper,
    Title,
    Sentence,
    Alternating,
    Toggle,
}

impl UserCase {
    pub fn short_name(self) -> &'static str {
        self.into()
    }
    pub fn example(&self) -> &'static str {
        match self {
            UserCase::Upper => "UPPER CASE",
            UserCase::Lower => "lower case",
            UserCase::Title => "Title Case",
            UserCase::Toggle => "tOGGLE cASE",
            UserCase::Camel => "camelCase",
            UserCase::Pascal => "PascalCase",
            UserCase::UpperCamel => "UpperCamelCase",
            UserCase::Snake => "snake_case",
            UserCase::UpperSnake => "UPPER_SNAKE_CASE",
            UserCase::Constant => "CONSTANT_CASE",
            UserCase::Kebab => "kebab-case",
            UserCase::Cobol => "COBOL-CASE",
            UserCase::UpperKebab => "UPPER-KEBAB-CASE",
            UserCase::Train => "Train-Case",
            UserCase::Flat => "flatcase",
            UserCase::UpperFlat => "UPPERFLATCASE",
            UserCase::Ada => "Ada_Case",
            UserCase::Sentence => "Sentence case",
            UserCase::Alternating => "aLtErNaTiNg CaSe",
        }
    }
}

impl TryFrom<Case<'_>> for UserCase {
    type Error = &'static str;

    fn try_from(case: Case<'_>) -> Result<Self, Self::Error> {
        Ok(match case {
            Case::Snake => UserCase::Snake,
            Case::Constant => UserCase::Constant,
            Case::UpperSnake => UserCase::UpperSnake,
            Case::Ada => UserCase::Ada,
            Case::Kebab => UserCase::Kebab,
            Case::Cobol => UserCase::Cobol,
            Case::UpperKebab => UserCase::UpperKebab,
            Case::Train => UserCase::Train,
            Case::Flat => UserCase::Flat,
            Case::UpperFlat => UserCase::UpperFlat,
            Case::Pascal => UserCase::Pascal,
            Case::UpperCamel => UserCase::UpperCamel,
            Case::Camel => UserCase::Camel,
            Case::Lower => UserCase::Lower,
            Case::Upper => UserCase::Upper,
            Case::Title => UserCase::Title,
            Case::Sentence => UserCase::Sentence,
            Case::Alternating => UserCase::Alternating,
            Case::Toggle => UserCase::Toggle,
            Case::Custom { .. } => return Err("Custom case is not supported"),
            _ => unreachable!("Unhandled case variant"),
        })
    }
}

impl From<UserCase> for Case<'static> {
    fn from(user_case: UserCase) -> Self {
        match user_case {
            UserCase::Snake => Case::Snake,
            UserCase::Constant => Case::Constant,
            UserCase::UpperSnake => Case::UpperSnake,
            UserCase::Ada => Case::Ada,
            UserCase::Kebab => Case::Kebab,
            UserCase::Cobol => Case::Cobol,
            UserCase::UpperKebab => Case::UpperKebab,
            UserCase::Train => Case::Train,
            UserCase::Flat => Case::Flat,
            UserCase::UpperFlat => Case::UpperFlat,
            UserCase::Pascal => Case::Pascal,
            UserCase::UpperCamel => Case::UpperCamel,
            UserCase::Camel => Case::Camel,
            UserCase::Lower => Case::Lower,
            UserCase::Upper => Case::Upper,
            UserCase::Title => Case::Title,
            UserCase::Sentence => Case::Sentence,
            UserCase::Alternating => Case::Alternating,
            UserCase::Toggle => Case::Toggle,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn usercase_roundtrip() {
        for user_case in UserCase::iter() {
            let roundtrip = UserCase::try_from(Case::from(user_case)).unwrap();
            assert_eq!(user_case, roundtrip, "Roundtrip failed for {:?}", user_case);
        }
    }

    #[test]
    fn unknown_case_parse() {
        assert!("snek".parse::<UserCase>().is_err());
    }
}
