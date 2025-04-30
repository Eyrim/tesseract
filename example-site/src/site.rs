use tesseract::{attributes::GlobalAttributes, body, div, html::{Div, HTMLElement}, tag};

#[derive(Default)]
pub struct Site {}

impl HTMLElement for Site {
    fn render(&self) -> String {
        tag!(
            name: "html",
            body: body!(
                Head::default().render(),
                //TODO: The use of the name `body` in the macros is confusing because of the tag
                //named body
                Body::default().render()
            )
        )
    }
}

#[derive(Default)]
pub struct Head {}

impl HTMLElement for Head {
    fn render(&self) -> String {
        tag!(
            name: "head",
            body: body!(
                tag!(
                    name: &GlobalAttributes::Title.to_string(),
                    body: "Example Page"
                ),
                tag!(
                    name: &GlobalAttributes::Lang.to_string(),
                    body: "en-gb"
                )
            )
        )
    }
}

#[derive(Default)]
pub struct Body {}

impl HTMLElement for Body {
    fn render(&self) -> String {
        tag!(
            name: "body",
            body: div!(
                tag!(
                    name: "p",
                    body: "Hi!"
                ),
                tag!(
                    name: "p",
                    body: "This is a test page!"
                )
            )
        )
    }
}

