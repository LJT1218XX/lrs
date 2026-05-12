
use std::fs;
use clap::Parser;//要用的是什么


#[derive(Parser)] //什么作用
#[command(name = "lrs", version = "1.0", about = "简化版ls")] //这是什么意思
struct Args {
    ///显示文件类型，修改时间
    #[arg(short = 'l')]//这是什么作用
    long: bool,

    ///显示隐藏文件
    #[arg(short = 'a')]
    all: bool,
}

fn main() -> std::io::Result<()> {
    //let args: Vec<String> = env::args().collect();
    //let long_format = args.contains(&"-l".to_string());
    //let show_all = args.contains(&"-a".to_string());
    let args = Args::parse();
    let long_format = args.long;
    let show_all = args.all;
    

    let mut entries = Vec::new();

    for entry in fs::read_dir(".")? {
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

    for (filename, size, file_type, meta) in &entries {
        let type_char = if file_type.is_dir() {
            'd'
        } else if file_type.is_symlink() {
            'l'
        } else {
            '-'
        };

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
                size,
                mtime,
                filename,
                width = max_width
            );
        } else {
            println!(
                "{}  {:>width$}  {}",
                type_char,
                size,
                filename,
                width = max_width
            );
        }
    }
    Ok(())
}


fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"]; //这里为何要加: &[&str]，是要将他们存入栈中吗
    let mut size = size as f64;//为何要加as f64
    for unit in UNITS {
        if size < 1024.0 {
            if *unit == "B" {  //这里为何要在变量前加*
                return format!("{:.0} {}", size, unit); //这里的{:.0}代表什么
            }
            return format!("{:.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.1} TiB", size)
}