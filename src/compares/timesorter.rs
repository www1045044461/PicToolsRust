use crate::compares::sorter_buidler::TAction;
use std::fs::Metadata;
use std::mem::swap;
use std::string::String;
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
pub struct TimeSorter/*<'a>*/ {
    // pub input_files:&'a Vec<Metadata>,
    input_files: Vec<Node>,
    output_files: Vec<Node>,
    has_changed:bool,
    pub first: i32,
    pub is_increase: bool,
}

impl TimeSorter {
    /// 创建按照时间排序的方式的排序器
    ///
    /// @param first:第一个文件序号
    ///
    /// @param files:文件的列表
    #[allow(dead_code)]
    fn from(_first: i32, _is_add: bool, type_: TimeSortType, _files: &Vec<(&str, Metadata)>) -> Self
    {
        let mut input_files: Vec<Node> = Vec::with_capacity(_files.len());

        let mut output_files: Vec<Node> = Vec::with_capacity(_files.len());

        //根据创建时间进行排序
        if type_ == TimeSortType::CreateTime {
            for item in _files {
                input_files.push(Node {
                    name: String::from(item.0),
                    time_stamp: item.1.created().unwrap(),
                });

                output_files.push(Node {
                    name: String::from(item.0),
                    time_stamp: item.1.created().unwrap(),
                });
            }
        } else {
            for item in _files {
                input_files.push(Node {
                    name: String::from(item.0),
                    time_stamp: item.1.modified().unwrap(),
                });

                output_files.push(Node {
                    name: String::from(item.0),
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
        }
    }

    /// TODO:这里是否可以考虑--当静态分发时是否可以通过空的struct<bool>与不同的实现绑定来实现,虽然这里实际上是运行期的动态;
    ///
    /// 根据带入闭包返回的true还是false对结果vec进行时间性质的排序
    ///
    /// condition_func 判断内容
    #[allow(dead_code)]
    fn sort_by_time<TCondition>(&mut self, condition_func:TCondition)
        where TCondition:Fn(SystemTime,SystemTime)->bool
    {
        for i in 0..self.output_files.len() - 1 {
            for j in 1..self.output_files.len()
            {
                if condition_func(self.output_files[i].time_stamp,self.output_files[j].time_stamp)
                {
                    swap(&mut self.output_files[i], &mut self.input_files[j]);
                }
            }
        }
    }
}

impl TAction for TimeSorter {
    fn re_sort(&mut self) {

        // let mut prediction : dyn for<'a> Fn(&'a SystemTime, &'a SystemTime) -> bool = |a, b|{a<b};

        let func_increase = |a, b| { a < b };

        let func_decline = |a, b| { a > b };

        //rust尽可能利用编译器的上下文推导,不需要显式的定义代理类型
        let prediction =
            if self.is_increase == false
            { func_increase } else { func_decline };

        self.sort_by_time(prediction);
    }

    fn change_preview(&self) -> Vec<(String, String)> {
        let mut result = Vec::with_capacity(self.output_files.len());

        for i in 0..self.output_files.len() {
            //TODO:格式化头个index
            let new_i = (i + (self.first as usize)).to_string();
            let new_name = new_i+"_" + &self.output_files[i].name;
            result.push(
                (
                    String::from(&self.output_files[i].name),new_name
                )
            );
        }
        result
    }

    fn do_change(&mut self) {
        todo!()
    }
}