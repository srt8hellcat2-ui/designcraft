Open Microsoft Publisher (.pub) files as a converted copy

This adds a Publisher importer (`designcraft-pub`, layer 3) and opens Publisher 2002–2021 files through `file.open` and `file.openBytes`, the Open dialog and the web file picker. Each file opens as a new, unsaved copy. The `.pub` is only read, never written, and Save asks for a `.designcraft` file in the same folder. Every open returns `warnings` (what was lost), `notes` (other diagnostics, such as text that doesn't fit its frame) and a notice that this is a converted copy, shown above the document.

**What comes over:** pages and master pages; text frames with fonts by name, bold/italic, size, colour, caps, superscript/subscript, tracking, alignment, spacing, indents, bullets and line spacing; linked frames; rectangles, ovals, the simple preset shapes, lines, freeform curves and the heart preset; fills, gradients, fill transparency, line weight, dashes, arrowheads and the simple shadow; groups; PNG, JPEG, TIFF and bitmap pictures with crops; WMF and EMF pictures as SVG; WordArt as editable text (the default arch follows its curve); tables with merged cells and cell text.

**Not supported yet (each is reported as a warning or a note):** Publisher 97/98/2000 files (refused, with a message to re-save in a newer Publisher); table cell fills, borders and padding; WordArt warps other than the default arch; pattern, texture and picture fills; PICT pictures; text and bitmaps inside WMF/EMF; custom tab stops, numbering, and fields such as page numbers. Publisher's fonts are asked for by name, so text can set differently where they aren't installed.

**Clean-room provenance:** no other Publisher reader was used. Nobody working on it read libmspub, Apache POI's Publisher code or any other `.pub` reader's source. The notes in `docs/pub-format/` were written from the bytes of sample files and from public specifications (MS-CFB, MS-ODRAW, MS-WMF, MS-EMF, and two ECMA-376 shape-definition files). The Contents, text-store and bounded-reader modules were then rewritten from those notes alone. The scoring exports that come with the samples were used only to score, never to work out an encoding. No Publisher-made file is committed. The two test files are written byte for byte by the tests' own code, and a test checks they match.

**Test results:** upstream's gates pass (fmt, clippy, assets, layers, wasm, doctests and the workspace tests). The only failures are three UI tests that need a GPU or Japanese fonts; they fail the same way on unchanged main. On 142 pinned sample files kept outside the repository, the importer opens 142/142, with 111/111 positions within 1 pt, 52/52 texts and 111/111 styles. Of 24 real-world files, 19 open, 2 are refused (older Publisher) and 3 are reported as damaged. Every opened file saves, reopens, edits and exports to PDF.

Tested on Windows 11 by the author: opened Sample2.pub as a converted copy, edited it into a flyer (page 1) and a business card (page 2), and printed both; Linux and wasm builds pass upstream gates; not tested on macOS or other printers.

If you have Publisher 97, 98 or 2000 files that may be used for testing, please share them, so support for those versions can be added.

Refs #700
