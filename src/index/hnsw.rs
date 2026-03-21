pub struct HnswIndex{
    nodes: Vec<HnswNode>,
    //entry_point: HnswNode, //因为一开始有可能是空的,没有接入点,是None.
    entry_point:Option<Id>,
    //max_level: u64 
    max_level: usize
}

pub struct HnswNode{
   id: Id,
   data: Vec<Record>,
   //level: u64,
   level: usize,
   //neighbors: Vec<Id> // 不能这么写,因为HNSW是分层图.
   neighbors: Vec<Vec<Id>> //neighbors[i]：第 i 层的邻居
}