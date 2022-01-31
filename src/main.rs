use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::anyhow;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_todo_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    })
}

fn main()  -> anyhow::Result<()> {
    // 获取命令行输入参数
    let CommandLineArgs { action, todo_file } = CommandLineArgs::from_args();

    let todo_file = todo_file
        .or_else(find_default_todo_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => tasks::add_task(todo_file, Task::new(text)),
        List => tasks::list_tasks(todo_file),
        Done { position } => tasks::complete_task(todo_file, position)
    }?;
    Ok(())
}
