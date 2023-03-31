use domain::entities::Article;

pub trait ArticleRepository {
    fn add(&self, article: Article) -> Result<String, anyhow::Error>;
}
