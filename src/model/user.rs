// 用户模型及关联函数
use crate::RB;
use rbatis::crud::CRUD;
use rbatis::Error;

use crypto::digest::Digest;
use crypto::md5::Md5;

// 分类表
#[crud_table(table_name:user)]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<u32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
}

// 分类表Vo
#[crud_table(table_name:user)]
#[derive(Clone, Debug)]
pub struct UserVo {
    pub id: Option<u32>,
    pub username: Option<String>,
    pub nickname: Option<String>,
}

/// md5 加密函数
/// ---
/// @parameter  input           S
/// @return     encodeString    String
/// ---
fn md5<S: Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

impl User {
    /// 登录
    /// ---
    /// @parameter  username    &str
    /// @parameter  password    &str
    /// @return     Result<User, Error>
    /// ---
    pub async fn login(username: &str, password: &str) -> Result<Option<User>, Error> {
        let encode_pwd = md5(password);
        println!("加密后的密码： {}", encode_pwd);
        let w = RB
            .new_wrapper()
            .eq("username", username)
            .and()
            .eq("password", encode_pwd);
        RB.fetch_by_wrapper(w).await
    }

    /// 获取用户列表
    /// ---
    /// @return     Result<Vec<User>, Error>
    /// ---
    pub async fn get_user_list() -> Result<Vec<UserVo>, Error> {
        RB.fetch_list::<UserVo>().await
    }

    /// 删除用户
    /// ---
    /// @return     Result<u64, Error>
    /// ---
    pub async fn delete_user(id: &str) -> Result<u64, Error> {
        RB.remove_by_column::<User, _>("id", id).await
    }
}
