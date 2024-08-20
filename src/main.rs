use clap::Parser;
use fileinfos::param;
use std::fs::Metadata;
use std::{env, fs};

mod compares;
mod fileinfos;

fn main() {
    let cur_path = env::current_dir().unwrap();

    // println!("当前目录:{}",&cur_path.to_str().unwrap());

    let _di = fs::read_dir(cur_path);

    let mut inputs:Vec<(String,Metadata)> = Vec::new();

    //TODO:这里可以添加过滤器和选择器,先一致处理!
    for item in _di.unwrap() {
        let item = item.unwrap();
        if item.path().is_file()
        {
            let meta = fs::metadata(item.path()).unwrap();
            // println!("{:?}--{}--{:?}--{:?}", item.path(), meta.len(), meta.created(), meta.modified());
            inputs.push((
                String::from(
                    item.path().to_str().unwrap()
                ),
                meta
            ));
        }
    }

    println!("选中文件数量:{}",&inputs.len());

    let cli = param::Parameters::parse();
    if let Some(m) = &cli.sub_commands {
        println!("指令模式:{:?}", m);
    } else {
        println!("无效的指令模式");
    }
}
