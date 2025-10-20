use serde::Serialize;

#[derive(Serialize)]
pub struct Uri {
    pub uri: &'static str,
    pub template: &'static str,
    pub crawlable: bool,
    pub changefreq: Option<ChangeFreq>,
    pub priority: Option<f32>,
    pub(crate) lastmod: Option<&'static str>,
}

impl Uri {
    pub fn new(
        uri: &'static str,
        template: &'static str,
        crawlable: bool,
        changefreq: Option<ChangeFreq>,
        priority: Option<f32>,
    ) -> Self {
        Self {
            uri,
            template,
            crawlable,
            changefreq,
            priority,
            lastmod: None,
        }
    }
}

#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub enum ChangeFreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}
