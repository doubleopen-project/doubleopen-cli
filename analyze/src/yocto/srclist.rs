use serde::{Deserialize, Serialize};

/// # Srclist for Yocto recipe
///
/// [meta-doubleopen](https://github.com/doubleopen-project/meta-doubleopen) creates srclist files
/// for Yocto recipes to provide relationship between [binaries](Binary) built from the recipe and
/// the recipe's [source](Source) files.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Srclist {
    /// Binaries built from the corresponding Yocto recipe.
    pub binaries: Vec<Binary>,
}

/// Binary file in Yocto's [srclist](Srclist).
#[derive(Serialize, Deserialize, Debug)]
pub struct Binary {
    /// Path to the file.
    pub path: String,

    /// SHA256 hash value of the file.
    pub sha256: String,

    /// Source files used to build the binary.
    pub sources: Vec<Source>,
}

/// Source file in Yocto's [srclist](Srclist).
#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    /// Path to the file for Yocto's build environment.
    pub path: String,

    /// SHA256 hash value of the file.
    pub sha256: Option<String>,
}

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn deserialize_srclist() {
        let srclist = read_to_string("../tests/examples/analyze/acl.srclist.json").unwrap();
        let srclist: Srclist = serde_json::from_str(&srclist).unwrap();
        assert_eq!(srclist.binaries.len(), 4 as usize);
        assert_eq!(srclist.binaries[0].path, "/home/mikko/doubleopen/poky/build/tmp/work/core2-64-poky-linux/acl/2.2.52-r0/package/usr/bin/getfacl");
        assert_eq!(
            srclist.binaries[0].sha256,
            "cfa7a01ebec7f8ea896d7c869843b340fa941025e7f2f067ecf7a379eac59e50"
        );
        assert_eq!(srclist.binaries[0].sources.len(), 46);
        assert_eq!(
            srclist.binaries[0].sources[0].path,
            "/usr/src/debug/glibc/2.30-r0/git/sysdeps/x86_64/start.S"
        );
        assert_eq!(srclist.binaries[0].sources[0].sha256, None);
        assert_eq!(
            srclist.binaries[0].sources[3].path,
            "/usr/src/debug/acl/2.2.52-r0/acl-2.2.52/getfacl/getfacl.c"
        );
        assert_eq!(
            srclist.binaries[0].sources[3].sha256,
            Some("10c64c686dbb1c9fe571d104ccf200d061e74ea34fcdfa1bbd0203608a66084b".to_string())
        );
    }
}
