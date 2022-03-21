mod controller;
mod model;
mod response;
mod util;

#[macro_use]
extern crate rbatis;

#[macro_use]
extern crate lazy_static;

use rbatis::rbatis::Rbatis;
use rocket::fairing::AdHoc;
use rocket::serde::json::{serde_json::json, Value};
use rocket::{catch, catchers, routes, Request};
use std::sync::Arc;

use jwt_simple::prelude::HS256Key;

use response::resp_obj::RespData;

use crate::controller::user_controller;
use crate::controller::category_controller;
use crate::controller::article_controller;


// 定义全局变量
lazy_static! {
    // Rbatis 类型变量 RB，用于数据库查询
    static ref RB: Rbatis = Rbatis::new();
    // HS256Key 类型变量 KEY，用于 jwt
    static ref KEY: HS256Key = HS256Key::generate();
}

// 404 catcher
#[catch(404)]
fn not_found(_req: &Request) -> Value {
    json!(RespData {
        code: 404,
        msg: "404 - Not Found!",
        data: ()
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启用日志输出
    // fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    // 初始化数据库连接池
    RB.link("mysql://root:12345678@127.0.0.1:3306/lsp_blog")
        .await
        .unwrap();

    let rb = Arc::new(&RB);

    rocket::build()
        .register("/", catchers![not_found])
        .mount(
            "/api",
            routes![
                user_controller::login,                     // 登录
                user_controller::list,                      // 用户列表
                user_controller::delete,                    // 删除用户
                category_controller::list,                  // 分类列表
                category_controller::detail,                // 分类详情
                category_controller::cate_artlist,          // 分类下的文章
                category_controller::create,                // 创建分类
                category_controller::update,                // 更新分类
                category_controller::delete,                // 删除分类
                article_controller::list,                   // 文章列表
                article_controller::detail,                 // 文章详情
                article_controller::editing_article_detail, // 编辑文章
                article_controller::hot,                    // 最热文章
                article_controller::delete,                 // 删除文章
                article_controller::create,                 // 新增文章
                article_controller::update,                 // 更新文章
                article_controller::admin_search,           // 搜索文章（后台）
                article_controller::search,                 // 搜索文章（前台）

            ],
        )
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await?;

    Ok(())
}
