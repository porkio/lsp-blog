use crate::KEY;
use jwt_simple::prelude::Duration;
use jwt_simple::prelude::{Claims, MACLike, NoCustomClaims};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

/// 生成token函数
/// ---
/// @parameter      username        &str
/// @return         token           String
/// ---
pub fn create_token(username: &str) -> String {
    // 创建令牌
    let claims = Claims::create(Duration::from_hours(2)).with_issuer(username);
    KEY.authenticate(claims).unwrap()
}

// Token请求守卫
pub struct Token;

// 拦截请求进行token校验
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();
    // 从请求头中获取token并进行校验
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(token) = header_auth {
            // println!("请求携带的token为：{:?}", token);
            let claims_res = KEY.verify_token::<NoCustomClaims>(&token, None);
            if let Ok(claims) = claims_res {
                match claims.issuer {
                    // 验证通过
                    Some(_t) => return Outcome::Success(Token {}),
                    // 不通过
                    None => return Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}
