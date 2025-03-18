use std::{fs::File, io::Write};
use tera::Context;

mod constants;
mod data;
mod templates;

use templates::TEMPLATES;

fn main() {
    let mut tera_ctx = Context::new();
    tera_ctx.insert("author", constants::AUTHOR);
    tera_ctx.insert("keywords", constants::KEYWORDS);
    tera_ctx.insert("canonical", &format!("https://{}", constants::HOST));
    tera_ctx.insert("repo", constants::REPO);

    let pp = {
        let mut ctx = tera_ctx.clone();
        ctx.insert("projects", &data::projects::PROJECTS);
        TEMPLATES.render("pages/projects.tera", &ctx).unwrap()
    };

    let mut f = File::create("build/projects/index.html").unwrap();
    f.write_all(pp.as_bytes()).unwrap();
    f = File::create("build/donate/index.html").unwrap();
    f.write_all(
        TEMPLATES
            .render("pages/donate.tera", &tera_ctx)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    let mut f = File::create("build/legal/index.html").unwrap();
    f.write_all(
        TEMPLATES
            .render("pages/legal.tera", &tera_ctx)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    f = File::create("build/404.html").unwrap();
    f.write_all(
        TEMPLATES
            .render("pages/error.tera", &tera_ctx)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    f = File::create("build/index.html").unwrap();
    f.write_all(
        TEMPLATES
            .render("pages/home.tera", &tera_ctx)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}
