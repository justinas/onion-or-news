all: release

release:
	cd oon_web && eslint .
	cargo clippy
	cargo build --release
	tar -cz -f release.tar.gz \
		-C $(PWD)/target/release oon_scraper oon_web \
		-C $(PWD)/oon_web static

.PHONY: release
