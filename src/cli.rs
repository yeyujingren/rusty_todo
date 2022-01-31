// 用户交互模块

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// 向 todo list 中加入新的task
    Add { 
        /// task 描述文字
        #[structopt()]
        text: String
    },

    /// 按照位置从 todo list 中删除条目
    Done { 
        #[structopt()]
        position: usize
    },

    /// todo list 中所有的tasks
    List
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusty todo",
    about = "A command line todo app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// 自定义不同的 todo list 缓存日志文件
    #[structopt(parse(from_os_str), short, long)]
    pub todo_file: Option<PathBuf>
}