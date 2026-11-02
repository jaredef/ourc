pub enum Level {
    Bug,
    Error,
    Warning,
    Note,
    Help,
}

pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub span: Option<Span>,
    pub children: Vec<SubDiagnostic>,
    pub code: Option<&'static str>,
}

pub struct SubDiagnostic {
    pub level: Level,
    pub message: String,
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            span: None,
            children: Vec::new(),
            code: None,
        }
    }

    pub fn span(&mut self, sp: Span) -> &mut Self {
        self.span = Some(sp);
        self
    }

    pub fn with_code(&mut self, code: &'static str) -> &mut Self {
        self.code = Some(code);
        self
    }

    pub fn note(&mut self, msg: &str) -> &mut Self {
        self.children.push(SubDiagnostic {
            level: Level::Note,
            message: msg.to_string(),
            span: None,
        });
        self
    }

    pub fn is_error(&self) -> bool {
        matches!(self.level, Level::Bug | Level::Error)
    }
}

pub fn emit(diags: &[Diagnostic]) -> usize {
    diags.iter().filter(|d| d.is_error()).count()
}
