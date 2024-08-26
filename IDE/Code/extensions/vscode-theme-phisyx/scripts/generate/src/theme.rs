// ----------- //
// Énumération //
// ----------- //

#[derive(Debug, Clone, Copy)]
#[derive(clap::ValueEnum)]
pub enum ThemeVariant {
    LightBlack,
}

// -------------- //
// Implémentation //
// -------------- //

impl ThemeVariant {
    pub fn dir(&self) -> &str {
        match self {
            Self::LightBlack => "light-black",
        }
    }

    pub fn name(&self) -> String {
        format!("{}-theme", self.dir())
    }
}
