///
/// @author Kim Huang
/// @date 2022/4/11 下午1:46
///
use serde::{Serialize,Deserialize};
use bincode;

///
/// ?Sized 表示T的大小编译期间不可知
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where T: Serialize
{
    bincode::serialize(value).unwrap()
}


///
/// 数据传的引用 所以需要生命周期
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> T
    where
        T: serde::de::Deserialize<'a>,{
    bincode::deserialize(bytes).unwrap()
}


