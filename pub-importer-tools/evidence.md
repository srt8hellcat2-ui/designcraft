Equivalence differences and causes
- SampleNewsletter.pub overset notes, wordart__arch.pub fitted size/baseline shift:
  fork with crates/fonts reverted to its adopted 0.12 base (no substitute.rs, no bundled
  Barlow/NiftyNarrowSerif) produces results identical to the port for all 166 files except
  fill__transparency-50 (cmp: dump-fork-nosub vs dump-port = 1 differ, fills only).
  fork vs fork-nosub differ exactly in those two files.
- fill__transparency-50.pub swatch name: "Publisher Transparent Fill {doc.swatches.len()}";
  fork new doc has 13 default swatches (incl. CMY mixes), upstream 10, so 13 vs 10. Gradient
  value identical (stops, colour, opacity 0.5).
