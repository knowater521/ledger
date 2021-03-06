/**
* 序列化的思路大致是这样的：
* 基础数字类型(u8 u16 u32 u64 usize 和 i8 i16 i32 i64 size) 注意的是大端小端表示法,这里使用了crate byteorder
* Hash 类型的长度都是固定好的，底层数据结构是 u8 数组,所以可以直接拷贝
* String 和 Bytes 底层数据结构都是 Vec<u8>,长度不固定,因此需要在序列化之前计算出长度（compact）

* 反序列化把思路倒过来就行了，目前测试用例只覆盖了一部分，需要补充
*/