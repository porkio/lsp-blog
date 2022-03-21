use chrono::Local;
use rocket::form::FromForm;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{delete, get, post, put};
use serde::Deserialize;

use crate::util::token::Token;
use crate::model::article::{Article, ArticleForUpdateVo, ArticleVo};
use crate::response::resp_obj::{RespData, RespWithPagination};

/// 接收分页查询字符串的结构体
#[derive(Debug, PartialEq, FromForm)]
pub struct Page {
    page: u64,
    pageSize: u64,
}

/// 分页查询文章列表
#[get("/articles?<page..>")]
pub async fn list(page: Page) -> Value {
    // page_size 每页条数
    let page_size = page.pageSize;
    // current_page 当前页
    let current_page = page.page;
    // total 总记录数
    let total = Article::find_total().await.unwrap();

    // 分页查询（带分类信息）
    let page = (page.page - 1) * page_size;
    let articles = Article::find_all_by_pagination_with_category(&page.to_string(), &page_size.to_string()).await;
    if let Ok(arts) = articles {
        let data = RespWithPagination::<Vec<ArticleVo>> {
            code: 200,
            msg: "Success",
            data: arts,
            current_page: current_page,
            page_size: page_size,
            total: total,
        };
        return json!(data);
    }
    return json!(RespData {
        code: 500,
        msg: "error",
        data: (),
    });
}

/// 文章详情
#[get("/article/<id>")]
pub async fn detail(id: u32) -> Value {
    let article = Article::find_by_id(id).await;
    if let Ok(art) = article {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: art,
        });
    }
    return json!(RespData {
        code: 500,
        msg: "error",
        data: (),
    });
}

/// 正在编辑的文章详情
#[get("/article/edit/<id>")]
pub async fn editing_article_detail(id: &str) -> Value {
    let res = Article::find_editing_by_id(id).await;
    match res {
        Ok(op_art) => match op_art {
            Some(art) => {
                return json!(RespData {
                    code: 200,
                    msg: "Success",
                    data: art,
                });
            }
            None => {
                return json!(());
            }
        },
        Err(_) => {
            return json!(());
        }
    }
}

/// 最热文章
#[get("/article/hot")]
pub async fn hot() -> Value {
    let hots = Article::hot_list().await;
    if let Ok(hs) = hots {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: hs
        });
    }
    return json!(());
}

/// 删除文章
#[delete("/article/<id>")]
pub async fn delete(id: u32, _t: Token) -> Value {
    let res = Article::remove(id).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "删除文章成功",
                data: ()
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "删除文章失败",
                data: ()
            });
        }
    }
}

/// 接收前端post/put提交的数据
#[derive(Deserialize)]
pub struct PostOrPutArticleData {
    pub title: String,
    pub description: String,
    pub cate_id: u32,
    pub content: String,
    pub tags: Vec<u32>,
}

/// 新增文章
#[post("/article", data = "<post_data>")]
pub async fn create(post_data: Json<PostOrPutArticleData>, _t: Token) -> Value {
    let art = Article {
        id: None,
        title: Some(post_data.title.clone()),
        description: Some(post_data.description.clone()),
        content: Some(post_data.content.clone()),
        cate_id: Some(post_data.cate_id),
        istop: Some(false),
        created_at: Some(Local::now().timestamp()),
        updated_at: Some(Local::now().timestamp()),
    };

    // 标签 ids
    let tag_ids: Vec<u32> = post_data.tags.clone();

    // 执行添加
    let res = Article::add_article(art, tag_ids).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "新增文章成功",
                data: ()
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "新增文章失败",
                data: ()
            });
        }
    }
}

/// 更新文章
#[put("/article/<id>", data = "<put_data>")]
pub async fn update(id: u32, put_data: Json<PostOrPutArticleData>, _t: Token) -> Value {
    let art_edit_obj = ArticleForUpdateVo {
        title: put_data.title.clone(),
        description: put_data.description.clone(),
        content: put_data.content.clone(),
        cate_id: put_data.cate_id,
        tags: put_data.tags.clone(),
    };

    // 执行更新
    let res = Article::update(id, art_edit_obj).await;

    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "更新文章成功",
                data: (),
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "更新文章失败",
                data: (),
            });
        }
    }
}

/// 接收文章搜索的查询字符串结构题
#[derive(Debug, PartialEq, FromForm)]
pub struct SearchData {
    title: String,
    category: u32,
}

/// 文章搜索（后台）
#[get("/article/search?<sdata..>")]
pub async fn admin_search(sdata: SearchData) -> Value {
    let title = sdata.title;
    let category = sdata.category;

    let res = Article::admin_search(title, category).await;

    match res {
        Ok(arts) => {
            return json!(RespData {
                code: 200,
                msg: "Success",
                data: arts,
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "Failed",
                data: (),
            });
        }
    }
}

/// 文章搜索（前台）
#[get("/article/search/<keyword>")]
pub async fn search(keyword: &str) -> Value {
    let res = Article::search(keyword).await;

    match res {
        Ok(arts) => {
            return json!(RespData {
                code: 200,
                msg: "Success",
                data: arts,
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "Failed",
                data: (),
            });
        }
    }
}