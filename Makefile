.PHONY: ui fmt clippy test ci build clean

ui:
	cd admin-ui && pnpm install --frozen-lockfile && pnpm build

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::field_reassign_with_default -A clippy::large_enum_variant

test:
	cargo test --locked

ci: ui fmt clippy test

build: ui
	cargo build --release

clean:
	cargo clean
	cd admin-ui && rm -rf node_modules dist
