pub mod ArticleService {
    use domain::entities::Article;
    use infrastructure::repositories::ArticleRepository;
    use warp::hyper::Response;

    pub fn create_article(repo: impl ArticleRepository, article: Article) -> Response<String> {
        let res = match repo.add(article) {
            Ok(new_article_id) => Response::builder().body(new_article_id).unwrap(),
            _ => Response::builder().status(200).body(String::from("")).unwrap()
        };

        res
    }
}

