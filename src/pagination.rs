use mongodb::bson::{doc};
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationResponse<T> {
    pub(crate) meta: PaginationMeta,
    pub(crate) data: Vec<T>
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub(crate) total: u64,
    pub(crate) per_page: i64,
    pub(crate) current_page: i64,
    pub(crate) last_page: i64,
    pub(crate) first_page_url: String,
    pub(crate) last_page_url: String,
    pub(crate) next_page_url: Option<String>,
    pub(crate) previous_page_url: Option<String>,
}


pub struct PaginationBuilder<T> {
    data: Vec<T>,
    total: u64,
    page: i64,
    size: i64,
    base_url: String
}

impl<T> PaginationBuilder<T> {
    pub fn new(data: Vec<T>, total: u64, page: i64, size: i64, base_url: String) -> Self {
        PaginationBuilder {
            data,
            total,
            page,
            size,
            base_url,
        }
    }

    pub fn build_response(&self) -> PaginationResponse<T>
        where T: Clone
    {
        let last_page = (self.total as f64 / self.size as f64).ceil() as i64;

        let first_page_url = format!("{}?page=1&size={}", self.base_url, self.size);
        let last_page_url = format!("{}?page={}&size={}", self.base_url, last_page, self.size);
        let next_page_url = if self.page < last_page { Some(format!("{}?page={}&size={}", self.base_url, self.page + 1, self.size)) } else { None };
        let previous_page_url = if self.page > 1 { Some(format!("{}?page={}&size={}", self.base_url, self.page - 1, self.size)) } else { None };

        let meta = PaginationMeta {
            total: self.total as u64,
            per_page: self.size,
            current_page: self.page,
            last_page,
            first_page_url,
            last_page_url,
            next_page_url,
            previous_page_url,
        };

        PaginationResponse { meta, data: self.data.clone() }
    }
}
