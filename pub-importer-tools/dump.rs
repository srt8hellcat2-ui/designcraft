//! Scratch equivalence dumper (never committed).
#[test]
#[ignore]
fn dump_every_import() {
    let out = std::path::PathBuf::from(std::env::var("PUB_DUMP").unwrap());
    std::fs::create_dir_all(&out).unwrap();
    for dir in std::env::var("PUB_DIRS").unwrap().split(':') {
        let tag = std::path::Path::new(dir).file_name().unwrap().to_string_lossy().to_string();
        let mut files: Vec<_> = std::fs::read_dir(dir).unwrap().flatten().map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("pub"))).collect();
        files.sort();
        for f in files {
            let bytes = std::fs::read(&f).unwrap();
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let v = match designcraft_pub::import_pub(&bytes) {
                Ok(i) => serde_json::json!({"ok": true, "warnings": i.warnings, "notes": i.notes, "doc": serde_json::to_value(&i.document).unwrap()}),
                Err(e) => serde_json::json!({"ok": false, "error": e.to_string(), "kind": format!("{e:?}").split('(').next().unwrap_or("").to_string()}),
            };
            std::fs::write(out.join(format!("{tag}__{name}.json")), serde_json::to_vec_pretty(&v).unwrap()).unwrap();
        }
    }
}
