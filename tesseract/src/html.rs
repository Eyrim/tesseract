macro_rules! properties {
    ($key:ident = $value:expr) => {
        format!("{}={} ", $key, $value)
    };
    ($($key:expr, $value:expr);+) => {
        {
            let mut v: Vec<String> = Vec::new();

            $(
                v.push(format!("{} = {}", $key, quotes!($value)));
            )+

           v.join(" ")
        }
    };
}

macro_rules! quotes {
    ($value:expr) => {
        format!(r#""{}""#, $value)
    };
}

macro_rules! tag {
    ($tag:expr, $properties:expr, $body:expr) => {
        format!("<{} {}>\n{}\n<{}/>", $tag, $properties, $body, $tag)
    };
}

pub trait HTMLElement {
    fn render(&self) -> String;
}

pub struct Head<'a> {
    pub title: &'a str,
    pub lang: &'a str,
}

impl HTMLElement for Head<'_> {
    fn render(&self) -> String {
        tag!(
            "head",
            properties!("title", self.title; "lang", self.lang),
            ""
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_head_tag() {
        let head = Head {
            title: "test tag",
            lang: "en-gb",
        };

        let expected = "<head title = \"test tag\" lang = \"en-gb\">\n\n<head/>";

        assert_eq!(head.render(), expected);
    }
}
