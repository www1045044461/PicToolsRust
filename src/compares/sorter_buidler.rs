use std::fs::Metadata;
use crate::compares::timesorter::TimeSorter;
use crate::fileinfos::param::Parameters;

/// 排序重命名的Traits
pub trait TAction{
    /// 重新排序生成新的文件顺序的方法
    fn re_sort(&self)->Vec<Metadata>;
}

pub struct SortBuilder{
}

impl SortBuilder{
    fn create(files:Vec<Metadata>,param:Parameters)->impl TAction
    {
        TimeSorter{}
    }
}