use crate::model::tag::Tag;
use crate::util::token::Token;
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};
use rocket::{delete, get, post, put};
use serde::Deserialize;

use crate::response::resp_obj::RespData;

/// 全部标签
#[get("/tags")]
pub async fn list() -> Value {
    let tags = Tag::find_all_with_blogcount().await;
    if let Ok(ts) = tags {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: ts,
        });
    }
    return json!(());
}

/// 某个标签下文章列表
#[get("/tag/<tid>/articles")]
pub async fn tag_articles(tid: u32) -> Value {
    let articles = Tag::find_articles_by_tagid(&tid.to_string()).await;
    if let Ok(arts) = articles {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: arts,
        });
    }
    return json!(());
}

/// 接收前端post/put提交的数据
#[derive(Deserialize)]
pub struct PostData<'a> {
    pub name: &'a str,
}

/// 新增标签
#[post("/tags", data = "<post_data>")]
pub async fn create(post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = post_data.name;
    let tag_res = Tag::create(name).await;
    if let Ok(res) = tag_res {
        // 受影响行数
        let affected_rows = res.rows_affected;
        if affected_rows > 0 {
            return json!(RespData {
                code: 200,
                msg: "新增标签成功",
                data: (),
            });
        }
        return json!(RespData {
            code: 500,
            msg: "新增标签失败",
            data: (),
        });
    }
    return json!(RespData {
        code: 500,
        msg: "新增标签失败",
        data: (),
    });
}

/// 更新标签
#[put("/tags/<id>", data = "<put_data>")]
pub async fn update(id: u32, put_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = put_data.name;
    let res = Tag::update(id, name).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "更新标签成功",
                data: (),
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "更新标签失败",
                data: (),
            });
        }
    }
}

/// 删除标签
#[delete("/tags/<id>")]
pub async fn remove(id: u32, _t: Token) -> Value {
    let res = Tag::remove(id).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "删除标签成功",
                data: (),
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "删除标签失败",
                data: (),
            });
        }
    }
}
