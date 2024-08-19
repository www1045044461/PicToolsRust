use std::fs::Metadata;
use crate::compares::sorter_buidler::TAction;

pub struct TimeSorter{

}

impl TAction for TimeSorter{
    fn re_sort(&self) -> Vec<Metadata> {
        todo!()
    }
}