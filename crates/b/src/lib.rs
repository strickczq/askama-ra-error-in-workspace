use askama::Template;

#[derive(Template)]
#[template(path = "b.md")]
pub struct B {
    value: u32,
}
