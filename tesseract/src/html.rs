extern crate html_element;

pub trait HTMLElement {
    fn render() -> String;
}

#[derive(html_element::HTMLElement)]
pub struct Head<'a> {
    pub title: &'a str,
    pub lang: &'a str,
}

impl HTMLElement for Head<'_> {
    fn render() -> String {
        todo!()
    }
}

