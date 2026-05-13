use clap::{Parser};
use std::fs;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "lrs", version = "1.0", about = "简化版ls")]
struct Args {
    ///显示文件类型，修改时间
    #[arg(short = 'l')]
    long: bool,

    ///显示隐藏文件
    #[arg(short = 'a')]
    all: bool,

    ///指定目录路径
    path: Option<String>
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let long_format = args.long;
    let show_all = args.all;
    let dir= args.path.unwrap_or_else(|| ".".to_string());

    let mut entries = Vec::new();

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        let path = entry.path();
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid>")
            .to_string();

        if !show_all && filename.starts_with('.') {
            continue;
        }
        let file_type = meta.file_type();
        let size = format_size(meta.len());

        entries.push((filename, size, file_type, meta));
    }

    let max_width = entries
        .iter()
        .map(|(_, size, _, _)| size.len())
        .max()
        .unwrap_or(8);

    entries.sort_by(|a, b| b.2.is_dir().cmp(&a.2.is_dir()).then(a.0.cmp(&b.0)));

    for (filename, size, file_type, meta) in &entries {
        let type_char = if file_type.is_dir() {
            'd'
        } else if file_type.is_symlink() {
            'l'
        } else {
            '-'
        };

        let display_size = if file_type.is_dir() {
            String::from("------")
        } else {
            size.clone()
        };

        let display_name = if file_type.is_dir() {
            filename.blue()
        } else if file_type.is_symlink(){
            filename.cyan()
        } else if ext_in(&filename, &[".exe", "bat", "com", "cmd"]) {
            filename.green()
        } else if ext_in(&filename, &[".png", ".jpg", ".jpeg", ".gif", ".bmp", ".webp", ".svg", ".ico"]) {
            filename.magenta()
        } else if ext_in(&filename, &[".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a"]) {
            filename.yellow()
        } else {
            filename.normal()
        }.to_string();

        if long_format {
            let mtime = match meta.modified() {
                Ok(time) => {
                    let elapsed = time.elapsed().unwrap_or_default();
                    let secs = elapsed.as_secs();

                    if secs < 60 {
                        format!("{}秒钟前", secs)
                    } else if secs < 3600 {
                        format!("{}分钟前", secs / 60)
                    } else if secs < 86400 {
                        format!("{}小时前", secs / 3600)
                    } else {
                        format!("{}天前", secs / 86400)
                    }
                }
                Err(_) => "未知".to_string(),
            };
            println!(
                "{}  {:>width$}  {:>8}  {}",
                type_char,
                display_size,
                mtime,
                display_name,
                width = max_width
            );
        } else {
            println!(
                "{}  {:>width$}  {}",
                type_char,
                display_size,
                display_name,
                width = max_width
            );
        }
    }
    Ok(())
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = size as f64;
    for unit in UNITS {
        if size < 1024.0 {
            if *unit == "B" {
                return format!("{:.0} {}", size, unit);
            }
            return format!("{:.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.1} TiB", size)
}

fn ext_in(name: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| name.ends_with(ext))
}