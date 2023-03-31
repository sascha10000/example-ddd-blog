use super::AuthorDto;
use domain::entities::Article;

pub struct ArticleDto {
    id: u64,
    title: String,
    author: AuthorDto,
    text : String
}

impl From<Article> for ArticleDto {
    fn from(value: Article) -> Self {
        ArticleDto { id: value.id, title: value.title, author: value.author.into(), text: value.text }
    }
}
