# lysbilder<sup>1</sup>

## the slide system no one needed.

<sup>1</sup>: that's norwegian for _slides_.

no really, no one needs this.

[But it works](https://badboy.github.io/lysbilder/) (press Space to click through the slides).

## usage

create a new cargo library project with `cargo new --lib your-slide-deck`  
copy [`build.rs`](https://raw.githubusercontent.com/badboy/lysbilder/master/build.rs) and [`lysbilder.html`](https://raw.githubusercontent.com/badboy/lysbilder/master/lysbilder.html) into your project  
write your slides in `slides.md`. slides are separated by `---`  
build your slide deck: `cargo build`.

the output will be available in `doc` as HTML.

## how

`build.rs` parses your `slide.md` (and by *parse* I mean *split it at ---*),
generates [some Rust code](src/lib.rs) and then calls `rustdoc` to render it all.
the small javascript snippet in `lysbilder.html` takes care of handling key input on the web page.

## why

everyone should have build at least one slide system.
now i did.

## license

MIT. See [LICENSE](LICENSE).
