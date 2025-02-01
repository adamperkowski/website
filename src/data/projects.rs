use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: &'static str,
    description: &'static str,
    emoji: Option<&'static str>,
    icon: Option<&'static str>,
    banner: Option<&'static str>,
    url: Option<&'static str>,
    repo: Option<&'static str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "nvrs",
        description: "fast new version checker for software releases",
        emoji: Some("🚦"),
        icon: None,
        banner: None,
        url: Some("https://nvrs.adamperkowski.dev"),
        repo: Some("https://github.com/adamperkowski/nvrs"),
    },
    Project {
        name: "HighlightOS",
        description: "x86_64 OS (kernel) made from scratch in Rust & Assembly",
        emoji: Some("🌄"),
        icon: None,
        banner: None,
        url: Some("https://os.adamperkowski.dev"),
        repo: Some("https://github.com/adamperkowski/highlightos"),
    },
    Project {
        name: "Jule",
        description: "Efficient, fast & reliable safe programming language",
        emoji: None,
        icon: Some("jule.svg"),
        banner: None,
        url: Some("https://jule.dev"),
        repo: None,
    },
    Project {
        name: "Linutil",
        description: "Distro-agnostic toolbox for simplifying Linux tasks",
        emoji: Some("🐧"),
        icon: None,
        banner: None,
        url: None,
        repo: Some("https://github.com/christitustech/linutil"),
    },
    Project {
        name: "snapbox",
        description: "HTTP client library for Jule",
        emoji: Some("🗳️"),
        icon: None,
        banner: None,
        url: Some("https://snapbox.adamperkowski.dev"),
        repo: Some("https://github.com/adamperkowski/snapbox"),
    },
    Project {
        name: "CLIQ",
        description: "CLI library for Jule",
        emoji: None,
        icon: Some("cliq.webp"),
        banner: None,
        url: None,
        repo: Some("https://github.com/adamperkowski/cliq"),
    },
    Project {
        name: "JuleProtonUp",
        description: "Fast and lightweight ProtonUp alternative",
        emoji: Some("⬆️"),
        icon: None,
        banner: None,
        url: None,
        repo: Some("https://github.com/adamperkowski/jpu"),
    },
    Project {
        name: "dwm",
        description: "My build of dwm + st + some dotfiles",
        emoji: Some("🪟"),
        icon: None,
        banner: None,
        url: None,
        repo: Some("https://github.com/adamperkowski/dwm"),
    },
    Project {
        name: "PKGBUILDs",
        description: "Sources of AUR packages I maintain",
        emoji: Some("📦"),
        icon: None,
        banner: None,
        url: Some("https://aur.archlinux.org/packages?K=adamperkowski&SeB=m"),
        repo: Some("https://github.com/adamperkowski/PKGBUILDs"),
    },
    Project {
        name: "This website",
        description: "The source code of this website",
        emoji: Some("🫨"),
        icon: None,
        banner: Some("websitestats.webp"),
        url: Some("/"),
        repo: Some("https://github.com/adamperkowski/website"),
    },
];
