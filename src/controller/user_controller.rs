use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post};

use crate::model::user::User;
use crate::response::resp_obj::RespData;
use crate::util::token::{create_token, Token};

/// 接收前端post提交的用户名和密码
#[derive(Deserialize)]
pub struct PostData<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

/// 响应数据
#[derive(Serialize)]
pub struct LoginResData {
    pub username: String,
    pub token: String,
}

/// 用户登录
#[post("/admin/login", data = "<post_data>")]
pub async fn login(post_data: Json<PostData<'_>>) -> Value {
    // println!("username: {}, password: {}", post_data.username, post_data.password);
    // 从数据库中检查用户名和密码是否匹配
    let user = User::login(post_data.username, post_data.password).await;
    if let Ok(op_user) = user {
        if let Some(user) = op_user {
            let user_name = user.username.unwrap();
            // 生成token
            let token = create_token(&user_name);
            // 响应数据
            let data = LoginResData {
                username: user_name,
                token: token,
            };
            return json!(RespData {
                code: 200,
                msg: "Login Successful.",
                data: data,
            });
        }
        return json!(RespData {
            code: 400,
            msg: "Username or password error.",
            data: (),
        });
    }
    return json!(RespData {
        code: 400,
        msg: "Username or password error.",
        data: (),
    });
}

/// 用户列表
#[get("/users")]
pub async fn list() -> Value {
    let users = User::get_user_list().await;
    match users {
        Ok(users) => {
            return json!(RespData {
                code: 200,
                msg: "Get user list successful.",
                data: users,
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "Error",
                data: (),
            });
        }
    }
}

/// 删除用户
#[delete("/user/<id>")]
pub async fn delete(id: &str, _t: Token) -> Value {
    let res = User::delete_user(id).await;
    match res {
        Ok(_) => {
            return json!(RespData {
                code: 200,
                msg: "Delete user successful.",
                data: (),
            });
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "Error",
                data: (),
            });
        }
    }
}
