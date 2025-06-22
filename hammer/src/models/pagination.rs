use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct PaginatedResponse<T> {
    pub(crate) data: Vec<T>,
    pub(crate) total: usize,
    pub(crate) page: usize,
    pub(crate) per_page: usize,
    pub(crate) total_pages: usize,
}
