use std::env;
use std::fs;
use std::path::Path;

fn main() {
  let outdir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&outdir).join("bin_targets.rs");
  let mut bin_targets = String::new();


  let bin_dir = Path::new("src/bin");
  for entry in fs::read_dir(&bin_dir).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.extension().and_then(|s| s.to_str()) == Some("rs") {
      let file_stem = path.file_stem().unwrap().to_str().unwrap();
      bin_targets.push_str(&format!( r#"
        [[bin]]
        name = "{}"
        path = "src/bin/{}.rs"
        "#, file_stem, file_stem));
    }
  }
  fs::write(dest_path, bin_targets).unwrap();
}