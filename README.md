# nanodom

a _super_ simple DOM/tree implemention built on top of [`quick-xml`](https://crates.io/crates/quick-xml).

meant to be as ergnomic as possible (parsing directly to and from `&str`), and allow maximal flexibility

inspired by [`minidom`](https://crates.io/crates/minidom), but is meant to do even less.

## why?

if you've ever tried to parse something like `<asdf></asdf>` in `minidom`, you'll get the error `MissingNamespace`.
This is probably a good thing, but I didn't want to deal with `xmlns=` attributes.

I also wanted more direct access to the resulting tree.
`nanodom` gives you direct access to the structs and underlying implementation.

finally, I wanted the nicety of `impl Display` to turn the tree back into a `String`.
