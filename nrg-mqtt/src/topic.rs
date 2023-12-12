use itertools::Itertools;
use thiserror::Error;

#[derive(Debug)]
pub struct Pattern {
    pub parts: Vec<Part>,
    /// Does the topic end with '#'?
    pub is_prefix: bool,
}

impl Pattern {
    pub fn parse(topic: &str) -> Result<Self, PatternError> {
        let mut parts = topic
            .split('/')
            .map(|p| match p {
                "+" => ParsedPart::WildcardSingleLevel,
                "#" => ParsedPart::WildcardMultiLevel,
                s => ParsedPart::String(s.to_owned()),
            })
            .collect::<Vec<_>>();
        let is_prefix = parts.last() == Some(&ParsedPart::WildcardMultiLevel);
        if is_prefix {
            parts.pop();
        };
        let parts = parts
            .into_iter()
            .map(|p| match p {
                ParsedPart::String(s) => Ok(Part::String(s)),
                ParsedPart::WildcardSingleLevel => Ok(Part::WildcardSingleLevel),
                ParsedPart::WildcardMultiLevel => {
                    Err(PatternError::MultiLevelWildcardNotLastCharacter)
                }
            })
            .try_collect()?;
        Ok(Self { parts, is_prefix })
    }
    pub fn matches(&self, topic: &str) -> bool {
        let mut parts = topic.split('/');
        for pattern_part in &self.parts {
            let Some(part) = parts.next() else {
                return false;
            };
            if !pattern_part.matches(part) {
                return false;
            }
        }
        self.is_prefix || parts.next().is_none()
    }
}

#[derive(Debug, Error)]
pub enum PatternError {
    #[error("# only allowed as last character")]
    MultiLevelWildcardNotLastCharacter,
}

#[derive(Debug, Eq, PartialEq)]
enum ParsedPart {
    String(String),
    WildcardSingleLevel,
    WildcardMultiLevel,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Part {
    String(String),
    WildcardSingleLevel,
}

impl Part {
    pub fn matches(&self, part: &str) -> bool {
        match self {
            Part::String(s) => s == part,
            Part::WildcardSingleLevel => true,
        }
    }
}
