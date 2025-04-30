[![Rust](https://github.com/Eyrim/tesseract/actions/workflows/rust.yml/badge.svg)](https://github.com/Eyrim/tesseract/actions/workflows/rust.yml)

# tesseract

React is a pretty awesome library, JavaScript is a pretty garbage language.

This isn't React, but the development flow of "create a component and chuck it around" is very much here.

Tesseract is a sort-of^\*^ server-side-rendering library to create web pages in Rust.
Tesseract is macro-driven and is intended to read as HTMl if done properly.
Conveniently, most good React practices apply here:
* For the love of god use components
* State is evil^\*\*^, so it's not really supported

\*
: It's more like one-time server side rendering (SSR), I.E. you build your app in rust and use my handy dandy macros to convert it to HTMl which you can then serve.
There is no reason you couldn't use this for real SSR, but currently you'd need to do some of your own plumbing; the macros produce `String`s at the end of the day.

\*\*
: State is fine, poor handling of state is the devil.
Currently there is no support for it, but there is planned support using a `javascript!()` macro, and potentially even `wasm!()`

