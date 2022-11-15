use anyhow::bail;

use itertools::Itertools;
use multimap::MultiMap;
use regex::internal::Char;
use serde::{Deserialize, Serialize};
use std::{io::BufWriter, num::NonZeroU8, sync::Arc};

use crate::components::{Alignment, MarkdownElement, MarkdownTable};

use super::{character_stats::CharacterStats, BackgroundChoice, ClassLevel};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub name: Arc<String>,
    pub background: BackgroundChoice,

    pub levels: Arc<Vec<ClassLevel>>,

    pub stats: CharacterStats,

    pub backstory: Arc<String>,
}

impl Character {

    pub fn new()-> Self{
        Self{
            name: Default::default(),
            background: BackgroundChoice(Default::default()),
            levels: Default::default(),
            stats: Default::default(),
            backstory: Default::default(),
        }
    }

    pub fn to_markdown_elements(&self) -> Vec<MarkdownElement> {
        let mut vec: Vec<MarkdownElement> = Default::default();

        vec.push(MarkdownElement::header(self.name.clone(), 1).as_line());
        vec.push(MarkdownElement::header(self.background.0.clone(), 2).as_line());

        vec.push(MarkdownElement::horizontal_rule());

        let mut level_table = MarkdownTable::new(Alignment::Center);
        let class_levels = self
            .levels
            .iter()
            .map(|x| x.class.clone())
            .sorted()
            .dedup_with_count();
        level_table.set_headers(class_levels.clone().map(|x| x.1));
        level_table.add_row(class_levels.map(|x| x.0));

        vec.push(level_table.into());

        vec.push(MarkdownElement::unordered_list(
            self.levels.iter().filter_map(|x| x.feature.clone()),
        ));

        vec.push(MarkdownElement::horizontal_rule());

        let mut stats_table = MarkdownTable::new(Alignment::Center);

        stats_table.set_headers(self.stats.all().map(|x| x.0).iter());
        stats_table.add_row(self.stats.all().map(|x| x.1).iter());

        vec.push(stats_table.into());
        vec.push(MarkdownElement::horizontal_rule());

        vec.push(MarkdownElement::text(self.backstory.clone()));

        vec
    }

    pub fn to_markdown_string(&self) -> String {
        self.to_markdown_elements().iter().join("")
    }

    pub fn to_html_string(&self) -> String {
        use pulldown_cmark::{html, Options, Parser};

        let markdown_input = &self.to_markdown_string();

        // Set up options and parser. Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown_input, options);

        // Write to String buffer.
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
