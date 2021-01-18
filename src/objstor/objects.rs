use chrono::Utc;

pub struct Object {
    pub id: String,
    pub isdir: bool,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub mtime: Utc,
    pub text: String,
    pub md5: String,
    pub sha256: String,
}
