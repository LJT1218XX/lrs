use clap::{Parser};
use colored::Colorize;
use std::fs;

#[derive(Parser)]
#[command(name = "lrs", version = "1.0", about = "简化版ls")]
struct Args {
    ///显示文件类型，修改时间
    #[arg(short = 'l')]
    long: bool,

    ///显示隐藏文件
    #[arg(short = 'a')]
    all: bool,

    ///递归列出子目录
    #[arg(short = 'R')]
    recursive: bool,

    ///指定目录路径
    path: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let long_format = args.long;
    let show_all = args.all;
    let dir = args.path.unwrap_or_else(|| ".".to_string());

    list_dir(&dir, show_all, long_format, args.recursive)
}

//格式化文件大小
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

#[derive(Debug)]
enum EntryKind {
    Directory,
    Symlink,
    Executable,
    Image,
    Audio,
    Other,
}

trait FileDisplay {
    fn icon_char(&self) -> char;
    fn colorize(&self, name: &str) -> String;    
}

impl FileDisplay for EntryKind {
    fn icon_char(&self) -> char {
        match self {
            EntryKind::Directory => 'd',
            EntryKind::Symlink => 'l',
            _ => '-',
        }
    }

    fn colorize(&self, name: &str) -> String {
        match self {
            EntryKind::Directory => name.blue(),
            EntryKind::Symlink => name.cyan(),
            EntryKind::Executable => name.green(),
            EntryKind::Image => name.magenta(),
            EntryKind::Audio => name.yellow(),
            EntryKind::Other => name.normal(),
        }.to_string()
    }
}

fn classify(file_type: &std::fs::FileType, name:&str) -> EntryKind {
    if file_type.is_dir() {
        EntryKind::Directory
    } else if file_type.is_symlink() {
        EntryKind::Symlink
    } else if ext_in(name, &[".exe", ".bat", ".com", ".cmd"]) {
        EntryKind::Executable
    } else if ext_in(name, &[".png", ".jpg", ".jpeg", ".gif", ".bmp", ".webp", ".svg", ".ico"]){
        EntryKind::Image
    } else if ext_in(name, &[".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a"]) {
        EntryKind::Audio
    } else {
        EntryKind::Other
    }
}

//获得文件后缀名用来匹配文件类型
fn ext_in(name: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| name.ends_with(ext))
}

//展示文件的核心代码
fn list_dir(dir: &str, show_all: bool, long_format: bool, recursive: bool) -> std::io::Result<()> {
    //打印当前目录
    println!("{} :", dir);

    let mut entries = Vec::new();

    //读取指定目录下的文件并将其push到Vector中
    for entry in fs::read_dir(&dir)? {
        //声明一个文件实体
        let entry = entry?;
        //将文件元数据输入实体
        let meta = entry.metadata()?;
        //从元数据中得到路径
        let path = entry.path();
        //从路径中得到文件名
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid>")
            .to_string();

        //判断是否为隐藏文件
        if !show_all && filename.starts_with('.') {
            continue;
        }
        //从元数据中获得文件类型
        let file_type = meta.file_type();
        //对文件类型进行分类
        let kind = classify(&file_type, &filename);
        //得到文件大小，并调用format_size函数进行格式化
        let size = format_size(meta.len());

        //将所有数据以元组的形式push到容器中
        entries.push((filename, size, file_type, meta, kind));
    }

    //获得所有文件的大小，对文件大小动态展示
    let max_width = entries
        .iter()
        .map(|(_, size, _, _, _)| size.len())
        .max()
        .unwrap_or(8);

    //对文件进行排序，优先将文件夹放到上边
    entries.sort_by(|a, b| b.2.is_dir().cmp(&a.2.is_dir()).then(a.0.cmp(&b.0)));

    //对文件类型分类后标注
    for (filename, size, file_type, meta, kind) in &entries {
        let type_char = kind.icon_char();

        //文件夹不计算大小，以------代替
        let display_size = if file_type.is_dir() {
            String::from("------")
        } else {
            size.clone()
        };

        //根据文件类型动态变化文字颜色
        let display_name = kind.colorize(&filename);

        //获得并格式化文件的修改时间
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
                } //文件修改时间可能被篡改
                Err(_) => "未知".to_string(),
            };
            //动态展示文件数据
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
    if recursive {
        for (filename, _size, file_type, _meta, _kind) in &entries {
            if file_type.is_dir() && filename != "." && filename != ".." {
                println!("");
                let sub_path = format!("{}/{}", dir.trim_end_matches('/'), filename);
                list_dir(&sub_path, show_all, long_format, recursive)?;
            }
        }
    }
    Ok(())
}
