use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub pages: Vec<Page>,
    pub variables: HashMap<String, f64>, // Minimal reactive state for now
    pub settings: DocumentSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentMetadata {
    pub id: String,
    pub title: String,
    pub format_version: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentSettings {
    pub background_color: [u8; 4],
}

impl Default for DocumentSettings {
    fn default() -> Self {
        Self {
            background_color: [255, 255, 255, 255],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub size: (f32, f32), // Width, Height
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: String,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub content: BlockContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockContent {
    Text(String),
    MathExpression(String), // The raw string, parsed on demand
    MathResult(f64),
    Handwriting(Vec<Stroke>),
    Graph2D(String), // Expression to graph
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub color: [u8; 4],
    pub width: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrokePoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub timestamp: u64,
}

impl Document {
    pub fn new(title: String) -> Self {
        Self {
            metadata: DocumentMetadata {
                id: uuid::Uuid::new_v4().to_string(),
                title,
                format_version: 1,
                created_at: 0, // Mock timestamp
                updated_at: 0,
            },
            pages: vec![Page {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Page 1".into(),
                size: (800.0, 1200.0),
                blocks: vec![],
            }],
            variables: HashMap::new(),
            settings: DocumentSettings::default(),
        }
    }
}
