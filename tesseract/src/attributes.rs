/// These are the global attributes, those which apply to every element within an HTML file.
///
/// See [Global Attributes on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes)
pub enum GlobalAttributes {
    /// Provides a hint for generating the keyboard shortcut for an element, contains a single printable character
    ///
    /// See [Access Key on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/accesskey)
    AccessKey,
    /// Controls how text is automatically capitalized on non-physical input devices. I.E virtual keyboards on a mobile phone
    ///
    /// See [Autocapitalize on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autocapitalize)
    AutoCapitalize,
    /// The language of the element, defaults to an empty string (language unknown) so should always be set.
    /// Uses the format specified in [RFC-5646](https://datatracker.ietf.org/doc/html/rfc5646)
    ///
    /// See [Lang on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/lang)
    Lang,
    /// Contains [CSS Styling Declarations](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference) to apply to the element
    ///
    /// See [Style on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/style)
    Style,
    /// The title of the element.
    ///
    /// Important to note is that this should be used on all iframe's to provide information about
    /// the content to screen readers
    ///
    /// See [Title on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/title)
    Title,
}
