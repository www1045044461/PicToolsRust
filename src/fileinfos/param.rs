use clap::{Parser,Subcommand,ValueEnum,builder::PossibleValue};
use std::fmt;
use std::str::FromStr;

/// 命令的参数
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct Parameters {
    /// 0 递减
    /// 1 递增
    pub operation: i8,

    /// 第一个数值的开始模式
    pub first_number: i32,

    //TODO:不能再次使用subcommand了,我想实现类似于管道的操作!

    /// 子命令
    #[command(subcommand)]
    pub sub_commands: Option<SubCommands>,
}

/// 扩展模式下每个的片段的比较模式
#[derive(Debug, Clone)]
pub enum SegmentCompare
{
    /// StringInc 按照字符顺序递增
    SI,
    /// StringDec 按照字符顺序递减
    SD,
    /// NumericInc 按照数字顺序递增
    NI,
    /// NumericDec 按照数字顺序递减
    ND,
    ///  MixMode-String-Inc-Numeric-Inc 混合模式-字符串递增-数字递增
    MSini,
    ///  MixMode-String-Inc-Numeric-Dec 混合模式-字符串递增-数字递减
    MSind,
    ///  MixMode-String-Dec-Numeric-Inc 混合模式-字符串递减-数字递增
    MSdni,
    ///  MixMode-String-Dec-Numeric-Dec 混合模式-字符串递减-数字递增
    MSdnd,
    /// 非法的参数
    Unknown,
}

impl From<&str> for SegmentCompare {
    fn from(value: &str) -> Self {
        match value {
            "SI" => SegmentCompare::SI,
            "SD" => SegmentCompare::SD,
            "NI" => SegmentCompare::NI,
            "ND" => SegmentCompare::ND,
            "MSini" => SegmentCompare::MSini,
            "MSind" => SegmentCompare::MSind,
            "MSdni" => SegmentCompare::MSdni,
            "MSdnd" => SegmentCompare::MSdnd,
            _ => SegmentCompare::Unknown
        }
    }
}

impl ValueEnum for SegmentCompare {
    ///用于返回所有可能类型数值
    fn value_variants<'a>() -> &'a [Self] {
        const VAR1: &[SegmentCompare] = &[
            SegmentCompare::SI,
            SegmentCompare::SD,
            SegmentCompare::NI,
            SegmentCompare::ND,
            SegmentCompare::MSini,
            SegmentCompare::MSind,
            SegmentCompare::MSdni,
            SegmentCompare::MSdnd,
            SegmentCompare::Unknown,
        ];
        VAR1
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            SegmentCompare::SI => PossibleValue::new("SI"),
            SegmentCompare::SD => PossibleValue::new("SD"),
            SegmentCompare::NI => PossibleValue::new("NI"),
            SegmentCompare::ND => PossibleValue::new("ND"),
            SegmentCompare::MSini => PossibleValue::new("MSini"),
            SegmentCompare::MSind => PossibleValue::new("MSind"),
            SegmentCompare::MSdni => PossibleValue::new("MSdni"),
            SegmentCompare::MSdnd => PossibleValue::new("MSdnd"),
            SegmentCompare::Unknown => PossibleValue::new("Unknown")
        })
    }
}

// ///解析工厂太高端现在先不写
// impl ValueParserFactory for SegmentCompare{
//     type Parser = ();
//
//     fn value_parser() -> Self::Parser {
//         todo!()
//     }
// }

#[derive(Debug)]
pub struct ParseSegmentCompareError(String);

impl fmt::Display for ParseSegmentCompareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseSegmentCompareError {}

impl FromStr for SegmentCompare {
    type Err = ParseSegmentCompareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SI" => Ok(SegmentCompare::SI),
            "SD" => Ok(SegmentCompare::SD),
            "NI" => Ok(SegmentCompare::NI),
            "ND" => Ok(SegmentCompare::ND),
            "MSini" => Ok(SegmentCompare::MSini),
            "MSind" => Ok(SegmentCompare::MSind),
            "MSdni" => Ok(SegmentCompare::MSdni),
            "MSdnd" => Ok(SegmentCompare::MSdnd),
            _ => Err(ParseSegmentCompareError(format!("error input:{}", s))),
        }
    }
}

/// 子命令模式2
#[derive(Debug, Clone, Subcommand)]
pub enum SubCommands {
    /// CreateTime 按照创建时间比较
    CT {},

    /// ModifyTime 根据修改时间比较
    MT {},

    ///FullName 全名按照比较,整个文件名按照string字节序、数字顺序或者混合模式比较
    FN {
        /// 比较方式
        ///
        /// 0 适合整个文件名都是数字的  1234.jpg
        ///
        /// 1 整个文件按照字节序比较
        ///
        /// 2 整个文件名字母和最后的数字数字分别比较(字母在前,数字在后) wdaw234.jpg
        method:i8,
    },

    /// Selected-Segment 选中某个片段进行排序
    SS {
        /// selected segment:选中比较的片段顺序从0开始
        selected_segment: i32,
        /// 选中片段的比较方法
        method:SegmentCompare
    },

    /// 所有片段分别比较 All-Segments
    AS {
        /// 分段数
        count: i32,

        /// 每段的比较方式,数量要等同于count的值
        methods : Vec<SegmentCompare>,
    },
}
