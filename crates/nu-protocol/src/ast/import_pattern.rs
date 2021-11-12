use crate::Span;

#[derive(Debug, Clone)]
pub enum ImportPatternMember {
    Glob { span: Span },
    Name { name: Vec<u8>, span: Span },
    List { names: Vec<(Vec<u8>, Span)> },
}

#[derive(Debug, Clone)]
pub struct ImportPatternHead {
    pub name: Vec<u8>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportPattern {
    pub head: ImportPatternHead,
    pub members: Vec<ImportPatternMember>,
}
