use rust_blog::{Article, FileStorage};
use std::fs;
use std::path::Path;

#[test]
fn test_article_creation() {
    let article = Article::new("Test Title".to_string(), "Test Content".to_string());

    assert!(!article.id.is_empty());
    assert_eq!(article.title, "Test Title");
    assert_eq!(article.content, "Test Content");
    assert_eq!(article.created_at, article.updated_at);
}

#[test]
fn test_article_update() {
    let mut article = Article::new("Original Title".to_string(), "Original Content".to_string());
    let original_created_at = article.created_at;

    article.update(Some("Updated Title".to_string()), None);

    assert_eq!(article.title, "Updated Title");
    assert_eq!(article.content, "Original Content");
    assert_eq!(article.created_at, original_created_at);
    assert!(article.updated_at > original_created_at);
}

#[test]
fn test_file_storage_operations() {
    let test_dir = "test_data";
    let storage = FileStorage::new(test_dir.to_string());

    // Clean up test directory
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir).unwrap();
    }

    // Test save and load
    let article = Article::new("Test Article".to_string(), "This is a test".to_string());
    let article_id = article.id.clone();

    storage.save_article(&article).unwrap();
    let loaded_article = storage.load_article(&article_id).unwrap();

    assert_eq!(article.id, loaded_article.id);
    assert_eq!(article.title, loaded_article.title);
    assert_eq!(article.content, loaded_article.content);

    // Test list articles
    let articles = storage.list_articles().unwrap();
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].0, article_id);
    assert_eq!(articles[0].1, "Test Article");

    // Test delete
    storage.delete_article(&article_id).unwrap();
    let articles = storage.list_articles().unwrap();
    assert_eq!(articles.len(), 0);

    // Clean up
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir).unwrap();
    }
}
