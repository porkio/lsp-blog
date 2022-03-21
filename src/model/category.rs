// 分类模型及关联函数

use crate::RB;
use chrono::Local;
use rbatis::crud::CRUD;
use rbatis::db::DBExecResult;
use rbatis::sql;
use rbatis::Error;

use super::article::Article;

// 分类表
#[crud_table(table_name:category)]
#[derive(Clone, Debug)]
pub struct Category {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

// 分类表Vo
#[crud_table(table_name:category)]
#[derive(Clone, Debug)]
pub struct CategoryVo {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub blog_count: Option<u32>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Category {
    /// 查询全部分类（带文章数量）
    /// ---
    /// @return     Result<Vec<CategoryVo>, Error>
    /// ---
    #[sql(
        RB,
        "SELECT id, name, created_at, updated_at, (select count(*) FROM article WHERE category.id = article.cate_id) as blog_count from category"
    )]
    pub async fn find_all_with_blogcount() -> Result<Vec<CategoryVo>, Error> {}

    /// 根据id查询分类
    /// ---
    /// @parameter      id          &str
    /// @return         Result<Option<Category>, Error>
    /// ---
    pub async fn find_by_id(id: &str) -> Result<Option<Category>, Error> {
        let w = RB.new_wrapper().eq("id", id);
        RB.fetch_by_wrapper(w).await
    }

    /// 查询分类下的全部文章
    /// ---
    /// @parameter      cate_id     &str
    /// @return         Result<Vec<Article>, Error>
    #[sql(
        RB,
        "SELECT a.* FROM article a, category c WHERE a.cate_id = c.id AND c.id = ?"
    )]
    pub async fn find_articles_by_cateid(cate_id: &str) -> Result<Vec<Article>, Error> {}

    /// 新增分类
    /// ---
    /// @parameter      name        &str
    /// @return         Result<DBExecResult, Error>
    /// ---
    pub async fn create(name: &str) -> Result<DBExecResult, Error> {
        let cate = Category {
            id: None,
            name: Some(name.to_string()),
            created_at: Some(Local::now().timestamp()),
            updated_at: Some(Local::now().timestamp()),
        };
        RB.save(&cate, &[]).await
    }

    /// 更新分类
    /// ---
    /// @parameter      id              &str
    /// @parameter      new_name        &str
    /// @return         Result<u64, Error>
    /// ---
    pub async fn update(id: &str, new_name: &str) -> Result<u64, Error> {
        let old_cate = Self::find_by_id(id).await;
        match old_cate {
            Ok(ocate) => match ocate {
                Some(c) => {
                    let ncate = Category {
                        name: Some(new_name.to_string()),
                        updated_at: Some(Local::now().timestamp()),
                        ..c
                    };
                    let w = RB.new_wrapper().eq("id", id);
                    return RB.update_by_wrapper(&ncate, w, &[]).await
                }
                None => return Err(Error::E("该分类不存在".to_string()))
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// 删除分类
    /// ---
    /// @parameter      id      &str
    /// @return Result<u64, Error>
    /// ---
    pub async fn remove(id: &str) -> Result<u64, Error> {
        let cate = Self::find_by_id(id).await;
        match cate {
            Ok(c1) => match c1 {
                Some(c2) => return RB.remove_by_column::<Category, _>("id", c2.id).await,
                None => return Err(Error::E("该分类不存在".to_string())),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}
