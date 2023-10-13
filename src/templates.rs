use anyhow::{Ok, Result};
use minijinja::{context, path_loader, Environment};

use crate::{Donation, TemplateData};

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
        let tmpl = self.env.get_template("donations.html")?;
        let data = TemplateData::new(&donations);
        let html_string = tmpl.render(context!(data))?;
        Ok(html_string)
    }
}
