use askama::Template;

#[derive(Template)]
#[template(path = "a.md")]
pub struct A {
    value: u32,
}
