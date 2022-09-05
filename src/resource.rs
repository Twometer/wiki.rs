use std::collections::HashMap;

use wry::{
    http::{Response, ResponseBuilder},
    Error,
};

pub enum MimeType {
    Text,
    Html,
    Css,
    Js,
    Png,
    Jpg,
    OctetStream,
}

impl MimeType {
    fn from_extension(extension: &str) -> MimeType {
        match extension {
            ".txt" => MimeType::Text,
            ".html" => MimeType::Html,
            ".js" => MimeType::Js,
            ".jpg" | ".jpeg" => MimeType::Jpg,
            ".png" => MimeType::Png,
            ".css" => MimeType::Css,
            _ => MimeType::OctetStream,
        }
    }

    fn to_string(&self) -> String {
        match self {
            MimeType::Text => "text/plain",
            MimeType::Html => "text/html",
            MimeType::Css => "text/css",
            MimeType::Js => "application/javascript",
            MimeType::Png => "image/png",
            MimeType::Jpg => "image/jpg",
            MimeType::OctetStream => "application/octet-stream",
        }
        .to_string()
    }

    fn is_binary(&self) -> bool {
        match self {
            MimeType::Text => false,
            MimeType::Html => false,
            MimeType::Css => false,
            MimeType::Js => false,
            MimeType::Png => true,
            MimeType::Jpg => true,
            MimeType::OctetStream => true,
        }
    }
}

enum ResourceData {
    Binary(Vec<u8>),
    String(String),
}

pub struct ResourceFile {
    mime_type: String,
    data: ResourceData,
}

impl ResourceFile {
    pub fn new(mime_type: MimeType, data: Vec<u8>) -> Self {
        let resource_data = if mime_type.is_binary() {
            ResourceData::Binary(data)
        } else {
            ResourceData::String(String::from_utf8(data).expect("Failed to load resource file"))
        };

        Self {
            mime_type: mime_type.to_string(),
            data: resource_data,
        }
    }

    pub fn to_binary(&self) -> Vec<u8> {
        match &self.data {
            ResourceData::Binary(bin) => bin.to_owned(),
            ResourceData::String(str) => str.as_bytes().to_owned(),
        }
    }

    pub fn to_string(&self) -> String {
        match &self.data {
            ResourceData::Binary(bin) => {
                String::from_utf8(bin.to_owned()).expect("Failed conversion to_string")
            }
            ResourceData::String(str) => str.to_owned(),
        }
    }
}

impl Into<Result<Response, Error>> for &ResourceFile {
    fn into(self) -> Result<Response, Error> {
        ResponseBuilder::new()
            .mimetype(&self.mime_type)
            .body(self.to_binary())
    }
}

pub struct ResourceManager {
    resource_files: HashMap<String, ResourceFile>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resource_files: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, data: &[u8]) {
        self.resource_files.insert(
            name.to_string(),
            ResourceFile::new(Self::infer_mime(name), data.to_owned()),
        );
    }

    pub fn find_resource(&self, name: &str) -> Option<&ResourceFile> {
        self.resource_files.get(name)
    }

    fn infer_mime(name: &str) -> MimeType {
        let extension_sep_idx = name.rfind(".");
        match extension_sep_idx {
            Some(extension_sep_idx) => {
                let ext = &name[extension_sep_idx..];
                MimeType::from_extension(ext)
            }
            None => MimeType::OctetStream,
        }
    }
}
