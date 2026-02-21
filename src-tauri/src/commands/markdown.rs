// ============================
// Markdown 模块 — Rust IPC 命令
// ============================
//
// 职责：文件 I/O 操作（读取文件、打开对话框、扫描文件夹）
// 前端通过 invoke("命令名", { 参数 }) 调用这些函数
//
// 设计原则：
// - 前端只负责渲染，所有文件系统操作都在 Rust 端完成（安全）
// - 文件夹扫描只读元数据 + 前 8KB 预览，不加载全文（性能）
// - 全文内容在用户点击卡片时才通过 read_markdown_file 按需加载

use std::fs;
use std::io::Read;
use std::path::Path;
use tauri_plugin_dialog::{DialogExt, FilePath};
use walkdir::WalkDir; // 第三方库：递归遍历目录树

/// 单个 Markdown 文件的元数据（传给前端展示卡片用）
///
/// 注意：不包含完整内容，只有 preview（前 3 行）
/// #[serde(rename_all = "camelCase")] 会把 snake_case 字段名
/// 自动转为 camelCase 传给前端（Rust 用 relative_path，JS 收到 relativePath）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownFileInfo {
    /// 文件的绝对路径，用于后续 read_markdown_file 读取全文
    pub path: String,
    /// 相对于所选文件夹的路径（如 "docs/notes/intro.md"），用于卡片上显示
    pub relative_path: String,
    /// 文件名（如 "intro.md"）
    pub name: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 最后修改时间，ISO 8601 / RFC 3339 格式的字符串
    pub modified: String,
    /// 文件内容预览：前 3 个非空行，最多 200 字符
    pub preview: String,
    /// 估算字数（基于前 8KB 内容，按空白字符分割计数）
    pub word_count: u32,
}

// ─────────────────────────────────────────────
// 命令 1：读取单个 Markdown 文件的完整内容
// ─────────────────────────────────────────────
// 前端调用：invoke("read_markdown_file", { path: "/abs/path/to/file.md" })
// 返回：文件的完整文本内容
//
// #[tauri::command] 宏会自动：
//   1. 把这个函数注册为 IPC 命令
//   2. 把参数从 JSON 反序列化
//   3. 把返回值序列化为 JSON
#[tauri::command]
pub async fn read_markdown_file(path: String) -> Result<String, String> {
    // fs::read_to_string 读取整个文件为 String
    // .map_err() 把 std::io::Error 转为 String（Tauri IPC 要求错误类型是 String）
    fs::read_to_string(&path).map_err(|e| format!("Failed to read '{}': {}", path, e))
}

// ─────────────────────────────────────────────
// 命令 2：打开系统「选择文件」对话框
// ─────────────────────────────────────────────
// 前端调用：invoke("open_markdown_dialog")
// 返回：用户选中的文件路径，或 null（用户取消）
//
// app: tauri::AppHandle — Tauri 自动注入，不需要前端传
#[tauri::command]
pub async fn open_markdown_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let file = app
        .dialog()
        .file()
        // 过滤器：对话框里只显示 .md / .markdown / .mdx 文件
        .add_filter("Markdown", &["md", "markdown", "mdx"])
        // blocking_pick_file() — 阻塞等待用户选择（桌面端用法）
        // 返回 Option<FilePath>，用户取消则为 None
        .blocking_pick_file();

    // FilePath 是枚举（不是结构体！），有两个变体：
    //   FilePath::Path(PathBuf) — 普通文件路径（桌面端）
    //   FilePath::Url(Url)      — URL 形式（移动端可能返回这种）
    // 坑：不能直接写 file_path.path，必须 match 枚举
    match file {
        Some(file_path) => match file_path {
            FilePath::Path(p) => Ok(Some(p.to_string_lossy().to_string())),
            FilePath::Url(u) => Ok(Some(u.to_string())),
        },
        None => Ok(None), // 用户取消选择，返回 null 给前端
    }
}

// ─────────────────────────────────────────────
// 命令 3：打开系统「选择文件夹」对话框
// ─────────────────────────────────────────────
// 前端调用：invoke("open_folder_dialog")
// 返回：用户选中的文件夹路径，或 null（用户取消）
//
// 和 open_markdown_dialog 几乎一样，
// 区别：用 blocking_pick_folder() 而不是 blocking_pick_file()
#[tauri::command]
pub async fn open_folder_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let folder = app.dialog().file().blocking_pick_folder();

    match folder {
        Some(file_path) => match file_path {
            FilePath::Path(p) => Ok(Some(p.to_string_lossy().to_string())),
            FilePath::Url(u) => Ok(Some(u.to_string())),
        },
        None => Ok(None),
    }
}

// ─────────────────────────────────────────────
// 命令 4：递归扫描文件夹，列出所有 Markdown 文件
// ─────────────────────────────────────────────
// 前端调用：invoke("list_markdown_files", { folderPath: "/abs/path/to/folder" })
// 返回：MarkdownFileInfo 数组，按 relativePath 字母排序
//
// 性能考虑：
//   - 只读每个文件的前 8KB（不是全文），用于预览和字数估算
//   - 全文内容在用户点击卡片时才通过 read_markdown_file 按需加载
//   - walkdir 库非常高效，扫描数千个文件也很快
#[tauri::command]
pub async fn list_markdown_files(folder_path: String) -> Result<Vec<MarkdownFileInfo>, String> {
    let root = Path::new(&folder_path);

    // 安全检查：确保传入的路径是一个目录
    if !root.is_dir() {
        return Err(format!("'{}' is not a directory", folder_path));
    }

    // 支持的 Markdown 文件扩展名
    let md_extensions = ["md", "markdown", "mdx"];
    let mut files: Vec<MarkdownFileInfo> = Vec::new();

    // WalkDir::new(root) — 从 root 开始递归遍历目录树
    //   .follow_links(true)  — 跟随符号链接（symlink）
    //   .into_iter()         — 转为迭代器
    //   .filter_map(|e| e.ok()) — 跳过无法访问的条目（权限不足等），不报错
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // 跳过目录，只处理文件
        if !path.is_file() {
            continue;
        }

        // 取文件扩展名，转小写后检查是否是 Markdown
        // 链式调用解释：
        //   .extension()         → Option<&OsStr>，可能没有扩展名
        //   .and_then(|e| ...)   → 如果是 Some 则继续处理，None 则短路
        //   .to_str()            → OsStr 转 &str，非 UTF-8 时返回 None
        //   .unwrap_or("")       → None 时用空字符串兜底
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        // 不是 Markdown 文件就跳过
        if !md_extensions.contains(&ext.as_str()) {
            continue;
        }

        // 读取文件元数据（大小、修改时间等）
        // match 模式：成功则绑定到 m，失败则 continue 跳过这个文件
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        // 获取最后修改时间，转为 RFC 3339 字符串（如 "2026-02-21T10:30:00+08:00"）
        // chrono::Local 使用本地时区，前端显示更直观
        let modified = metadata
            .modified()
            .ok() // SystemTimeError 转为 None
            .map(|t| {
                let dt: chrono::DateTime<chrono::Local> = t.into();
                dt.to_rfc3339()
            })
            .unwrap_or_default(); // 获取不到时间就用空字符串

        // 计算相对路径：去掉文件夹根路径前缀
        // 例如 root="/Users/x/docs", path="/Users/x/docs/notes/a.md"
        // → relative_path = "notes/a.md"
        let relative_path = path
            .strip_prefix(root)
            .unwrap_or(path) // strip 失败就用完整路径兜底
            .to_string_lossy() // OsStr → Cow<str>，非 UTF-8 字符用 ? 替代
            .to_string();

        // 提取文件名（如 "a.md"）
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // 读取预览文本和估算字数（只读前 8KB）
        let (preview, word_count) = read_preview(path);

        files.push(MarkdownFileInfo {
            path: path.to_string_lossy().to_string(),
            relative_path,
            name,
            size: metadata.len(),
            modified,
            preview,
            word_count,
        });
    }

    // 按相对路径排序，保证前端展示顺序稳定
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(files)
}

// ─────────────────────────────────────────────
// 辅助函数：读取文件预览和估算字数
// ─────────────────────────────────────────────
// 参数：文件路径
// 返回：(预览文本, 估算字数) 的元组
//
// 只读前 8KB 而不是整个文件，原因：
//   1. 性能 — 文件夹可能有几百个文件，不能每个都全读
//   2. 预览只需要前几行
//   3. 字数只是估算，粗略值就够了
//
// 注意：这不是 #[tauri::command]，是普通的 Rust 函数，只在本模块内部使用
fn read_preview(path: &Path) -> (String, u32) {
    // 打开文件，失败则返回空预览和 0 字数
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return (String::new(), 0),
    };

    // 创建 8KB 缓冲区，只读这么多
    let mut buf = vec![0u8; 8192];
    let bytes_read = match file.read(&mut buf) {
        Ok(n) => n, // n 是实际读到的字节数（文件小于 8KB 时 n < 8192）
        Err(_) => return (String::new(), 0),
    };
    buf.truncate(bytes_read); // 截断到实际读取的长度

    // from_utf8_lossy：把字节转为字符串
    // "lossy" 意味着遇到非法 UTF-8 字节时用 U+FFFD (�) 替代，不会报错
    let text = String::from_utf8_lossy(&buf);

    // 估算字数：按空白字符（空格、换行、制表符）分割后计数
    // as u32 — usize 转 u32，文件前 8KB 的字数不可能溢出 u32
    let word_count = text.split_whitespace().count() as u32;

    // 提取预览：取前 3 个非空行
    // .lines()   — 按 \n 分割为迭代器
    // .filter()  — 过滤掉空行和只有空白的行
    // .take(3)   — 只取前 3 个
    // .collect() — 收集为 Vec
    let preview_lines: Vec<&str> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .collect();

    // 把 3 行合并为一个字符串，用换行符连接
    let mut preview = preview_lines.join("\n");

    // 预览文本超过 200 字符就截断，加省略号
    if preview.len() > 200 {
        preview.truncate(200);
        preview.push_str("…");
    }

    (preview, word_count)
}
