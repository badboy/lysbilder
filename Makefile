slides: doc/lib/index.html

doc/lib/index.html: slides.md
	cargo build

watch:
	cargo watch -x build -i src/lib.rs
.PHONY: watch

serve: slides
	echo "Serverd at http://localhost:8000/"
	cd doc && http
.PHONY: serve


clean:
	rm -r doc
.PHONY: clean
