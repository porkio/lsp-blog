// 文章模型及关联函数

use crate::RB;
use rbson::bson;
use chrono::Local;
use rbatis::crud::CRUD;
use rbatis::db::DBExecResult;
use rbatis::executor::Executor;
use rbatis::executor::ExecutorMut;
use rbatis::Error;

// 文章表
#[crud_table(table_name:article)]
#[derive(Clone, Debug)]
pub struct Article {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<u32>,
    pub istop: Option<bool>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

// 文章表对应的输出对象vo
#[crud_table(table_name:article)]
#[derive(Clone, Debug)]
pub struct ArticleVo {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<u32>,
    pub cate_name: Option<String>,
    pub istop: Option<bool>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

// 文章表对应的文章编辑时输出对象vo
#[crud_table(table_name:article)]
#[derive(Clone, Debug)]
pub struct ArticleEditVo {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<u32>,
    pub cate_name: Option<String>,
    pub is_top: Option<bool>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

// 封装前端提交过来的数据
pub struct ArticleForUpdateVo {
    pub title: String,
    pub description: String,
    pub content: String,
    pub cate_id: u32,
    pub tags: Vec<u32>,
}

impl Article {
    /// 获取全部记录数量
    pub async fn find_total() -> Result<u64, Error> {
        RB.fetch_count::<Article>().await
    }

    /// 分页查询文章列表
    #[sql(
        RB,
        "select a.*,c.name as cate_name from article a, category c WHERE a.cate_id = c.id limit ?,?"
    )]
    pub async fn find_all_by_pagination_with_category(
        page: &str,
        per_page: &str,
    ) -> Result<Vec<ArticleVo>, Error> {}

    /// 查询全部文章（带分类信息）
    #[sql(
        RB,
        "select a.*,c.name as cate_name from article a, category c WHERE a.cate_id = c.id"
    )]
    pub async fn find_all_with_category() -> Result<Vec<ArticleVo>, Error> {}

    /// 根据id查询文章详情
    pub async fn find_by_id(id: u32) -> Result<Option<Article>, Error> {
        let w = RB.new_wrapper().eq("id", id);
        RB.fetch_by_wrapper(w).await
    }

    /// 根据id查询正在编辑的文章
    #[sql(
        RB,
        "SELECT a.id,a.title,a.description,a.content,a.cate_id,GROUP_CONCAT(att.tag_id) as tags FROM article a LEFT JOIN article_to_tag att ON a.id = att.article_id WHERE a.id = ? GROUP BY a.id;"
    )]
    pub async fn find_editing_by_id(id: &str) -> Result<Option<ArticleEditVo>, Error> {}

    /// 文章详情（带分类信息）
    #[sql(
        RB,
        "SELECT a.*,c.name as cate_name FROM article a, category c WHERE a.cate_id = c.id AND a.id = ?"
    )]
    pub async fn find_by_id_with_category(id: &str) -> Result<Option<ArticleVo>, Error> {}

    /// 最热文章
    pub async fn hot_list() -> Result<Vec<Article>, Error> {
        let w = RB.new_wrapper().order_by(false, &["created_at"])
            .push_sql("limit 9");
        RB.fetch_list_by_wrapper(w).await
    }

    /// 前台文章搜索
    pub async fn search(keyword: &str) -> Result<Vec<Article>, Error> {
        let w = RB.new_wrapper().like("title", keyword);
        RB.fetch_list_by_wrapper(w).await
    }

    /// 新增文章
    pub async fn add_article(article: Article, tag_ids: Vec<u32>) -> rbatis::core::Result<()> {
        // 创建事务对象
        let mut tx = RB.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                // println!("事务回滚成功");
            } else {
                // println!("不需要回滚");
            }
        });

        // 向文章表插入数据
        let art_res = tx.exec("INSERT INTO article (title, description, content, cate_id, istop, created_at, updated_at) VALUES (?,?,?,?,?,?,?);",
            vec![
                bson!(article.title), 
                bson!(article.description), 
                bson!(article.content),
                bson!(article.cate_id),
                bson!(article.istop),
                bson!(article.created_at),
                bson!(article.updated_at),
            ]).await;
        
        // 获取新添加文章的id
        match art_res {
            Ok(res) => {
                let new_art_id = res.last_insert_id;
                // 向article_to_tag表中插入数据
                for i in tag_ids.iter() {
                    tx.exec(
                        "INSERT INTO article_to_tag (article_id, tag_id) VALUES (?,?);",
                        vec![bson!(new_art_id), bson!(*i)],
                    ).await;
                }
            }
            Err(e) => {
                tx.rollback().await;
                return Err(e);
            }
        }
        // 提交事务
        tx.commit().await;
        return Ok(());
    }

    /// 删除文章
    pub async fn remove(id: u32) -> rbatis::core::Result<()> {
        let art = Self::find_by_id(id).await;
        // println!("删除查询到的文章: {:?}", art);
        match art {
            Ok(a1) => match a1 {
                Some(a2) => {
                    // 创建事务对象
                    let mut tx = RB.acquire_begin().await?.defer_async(|mut tx1| async move {
                        if !tx1.is_done() {
                            tx1.rollback().await;
                        } else {}
                    });

                    // 事务1: 删除article_to_tag表中关联文章的数据
                    tx.exec(
                        "DELETE FROM article_to_tag WHERE article_id = ?;",
                        vec![bson!(a2.id)],
                    ).await;
                    
                    // 事务2: 删除article表中对应id文章
                    tx.exec(
                        "DELETE FROM article WHERE id = ?;",
                        vec![bson!(a2.id)]
                    ).await;

                    // 提交事务
                    tx.commit().await;
                    return Ok(());
                }
                None => return Err(Error::E("文章不存在".to_string())),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// 更新文章
    pub async fn update(id: u32, put_art: ArticleForUpdateVo) -> rbatis::core::Result<()> {
        let old_art = Self::find_by_id(id).await;
        match old_art {
            Ok(oart) => match oart {
                Some(a) => {
                    let tags = put_art.tags;
                    
                    // 创建事务对象
                    let mut tx = RB.acquire_begin().await?.defer_async(|mut tx1| async move {
                        if !tx1.is_done() {
                            tx1.rollback().await;
                        } else {}
                    });

                    // 事务1: 删除article_to_tag表中关联数据
                    tx.exec(
                        "DELETE FROM article_to_tag WHERE article_id = ?;",
                        vec![bson!(a.id)],
                    ).await;

                    // 事务2: 添加article_to_tag关联关系
                    for i in tags {
                        tx.exec(
                            "INSERT INTO article_to_tag VALUES (?,?);",
                            vec![bson!(a.id), bson!(i)],
                        ).await;
                    }

                    // 事务3: 更新article表
                    tx.exec(
                        "UPDATE article SET title = ?, description = ?, content = ?, cate_id = ?, updated_at = ?, WHERE id = ?;",
                        vec![
                            bson!(put_art.title),
                            bson!(put_art.description),
                            bson!(put_art.content),
                            bson!(put_art.cate_id),
                            bson!(Local::now().timestamp()),
                            bson!(a.id)
                        ]
                    ).await;

                    // 提交事务
                    tx.commit().await;
                    return Ok(());
                }
                None => return Err(Error::E("该分类不存在".to_string())),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// 后台文章搜索
    pub async fn admin_search(title: String, category: u32) -> Result<Vec<Article>, Error> {
        let w;
        if title == "".to_string() && category != 0 {
            w = RB.new_wrapper().eq("cate_id", category);
            return RB.fetch_list_by_wrapper(w).await;
        }

        if title == "".to_string() && category == 0 {
            return RB.fetch_list().await;
        }

        if title != "".to_string() && category == 0 {
            w = RB.new_wrapper().like("title", title);
            return RB.fetch_list_by_wrapper(w).await;
        }

        if title != "".to_string() && category != 0 {
            w = RB.new_wrapper()
                .like("title", title)
                .eq("cate_id", category);
            return RB.fetch_list_by_wrapper(w).await;
        }

        // 全参数搜索查询
        // sql:  "SELECT a.id,a.title,a.description,a.content,a.cate_id,a.istop,a.created_at,a.updated_at,c.name AS cate_name FROM article a,category c,(SELECT * FROM article a1 INNER JOIN article_to_tag att1 ON a1.id = att1.article_id WHERE att1.tag_id = ?) as z WHERE a.title LIKE '%?%' AND c.id = ? GROUP BY a.id;"
        return RB.fetch_list().await;
    }
}
