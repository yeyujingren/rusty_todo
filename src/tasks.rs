// 文件处理模块

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind};
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    /// 储存每个 task 的描述
    pub text: String,

    /// 储存当前 task 的创建时间
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // {:<50}：填充了 50 个空格的左对齐字符串。
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

// 添加 task 向 task list 中
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // 打开文件
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    //将新增的修改添加到最后
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())

}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    // 将文件的内容用作任务 Vec。
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?
    };

    // 读取完成之后将读取指针重制
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // 打开文件
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    
    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.remove(task_position - 1);

    // 将修改后的任务列表写回文件
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // 打开文件
    let file = OpenOptions::new().read(true).open(journal_path)?;

    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

