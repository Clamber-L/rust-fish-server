use sea_orm::ItemsAndPagesNumber;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    #[serde(rename = "page")]
    pub page_num: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    #[serde(rename = "totalPage")]
    pub total_page: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    #[serde(rename = "items")]
    pub items: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(
        page_num: u64,
        page_size: u64,
        items_and_pages_number: ItemsAndPagesNumber,
        items: Vec<T>,
    ) -> Self {
        Self {
            page_num,
            page_size,
            total_page: items_and_pages_number.number_of_pages,
            total_size: items_and_pages_number.number_of_items,
            items,
        }
    }
}
