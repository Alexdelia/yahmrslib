use std::fmt::Display;

pub struct AskKey {
    pub key: char,
    pub description: Option<String>,
    pub alt: bool,
    pub color: Option<String>,
}

impl AskKey {
    pub fn new(
        key: char,
        description: Option<impl Into<String>>,
        alt: bool,
        color: Option<impl Into<String>>,
    ) -> Self {
        Self {
            key,
            description: description.map(|d| d.into()),
            alt,
            color: color.map(|c| c.into()),
        }
    }
}

impl From<char> for AskKey {
    fn from(key: char) -> Self {
        Self {
            key,
            description: None,
            alt: true,
            color: None,
        }
    }
}

impl Display for AskKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.color.as_ref().unwrap_or(&String::from("\x1b[1m"));

        if self.alt {
            write!(
                f,
                "{c}{}\x1b[0m|{c}{}\x1b[0m",
                self.key.to_ascii_lowercase(),
                self.key.to_ascii_uppercase()
            )?;
        } else {
            write!(f, "{c}{}\x1b[0m", self.key)?;
        }

        if let Some(description) = &self.description {
            write!(f, " ({c}{}\x1b[0m)", description)?;
        }

        Ok(())
    }
}

pub fn ask(question: &str, key: &[AskKey], enter_redirect: Option<char>) -> char {
    print!("{} {B}[{G}y{N_C}/{R}n{N_C}]{D} ", action);
    std::io::stdout().flush().expect("flush failed");
    let g = getch::Getch::new();

    loop {
        let c = g.getch().expect("getch failed") as char;

        if c == 'y' || c == 'Y' || c == '\n' {
            println!();
            return true;
        } else if c == 'n' || c == 'N' {
            println!();
            return false;
        }

        print!("\nwaiting for '{G}y{D}' ({G}yes{D}) or '{R}n{D}' ({R}no{D}), not '{M}{c}{D}' ",);
        std::io::stdout().flush().expect("flush failed");
    }
}
