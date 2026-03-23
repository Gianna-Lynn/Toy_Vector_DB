use std::vec;

use crate::types::{Id, Vector};
pub struct HnswIndex{
    nodes: Vec<HnswNode>, //保存当前图中所有节点
    //entry_point: HnswNode, //因为一开始有可能是空的,没有接入点,是None.
    entry_point:Option<Id>,    //保存入口节点Id
    //max_level: u64 
    max_level: usize           //保存图中最高层数
}

pub struct HnswNode{
   id: Id,
   //data: Vec<Record>, //不能这么写.一个HNSW节点只对应一条向量记录,也就是一个Record, 不要再建立一个Vec<Record>
   data: Vector,
   //level: u64,
   level: usize,
   //neighbors: Vec<Id> // 不能这么写,因为HNSW是分层图.
   neighbors: Vec<Vec<Id>> //neighbors[i]：第 i 层的邻居
}

impl HnswIndex{
    // 关联函数: 不依赖具体对象,不需要self参数
    // 实例方法: 依赖对象当前的状态,必须有&self参数
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
            entry_point:None,
            max_level: 0,
        }
    }
    pub fn len(&self) ->  usize{
        // return nodes.len(); // 不可以这么写,因为这个方法是面向当前对象的,必须有self
        return self.nodes.len();
    }
    pub fn is_empty(&self) -> bool{
        // return nodes.empty();
        return self.nodes.is_empty();
    }

    pub fn insert(&mut self, mut node:HnswNode){
        if self.is_empty(){
            self.entry_point = Some(node.id);   // 不清楚为什么要加这个Some()
                                                //entry_point的类型是Option<Id>,表示没有值None, 要么有值Some(id)
            //node.neighbors = Empty;             //没有Empty这个写法,一开始, node也不是mut node,默认不能修改它的字段.
            node.neighbors = Vec::new();          //空的邻居表应该是空的vector
            
            node.neighbors = vec![Vec::new(); node.level];
            self.max_level =  node.level;
        }
        // self.nodes.append(node);    //Vec::append需要另一个Vec. 这不是"把单个元素放进Vector"的方法.
        self.nodes.push(node);
    }


}


impl HnswNode{
    pub fn new(id : Id, data:Vector, level:usize) -> Self{
        // self.neighbors = Vec::new();
        // for (int i = 0; i < self.level; i++){
        //     new_vec = Vec::new();
        //     self.neighbors.push(new_vec);
        // }
        let mut neighbors = Vec::new();
        for _ in 0..=level {
            neighbors.push(Vec::new());
        }

        Self{
            id,
            data, 
            level,
            neighbors,
        }
    }
}