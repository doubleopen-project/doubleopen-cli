struct InputFile {
    sha256: String,
}

struct OutputFile {
    hash: Hash,
}

struct Hash {
    sha1: String,
    md5: String,
    sha256: String,
}

struct Findigs {
    scanner: Vec<String>,
    conclusion: Vec<String>,
}
