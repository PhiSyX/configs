use std::collections::BTreeMap;

// --------- //
// Structure //
// --------- //


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VSCodeTheme {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
    colors: Option<BTreeMap<String, String>>,
    pub token_colors: Vec<VSCodeTokenColor>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VSCodeTokenColor {
    pub scope: Vec<String>,
    pub settings: VSCodeTokenColorSettings,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VSCodeTokenColorSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,
}

// -------------- //
// Implémentation //
// -------------- //

impl VSCodeTheme {
    pub fn add_token_color(&mut self, token_color: VSCodeTokenColor) {
        self.token_colors.push(token_color);
    }

    pub fn merge_token_color(
        &mut self,
        current_token_color: &VSCodeTokenColor,
        new_token_color: &VSCodeTokenColor,
    ) {
        let token_color = VSCodeTokenColor::from((current_token_color, new_token_color));
        self.token_colors.push(token_color);
    }

    pub fn remove_token_color_from(
        &mut self,
        idx: usize
    ) {
        self.token_colors.remove(idx);
    }

    pub fn get_token_color(&self, token_color: &VSCodeTokenColor) -> Option<(usize, &VSCodeTokenColor)> {
        self.token_colors
            .iter()
            .enumerate()
            .find(|(_, root_token_color)| {
                match (
                    token_color.settings.foreground.as_deref(),
                    token_color.settings.font_style.as_deref(),
                ) {
                    (None, None) => false,
                    (None, Some(fs)) => {
                        root_token_color.settings.foreground.is_none()
                            && root_token_color
                                .settings
                                .font_style
                                .as_deref()
                                .filter(|&rfs| rfs == fs)
                                .is_some()
                    }
                    (Some(fg), None) => {
                        root_token_color
                            .settings
                            .foreground
                            .as_deref()
                            .filter(|&rfg| rfg == fg)
                            .is_some()
                            && root_token_color.settings.font_style.is_none()
                    }
                    (Some(fg), Some(fs)) => {
                        root_token_color
                            .settings
                            .foreground
                            .as_deref()
                            .filter(|&rfg| rfg == fg)
                            .is_some()
                            && root_token_color
                                .settings
                                .font_style
                                .as_deref()
                                .filter(|&rfs| rfs == fs)
                                .is_some()
                    }
                }
            })
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'a> From<&'a Self> for VSCodeTheme {
    fn from(this: &'a Self) -> Self {
        Self {
            name: this.name.clone(),
            colors: this.colors.clone(),
            schema: this.schema.clone(),
            ty: this.ty.clone(),
            ..Default::default()
        }
    }
}

impl<'a> From<(&'a Self, &'a Self)> for VSCodeTokenColor {
    fn from((l, r): (&'a Self, &'a Self)) -> Self {
        let mut scope = Vec::default();
        scope.extend_from_slice(&l.scope);
        scope.extend_from_slice(&r.scope);

        let mut settings = VSCodeTokenColorSettings {
            font_style: Default::default(),
            foreground: Default::default(),
        };

        settings.font_style = r.settings.font_style.clone();
        settings.foreground = r.settings.foreground.clone();

        Self { scope, settings }
    }
}
