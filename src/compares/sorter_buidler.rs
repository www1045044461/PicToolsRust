/// 排序重命名的Traits
#[allow(dead_code)]
pub trait TAction{
    /// 重新排序生成新的文件顺序的方法
    fn re_sort(&mut self);

    /// 预览修改内容
    fn change_preview(&self)->Vec<(String,String)>;

    /// 执行文件
    fn do_change(&mut self);
}