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

pub struct DefaultTAction;

/// 给定TAction接口默认的实现以便区分操作类型
impl TAction for  DefaultTAction
{
    fn re_sort(&mut self) {
        todo!()
    }

    fn change_preview(&self) -> Vec<(String, String)> {
        todo!()
    }

    fn do_change(&mut self) {
        todo!()
    }
}

impl Default for Box<dyn TAction>{
    fn default() -> Self {
        Box::new(DefaultTAction)
    }
}