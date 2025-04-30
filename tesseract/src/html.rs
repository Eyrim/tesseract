/// Surround an expression in quotes, useful for building HTML elements
///
/// quotes!("hi") --> ""hi""
/// quotes!(9) --> "9"
#[macro_export]
macro_rules! quotes {
    ($value:expr) => {
        format!(r#""{}""#, $value)
    };
}

#[cfg(test)]
mod quotes_tests {
    #[test]
    fn should_wrap_i32_in_quotes() {
        let value: i32 = 4;

        let expected = format!("\"{}\"", value);

        let actual = quotes!(value);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_wrap_str_in_quotes() {
        let value = "value";

        let expected = format!("\"{}\"", value);

        let actual = quotes!(value);

        assert_eq!(actual, expected);
    }
}

/// Create HTML properties for a specfic tag
///
/// These properties take the form of
/// `key = "value" second_key = "new_value"`
///
/// See [tag] for a macro to create a full HTML tag, with a body and a closing tag
#[macro_export]
macro_rules! properties {
    ($key:expr, $value:expr) => {
        format!("{}={}", $key, quotes!($value))
    };
    ($($key:expr, $value:expr);+) => {
        {
            let mut v: Vec<String> = Vec::new();

            $(
                v.push(properties!($key, $value));
            )+

           v.join(" ")
        }
    };
}

#[cfg(test)]
mod properties_tests {
    #[test]
    fn should_build_single_property() {
        let key = "key";
        let value = "value";

        let expected = format!("{}=\"{}\"", key, value);

        let actual = properties!(key, value);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_build_multiple_properties() {
        let key_one = "key_one";
        let value_one = "value_one";
        let key_two = "key_two";
        let value_two = "value_two";

        let expected = format!(
            "{}=\"{}\" {}=\"{}\"",
            key_one, value_one, key_two, value_two
        );

        let actual = properties!(key_one, value_one; key_two, value_two);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_build_property_of_i32() {
        let key = "key";
        let value: i32 = 4;

        let expected = format!("{}=\"{}\"", key, value);

        let actual = properties!(key, value);

        assert_eq!(actual, expected);
    }
}

#[macro_export]
macro_rules! body {
    ($content:expr) => {
        format!("{}", $content)
    };
    ($($body:expr),+) => {
        {
            let mut bodies: Vec<String> = Vec::new();

            $(
                bodies.push(body!($body));
            )+

            bodies.join("\n")
        }
    }
}

//TODO: maybe this could generate a struct and we can then directly pass in HTMLElements without
//having to use ::default().render() each time to get the strings

/// Build a full HTML tag, with a body and [properties]
#[macro_export]
macro_rules! tag {
    (name: $tag:expr, properties: $properties:expr, body: $body:expr) => {
        format!(
            "{}\n{}\n</{}>",
            $crate::html::create_opening_tag($tag, $properties),
            body!($body),
            $tag,
        )
    };
    (name: $tag:expr, body: $body:expr) => {
        format!(
            "{}\n{}\n</{}>",
            $crate::html::create_opening_tag($tag, ""),
            $crate::body!($body),
            $tag,
        )
    };
}

#[cfg(test)]
mod tag_tests {
    use crate::attributes::GlobalAttributes;

    #[test]
    fn should_create_tag() {
        let class_name = "class_name";
        let tag_name = "div";
        let body = "test";

        let expected = format!(
            "<{} {}=\"{}\">\n{}\n</{}>",
            tag_name,
            GlobalAttributes::Class,
            class_name,
            body,
            tag_name
        );

        let actual = tag!(
            name: tag_name,
            properties: properties!(GlobalAttributes::Class, class_name).as_str(),
            body: body
        );

        assert_eq!(actual, expected);
    }
}

macro_rules! self_closing_tag {
    (name: $tag:expr, properties: $properties:expr) => {
        format!("{} />", $crate::html::create_opening_tag($tag, $properties))
    };
}

pub fn create_opening_tag(tag_name: &str, properties: &str) -> String {
    if properties.is_empty() {
        format!("<{}>", tag_name)
    } else {
        format!("<{} {}>", tag_name, properties)
    }
}

pub trait HTMLElement {
    fn render(&self) -> String;
}

#[derive(Default, Debug)]
pub struct Head<'a> {
    pub title: &'a str,
    pub lang: &'a str,
}

impl HTMLElement for Head<'_> {
    fn render(&self) -> String {
        tag!(
            name: "head",
            properties: "",
            body: body!(
                tag!(name: "title", body: "test page"), tag!(name: "lang", body: "en-gb")
            )
        )
    }
}

#[cfg(test)]
mod head_test {
    use super::{HTMLElement, Head};

    #[test]
    fn should_build_head_with_title_and_lang() {
        let expected = "<head>\n<title>\ntest page\n</title>\n<lang>\nen-gb\n</lang>\n</head>";

        let actual = Head::default().render();

        assert_eq!(actual, expected);
    }
}

#[macro_export]
macro_rules! div {
    ($content:expr) => {{ $crate::html::Div { content: $content }.render() }};
    ($($content:expr),+) => {
        $crate::n_strings!(
            $crate::div!($($content)+, "\n")
        )
    }
}

#[macro_export]
macro_rules! n_strings {
    ($builder:ident, $($value:expr),+, $delimiter:expr) => {
        {
            let mut v: Vec<String> = Vec::new();

            $(
                v.push($ident($($value)+));
            )+

           v.join($delimiter)
        }
    };
    ($($value:expr),+, $delimiter:expr) => {
        {
            let mut v: Vec<String> = Vec::new();

            $(
                v.push($($value)+);
            )+

           v.join($delimiter)
        }
    };
}

pub struct Div<'a> {
    pub content: &'a str,
}

impl HTMLElement for Div<'_> {
    fn render(&self) -> String {
        tag!(
            name: "div",
            body: self.content
        )
    }
}
