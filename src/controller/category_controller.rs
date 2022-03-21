use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{delete, get, post, put};
use serde::Deserialize;

use crate::model::category::Category;
use crate::response::resp_obj::RespData;
use crate::util::token::Token;

/// 全部分类
#[get("/categories")]
pub async fn list() -> Value {
    // 查询全部分类（带博客数量）
    let catevos = Category::find_all_with_blogcount().await;
    if let Ok(cates) = catevos {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: cates
        });
    }
    return json!(());
}

/// 分类详情
#[get("/category/<id>")]
pub async fn detail(id: &str) -> Value {
    let cate = Category::find_by_id(id).await;
    if let Ok(oc) = cate {
        return json!(RespData::<Option<Category>> {
            code: 200,
            msg: "Success",
            data: oc
        });
    }
    return json!(());
}

/// 获取分类下文章列表
#[get("/category/<cid>/artlist")]
pub async fn cate_artlist(cid: u32) -> Value {
    let articles = Category::find_articles_by_cateid(&cid.to_string()).await;
    if let Ok(arts) = articles {
        return json!(RespData {
            code: 200,
            msg: "Success",
            data: arts
        });
    }
    return json!(());
}

/// 删除分类
#[delete("/categories/<id>")]
pub async fn delete(id: &str, _t: Token) -> Value {
    let res = Category::remove(id).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "删除分类成功",
                data: ()
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "删除分类失败",
                data: ()
            });
        }
    }
}

/// 接收前端 post/put 提交的数据
#[derive(Deserialize)]
pub struct PostData<'a> {
    pub name: &'a str,
}

/// 新增分类
#[post("/categories", data = "<post_data>")]
pub async fn create(post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = post_data.name;
    let create_res = Category::create(name).await;
    if let Ok(res) = create_res {
        // 受影响行数
        let affected_row = res.rows_affected;
        // println!("{}", affected_row);
        // 返回的 id
        // let id = res.last_insert_id.unwrap();
        if affected_row > 0 {
            return json!(RespData {
                code: 200,
                msg: "新增分类成功",
                data: ()
            });
        }
        return json!(RespData {
            code: 500,
            msg: "新增分类失败",
            data: ()
        });
    }
    return json!(RespData {
        code: 500,
        msg: "新增分类失败",
        data: ()
    });
}

/// 更新分类
#[put("/categories/<id>", data = "<put_data>")]
pub async fn update(id: &str, put_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = put_data.name;
    let res = Category::update(id, name).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "更新分类成功",
                data: ()
            });
        }
        Err(_) => { 
            return json!(RespData {
                code: 500,
                msg: "更新分类失败",
                data: ()
            });
        }
    }
}