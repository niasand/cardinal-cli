use anyhow::Result;
use search_cache::SearchResultNode;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchOutput {
    pub results: Vec<FileResult>,
    pub total: usize,
    pub returned: usize,
    pub duration_ms: u64,
}

#[derive(Serialize)]
pub struct FileResult {
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub kind: String,
    pub size: i64,
}

impl FileResult {
    pub fn from_search_result(node: SearchResultNode) -> Self {
        let path_display = node.path.display().to_string();
        let name = node
            .path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let extension = node.path.extension().map(|e| e.to_string_lossy().into_owned());

        let kind = match node.metadata.file_type_hint() {
            fswalk::NodeFileType::Dir => "directory",
            fswalk::NodeFileType::Symlink => "symlink",
            _ => "file",
        };

        let size = node
            .metadata
            .as_ref()
            .map(|m| m.size())
            .unwrap_or(-1);

        Self {
            path: path_display,
            name,
            extension,
            kind: kind.to_string(),
            size,
        }
    }
}

pub fn format_json(output: &SearchOutput) -> Result<String> {
    Ok(serde_json::to_string(output)?)
}

pub fn format_text(output: &SearchOutput) -> String {
    let mut lines = Vec::with_capacity(output.results.len() + 1);
    for (i, r) in output.results.iter().enumerate() {
        lines.push(format!(
            "[{}] {} ({} {})",
            i,
            r.path,
            humansize(r.size as u64),
            r.kind
        ));
    }
    lines.join("\n")
}

fn humansize(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    match bytes {
        b if b >= GB => format!("{:.1}GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.1}MB", b as f64 / MB as f64),
        b if b >= KB => format!("{:.1}KB", b as f64 / KB as f64),
        b => format!("{b}B"),
    }
}
