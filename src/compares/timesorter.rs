use clap::builder::TryMapValueParser;

use crate::compares::sorter_buidler::TAction;
use std::fs::Metadata;
use std::mem::swap;
use std::string::String;
use std::thread::sleep;
use std::time::SystemTime;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[allow(dead_code)]
pub enum TimeSortType {
    CreateTime,
    ModifyTime,
}

/// 内部使用部分
#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Node {
    pub time_stamp: SystemTime,
    pub name: String,
}

/// TODO:个人认为需要将TimeSorter应用到创建时间和修改时间就不能原封不动的保存Metadata类型
#[allow(dead_code)]
pub struct TimeSorter /*<'a>*/ {
    // pub input_files:&'a Vec<Metadata>,
    input_files: Vec<Node>,
    output_files: Vec<Node>,
    has_changed:bool,
    pub first: i32,
    pub is_increase: bool,
    pub is_sorted: bool,
}

impl TimeSorter {
    /// 创建按照时间排序的方式的排序器
    ///
    /// first:第一个文件序号
    ///
    /// is_add:是否按照增序排序
    ///
    /// files:文件的列表
    ///
    /// type_:时间类型(创建时间、修改时间)
    ///
    /// _files:文件列表
    #[allow(dead_code)]
    pub fn new(_first: i32, _is_add: bool, type_: TimeSortType, _files: &Vec<(String, Metadata)>) -> Self
    {
        let mut input_files: Vec<Node> = Vec::with_capacity(_files.len());

        let mut output_files: Vec<Node> = Vec::with_capacity(_files.len());

        //根据创建时间进行排序
        if type_ == TimeSortType::CreateTime {
            for item in _files {
                input_files.push(Node {
                    name: String::from(item.0.clone()),
                    time_stamp: item.1.created().unwrap(),
                });

                output_files.push(Node {
                    name: String::from(item.0.clone()),
                    time_stamp: item.1.created().unwrap(),
                });
            }
        } else {
            for item in _files {
                input_files.push(Node {
                    name: String::from(item.0.clone()),
                    time_stamp: item.1.modified().unwrap(),
                });

                output_files.push(Node {
                    name: String::from(item.0.clone()),
                    time_stamp: item.1.modified().unwrap(),
                });
            }
        }

        TimeSorter {
            first: _first,
            is_increase: _is_add,
            has_changed:false,
            input_files,
            output_files,
            is_sorted: false,
        }
    }

    /// TODO:这里是否可以考虑--当静态分发时是否可以通过空的struct<bool>与不同的实现绑定来实现,虽然这里实际上是运行期的动态;
    ///
    /// 根据带入闭包返回的true还是false对结果vec进行时间性质的排序
    ///
    /// condition_func 判断内容
    #[allow(dead_code)]
    fn sort_by_time<TCondition: Fn(&SystemTime, &SystemTime) -> bool>(
        &mut self,
        condition_func: TCondition,
    ) {
        for i in 0..self.output_files.len() - 1 {
            for j in 1..self.output_files.len() {
                if condition_func(
                    &self.output_files[i].time_stamp,
                    &self.output_files[j].time_stamp,
                ) {
                    swap(&mut self.output_files[i], &mut self.input_files[j]);
                }
            }
        }
    }
}

impl TAction for TimeSorter {
    fn re_sort(&mut self) {
        let mut prediction: Box<dyn Fn(&SystemTime, &SystemTime) -> bool> = Box::new(|a, b| a < b);

        if self.is_increase == false {
            prediction = Box::new(|a, b| a < b);
        } else {
            prediction = Box::new(|a, b| a >= b);
        }

        if self.is_sorted == false {
            self.sort_by_time(prediction);
            self.is_sorted = true;
        }
    }

    fn change_preview(&self) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::with_capacity(self.output_files.len());

        let len = match self.input_files.len() >= self.output_files.len() {
            true => self.output_files.len(),
            false => self.input_files.len(),
        };

        for i in 0..len {
            let new_str = format!("{}_{}", i, self.output_files[i].name);
            let old_str = self.input_files[i].name.clone();
            result.push((old_str, new_str));
        }

        result
    }

    fn do_change(&mut self) {
        let mut results = Vec::with_capacity(self.output_files.len());

        for i in 0..self.output_files.len() {
            //TODO:格式化第一个index
            let new_index = (i+ self.first as usize).to_string();
            let new_name = new_index + "_" + self.output_files[i].name.as_str();
            results.push(new_name);
        }

        //生成新文件名完毕
        for i in 0..self.output_files.len()
        {
            let ret = std::fs::rename(&self.output_files[i].name,
             &results[i]);
            if let Err(e) = ret
            {
                println!("Mv Failed Error{}:{} ==> {}", e.to_string(), &self.output_files[i].name,
                         &results[i]);
            }else {
                println!("Mv {} ==> {}",&self.output_files[i].name,
                         &results[i]);
            }
        }
    }
}
