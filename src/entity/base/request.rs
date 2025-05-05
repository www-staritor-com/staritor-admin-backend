use rocket::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PageRequest<T> {
    page: u64,
    size: u64,
    pub select: Option<T>,
}

impl<T> PageRequest<T> {
    pub fn page(&self) -> u64 {
        self.page - 1
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
