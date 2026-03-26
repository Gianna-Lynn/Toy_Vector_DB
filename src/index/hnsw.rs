use core::f32;

use crate::types::{Id, Vector};
use crate::distance;
#[derive(Debug)]
pub struct HnswIndex{
    nodes: Vec<HnswNode>, //保存当前图中所有节点
    //entry_point: HnswNode, //因为一开始有可能是空的,没有接入点,是None.
    entry_node_id:Option<Id>,    //保存入口节点Id
    //max_level: u64 
    index_max_level: usize           //保存图中最高层数
}
#[derive(Debug)]
pub struct HnswNode{
   id: Id,
   //data: Vec<Record>, //不能这么写.一个HNSW节点只对应一条向量记录,也就是一个Record, 不要再建立一个Vec<Record>
   data: Vector,
   //level: u64,
   node_max_level: usize,        // level表示该节点可到达的最高层编号.例如,如果level=3, 则该节点在0,1,2,3层都有邻居.如果level=0, 则该节点只有在第0层有邻居.
   //neighbors: Vec<Id> // 不能这么写,因为HNSW是分层图.
   neighbors: Vec<Vec<Id>> //neighbors[i]：第 i 层的邻居
}


impl HnswIndex{
    // 关联函数: 不依赖具体对象,不需要self参数
    // 实例方法: 依赖对象当前的状态,必须有&self参数
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
            entry_node_id:None,
            index_max_level: 0,
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

    pub fn insert(&mut self, mut node_to_insert:HnswNode){
        if self.is_empty(){
            self.entry_node_id = Some(node_to_insert.id);   // 不清楚为什么要加这个Some()
                                                //entry_point的类型是Option<Id>,表示没有值None, 要么有值Some(id)
            // //node.neighbors = Empty;             //没有Empty这个写法,一开始, node也不是mut node,默认不能修改它的字段.
            // //node.neighbors = Vec::new();          //空的邻居表应该是空的vector
            
            // node.neighbors = vec![Vec::new(); node.level];  //创建一个长度为node.level的Vec<Vec<Id>>,每个元素都是一个空的Vec<Id>
            //                                                 //neighbors.len()==level+1
            // // vec![value; count] 是 Rust 中创建一个包含 count 个 value 的 Vec
            // //这一部分删除了,因为在HnswNode::new()中已经创建好了neighbors了. 这里不需要再创建一次了.
            
            self.index_max_level =  node_to_insert.node_max_level;
        }
        // self.nodes.append(node);    //Vec::append需要另一个Vec. 这不是"把单个元素放进Vector"的方法.
        else{

            
            //else分支: 如果图非空
            let mut max_similarity:f32 = f32::NEG_INFINITY; 
            let mut max_idx:usize = 0;
            for (idx, old_node) in self.nodes.iter().enumerate(){
                let temp_similatary = distance::cosine_similarity(old_node.get_data(), node_to_insert.get_data());
                if temp_similatary > max_similarity{
                    max_similarity = temp_similatary;
                    max_idx = idx;
                }
                //max_similarity = temp_similatary.max(max_similarity);   //ご注意ください:这里max的用法用于比较两个f32
            }//endfor: 得到了和node_to_insert最近的一个已有节点在self.nodes中的下标max_idx
            // self.nodes[max_idx].add_neighbor(node_to_insert.id,node_to_insert.node_max_level);
            // node_to_insert.add_neighbor( self.nodes[max_idx].id,  self.get_nodes()[max_idx].node_max_level);
            
            //如果新来的节点层次更高, 要修改整个图的最高层
            if self.index_max_level <  node_to_insert.node_max_level{
                self.index_max_level =  node_to_insert.node_max_level;
                self.entry_node_id = Some(node_to_insert.id);
            }

            let lvls = std::cmp::min(self.nodes[max_idx].node_max_level, node_to_insert.node_max_level);
            
            for i in 0..=lvls{
                self.nodes[max_idx].add_neighbor(node_to_insert.id, i);
                node_to_insert.add_neighbor(self.nodes[max_idx].id,i);
            }
        
        }
        self.nodes.push(node_to_insert);
    }

    pub fn find_index_of_id_in_neighbors(target_id:Id, target_vector: &Vec<Id>) -> Option<usize>{
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
    pub fn get_node_by_id(&self, target_id:Id) -> Option<&HnswNode>{
        for x in self.nodes.iter(){
            if x.id == target_id{
                return Some(x);
            }
            
        } 
        None
    }

    //“按 id 找到节点然后修改它”。
    pub fn get_mut_node_by_id(&mut self, target_id:Id) -> Option<&mut HnswNode>{
        //因为要修改节点,所以需要&mut self,返回值也是&mut HnswNode
        //想要返回一个可变引用,就必须使用iter_mut()方法来遍历self.nodes,而不是iter()方法. 
        //iter_mut()会返回一个可变引用的迭代器,这样我们就可以修改迭代器中的元素了.
        //find()方法会返回一个Option<&mut HnswNode>,如果找到了满足条件的元素,
        //就返回Some(&mut HnswNode),否则返回None.
        self.nodes.iter_mut().find(|x| x.id==target_id)

    }

    // pub fn add_neighbor_to_node(&mut self, node_id:Id, neighbor_id:Id){
        
    // }

    pub fn get_nodes(&self) -> &Vec<HnswNode>{
        &self.nodes
    }
    
    pub fn search_v0(&self, query: &Vector) -> Option<Id>{
        if self.is_empty(){
            return None;
        }
        //如果图非空
        //let mut search_id_opt = self.entry_node_id;

        //改进逻辑,省去loop内部的match search_id_opt的分支.直接利用current_id.
        let mut current_id = if let Some(id) = self.entry_node_id{
            id
        }else{
            return None;
        };
        loop{
            // match search_id_opt{
            //     Some(search_id) =>{
                if let Some(node_to_search) = self.get_node_by_id(current_id){
                    let mut max_similarity:f32 = distance::cosine_similarity(node_to_search.get_data(), query);
                    let mut max_idx:usize = 0;
                    let mut flag = 0;
                    for (idx, neighbor_id) in node_to_search.neighbors[0].iter().enumerate(){
                        if let Some(neighbor_node) = self.get_node_by_id(*neighbor_id){
                            let temp_similatary = distance::cosine_similarity(neighbor_node.get_data(), query);
                            if temp_similatary > max_similarity{
                                max_similarity = temp_similatary;
                                max_idx = idx;
                                flag = 1;
                            }
                        }
                        //max_similarity = temp_similatary.max(max_similarity);   //ご注意ください:这里max的用法用于比较两个f32
                    }//endfor: 得到了和query向量最近的一个已有节点在neighbors[0]中的下标max_idx
                    if flag == 0 {
                        //利用node_to_search
                        //没有任何邻居比当前节点更好.
                        return Some(node_to_search.id);
                    }
                    else{
                        //利用neighbors[0][max_idx], 这个邻居比当前节点更好.
                        let next_id = node_to_search.neighbors[0][max_idx]; //等号右边是一个id.
                        // let next_node_opt = self.get_node_by_id(next_id);
                        current_id = next_id;
                    }
                }
                else {
                    return None;
                }
        }
        return None;
                // None =>{
                //     return None;
                // }
            
    }
        //return None;
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
            node_max_level: level,
            neighbors,
        }
    }

    //给某个节点的某一层添加一个邻居 id."我自己这一层加一个邻居"
    pub fn add_neighbor(&mut self, neighbor_id: Id, neighbor_lvl: usize){
        //self.neighbors[neighbor_lvl].append(neighbor_id); //append是拿来“把一个 Vec 里的所有元素接到另一个 Vec 后面”的。它要的不是一个单独的 Id，而是一个 Vec<Id>。
        if neighbor_lvl < self.neighbors.len(){
            self.neighbors[neighbor_lvl].push(neighbor_id);
        }
    }

    pub fn get_neighbors(&self) -> &Vec<Vec<u64>>{
        &self.neighbors
    }

    pub fn get_data(&self) -> &Vector{
        &self.data
    }

    pub fn get_id(&self) -> &Id{
        &self.id
    }

}