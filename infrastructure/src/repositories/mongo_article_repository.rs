use anyhow::anyhow;
use domain::entities::Article;
use crate::repositories::ArticleRepository;
use mongodb::{ Client, options::{ClientOptions, ResolverConfig} };

#[derive(Clone)]
pub struct MongoArticleRepository {
    mongo_client: Client,
}

impl ArticleRepository for MongoArticleRepository {
    fn add(&self, article: Article) -> Result<String, anyhow::Error> {
        let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let collection = &self.mongo_client.database("blog").collection::<Article>("articles");
            collection.insert_one(article.clone(), None).await
        });

        match res { 
            Ok(insert) => { 
                match insert.inserted_id.as_str() {
                    Some(id) => Ok(String::from(id)),
                    None => Err(anyhow!("Insertion into DB failed")),
                }
            },
            Err(_) => Err(anyhow!("Mongo Error")), // TODO: needs to be unwrapped correctly
        }
    }
}

impl MongoArticleRepository {
    pub async fn new(uri: &str) -> Result<Self, mongodb::error::Error> {
        let options = ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        Ok(MongoArticleRepository { mongo_client: client })
    }
}
