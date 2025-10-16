use serde::Serialize;

#[derive(Serialize)]
pub struct Badge {
    pub src: &'static str,
    pub alt: &'static str,
    pub href: Option<&'static str>,
}

pub const BADGES: &[Badge] = &[Badge {
    src: "/img/badges/mimi-the-car.gif",
    alt: "mimi the car (adam's site)",
    href: Some(crate::constants::HOST),
}];
