use clap::Parser;
use fileinfos::param;
use std::fs::Metadata;
use std::{env, fs};
use std::cmp::PartialEq;
use std::ptr::null;
use crate::compares::sorter_buidler::TAction;
use crate::compares::timesorter::{TimeSortType, TimeSorter};
use crate::compares::timesorter::TimeSortType::CreateTime;
use crate::fileinfos::param::SubCommands;

mod compares;
mod fileinfos;

fn main() {
    let cur_path = env::current_dir().unwrap();

    // println!("当前目录:{}",&cur_path.to_str().unwrap());

    let _di = fs::read_dir(cur_path);
    let mut inputs = Vec::new();


    //TODO:这里可以添加过滤器和选择器,先一致处理!
    for item in _di.unwrap() {
        let item = item.unwrap();
        if item.path().is_file()
        {
            let meta = fs::metadata(item.path()).unwrap();
            // println!("{:?}--{}--{:?}--{:?}", item.path(), meta.len(), meta.created(), meta.modified());
            println!("是个文件:{} 其路径:{} 父路径:{}",item.file_name().to_str().unwrap(),
                     item.path().to_str().unwrap(),
                     item.path().parent().unwrap().to_str().unwrap());
            inputs.push((
                String::from(
                    item.path().to_str().unwrap()
                ),
                meta
            ));
        }else {
            println!("是个路径:{}",item.path().to_str().unwrap());
        }
    }

    println!("选中文件数量:{}",&inputs.len());

    //给定默认值
    let mut call_func:Box<dyn TAction> =
       Box::from(compares::sorter_buidler::DefaultTAction{});

    let cli = param::Parameters::parse();

    let is_incrase = if cli.operation == 1 {true} else {false};

    if let Some(m) = &cli.sub_commands {
        println!("指令模式:{:?}", m);
        if let SubCommands::CT{} = m {
            call_func = Box::new(TimeSorter::new(cli.first_number,is_incrase,TimeSortType::CreateTime,&inputs));
        } else if let SubCommands::MT{} = m {
            call_func = Box::new(TimeSorter::new(cli.first_number,is_incrase,TimeSortType::ModifyTime,&inputs));
        } else if let SubCommands::FN {method} = m {
            //method == 0
        }
    } else {
        println!("无效的指令模式");
    }

    call_func.re_sort();
    let review = call_func.change_preview();
    for  item in  review {
        println!("{}==>{}",item.0,item.1);
    }

    call_func.do_change();

}
