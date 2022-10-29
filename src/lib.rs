use anyhow::Result;
use bytes::Bytes;
use feed_rs::model::Entry;
#[cfg(feature = "serde")]
pub use serde;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedMem {
    pub url: String,
    pub remembered_ids: Vec<String>,
}

impl FeedMem {
    pub fn new(url: String) -> Self {
        Self {
            url,
            remembered_ids: Vec::new(),
        }
    }

    pub fn with_remembered_ids(url: String, remembered_ids: Vec<String>) -> Self {
        Self {
            url,
            remembered_ids,
        }
    }

    pub fn with_capacity(url: String, capacity: usize) -> Self {
        Self {
            url,
            remembered_ids: Vec::with_capacity(capacity),
        }
    }

    pub async fn get_new_entries(&mut self, do_trim: bool) -> Result<Vec<Entry>> {
        let bytes = reqwest::get(&self.url).await?.bytes().await?;
        self.get_new_entries_from_bytes(bytes, do_trim)
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_get_new_entries(&mut self, do_trim: bool) -> Result<Vec<Entry>> {
        let bytes = reqwest::blocking::get(&self.url)?.bytes()?;
        self.get_new_entries_from_bytes(bytes, do_trim)
    }

    fn get_new_entries_from_bytes(&mut self, bytes: Bytes, do_trim: bool) -> Result<Vec<Entry>> {
        let entries = feed_rs::parser::parse(&bytes[..])?.entries;
        let mut new_entries = Vec::new();
        for entry in entries.iter() {
            if !self.remembered_ids.contains(&entry.id) {
                self.remembered_ids.push(entry.id.clone());
                new_entries.push(entry.clone());
            }
        }
        if do_trim {
            self.remembered_ids
                .retain(|id| entries.iter().map(|e| &e.id).any(|eid| eid == id));
        }
        Ok(new_entries)
    }
}
