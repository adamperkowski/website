use std::fmt;

use crate::Uri;

pub struct RobotsTXT {
    allowed: Vec<&'static str>,
    disallowed: Vec<&'static str>,
    sitemap_url: &'static str,
}

impl fmt::Display for RobotsTXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "User-agent: *\n")?;

        for path in &self.disallowed {
            writeln!(f, "Disallow: {}", path)?;
        }

        for path in &self.allowed {
            writeln!(f, "Allow: {}", path)?;
        }

        writeln!(f, "\nSitemap: {}", self.sitemap_url)
    }
}

impl RobotsTXT {
    pub fn from_uris(uris: &[Uri]) -> Self {
        let mut allowed = Vec::new();
        let mut disallowed = Vec::new();

        for uri in uris {
            if uri.crawlable {
                allowed.push(uri.uri);
            } else {
                disallowed.push(uri.uri);
            }
        }

        RobotsTXT {
            allowed,
            disallowed,
            sitemap_url: "https://example.com/sitemap.xml",
        }
    }
}

mod t {
    #[test]
    fn robots_txt_display() {
        let uris = &[
            Uri::new("/", "", true, None, None),
            Uri::new("/private", "", false, None, None),
        ];

        let robots = RobotsTXT::from_uris(uris);
        let expected = "User-agent: *\n\nDisallow: /private\nAllow: /\n\nSitemap: https://example.com/sitemap.xml\n";

        assert_eq!(robots.to_string(), expected);
    }
}
