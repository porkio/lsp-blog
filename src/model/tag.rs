// 标签模型及关联函数
use crate::RB;
use chrono::Local;
use rbatis::crud::CRUD;
use rbatis::db::DBExecResult;
use rbatis::{crud_table, Error};

use super::article::Article;

// 标签表
#[crud_table(table_name:tag)]
#[derive(Clone, Debug)]
pub struct Tag {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

// 标签表VO
#[crud_table(table_name:tag)]
#[derive(Clone, Debug)]
pub struct TagVo {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub blog_count: Option<u32>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Tag {
    /// 查询全部标签（带文章数量）
    /// ---
    /// @return     Result<Vec<Tag>, Error>
    /// ---
    #[sql(
        RB, "SELECT `id`, `name`, `created_at`, `updated_at`, (SELECT COUNT(*) FROM `article` INNER JOIN `article_to_tag` on `article_id` WHERE `tag`.`id` = `article_to_tag`.`id`) AS `blog_count` FROM `tag`"
    )]
    pub async fn find_all_with_blogcount() -> Result<Vec<TagVo>, Error> {}

    /// 根据id查询标签
    /// ---
    /// @parameter      id      u32
    pub async fn find_by_id(id: u32) -> Result<Option<Tag>, Error> {
        let w = RB.new_wrapper().eq("id", id.to_string());
        RB.fetch_by_wrapper(w).await
    }

    /// 查询标签下的全部文章
    /// ---
    /// @parameter      id      &str
    /// @return         Result<Vec<Article, Error>
    /// ---
    #[sql(
        RB, "SELECT a.* FROM article a WHERE a.id IN (SELECT at2.article_id FROM article_tag at2 WHERE at2.tag_id = ?)"
    )]
    pub async fn find_articles_by_tagid(tag_id: &str) -> Result<Vec<Article>, Error> {}

    /// 新增标签
    /// ---
    /// @parameter      name        &str
    /// @return         Result<DBExecResult, Error>
    /// ---
    pub async fn create(name: &str) -> Result<DBExecResult, Error> {
        let tag = Tag {
            id: None,
            name: Some(name.to_string()),
            created_at: Some(Local::now().timestamp()),
            updated_at: Some(Local::now().timestamp()),
        };
        RB.save(&tag, &[]).await
    }

    /// 更新标签
    /// ---
    /// @parameter      id          u32
    /// @parameter      new_name    u32
    /// @return         Result<u64, Error>
    /// ---
    pub async fn update(id: u32, new_name: &str) -> Result<u64, Error> {
        let old_tag = Self::find_by_id(id).await;
        match old_tag {
            Ok(otag) => match otag {
                Some(t) => {
                    let ntag = Tag {
                        name: Some(new_name.to_string()),
                        updated_at: Some(Local::now().timestamp()),
                        ..t
                    };
                    let w = RB.new_wrapper().eq("id", id);
                    return RB.update_by_wrapper(&ntag, w, &[]).await;
                }
                None => return Err(Error::E("该标签不存在".to_string())),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// 删除标签
    /// ---
    /// @parameter      id      u32
    /// @return         Result<u64, Error>
    /// ---
    pub async fn remove(id: u32) -> Result<u64, Error> {
        let tag = Self::find_by_id(id).await;
        match tag {
            Ok(t1) => match t1 {
                Some(t2) => return RB.remove_by_column::<Tag, _>("id", t2.id).await,
                None => return Err(Error::E("该标签不存在".to_string())),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}
