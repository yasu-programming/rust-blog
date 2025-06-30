use crate::models::Article;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub struct FileStorage {
    data_dir: String,
}

impl FileStorage {
    pub fn new(data_dir: String) -> Self {
        Self { data_dir }
    }

    pub fn ensure_data_dir(&self) -> io::Result<()> {
        fs::create_dir_all(&self.data_dir)
    }

    fn get_article_path(&self, id: &str) -> String {
        format!("{}/{}.json", self.data_dir, id)
    }

    fn get_index_path(&self) -> String {
        format!("{}/index.json", self.data_dir)
    }

    pub fn save_article(&self, article: &Article) -> io::Result<()> {
        self.ensure_data_dir()?;

        let article_path = self.get_article_path(&article.id);
        let json = serde_json::to_string_pretty(article)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(article_path, json)?;

        self.update_index(&article.id, &article.title)?;
        Ok(())
    }

    pub fn load_article(&self, id: &str) -> io::Result<Article> {
        let article_path = self.get_article_path(id);
        let content = fs::read_to_string(article_path)?;
        let article: Article = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(article)
    }

    pub fn list_articles(&self) -> io::Result<Vec<(String, String)>> {
        let index_path = self.get_index_path();
        if !Path::new(&index_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(index_path)?;
        let index: HashMap<String, String> = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(index.into_iter().collect())
    }

    pub fn delete_article(&self, id: &str) -> io::Result<()> {
        let article_path = self.get_article_path(id);
        if Path::new(&article_path).exists() {
            fs::remove_file(article_path)?;
        }

        self.remove_from_index(id)?;
        Ok(())
    }

    fn update_index(&self, id: &str, title: &str) -> io::Result<()> {
        let index_path = self.get_index_path();
        let mut index: HashMap<String, String> = if Path::new(&index_path).exists() {
            let content = fs::read_to_string(&index_path)?;
            serde_json::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            HashMap::new()
        };

        index.insert(id.to_string(), title.to_string());

        let json = serde_json::to_string_pretty(&index)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(index_path, json)?;
        Ok(())
    }

    fn remove_from_index(&self, id: &str) -> io::Result<()> {
        let index_path = self.get_index_path();
        if !Path::new(&index_path).exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&index_path)?;
        let mut index: HashMap<String, String> = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        index.remove(id);

        let json = serde_json::to_string_pretty(&index)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(index_path, json)?;
        Ok(())
    }
}
