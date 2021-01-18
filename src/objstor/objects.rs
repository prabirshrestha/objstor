use chrono::Utc;

pub enum ObjectKind {
    Dir,
    Blob,
}

pub struct Object {
    pub id: String,
    pub pid: Option<String>,
    pub kind: ObjectKind,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub mtime: Utc,
    pub text: String,
    pub md5: String,
    pub sha256: String,
}
