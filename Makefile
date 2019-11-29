all: release

release:
	cd oon_web && eslint .
	cargo clippy
	cargo fmt --all -- --check
	cargo build --release
	tar -cz -f release.tar.gz \
		-C $(PWD)/target/release oon_scraper oon_web \
		-C $(PWD)/oon_web static

setup-hooks:
	ln -f -s $(PWD)/pre-commit .git/hooks/pre-commit

.PHONY: release setup-hooks
