use lazy_static::lazy_static;
use rust_embed::Embed;
use std::error::Error;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        match tera.add_raw_templates(TemplateFiles::iter().map(|file| {
            let raw = TemplateFiles::get(&file).unwrap();
            let content = std::str::from_utf8(raw.data.as_ref()).unwrap();

            (file.to_string(), content.to_string())
        })) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("failed to load templates: {e}");
                if let Some(msg) = e.source() {
                    eprintln!("{msg}");
                }
            }
        }

        tera
    };
}

#[derive(Embed)]
#[folder = "templates/"]
struct TemplateFiles;
