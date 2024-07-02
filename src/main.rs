#![feature(let_chains)]

use clap::{Parser, Subcommand};
use std::vec;

mod file_compare;

/// 命令的参数
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct Parameters {
    /// 0 递减
    /// 1 递增
    operation: i8,
    /// 第一个数值的开始模式
    first_number: i32,
    
    //TODO:不能再次使用subcommand了,我想实现类似于管道的操作!
    
    /// 子命令
    #[command(subcommand)]
    sub_commands: Option<SubCommands>,
}

#[derive(Debug,Clone)]
pub enum  Selector{
    /// 默认模式，选中整个文件的模式
    Default{},
    /// 选中首字母范围
    /// 
    /// 当beg>=end的情况下非法
    FirstNumberRange{
        ///beg: 开始数字 <=0 代表从当前最小的值开始
        beg:i32,
        ///end: 结束数字 <=0 代表到当前最大值结束
        end:i32
    },
}

/// 子命令模式
#[derive(Debug, Clone, Subcommand)]
pub enum SubCommands {
    /// 按照创建时间比较
    CreationTime{
    },
    
    /// 全名按照比较
    FullName{
        /// 比较方式 
        /// 
        /// 0 适合整个文件名都是数字的
        /// 
        /// 1 整个文件按照字节序比较
        /// 
        /// 2 整个文件名字母和最后的数字数字分别比较(字母在前,数字在后)
        method:i8,
    },
    
    /// 选中某个片段进行排序
    SubSegment {
        /// selected segment:选中比较的数据内容
        selected_segment: i32,
        /// 比较方式 
        ///
        /// 0 适合整个文件名都是数字的
        ///
        /// 1 整个文件按照字节序比较
        ///
        /// 2 整个文件名字母和最后的数字数字分别比较(字母在前,数字在后)
        method:i8,
    },
    /// 所有片段分别比较
    Segments {
        /// 分段数
        count: i32,
        /// 每段的比较方式,数量要等同于count的值
        /// 
        /// 每段的比较方法方法的
        /// 
        /// 比较方式 
        /// 
        /// 0 适合整个文件名都是数字的
        ///
        ///1 整个文件按照字节序比较
        ///
        ///2 整个文件名字母和最后的数字数字分别比较(字母在前,数字在后)
        methods : Vec<i8>,
    },
}

fn main() {
    let cli = Parameters::parse();

    if let Some(m) = &cli.sub_commands {
        println!("指令模式:{:?}", m);
    } else {
        println!("无效的指令模式");
    }
}
