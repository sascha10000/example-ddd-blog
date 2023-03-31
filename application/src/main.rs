mod dto;
mod services;

use infrastructure::repositories::MongoArticleRepository;
use warp::Filter;

#[tokio::main]
async fn main() {
    let article_repo = MongoArticleRepository::new("mongodb://localhost").await.unwrap();
    let article_repo_filter = warp::any().map(move || article_repo.clone());

    let create_article = warp::post()
        .and(warp::path("article"))
        .and(warp::path::end())
        .and(article_repo_filter.clone())
        .and(warp::body::json())
        .map(|article_repo, article| crate::services::ArticleService::create_article(article_repo, article));

    warp::serve(create_article)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
