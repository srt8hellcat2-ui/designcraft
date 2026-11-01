# Publisher importer: resume note

Scratch tools for checking branch `pub-importer` (8 commits, `112f1c51..50366c58`, on upstream
main `008f519c`). They are not part of the pull request.

## Files
- `corpus.rs`: the fork's corpus scorer (niftydesign `crates/pub/tests/corpus.rs`), with
  `layer_options: None,` added for upstream's `Graphic`. Copy to `crates/pub/tests/zz_corpus.rs`.
- `dump.rs`: writes one JSON per sample (ok/error, warnings, notes, document). Copy to
  `crates/pub/tests/zz_dump.rs` (needs `PUB_DIRS`, `PUB_DUMP`).
- `cmp.py DIR_A DIR_B`: compares two dump folders (ok, error, warnings, notes, stories, fills).
- `gates.sh REPO TARGET_DIR OUT`: runs the `cargo xtask ci` steps one by one, tests with
  `--no-fail-fast`.
- `evidence.md`: causes of the three fork/branch differences.

## Results so far
- Full gates on commits 2, 4, 6: all steps pass except 3 ui-egui tests, which also fail on
  unchanged upstream main: `canvas::tests::live_corner_widget_and_diamonds_draw` (no GPU
  adapter), `canvas::tests::a_later_field_taking_the_keyboard_ends_the_composition_in_that_pass`
  and `panels::font_menu_ui_tests::the_class_filter_and_similar_fonts_narrow_the_list` (no
  Japanese font; CRAFT_FONTS_DIR unset).
- Commits 7, 8: fmt, clippy and the engine/UI Publisher tests only.
- At commit 6, 166/166 imports identical to a single-step port of the whole importer except the
  creation timestamp. Score identical to the fork: 142/142, 152/152, 111/111, 52/52, 111/111,
  pictures 12/41 (29 unmatched); real batch 9 notes / 10 warnings / 2 refused / 3 damaged;
  save/reopen/edit/PDF on 161 files. Three differences, causes in `evidence.md`.

## Not done
1. Full CI on the final branch:
   `cargo xtask ci`, then `cargo test --workspace --lib --bins --tests --no-fail-fast`, then
   `cargo test --workspace --doc && cargo xtask i18n-report && cargo xtask assets && cargo xtask layers && cargo xtask wasm`
2. v0.15.2 proof:
   `git remote add upstream https://github.com/storytold/designcraft.git && git fetch upstream main --tags`
   `git worktree add ../wt-0152 v0.15.2 && cd ../wt-0152 && git cherry-pick 112f1c51..50366c58`
   `cargo check --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p designcraft-pub && cargo test -p designcraft-engine --test pub_workflow --test web_pub`
3. Final scoring (after `bash scripts/restore-niftydata.sh` in niftydesign):
   `PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures cargo test -p designcraft-pub --test zz_corpus -- --ignored --nocapture`
   `PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures PUB_REAL_CORPUS=/tmp/niftydata/apache-pub PUB_POI_TABLES=/tmp/niftydata/apache-pub cargo test -p designcraft-pub -p designcraft-ui-egui -- --ignored --skip write_`
4. Headless open through the CLI:
   `cargo run -p designcraft-cli -- run --in crates/pub/tests/data/hello.pub --export /tmp/hello.png`
5. `cargo check -p designcraft-web --target wasm32-unknown-unknown`; pixel comparison; Windows,
   macOS, native dialogs, real Microsoft fonts.

## Recommended pull request
Title: Open Microsoft Publisher (.pub) files as a converted copy. Description: see the final
report of the session (scope supported / not supported, clean-room provenance, data-file note,
request for Publisher 97/98/2000 samples).
