use std::path::{Path, PathBuf};

pub struct P {
    pub r: String,
    pub s: String,
}

impl P {
    pub fn foo<P: AsRef<Path>>(&self, x: P) -> PathBuf {
        x.as_ref().to_path_buf().join(&self.r)
    }
}

fn main() {
    let p = P { r: "R".to_string(), s: "S".to_string() };
    let b = PathBuf::from(".");
    let ss = p.s.clone();
    let bb = p.foo(&b);
    println!("bb: {}", bb.display());
}
