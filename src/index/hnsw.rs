use core::f32;
use rand::Rng;
use std::collections::HashSet;
use std::f32::{INFINITY};

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
#[derive(Debug, Clone)]
pub struct HnswNode {
    id: Id,
    //data: Vec<Record>, //不能这么写.一个HNSW节点只对应一条向量记录,也就是一个Record, 不要再建立一个Vec<Record>
    data: Vector,
    //level: u64,
    node_max_level: usize, // level表示该节点可到达的最高层编号.例如,如果level=3, 则该节点在0,1,2,3层都有邻居.如果level=0, 则该节点只有在第0层有邻居.
    //neighbors: Vec<Id> // 不能这么写,因为HNSW是分层图.
    neighbors: Vec<Vec<Id>>, //neighbors[i]：第 i 层的邻居
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
    pub fn add_neighbor(&mut self, neighbor_id: Id, level_to_add: usize) {
        //self.neighbors[neighbor_lvl].append(neighbor_id); //append是拿来“把一个 Vec 里的所有元素接到另一个 Vec 后面”的。它要的不是一个单独的 Id，而是一个 Vec<Id>。
        if level_to_add < self.neighbors.len() {
            self.neighbors[level_to_add].push(neighbor_id);
        }
    }




    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_data(&self) -> &Vector {
        &self.data
    }

    pub fn get_node_max_level(&self) -> &usize{
        &self.node_max_level

    }
    pub fn get_neighbors(&self) -> &Vec<Vec<u64>> {
        &self.neighbors
    }

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
        //因为未来想要要修改节点,所以需要&mut self,返回值也是&mut HnswNode
        //想要返回一个可变引用,就必须使用iter_mut()方法来遍历self.nodes,而不是iter()方法.
        //iter_mut()会返回一个可变引用的迭代器,这样我们就可以修改迭代器中的元素了.
        //find()方法会返回一个Option<&mut HnswNode>,如果找到了满足条件的元素,
        //就返回Some(&mut HnswNode),否则返回None.
        self.nodes.iter_mut().find(|x| x.id == target_id)
    }

    //把编号为neighbor_id的节点添加到node_id的第level层的邻居Vec中
    pub fn add_neighbor_to_node_at_level(&mut self, node_id:Id, neighbor_id:Id, level: usize){
        // let neighbor_node = self.get_node_by_id(neighbor_id)
        //     .expect("in add_neighbor_to_node, in get_mut_node_by_id, got None");
        // //下面这地方只需要值, 不需要引用, 所以都加上*解引用, 只要值.
        // let neighbor_node_id_value = *neighbor_node.get_id();
        // 改进: 实际上根本不需要解引用, 因为neighbor_node_id_value就是函数参数中的neighbor_node, 本来就有了, 白绕一大圈
        
        let node = self.get_mut_node_by_id(node_id)
            .expect("in add_neighbor_to_node, in get_mut_node_by_id, got None");
        node.add_neighbor(neighbor_id, level);
    }

    pub fn get_nodes(&self) -> &Vec<HnswNode> {
        &self.nodes
    }

    pub fn get_entry_node_id(&self) -> Option<u64> {
        self.entry_node_id
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

    pub fn get_nearest(&self, set: &HashSet<Id>, query: &Vector) -> Option<Id> {
        //ご注意ください:如果函数签名中返回值仅仅写Id, 那么假如集合HashSet为空,就不会有合法的返回值.
        //需要改成返回Option<Id>
        //unimplemented!();
        let mut max_similarity = -INFINITY;
        let mut max_elem_id: Option<Id> = None;

        for elem_id in set {
            let elem_node = self.get_node_by_id(*elem_id).expect("elem_node不存在");
            let temp_similarity = distance::cosine_similarity(elem_node.get_data(), query);
            if temp_similarity > max_similarity {
                max_similarity = temp_similarity;
                max_elem_id = Some(*elem_id);
            } //endif
        } //endfor
        max_elem_id
    }

    pub fn get_furthest(&self, set: &HashSet<Id>, query: &Vector) -> Option<Id> {
        let mut min_similarity = INFINITY;
        let mut min_elem_id: Option<Id> = None;

        for elem_id in set {
            let elem_node = self.get_node_by_id(*elem_id).expect("elem_node不存在");
            let temp_similarity = distance::cosine_similarity(elem_node.get_data(), query);
            if temp_similarity < min_similarity {
                min_similarity = temp_similarity;
                min_elem_id = Some(*elem_id);
            } //endif
        } //endfor
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
        if lvl >= node_to_search.get_neighbors().len() {
            //return None
            //语法问题:返回一个空集合,不要返回None,而是返回HashSet::new()
            return HashSet::new();
        }
        let neighbors_set: HashSet<Id> = node_to_search.get_neighbors()[lvl]
            .iter()
            .cloned()
            .collect();
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
            let c_id = self.get_nearest(&c_set, query).expect("c_id不存在");
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

                let e_node = self.get_node_by_id(e_id).expect("节点不存在");
                // 逻辑:cosine_similarity的值: 两个向量距离越近, similarity的值越大.不要写反了.
                if distance::cosine_similarity(&e_node.data, query)
                    >= distance::cosine_similarity(&f_node.data, query)
                    || w_set.len() < ef
                {
                    c_set.insert(e_id);
                    w_set.insert(e_id);
                    if w_set.len() > ef {
                        let id_to_remove_op = self.get_furthest(&w_set, query);
                        if let Some(id_to_remove) = id_to_remove_op {
                            w_set.remove(&id_to_remove);
                        }
                    }
                } //endif
            } //endfor
        }
        w_set
    }

    pub fn greedy_search_at_level(&self, query: &Vector, entry_id: Id, level: usize) -> Id {
        //在该层经过贪心搜索后得到的最终节点 id. 区分于search_layer_v0(返回的是一个set)
        let mut current_node = self
            .get_node_by_id(entry_id)
            .expect("未能根据entry_id检索到entry_node");
        let mut current_node_neighbors = &current_node.get_neighbors()[level];
        let mut max_similarity = distance::cosine_similarity(current_node.get_data(), query);
        let mut max_similarity_id = entry_id;
        loop {
            for neighbor in current_node_neighbors {
                let neighbor_node = self
                    .get_node_by_id(*neighbor)
                    .expect("未能检索到neighbor_node");
                let neighbor_node_similarity =
                    distance::cosine_similarity(neighbor_node.get_data(), query);
                if neighbor_node_similarity > max_similarity {
                    max_similarity = neighbor_node_similarity;
                    max_similarity_id = *neighbor_node.get_id();
                }
            }
            if current_node.get_id() == &max_similarity_id {
                return max_similarity_id;
            } else {
                current_node = self
                    .get_node_by_id(max_similarity_id)
                    .expect("未能更新current_node");
                current_node_neighbors = &current_node.get_neighbors()[level];
            }
        }
    }

    pub fn search_knn_v1(&self, query: &Vector, k: usize, ef_search: usize) -> Vec<Id> {
        //ef_searchs是第0层的搜索宽度.
        //返回: 按与 query 的相似度从高到低排序后的 top-k 节点 id
        //let entry_node_id=self.entry_node_id.expect("search_knn_v1: entry_node_id不存在");
        //let entry_node = self.get_node_by_id(entry_node_id).expect("search_knn_v1: 未能找到entry_node");
        //let entry_node_neighbors = entry_node.get_neighbors();
        //let neighbors_num = entry_node_neighbors.len();
        //let mut entry_node_neighbors_lvl = 0;

        //let mut current_entry_id = self.entry_node_id.expect("search_knn_v1: entry_node_id不存在");
        //这个地方不要直接panic, 而是返回一个空集合.
        let Some(mut current_entry_id) = self.entry_node_id else {
            return Vec::new();
        };
        for lvl in (1..=self.index_max_level).rev() {
            current_entry_id = self.greedy_search_at_level(query, current_entry_id, lvl);
        }
        let candidate_set = self.search_layer_v0(query, current_entry_id, 0, ef_search);
        let mut candidate_sequence: Vec<Id> = candidate_set.into_iter().collect();
        //into_iter消费了candidate_set, 以后就不能用了.
        candidate_sequence.sort_by(|id1, id2| {
            let node_1 = self.get_node_by_id(*id1).expect("id1找不到节点");
            let node_2 = self.get_node_by_id(*id2).expect("id2找不到节点");
            let score1 = distance::cosine_similarity(query, node_1.get_data());
            let score2 = distance::cosine_similarity(query, node_2.get_data());
            // partial_cmp 只做一件事：比较两个数，返回 Less、Equal、Greater。
            match score2.partial_cmp(&score1).unwrap() {
                // partical_cmp语句返回: scroe2 (  > / < / = ) score1
                // 这会导致高分排在更小下标,也就是降序
                // sort_by 约定：比较器返回 Less，就表示第一个参数要排在第二个参数前面；
                // 返回 Greater，就表示第一个参数要排在第二个参数后面。
                std::cmp::Ordering::Equal => id1.cmp(id2),
                other => other,
            }
        });

        if candidate_sequence.len() < k {
            return candidate_sequence;
        } else {
            return candidate_sequence[..k].to_vec();
        }
    }

    pub fn sample_level(&self) -> usize {
        //返回一个非负整数，表示新插入节点的最高层编号。
        //返回值应满足“高层更稀少、低层更多”的分布趋势.
        //连续随机抛硬币，直到失败为止, 或用某种几何分布近似方法
        let p = 0.5;
        let mut rng = rand::thread_rng();
        let mut level = 0usize;

        //以p的概率返回true. 大多数节点会停在0或者1层.
        while rng.gen_bool(p) {
            level += 1;
        }
        level
    }

    pub fn insert_v1(&mut self, id: Id, data: Vector, ef: usize, m: usize, m_max: usize) {
        // 给定一个新节点 (id, data)，将其插入现有 HNSW 索引中。
        
        // Phase 1: Create new node.
        // 为该节点采样层高
        let new_node_level = self.sample_level();
        // 创建节点
        let data_copy = data.clone();
        let new_node = HnswNode::new(id, data_copy, new_node_level);

        // Case 1: 
        // 若图为空：
        if self.is_empty(){
            // 设置 entry_node_id
            self.entry_node_id = Some(id);
            // 更新 index_max_level
            self.index_max_level = new_node_level;
            // 插入图中
            self.nodes.push(new_node);
            //结束
            return;
        }

        // Case 2: 图非空。

        // Phase 2: Top-down greedy routing.
        // - 从当前 entry_node_id 出发。
        // - 从 index_max_level 一路下降到 new_node_level + 1。
        // - 在这些层上调用 greedy_search_at_level(...)
        // - 得到一个更接近新节点向量的入口点。

        
        // debug: 遗漏: 因为前面的代码实在太多, 导致遗漏了"新节点进图"的这个动作.
        self.nodes.push(new_node); // 所有权分析: push方法接收的是无引用的参数, 获得了所有权.

        let mut search_result_id = self.get_entry_node_id().expect("in insert_v1, entry_node_id is none");

        // debug: 理解有误: greedy_search是在“新节点根本不存在的更高层”上完成的. 如果新节点的层数大于等于旧图最高层, 那么这个assert断言就会失败.
        //                 正确的做法应该是使用条件逻辑, 分为 "新节点之上有更高层"和 "新节点之上没有更高层"两种情况处理.
        //        盲信伪代码: 伪代码 line 5 的原文是从 旧图最高层开始向下遍历到新节点所在层的上一层, 但是没考虑到"新节点之上没有更高层"这种情况.
        //assert!(self.index_max_level >= new_node_level + 1, "in insert_v1, expected self.index_max_level >= new_node_level + 1, got < ");
        
        if self.index_max_level >= new_node_level + 1{
            // if这个条件不等式的成立保证了下面的for循环正常运行.
            // debug: 混淆点位于伪代码(line 6, line 7)
            // 伪代码line7中的ep已经不再是"整个Hnsw图的入口点", 而是一个"中间变量".
            // 因为我们已经有"返回最近的唯一答案"greedy_search_at_level函数,所以line 6和line 7被一起执行.
            // 伪代码中的ep, 实际上是下面这个for循环中的search_result_id
            for l in (self.index_max_level .. new_node_level + 1).rev(){
                // 所有权分析: 由于new_node在前面的"新节点进图"中, 所有权已经转移, 被消费掉了, 所以在这里调用new_node.get_data()就是非法的.
                //            解决方案就是在push之前, 就把get_data()的结果保存下来, 供这里调用.
                //            但是由于get_data()返回的是data: Vector,也就是insert_v1的参数data,所以这里可以直接用&data代替.
                search_result_id = self.greedy_search_at_level(&data,search_result_id, l);
            }

        }

        // Phase 3: Layer-by-layer neighbor search.
        // - 对每一层 level = min(index_max_level, new_node_level), ..., 0。
        let min_level = std::cmp::min(self.index_max_level, new_node_level);
        
        // debug: 对for循环范围声明不清楚. current_lvl需要从min_level开始一直反向遍历到0, 并且包含min_level和0.
        // for current_lvl in (0 .. min_level).rev(){    //这样的写法是从min_level_-1开始遍历到0
        for current_lvl in (0..=min_level).rev(){ //这样的写法是从min_level开始遍历到0
            // - 调用 search_layer_v0(query = new_node.data, entry_id = current_entry, level, ef_construction)。
            // - 获得候选集合 W_set。
            let w_set = self.search_layer_v0(&data, search_result_id, current_lvl, ef);
            // - 从候选中选出最多 M 个邻居。(尚未实现)
            let neighbors_set = &self.select_neighbors_simple(&w_set,&data,m);
            // - 将这些邻居与新节点双向连边。(line 11)
            for neighbor_node_id in neighbors_set{
                // debug: 遗漏导致的后果: 如果new_node没有及时进图的话,在图中通过id来查找node的动作就会失败,整个for循环都无法正常执行.
                self.add_neighbor_to_node_at_level(id, *neighbor_node_id, current_lvl);
                self.add_neighbor_to_node_at_level(*neighbor_node_id, id, current_lvl);
            }
            // - 为下一层更新入口点。(line 12)
            // 这个地方的引用体系堪称灾难, 字面意义上的堪称灾难.
            for e_id in neighbors_set{

                // let mut e_node = self.get_mut_node_by_id(*e_id)
                //     .expect("in insert_v1, get_node_by_id cannot handle e_id");
                // let e_conn = &e_node.get_neighbors()[current_lvl];
                // let e_conn_set: HashSet<u64> = e_conn.iter().take(m).copied().collect();

                let e_conn_set: HashSet<u64> = {
                    let e_node = self.get_node_by_id(*e_id)
                        .expect("in insert_v1, get_node_by_id cannot handle e_id");
                    e_node.get_neighbors()[current_lvl]
                        .iter()
                        .take(m)
                        .copied()
                        .collect()
                };

                if e_conn_set.len() > m_max {
                    // let e_new_conn_set = &self.select_neighbors_simple(&e_conn_set,e_node.get_data(), m_max);
                    // 改进: 这里返回的是一个有序Vec
                    let e_new_conn_vec: Vec<u64> = {
                        let e_node = self.get_node_by_id(*e_id)
                            .expect("in insert_v1, get_node_by_id cannot handle e_id");

                        self.select_neighbors_simple(&e_conn_set,e_node.get_data(), m_max)
                    };
                    //此处对应 line 16, 但目前不是很会写.

                    let e_node = self.get_mut_node_by_id(*e_id)
                        .expect("in insert_v1, get_node_by_id cannot handle e_id");
                    
                    //e_node.neighbors[current_lvl] = e_new_conn_set.iter().copied().collect();
                    // 改进: 利用vec直接赋值
                    e_node.neighbors[current_lvl] = e_new_conn_vec;
                }
            }

            //此处对应 line 17, 目前就是随便从w_set当中拿一个
            // debug: 理解有误.line 17的ep是一个局部变量, 不再代表"整个hnsw图的入口点"
            // self.entry_node_id = w_set.iter().next().copied();
            // TODO: 这个地方伪代码直观上看是把一个集合赋值给了一个id.这显然是有待改进的.不过为了通过编译, 我们姑且先这么写吧.
            search_result_id = w_set.iter().next().copied().expect("searchh_result_id赋值错误");
                 
        }
       
        // Phase 4: Update entry point if needed.
        // - 如果新节点层高高于当前 index_max_level。
        // - 更新 entry_node_id。
        // - 更新 index_max_level。
        if new_node_level > self.index_max_level{
            self.entry_node_id = Some(id);
            self.index_max_level = new_node_level;
        }
    }


    // 实现一个简化版本的邻居选择函数
    // cadidates: 候选集合, m:返回的邻居数目,如果不足m个,就全部返回.
    pub fn select_neighbors_simple(&self, candidate_set: &HashSet<Id>, query: &Vector, m: usize) -> Vec<Id>{
        
        let mut candidate_sequence: Vec<Id> = candidate_set.iter().copied().collect();
        //into_iter消费了candidate_set, 以后就不能用了.intor_iter()迭代出的是&Id, 而不是Id, 不能收集进入Vec<Id>
        //这个地方改成iter().copied(), 从迭代器中复制值,然后再colletct

        candidate_sequence.sort_by(|id1, id2| {
            let node_1 = self.get_node_by_id(*id1).expect("id1找不到节点");
            let node_2 = self.get_node_by_id(*id2).expect("id2找不到节点");
            let score1 = distance::cosine_similarity(query, node_1.get_data());
            let score2 = distance::cosine_similarity(query, node_2.get_data());
            // partial_cmp 只做一件事：比较两个数，返回 Less、Equal、Greater。
            match score2.partial_cmp(&score1).unwrap() {
                // partical_cmp语句返回: scroe2 (  > / < / = ) score1
                // 这会导致高分排在更小下标,也就是降序
                // sort_by 约定：比较器返回 Less，就表示第一个参数要排在第二个参数前面；
                // 返回 Greater，就表示第一个参数要排在第二个参数后面。
                std::cmp::Ordering::Equal => id1.cmp(id2),
                other => other,
            }
        });

        // let selected:HashSet<Id>= candidate_sequence.iter().take(m).copied().collect();
        // iter()：按引用遍历:遍历时拿到的是元素的引用，不是元素本身,迭代器走出来的每一项&u64
        // take(m)：只取前 m 个:少于m个甚至为空都没问题.
        // copied()：把 &Id 变成 Id
        // collect()：收集成 HashSet<Id>.这样会让返回的结果重新变得无序.
        
        // 改进: 返回Vec<Id>, 而不是Hashset<Id>, 保持有序.
        // 用下面的切片语句做到"在candidate_sequence中,取前m个, 如果不足m个,就全部返回."
        let take_count = m.min(candidate_sequence.len());
        return candidate_sequence[..take_count].to_vec();

        // 改进:
        // 用下面的.truncate(m)方法搭配判断语句,也能达到同样效果.
        // if candidate_sequence.len() > m {
        //     candidate_sequence.truncate(m);
        // }
        // return candidate_sequence;

    }
}