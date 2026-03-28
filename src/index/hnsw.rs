use core::f32;
use std::collections::HashSet;
use std::f32::INFINITY;

use crate::distance;
use crate::types::{Id, Vector};
#[derive(Debug)]
pub struct HnswIndex {
    nodes: Vec<HnswNode>, //保存当前图中所有节点
    //entry_point: HnswNode, //因为一开始有可能是空的,没有接入点,是None.
    entry_node_id: Option<Id>, //保存入口节点Id
    //max_level: u64
    index_max_level: usize, //保存图中最高层数
}
#[derive(Debug)]
pub struct HnswNode {
    id: Id,
    //data: Vec<Record>, //不能这么写.一个HNSW节点只对应一条向量记录,也就是一个Record, 不要再建立一个Vec<Record>
    data: Vector,
    //level: u64,
    node_max_level: usize, // level表示该节点可到达的最高层编号.例如,如果level=3, 则该节点在0,1,2,3层都有邻居.如果level=0, 则该节点只有在第0层有邻居.
    //neighbors: Vec<Id> // 不能这么写,因为HNSW是分层图.
    neighbors: Vec<Vec<Id>>, //neighbors[i]：第 i 层的邻居
}

impl HnswIndex {
    // 关联函数: 不依赖具体对象,不需要self参数
    // 实例方法: 依赖对象当前的状态,必须有&self参数
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            entry_node_id: None,
            index_max_level: 0,
        }
    }
    pub fn len(&self) -> usize {
        // return nodes.len(); // 不可以这么写,因为这个方法是面向当前对象的,必须有self
        return self.nodes.len();
    }
    pub fn is_empty(&self) -> bool {
        // return nodes.empty();
        return self.nodes.is_empty();
    }

    pub fn insert(&mut self, mut node_to_insert: HnswNode) {
        if self.is_empty() {
            self.entry_node_id = Some(node_to_insert.id); // 不清楚为什么要加这个Some()
                                                          //entry_point的类型是Option<Id>,表示没有值None, 要么有值Some(id)
                                                          // //node.neighbors = Empty;             //没有Empty这个写法,一开始, node也不是mut node,默认不能修改它的字段.
                                                          // //node.neighbors = Vec::new();          //空的邻居表应该是空的vector

            // node.neighbors = vec![Vec::new(); node.level];  //创建一个长度为node.level的Vec<Vec<Id>>,每个元素都是一个空的Vec<Id>
            //                                                 //neighbors.len()==level+1
            // // vec![value; count] 是 Rust 中创建一个包含 count 个 value 的 Vec
            // //这一部分删除了,因为在HnswNode::new()中已经创建好了neighbors了. 这里不需要再创建一次了.

            self.index_max_level = node_to_insert.node_max_level;
        }
        // self.nodes.append(node);    //Vec::append需要另一个Vec. 这不是"把单个元素放进Vector"的方法.
        else {
            //else分支: 如果图非空
            let mut max_similarity: f32 = f32::NEG_INFINITY;
            let mut max_idx: usize = 0;
            for (idx, old_node) in self.nodes.iter().enumerate() {
                let temp_similarity =
                    distance::cosine_similarity(old_node.get_data(), node_to_insert.get_data());
                if temp_similarity > max_similarity {
                    max_similarity = temp_similarity;
                    max_idx = idx;
                }
                //max_similarity = temp_similarity.max(max_similarity);   //ご注意ください:这里max的用法用于比较两个f32
            } //endfor: 得到了和node_to_insert最近的一个已有节点在self.nodes中的下标max_idx
              // self.nodes[max_idx].add_neighbor(node_to_insert.id,node_to_insert.node_max_level);
              // node_to_insert.add_neighbor( self.nodes[max_idx].id,  self.get_nodes()[max_idx].node_max_level);

            //如果新来的节点层次更高, 要修改整个图的最高层
            if self.index_max_level < node_to_insert.node_max_level {
                self.index_max_level = node_to_insert.node_max_level;
                self.entry_node_id = Some(node_to_insert.id);
            }

            let lvls = std::cmp::min(
                self.nodes[max_idx].node_max_level,
                node_to_insert.node_max_level,
            );

            for i in 0..=lvls {
                self.nodes[max_idx].add_neighbor(node_to_insert.id, i);
                node_to_insert.add_neighbor(self.nodes[max_idx].id, i);
            }
        }
        self.nodes.push(node_to_insert);
    }

    pub fn find_index_of_id_in_neighbors(target_id: Id, target_vector: &Vec<Id>) -> Option<usize> {
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
        for (idx, &x) in target_vector.iter().enumerate() {
            if x == target_id {
                return Some(idx);
            }
        }
        None
    }

    //而“按 id 找节点”是在做：
    //在 self.nodes: Vec<HnswNode> 里，找到 node.id == target_id 的那个节点
    pub fn get_node_by_id(&self, target_id: Id) -> Option<&HnswNode> {
        for x in self.nodes.iter() {
            if x.id == target_id {
                return Some(x);
            }
        }
        None
    }

    //“按 id 找到节点然后修改它”。
    pub fn get_mut_node_by_id(&mut self, target_id: Id) -> Option<&mut HnswNode> {
        //因为要修改节点,所以需要&mut self,返回值也是&mut HnswNode
        //想要返回一个可变引用,就必须使用iter_mut()方法来遍历self.nodes,而不是iter()方法.
        //iter_mut()会返回一个可变引用的迭代器,这样我们就可以修改迭代器中的元素了.
        //find()方法会返回一个Option<&mut HnswNode>,如果找到了满足条件的元素,
        //就返回Some(&mut HnswNode),否则返回None.
        self.nodes.iter_mut().find(|x| x.id == target_id)
    }

    // pub fn add_neighbor_to_node(&mut self, node_id:Id, neighbor_id:Id){

    // }

    pub fn get_nodes(&self) -> &Vec<HnswNode> {
        &self.nodes
    }

    pub fn search_v0(&self, query: &Vector) -> Option<Id> {
        if self.is_empty() {
            return None;
        }
        //如果图非空
        //let mut search_id_opt = self.entry_node_id;

        //改进逻辑,省去loop内部的match search_id_opt的分支.直接利用current_id.
        let mut current_id = if let Some(id) = self.entry_node_id {
            id
        } else {
            return None;
        };
        loop {
            // match search_id_opt{
            //     Some(search_id) =>{
            if let Some(node_to_search) = self.get_node_by_id(current_id) {
                let mut max_similarity: f32 =
                    distance::cosine_similarity(node_to_search.get_data(), query);
                let mut max_idx: usize = 0;
                let mut flag = 0;
                for (idx, neighbor_id) in node_to_search.neighbors[0].iter().enumerate() {
                    if let Some(neighbor_node) = self.get_node_by_id(*neighbor_id) {
                        let temp_similarity =
                            distance::cosine_similarity(neighbor_node.get_data(), query);
                        if temp_similarity > max_similarity {
                            max_similarity = temp_similarity;
                            max_idx = idx;
                            flag = 1;
                        }
                    }
                    //max_similarity = temp_similarity.max(max_similarity);   //ご注意ください:这里max的用法用于比较两个f32
                } //endfor: 得到了和query向量最近的一个已有节点在neighbors[0]中的下标max_idx
                if flag == 0 {
                    //利用node_to_search
                    //没有任何邻居比当前节点更好.
                    return Some(node_to_search.id);
                } else {
                    //利用neighbors[0][max_idx], 这个邻居比当前节点更好.
                    let next_id = node_to_search.neighbors[0][max_idx]; //等号右边是一个id.
                                                                        // let next_node_opt = self.get_node_by_id(next_id);
                    current_id = next_id;
                }
            } else {
                return None;
            }
        }
    }

    pub fn extract_nearest(&self, set: &HashSet<Id>, query: &Vector) -> Option<Id> {
        //ご注意ください:如果函数签名中返回值仅仅写Id, 那么假如集合HashSet为空,就不会有合法的返回值.
        //需要改成返回Option<Id>
        //unimplemented!();
        let mut max_similarity = -INFINITY;
        let mut max_elem_id: Option<Id> = None;

        for elem_id in set{
            let elem_node = self.get_node_by_id(*elem_id).expect("elem_node不存在");
            let temp_similarity = distance::cosine_similarity(elem_node.get_data(), query);
            if temp_similarity > max_similarity{
                max_similarity = temp_similarity;
                max_elem_id = Some(*elem_id);
            }//endif
        }//endfor
        max_elem_id
    }


    pub fn get_furthest(&self, set: &HashSet<Id>, query: &Vector) -> Option<Id> {
        let mut min_similarity = INFINITY;
        let mut min_elem_id: Option<Id> = None;

        for elem_id in set{
            let elem_node = self.get_node_by_id(*elem_id).expect("elem_node不存在");
            let temp_similarity = distance::cosine_similarity(elem_node.get_data(), query);
            if temp_similarity < min_similarity{
                min_similarity = temp_similarity;
                min_elem_id = Some(*elem_id);
            }//endif
        }//endfor
        min_elem_id
    }

    pub fn get_neighbors_at_lvl(&self, id: Id, lvl: usize) -> HashSet<Id> {
        //ご注意ください:如果你用的是一个不带有&self的静态方法,方法不能访问self.nodes,不能访问索引内部的图结构,只能依靠传入的参数工作.
        //因此,必须访问self.
        //这样一来就可以用self.get_neighbors_at_lvl(id, lvl), 而不是更麻烦的HnswIndex::get_neighbors_at_lvl(&self, id, lvl)
        //unimplemented!()
        //数据段职能理解错误:index_max_level仅指代整张图最高能有多少层.而单个节点拥有的层数可能小于最大层数.
        //                 所以这里if条件不应该是if lvl > self.index_max_level
        let node_to_search = self.get_node_by_id(id).expect("get_node_by_id不存在");
        
        //工程问题: 现在node_max_level并没有明确的更新机制,暂时不要用这个字段行事.
        // if lvl > node_to_search.node_max_level{
        //还有一个小问题,Rust的下标从0开始, 所以这个地方应该是>=而不是>
        //例如,get_neighbors().len()==3的时候,合法下标只有0,1,2,并不包括3.
        if lvl >= node_to_search.get_neighbors().len(){
            //return None  
            //语法问题:返回一个空集合,不要返回None,而是返回HashSet::new()    
            return HashSet::new();
        }
        let neighbors_set: HashSet<Id> = node_to_search.get_neighbors()[lvl].iter().cloned().collect();
        neighbors_set
    }

    pub fn search_layer_v0(
        &self,
        query: &Vector,
        entry_id: Id,
        level: usize,
        ef: usize,
    ) -> HashSet<Id> {

        assert!(ef >= 1, "ef至少为1");

        //已经访问过的节点集合v
        let mut v_set: HashSet<Id> = HashSet::new();
        //候选集合C
        let mut c_set: HashSet<Id> = HashSet::new();
        //目前找到的最近点集合W
        let mut w_set: HashSet<Id> = HashSet::new();

        //ご注意ください:初始化时要把entry_id放进这三个集合,否则一开始集合为空,后面的逻辑都不会执行.
        v_set.insert(entry_id);
        c_set.insert(entry_id);
        w_set.insert(entry_id);

        while !c_set.is_empty() {
            //ご注意ください:extract方法是取出, 要完成"拿出来+删掉".需要一个额外的删除语句.remove
            let c_id = self.extract_nearest(&c_set, query).expect("c_id不存在");
            c_set.remove(&c_id);
            //ご注意ください:get方法是查看, 要完成"拿出来读取", 而不是删掉.
            let mut f_id = self.get_furthest(&w_set, query).expect("f_id不存在");
            let c_node = self.get_node_by_id(c_id).expect("节点不存在");
            let mut f_node = self.get_node_by_id(f_id).expect("节点不存在");

            // 逻辑:cosine_similarity的值: 两个向量距离越近, similarity的值越大.不要写反了.
            if distance::cosine_similarity(&c_node.data, query)
                < distance::cosine_similarity(&f_node.data, query)
            {
                break;
            }

            let c_neighbors_set_at_lvl = self.get_neighbors_at_lvl(c_id, level);
            // 语法:此处c_neighbors_set_at_lvl是Option<HashSet<u64>>.
            // 如果直接接上for e_id in c_neighbors_set_at_lvl开始迭代Option<HashSet<u64>>,
            // 那么最多循环1次(当Option的类型是Some),因为Option最多只有1个元素.  
            // 两种修改方法,一是match语句,二是.expect("msg")
            for e_id in c_neighbors_set_at_lvl {
                if v_set.contains(&e_id) {
                    continue;
                }
                v_set.insert(e_id);
                
                f_id = self.get_furthest(&w_set, query).expect("f_id不存在");
                //逻辑:f_id被重新取了一次,但是f_node并没有被重新取,会导致下面的语句没能同步更新.
                f_node = self.get_node_by_id(f_id).expect("节点不存在");

                let e_node = self.get_node_by_id( e_id).expect("节点不存在");
                // 逻辑:cosine_similarity的值: 两个向量距离越近, similarity的值越大.不要写反了.
                if distance::cosine_similarity(&e_node.data, query)
                    > distance::cosine_similarity(&f_node.data, query)
                    || w_set.len() < ef
                {
                    c_set.insert(e_id);
                    w_set.insert(e_id);
                    if w_set.len() > ef {
                        let id_to_remove_op = self.get_furthest(&w_set, query);
                        if let Some(id_to_remove) = id_to_remove_op{
                            w_set.remove(&id_to_remove);
                        }
                        
                    }
                }//endif
            }//endfor
        }
        w_set
    }
}

impl HnswNode {
    pub fn new(id: Id, data: Vector, level: usize) -> Self {
        // self.neighbors = Vec::new();
        // for (int i = 0; i < self.level; i++){
        //     new_vec = Vec::new();
        //     self.neighbors.push(new_vec);
        // }
        let mut neighbors = Vec::new();
        for _ in 0..=level {
            neighbors.push(Vec::new());
        }

        Self {
            id,
            data,
            node_max_level: level,
            neighbors,
        }
    }

    //给某个节点的某一层添加一个邻居 id."我自己这一层加一个邻居"
    pub fn add_neighbor(&mut self, neighbor_id: Id, neighbor_lvl: usize) {
        //self.neighbors[neighbor_lvl].append(neighbor_id); //append是拿来“把一个 Vec 里的所有元素接到另一个 Vec 后面”的。它要的不是一个单独的 Id，而是一个 Vec<Id>。
        if neighbor_lvl < self.neighbors.len() {
            self.neighbors[neighbor_lvl].push(neighbor_id);
        }
    }

    pub fn get_neighbors(&self) -> &Vec<Vec<u64>> {
        &self.neighbors
    }

    pub fn get_data(&self) -> &Vector {
        &self.data
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }
}
