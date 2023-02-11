use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    ops::Range,
    path::Path,
};

use clap::{command, Parser};
use itertools::Itertools;

mod error;
use colored::Colorize;
pub use error::GrepError;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

/// 定义类型，这样，在使用时可以简化复杂类型的书写
// pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;
pub type StrategyFn = fn(&Path, &mut dyn BufRead, &Regex, &mut dyn Write) -> Result<(), GrepError>;

/// Yet another simplified grep built with Rust.
#[derive(Parser, Debug)]
#[command(author = "Air", version = "1.0.0", about, long_about = None)]
pub struct GrepConfig {
    /// regex pattern to match against file contents
    pattern: String,

    /// Glob of file pattern
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    pub fn match_with(&self, strategy: StrategyFn) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        // 生成所有符合通配符的文件列表
        let files: Vec<_> = glob::glob(&self.glob)?.collect();
        // 并行处理所有文件
        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let mut reader = BufReader::new(file);
                    let mut stdout = std::io::stdout();

                    if let Err(e) = strategy(filename.as_path(), &mut reader, &regex, &mut stdout) {
                        println!("Internal error: {:?}", e);
                    }
                }
            }
        });
        Ok(())
    }
}

/// 缺省策略，从头到尾串行查找，最后输出到 writer
pub fn default_strategy(
    path: &Path,
    reader: &mut dyn BufRead,
    pattern: &Regex,
    writer: &mut dyn Write,
) -> Result<(), GrepError> {
    let matches: String = reader
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            line.ok()
                .map(|line| {
                    pattern
                        .find(&line)
                        .map(|m| format_line(&line, lineno + 1, m.range()))
                })
                .flatten()
        })
        .filter_map(|v| v.ok_or(()).ok())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b"\n")?;
        writer.write(matches.as_bytes())?;
        writer.write(b"\n")?;
    }

    Ok(())
}

fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        // 找到匹配项的起始位置，注意对汉字等非 ascii 字符，我们不能使用 prefix.len()
        // 这是一个 O(n) 的操作，会拖累效率，这里只是为了演示的效果
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, Air~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "8".cyan(),
            "Air".red()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey Air!";
        let pattern = Regex::new(r"he\w+").unwrap();
        let mut reader = BufReader::new(&input[..]);
        let mut writer = Vec::new();
        default_strategy(path, &mut reader, &pattern, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hey Air!\n", 2, 0..3),
        ];
        assert_eq!(result, expected.join("\n"));
    }
}
