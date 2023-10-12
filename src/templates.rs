use anyhow::{Ok, Result};
use minijinja::{context, path_loader, Environment};
use serde::Serialize;

use crate::{aggregate_donations, sort_by_date, total_amount, Donation};

#[derive(Serialize)]
pub struct TemplateData {
    top: Vec<Donation>,
    latest: Vec<Donation>,
    total: f32,
    new_dono: bool,
}

impl TemplateData {
    pub fn new(raw_donations: &[Donation], new_dono: bool) -> Self {
        let total = total_amount(&raw_donations);
        let top = aggregate_donations(&raw_donations);
        let latest = sort_by_date(&raw_donations);

        Self {
            top,
            latest,
            total,
            new_dono,
        }
    }
}

pub struct Templates<'a> {
    env: Environment<'a>,
}

impl Templates<'static> {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        Self { env }
    }

    pub fn get_html(&self, donations: Vec<Donation>) -> Result<String> {
        let tmpl = self.env.get_template("donations.j2")?;
        let data = TemplateData::new(&donations, false);
        let html_string = tmpl.render(context!(data))?;
        Ok(html_string)
    }
}
