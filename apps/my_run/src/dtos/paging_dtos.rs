use serde::{Serialize, Deserialize};

/// ----------
/// Request DTO
/// ----------
#[derive(Clone, Deserialize, Debug)]
pub struct PagingQueryDTO<T> {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub data: Option<T>,
}

impl<T> PagingQueryDTO<T> {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn get_page_size(&self) -> u32 {
        self.page_size.unwrap_or(10)
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

/// ----------
/// Response DTO
/// ----------
#[derive(Clone, Serialize, Debug)]
pub struct PagingDto<T> {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_count: u32,
    pub data: Vec<T>,
}


impl<T> PagingDto<T> {
    pub fn new(data: Vec<T>, current_page: u32, total_count: u32, page_size: u32) -> Self {
        let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;
        PagingDto {
            current_page,
            total_pages,
            total_count,
            data,
        }
    }
}
