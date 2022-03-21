use serde::Serialize;

/// 封装响应数据结结构体
#[derive(Serialize)]
pub struct RespData<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}

/// 封装响应数据结构体（带分页信息）
#[derive(Serialize)]
pub struct RespWithPagination<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
    pub current_page: u64,
    pub page_size: u64,
    pub total: u64,
}
