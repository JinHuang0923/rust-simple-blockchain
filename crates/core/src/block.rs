///
/// @author Kim Huang
/// @date 2022/4/11 下午1:32
///
pub struct BlockHeader{
    pub time :i64,//时间戳
    pub tx_hash: String,//transaction merkle hash
    pub pre_hash: String,//上一个区块的hash指
}
pub struct Block{
    pub block_header:BlockHeader,
    pub hash: String, //区块头的Hash值
    pub data: String,//交易信息列表
}