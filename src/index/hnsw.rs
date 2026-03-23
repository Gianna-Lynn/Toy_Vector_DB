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
   level: usize,        // level表示该节点可到达的最高层编号.例如,如果level=3, 则该节点在0,1,2,3层都有邻居.如果level=0, 则该节点只有在第0层有邻居.
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
            // //node.neighbors = Empty;             //没有Empty这个写法,一开始, node也不是mut node,默认不能修改它的字段.
            // //node.neighbors = Vec::new();          //空的邻居表应该是空的vector
            
            // node.neighbors = vec![Vec::new(); node.level];  //创建一个长度为node.level的Vec<Vec<Id>>,每个元素都是一个空的Vec<Id>
            //                                                 //neighbors.len()==level+1
            // // vec![value; count] 是 Rust 中创建一个包含 count 个 value 的 Vec
            // //这一部分删除了,因为在HnswNode::new()中已经创建好了neighbors了. 这里不需要再创建一次了.
            
            self.max_level =  node.level;
        }
        // self.nodes.append(node);    //Vec::append需要另一个Vec. 这不是"把单个元素放进Vector"的方法.
        self.nodes.push(node);
    }

    pub fn find_id_in_neighbors(target_id:Id, target_vector: &Vec<Id>) -> Option<usize>{
        //根据某个节点的id寻找节点在一个邻居数组中的下标

        /* 
        let mut idx: u64 = 0;
        for x in target_vector.iter(){
            if  *x == target_id {
                return idx as u64;
            }
            idx += 1;
        }
        return idx as u64;
        */

        //利用enumerate可以同时获取元素的值和下标,不需要手动维护一个idx变量了.
        //返回的序列形如((0, &x0), (1, &x1), (2, &x2), ...), 其中0,1,2是下标, &x0, &x1, &x2是元素的引用.
        for (idx, &x) in target_vector.iter().enumerate(){
            if x == target_id{
                return Some(idx);
            }
        }
        None
    }


    //而“按 id 找节点”是在做：
    //在 self.nodes: Vec<HnswNode> 里，找到 node.id == target_id 的那个节点
    pub fn find_id_in_nodes(&self, target_id:Id) -> Option<&HnswNode>{
        for (_idx,x) in self.nodes.iter().enumerate(){
            if x.id == target_id{
                return Some(x);
            }
            
        } 
        None
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