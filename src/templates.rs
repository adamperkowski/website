use lazy_static::lazy_static;
use rust_embed::Embed;
use std::str::from_utf8;

lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let mut tera = tera::Tera::default();
        let _ = tera.add_raw_templates(Template::iter().map(|file| {
            let raw = Template::get(&file).unwrap();
            let content = from_utf8(raw.data.as_ref()).unwrap();
            (file.to_string(), content.to_string())
        }));
        tera
    };
}

#[derive(Embed)]
#[folder = "templates/"]
struct Template;
