mod theme;
mod vscode;

use std::io::Write;

// --------- //
// Structure //
// --------- //

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(short, long)]
    theme: theme::ThemeVariant,
}

// -------------- //
// Implémentation //
// -------------- //

impl Cli {
    fn args() -> Self {
        clap::Parser::parse()
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::args();

    let root_theme: vscode::VSCodeTheme = serde_json::from_reader(std::fs::File::open(format!(
        "themes/{}.json",
        args.theme.name()
    ))?)?;

    let Ok(files) = globwalk::glob(format!("themes/{}/*.json", args.theme.dir())) else {
        eprintln!("Erreur lors de la récupération des fichiers JSON du thème");
        return Ok(());
    };

    let mut new_theme = vscode::VSCodeTheme::from(&root_theme);

    for file in files {
        let file = file?;
        let file_theme: vscode::VSCodeTheme =
            serde_json::from_reader(std::fs::File::open(file.path())?)?;
        for file_token_color in file_theme.token_colors {
            if let Some((idx, current_token_color)) = new_theme.get_token_color(&file_token_color) {
                let current_token_color = current_token_color.clone();
                new_theme.merge_token_color(&current_token_color, &file_token_color);
                new_theme.remove_token_color_from(idx);
            } else {
                new_theme.add_token_color(file_token_color);
            }
        }
    }

    let mut root_theme = std::fs::File::create(format!("themes/{}.json", args.theme.name()))?;
    root_theme.write_all(serde_json::to_string_pretty(&new_theme)?.as_bytes())?;

    Ok(())
}
