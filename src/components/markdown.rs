use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct MarkdownElement(pub String);

impl Display for MarkdownElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownTable {
    alignment: Alignment,
    headers: Option<Vec<MarkdownElement>>,
    rows: Vec<Vec<MarkdownElement>>,
}

impl Display for MarkdownTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lengths = Vec::<usize>::new();
        let min_length = match self.alignment {
            Alignment::Left => 2,
            Alignment::Right => 2,
            Alignment::Center => 3,
        };

        for r in self.headers.iter().chain(self.rows.iter()) {
            for (i, e) in r.iter().enumerate() {
                let len = e.0.len();
                if let Some(current) = lengths.get_mut(i) {
                    if *current < len {
                        *current = len;
                    }
                } else {
                    lengths.push(len.max(min_length));
                }
            }
        }

        f.write_str("|")?;

        if let Some(headers) = &self.headers {
            for (i, h) in headers
                .iter()
                .cloned()
                .pad_using(lengths.len(), |_| MarkdownElement::default())
                .enumerate()
            {
                let width = lengths[i];
                f.write_fmt(format_args!("{:width$}|", h.0))?;
            }
        }
        f.write_str("  \n|")?;

        for l in &lengths {
            let dashes = l + 1 - min_length;
            if self.alignment == Alignment::Left || self.alignment == Alignment::Center {
                f.write_str(":")?;
            }

            f.write_str("-".repeat(dashes).as_str())?;

            if self.alignment == Alignment::Right || self.alignment == Alignment::Center {
                f.write_str(":")?;
            }
            f.write_str("|")?;
        }

        for r in &self.rows {
            f.write_str("  \n|")?;

            for (i, h) in r
                .iter()
                .cloned()
                .pad_using(lengths.len(), |_| MarkdownElement::default())
                .enumerate()
            {
                let width = lengths[i];
                f.write_fmt(format_args!("{:width$}|", h.0))?;
            }
        }

        f.write_str("  \n")?;

        Ok(())
    }
}

impl From<MarkdownTable> for MarkdownElement {
    fn from(value: MarkdownTable) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Right,
    Center,
}

impl MarkdownTable {
    pub fn new(alignment: Alignment) -> Self {
        Self {
            alignment,
            headers: Default::default(),
            rows: Default::default(),
        }
    }

    pub fn set_headers<T: Display, I: Iterator<Item = T>>(&mut self, iterator: I) {
        let headers = iterator
            .map(|x| MarkdownElement::text(x.to_string().replace("|", "&#124;")))
            .collect();
        self.headers = Some(headers);
    }

    pub fn add_row<T: Display, I: Iterator<Item = T>>(&mut self, iterator: I) {
        let row = iterator
            .map(|x| MarkdownElement(x.to_string().replace("|", "&#124;")))
            .collect();
        self.rows.push(row);
    }
}

impl MarkdownElement {
    pub fn text<T: Display>(text: T) -> Self {
        Self(text.to_string())
    }

    pub fn bold<T: Display>(text: T) -> Self {
        Self(format!("**{}**", text))
    }

    pub fn italics<T: Display>(text: T) -> Self {
        Self(format!("*{}*", text))
    }

    pub fn bold_italics<T: Display>(text: T) -> Self {
        Self(format!("***{}***", text))
    }

    pub fn block_quote<T: Display>(text: T) -> Self {
        let text = text
            .to_string()
            .lines()
            .map(|x| format!("> {}  ", x))
            .join("\n");
        Self(text)
    }

    pub fn header<T: Display>(text: T, level: usize) -> Self {
        let level = level.min(6).max(1);

        let string = "#".repeat(level) + " " + text.to_string().as_str();
        Self(string)
    }

    pub fn unordered_list<T: Display, I: Iterator<Item = T>>(iterator: I) -> Self {
        let text = iterator.map(|x| format!("- {}", x)).join("\n");
        //TODO nested lists
        //TODO escape numbers
        //TODO newline
        Self(text)
    }

    pub fn horizontal_rule() -> Self {
        Self("\n\n---\n\n".to_string())
    }

    pub fn as_line(self) -> Self {
        let string = self.0 + "  \n";
        Self(string)
    }
}
