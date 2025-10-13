use std::fmt;

use crate::ChangeFreq;

const HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">"#;
const FOOTER: &str = "</urlset>";

pub struct Sitemap(Vec<Url>);

struct Url {
    loc: &'static str,
    changefreq: Option<ChangeFreq>,
    priority: Option<f32>,
    lastmod: Option<&'static str>,
    alternates: Option<Vec<Alternate>>,
}

struct Alternate {
    hreflang: &'static str,
    href: &'static str,
}

impl fmt::Display for Sitemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{HEADER}")?;

        for url in self.0.iter() {
            writeln!(f, "{url}")?;
        }

        writeln!(f, "{FOOTER}")
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  <url>")?;
        writeln!(f, "    <loc>{}</loc>", self.loc)?;

        if let Some(changefreq) = &self.changefreq {
            writeln!(f, "    <changefreq>{}</changefreq>", changefreq)?;
        }
        if let Some(priority) = &self.priority {
            writeln!(f, "    <priority>{:.1}</priority>", priority)?;
        }
        if let Some(lastmod) = &self.lastmod {
            writeln!(f, "    <lastmod>{}</lastmod>", lastmod)?;
        }

        if let Some(alternates) = &self.alternates {
            for alternate in alternates {
                writeln!(f, "{alternate}")?;
            }
        }

        write!(f, "  </url>")
    }
}

impl fmt::Display for Alternate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"    <xhtml:link rel="alternate" hreflang="{}" href="{}"/>"#,
            self.hreflang, self.href
        )
    }
}

impl fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        };

        write!(f, "{s}")
    }
}

impl Sitemap {
    pub fn from_uris(uris: &[crate::Uri]) -> Self {
        let urls = uris
            .iter()
            .map(|u| Url {
                loc: u.uri,
                changefreq: u.changefreq.clone(),
                priority: u.priority,
                lastmod: u.lastmod,
                alternates: None,
            })
            .collect();

        Sitemap(urls)
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn sitemap_display() {
        let sitemap = Sitemap(vec![
            Url {
                loc: "https://example.com/",
                changefreq: Some(ChangeFreq::Daily),
                priority: Some(1.0),
                lastmod: Some("2000-01-01"),
                alternates: Some(vec![
                    Alternate {
                        hreflang: "en",
                        href: "https://example.com/en/",
                    },
                    Alternate {
                        hreflang: "pl",
                        href: "https://example.com/pl/",
                    },
                ]),
            },
            Url {
                loc: "https://example.com/about",
                changefreq: None,
                priority: Some(0.8),
                lastmod: None,
                alternates: None,
            },
        ]);

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <url>
    <loc>https://example.com/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
    <lastmod>2000-01-01</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="https://example.com/en/"/>
    <xhtml:link rel="alternate" hreflang="pl" href="https://example.com/pl/"/>
  </url>
  <url>
    <loc>https://example.com/about</loc>
    <priority>0.8</priority>
  </url>
</urlset>
"#;

        assert_eq!(sitemap.to_string(), expected);
    }
}
