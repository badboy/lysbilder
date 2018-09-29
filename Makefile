slides: src/lib.rs
	rustdoc --html-in-header lysbilder.html src/lib.rs

src/lib.rs: slides.md
	ruby build.rb slides.md > src/lib.rs

watch:
	cargo watch -s 'make slides'
.PHONY: watch

serve: slides
	echo "Serverd at http://localhost:8000"
	cd doc && http
.PHONY: serve
