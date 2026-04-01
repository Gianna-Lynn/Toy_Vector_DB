mod datasets;

use datasets::*;
use std::{collections::HashSet, u64, usize, vec};
use toy_vector_db::{
    index::hnsw::{HnswIndex, HnswNode},
    types::Id,
};

///id: u64, data: Vec<f32>, lvl: usize
fn build_index(nodes_data: Vec<(u64, Vec<f32>, usize)>) -> HnswIndex {
    let mut index = HnswIndex::new();

    // let nodes_data=[
    //     (1, vec![10.0, 0.0], 0),
    //     (2, vec![9.0, 1.0], 0),
    //     (3, vec![8.0, 2.0], 0),
    //     (4, vec![0.0, 10.0], 0),
    //     (5, vec![1.0 , 9.0], 0),
    //     (6, vec![2.0, 8.0], 0),
    // ];
    for (id, data, lvl) in nodes_data.iter() {
        index.insert(HnswNode::new(*id, data.to_vec(), *lvl));
    }
    return index;
}

#[test]
fn test_insert_two_nodes() {
    // 1. 创建 index
    let mut index = HnswIndex::new();
    // 2. 创建两个 node
    let node_1 = HnswNode::new(1, vec![1.0, 2.0, 3.0], 3);
    let node_2 = HnswNode::new(2, vec![2.0, 3.0, 4.0], 4);
    // 3. 插入
    index.insert(node_1);
    index.insert(node_2);

    
    // 4. 检查结果
    //println!("{:?}", index.get_nodes());
    assert_eq!(index.len(), 2, "index.len() is not 2");
    // 先拿到节点 → 再拿到某层邻居表 → 再检查里面是否包含某个 id
    let test_node_1 = index.get_node_by_id(1).expect("test_node_1 does not exist");
    assert!(
        test_node_1.get_neighbors()[0].contains(&2),
        "test_node_1 cannot view 2"
    );

    let test_node_2 = index.get_node_by_id(2).expect("test_node_2 does not exist");
    assert!(
        test_node_2.get_neighbors()[0].contains(&1),
        "test_node_2 cannot view 1"
    );
}

#[test]
fn test_insert_three_nodes() {
    // 1. 创建 index
    let mut index = HnswIndex::new();
    // 2. 创建三个 node
    let node_1 = HnswNode::new(1, vec![1.0, 2.0, 3.0], 3);
    let node_2 = HnswNode::new(2, vec![2.1, 3.1, 4.1], 4);
    let node_3 = HnswNode::new(3, vec![2.0, 3.0, 4.0], 4);
    // 3. 插入
    index.insert(node_1);
    index.insert(node_2);
    index.insert(node_3);
    // 4. 检查结果
    println!("{:#?}", index.get_nodes());

    assert_eq!(index.len(), 3, "index.len() is not 3");
    // 先拿到节点 → 再拿到某层邻居表 → 再检查里面是否包含某个 id
    let test_lvl = 1;
    let test_node_1 = index.get_node_by_id(1).expect("test_node_1 does not exist");
    assert!(
        test_node_1.get_neighbors()[test_lvl].contains(&2),
        "test_node_1 cannot view 2"
    );

    let test_node_2 = index.get_node_by_id(2).expect("test_node_2 does not exist");
    assert!(
        test_node_2.get_neighbors()[test_lvl].contains(&1),
        "test_node_2 cannot view 1"
    );
    assert!(
        test_node_2.get_neighbors()[test_lvl].contains(&3),
        "test_node_2 cannot view 3"
    );

    let test_node_3 = index.get_node_by_id(3).expect("test_node_3 does not exist");
    assert!(
        test_node_3.get_neighbors()[test_lvl].contains(&2),
        "test_node_3 cannot view 2"
    );
}

#[test]
fn test_search_v0() {
    // 1. 创建 index
    let mut index = HnswIndex::new();

    // 2. 创建node并插入.
    let nodes_data = [
        (1, vec![10.0, 0.0], 2),
        (2, vec![9.0, 1.0], 2),
        (3, vec![8.0, 2.0], 2),
        (4, vec![0.0, 10.0], 2),
        (5, vec![1.0, 9.0], 2),
        (6, vec![2.0, 8.0], 2),
    ];
    for (id, data, lvl) in nodes_data.iter() {
        index.insert(HnswNode::new(*id, data.to_vec(), *lvl));
    }

    let test_query = vec![6.0, 6.0];

    // 4. 检查结果(之前的部分)
    println!("{:#?}", index.get_nodes());

    // assert_eq!(index.len(), 3, "index.len() is not 3");
    // // 先拿到节点 → 再拿到某层邻居表 → 再检查里面是否包含某个 id
    // let test_lvl = 1;
    // let test_node_1 = index.get_node_by_id(1).expect("test_node_1 does not exist");
    // assert!(test_node_1.get_neighbors()[test_lvl].contains(&2), "test_node_1 cannot view 2");

    // let test_node_2 = index.get_node_by_id(2).expect("test_node_2 does not exist");
    // assert!(test_node_2.get_neighbors()[test_lvl].contains(&1), "test_node_2 cannot view 1");
    // assert!(test_node_2.get_neighbors()[test_lvl].contains(&3), "test_node_2 cannot view 3");

    // let test_node_3 = index.get_node_by_id(3).expect("test_node_3 does not exist");
    // assert!(test_node_3.get_neighbors()[test_lvl].contains(&2), "test_node_3 cannot view 2");

    // 5.检查search_v0的搜索结果.
    if let Some(result_id) = index.search_v0(&test_query) {
        if let Some(result_node) = index.get_node_by_id(result_id) {
            println!("The result is:");
            println!("{:#?}", result_node.get_id());
            println!("{:#?}", result_node.get_data());
        }
    }
}

#[test]
fn test_nearest_and_furthest() {
    // 1. 创建 index
    let mut index = HnswIndex::new();
    // 1.1 创建set
    let mut set: HashSet<Id> = HashSet::new();

    // 2. 创建node并插入.
    let nodes_data = [
        (1, vec![10.0, 0.0], 2),
        (2, vec![9.0, 1.0], 2),
        (3, vec![8.0, 2.0], 2),
        (4, vec![0.0, 10.0], 2),
        (5, vec![1.0, 9.0], 2),
        (6, vec![2.0, 8.0], 2),
    ];
    for (id, data, lvl) in nodes_data.iter() {
        index.insert(HnswNode::new(*id, data.to_vec(), *lvl));
        set.insert(*id);
    }

    let test_query = vec![6.0, 6.0];

    // 5.检查搜索结果.
    if let Some(result_id) = index.get_nearest(&set, &test_query) {
        assert!(result_id == 3 || result_id == 6);
        if let Some(result_node) = index.get_node_by_id(result_id) {
            println!("The nearest result is:");
            println!("{:#?}", result_node.get_id());
            println!("{:#?}", result_node.get_data());
        }
    } else {
        panic!("get_nearest方法返回None")
    }

    if let Some(result_id) = index.get_furthest(&set, &test_query) {
        assert!(result_id == 1 || result_id == 4);
        if let Some(result_node) = index.get_node_by_id(result_id) {
            println!("The furthest result is:");
            println!("{:#?}", result_node.get_id());
            println!("{:#?}", result_node.get_data());
        }
    } else {
        panic!("get_furthest方法返回None")
    }
}

#[test]
fn test_search_layer_v0() {
    // 1. 创建 index
    let mut index = HnswIndex::new();
    // 1.1 创建set
    let mut set: HashSet<Id> = HashSet::new();

    // 2. 创建node并插入.
    let nodes_data = [
        (1, vec![10.0, 0.0], 0),
        (2, vec![9.0, 1.0], 0),
        (3, vec![8.0, 2.0], 0),
        (4, vec![0.0, 10.0], 0),
        (5, vec![1.0, 9.0], 0),
        (6, vec![2.0, 8.0], 0),
    ];
    for (id, data, lvl) in nodes_data.iter() {
        index.insert(HnswNode::new(*id, data.to_vec(), *lvl));
        set.insert(*id);
    }

    let test_query = vec![6.0, 6.0];
    let result_set = index.search_layer_v0(&test_query, 2, 0, 3);
    let mut result_vec: Vec<_> = result_set.iter().cloned().collect();
    result_vec.sort();

    println!("{:#?}", result_vec);

    assert!(!result_set.is_empty());
    assert!(result_set.len() <= 3);
}

#[test]
fn test_greedy_search_at_level_old() {
    //let nodes_data = generate_nodes_data();

    //Test A
    //构造一个简单单层图，使得从某个入口点出发可以沿邻居不断接近查询点。
    //检查返回值是否为最终停止节点。
    let nodes_data = vec![
        (1, vec![10.0, 0.0], 0),
        (2, vec![9.0, 1.0], 0),
        (3, vec![8.0, 2.0], 0),
        (4, vec![0.0, 10.0], 0),
        (5, vec![1.0, 9.0], 0),
        (6, vec![2.0, 8.0], 0),
    ];
    let index = build_index(nodes_data);
    let test_query = vec![6.0, 6.0];
    let result_id = index.greedy_search_at_level(&test_query, 1, 0);
    println!("Test A result_id is:");
    println!("{:#?}", result_id);

    //TestB
    // 构造一个节点在某层没有任何更优邻居的情形。
    // 检查函数是否直接返回入口点本身。
    let nodes_data = vec![
        (1, vec![10.0, 0.0], 0),
        (2, vec![9.0, 1.0], 0),
        (3, vec![8.0, 2.0], 0),
        (4, vec![0.0, 10.0], 0),
        (5, vec![1.0, 9.0], 0),
        (6, vec![2.0, 8.0], 0),
    ];
    let index = build_index(nodes_data);
    let test_query = vec![8.1, 2.1];
    let result_id = index.greedy_search_at_level(&test_query, 1, 0);
    println!("Test B result_id is:");
    println!("{:#?}", result_id);
}

#[test]
fn test_search_knn_v1_old() {
    // Test C: empty index
    // 空图上调用 search_knn_v1(...)
    // 结果应为空
    let nodes_data:Vec<(u64, Vec<f32>, usize)> = vec![];
    let index = build_index(nodes_data);
    let test_query = vec![8.1, 2.1];
    println!("Test C get_entry_node_id is:");
    println!("{:#?}",index.get_entry_node_id());
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    println!("Test C result_vector is:");
    println!("{:#?}", result_vector);

    // Test D: single node
    // 单节点图
    // 查询任意向量
    // 返回结果应只包含该节点
    let nodes_data:Vec<(u64, Vec<f32>, usize)> = vec![
        (1, vec![10.0, 0.0], 0),
    ];
    let index = build_index(nodes_data);
    let test_query = vec![8.1, 2.1];
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    println!("Test D result_vector is:");
    println!("{:#?}", result_vector);

    // Test E: result size
    // 构造一个小图
    // 调用 search_knn_v1(query, k, ef_search)
    // 检查返回长度不超过 k

    // Test F: ranking
    // 构造一个小图
    // 调用查询
    // 检查返回结果按相似度非增排序
        let nodes_data = vec![
        (1, vec![10.0, 0.0], 3),
        (2, vec![9.0, 1.0], 1),
        (3, vec![8.0, 2.0], 4),
        (4, vec![0.0, 10.0], 3),
        (5, vec![1.0, 9.0], 2),
        (6, vec![2.0, 8.0], 0),
    ];
    let index = build_index(nodes_data);
    let test_query = vec![8.1, 2.1];
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    println!("Test E/F result_vector is:");
    println!("{:#?}", result_vector);

}


#[test]
fn test_greedy_search_at_level_greedy_chain_case() {
    //let nodes_data = generate_nodes_data();

    //Test A
    //构造一个简单单层图，使得从某个入口点出发可以沿邻居不断接近查询点。
    //检查返回值是否为最终停止节点。

    let case = greedy_chain_case();
    let index = build_index(case.nodes_data);
    let test_query = case.query;
    
    let result_id = index.greedy_search_at_level(&test_query, 1, 0);
    let expected_result_id = case
        .expected_result_id
        .expect("greedy_chain_case should define expected_result_id");
    assert_eq!(result_id, expected_result_id, 
        "test_greedy_search_at_level, greedy_chain_case: expected {}, got {}", expected_result_id, result_id);
    println!("Test A result_id is:");
    println!("{:#?}", result_id);

}

#[test]
fn test_greedy_search_at_level_beginning_is_the_best_case() {
    //TestB
    // 构造一个节点在某层没有任何更优邻居的情形。
    // 检查函数是否直接返回入口点本身。

    let case = beginning_is_the_best_case();
    let index = build_index(case.nodes_data);
    let test_query = case.query;
    let result_id = index.greedy_search_at_level(&test_query, 1, 0);
    let expected_result_id = case
        .expected_result_id
        .expect("beginning_is_the_best_case should define expected_result_id");
    assert_eq!(result_id, expected_result_id,
        "test_greedy_search_at_level,beginning_is_the_best_case: expected {}, got {}", expected_result_id, result_id);
    
    println!("Test B result_id is:");
    println!("{:#?}", result_id);
}

#[test]
fn test_search_knn_v1() {
    test_search_knn_v1_empty_case();
    test_search_knn_v1_single_node_case();
    test_search_knn_v1_unique_ranking_case();
}

#[test]
fn test_search_knn_v1_empty_case() {
    // Test C: empty index
    // 空图上调用 search_knn_v1(...)
    // 结果应为空
    let case = empty_case();
    let index = build_index(case.nodes_data);
    let test_query = case.query;

    println!("Test C get_entry_node_id is:");
    println!("{:#?}",index.get_entry_node_id());
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    assert!(result_vector.is_empty(), "空图时应当返回空结果");
    
    println!("Test C result_vector is:");
    println!("{:#?}", result_vector);
}

#[test]
fn test_search_knn_v1_single_node_case() {
    // Test D: single node
    // 单节点图
    // 查询任意向量
    // 返回结果应只包含该节点
    let case = single_node_case();
    let index = build_index(case.nodes_data);
    let test_query = case.query;
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    assert_eq!(result_vector.len(), 1);
    let expected_result_id = case
        .expected_result_id
        .expect("single_node_case should define expected_result_id");
    assert_eq!(result_vector[0], expected_result_id);
    
    println!("Test D result_vector is:");
    println!("{:#?}", result_vector);
}

#[test]
fn test_search_knn_v1_unique_ranking_case() {
    // Test E: result size
    // 构造一个小图
    // 调用 search_knn_v1(query, k, ef_search)
    // 检查返回长度不超过 k
    
    // Test F: ranking
    // 构造一个小图
    // 调用查询
    // 检查返回结果按相似度非增排序
    let case = unique_ranking_case();
    let index = build_index(case.nodes_data);
    let test_query = case.query;
    let result_vector = index.search_knn_v1(&test_query, 3, 3);
    // result_vector.len():返回结果的个数. case.k: 要求返回前k个结果. 
    // 要求前者小于等于后者.
    // 检查返回结果的个数不超过k.
    assert!(result_vector.len() <= case.k);
    // 检查返回结果的第一个元素是否是预期的expected_result_id.
    let expected_result_id = case
        .expected_result_id
        .expect("unique_ranking_case should define expected_result_id");
    // 检查expected_result_id是否是返回结果中相似度最高的那个节点的id.
    assert_eq!(result_vector[0], expected_result_id);
    // 检查返回结果是否按相似度非增排序.
    assert!(
        result_vector.windows(2).all( |pair| {
            let left_node = index.get_node_by_id(pair[0]).expect("left node not found");
            let right_node = index.get_node_by_id(pair[1]).expect("right node not found");
            let left_score = toy_vector_db::distance::cosine_similarity(&test_query, left_node.get_data());
            let right_score = toy_vector_db::distance::cosine_similarity(&test_query, right_node.get_data());

            left_score >= right_score
        }),
        "result_vector不满足非增排序"
    );

    println!("Test E/F result_vector is:");
    println!("{:#?}", result_vector);
}


// Test A: sample_level distribution sanity check
// 多次调用 sample_level()，统计不同 level 出现频次。
// 检查：
// count(level=0) 应不少于 count(level=1)
// count(level=1) 应不少于 count(level=2)
// 不要求严格数学检验，只做 sanity check。
#[test]
fn test_insert_v1_a() {

    
    // let case = unique_ranking_case();
    // // let test_query = unique_ranking_case().query;
    // let mut index = build_index_v1(&case);
    // let nodes_vec = index.get_nodes();

    // let mut cnt_lvl_0 = 0;
    // let mut cnt_lvl_1 =0;
    // let mut cnt_lvl_2 =0;
    
    
    // for node in nodes_vec{
    //     let lvl = node//我才意识到好像没有特别好的数据结构...

    // }
}

// Test B: insert into empty graph
// 空图执行一次 insert_v1
// 检查：
// len() == 1
// entry_node_id 存在
// 节点可被检索到
#[test]
fn test_insert_v1_b() {
}

// Test C: insert second node
// 连续插入两个节点
// 检查：
// 两节点都存在
// 某些共同层上发生双向连边
#[test]
fn test_insert_v1_c() {
}

// Test D: insert multiple nodes
// 插入多个节点
// 检查：
// len() 正确
// entry_node_id 合法
// 图中没有明显断裂到完全无法访问的新节点
#[test]
fn test_insert_v1_d() {
}

// Test E: higher-level node may update entry point
// 如果采样层数可控，或测试中允许手动指定层数版本：
// 插入一个层数更高的新节点
// 检查它是否成为新入口点
#[test]
fn test_insert_v1_e() {
}

// ========== 新增: insert_v1 最小集成测试 ==========
// ==========   利用Agent完成测试函数编写   ==========

// Test 1: 空图插一个点
// 期望: len == 1, entry_node_id 存在且有效
#[test]
fn test_insert_v1_single_node_minimal() {
    let mut index = HnswIndex::new();
    assert!(index.is_empty(), "初始图应为空");
    
    let id = 1u64;
    let data = vec![1.0, 2.0, 3.0];
    let ef = 10;
    let m = 2;
    let m_max = 4;
    
    index.insert_v1(id, data, ef, m, m_max);
    
    // 检查: len == 1
    assert_eq!(index.len(), 1, "插入一个点后，图大小应为1");
    
    // 检查: entry_node_id 存在
    assert!(
        index.get_entry_node_id().is_some(),
        "插入后应存在入口点"
    );
    let entry_id = index.get_entry_node_id().expect("入口点不存在");
    assert_eq!(entry_id, id, "入口点应为新插入的节点");
    
    // 检查: 节点可被检索到
    assert!(
        index.get_node_by_id(id).is_some(),
        "新插入的节点应该能被检索到"
    );
    
    println!("Test 1 passed: 单节点插入成功");
}

// Test 2: 连续插两个点
// 期望: 两个点都存在，至少在某一层有双向连边
#[test]
fn test_insert_v1_two_nodes_minimal() {
    let mut index = HnswIndex::new();
    
    let id1 = 1u64;
    let data1 = vec![1.0, 0.0, 0.0];
    let id2 = 2u64;
    let data2 = vec![1.1, 0.1, 0.0];
    
    let ef = 10;
    let m = 2;
    let m_max = 4;
    
    // 插入第一个节点
    index.insert_v1(id1, data1, ef, m, m_max);
    assert_eq!(index.len(), 1, "第一次插入后大小应为1");
    
    // 插入第二个节点
    index.insert_v1(id2, data2, ef, m, m_max);
    assert_eq!(index.len(), 2, "第二次插入后大小应为2");
    
    // 检查: 两个节点都存在
    let node1 = index.get_node_by_id(id1).expect("节点1不存在");
    let node2 = index.get_node_by_id(id2).expect("节点2不存在");
    
    // 检查: 至少一层有双向连边
    let mut found_bidirectional = false;
    let max_level = std::cmp::max(*node1.get_node_max_level(), *node2.get_node_max_level());
    let empty_vec: Vec<u64> = Vec::new();
    
    for lvl in 0..=max_level {
        // 获取两节点在当前层的邻居列表，如果当前层不存在邻居列表则使用空列表
        let neighbors1 = if lvl < node1.get_neighbors().len() {
            &node1.get_neighbors()[lvl]
        } else {
            &empty_vec
        };
        let neighbors2 = if lvl < node2.get_neighbors().len() {
            &node2.get_neighbors()[lvl]
        } else {
            &empty_vec
        };
        
        if neighbors1.contains(&id2) && neighbors2.contains(&id1) {
            found_bidirectional = true;
            println!("在第{}层找到双向连边", lvl);
            break;
        }
    }
    
    assert!(
        found_bidirectional,
        "两个节点之间应该至少在某一层有双向连边"
    );
    
    println!("Test 2 passed: 两个节点插入成功，双向连边存在");
}

// Test 3: 插多个点后，新插入节点不悬空
// 期望: 每个新插入的节点都能从入口点访问到（不是孤立的）
#[test]
fn test_insert_v1_multiple_nodes_connected() {
    let mut index = HnswIndex::new();
    
    let nodes_data = vec![
        (1u64, vec![1.0, 0.0, 0.0]),
        (2u64, vec![1.0, 1.0, 0.0]),
        (3u64, vec![0.0, 1.0, 0.0]),
        (4u64, vec![0.0, 0.0, 1.0]),
    ];
    
    let ef = 10;
    let m = 2;
    let m_max = 4;
    
    for (id, data) in nodes_data.iter() {
        index.insert_v1(*id, data.clone(), ef, m, m_max);
    }
    
    // 检查: 所有节点都被插入
    assert_eq!(index.len(), 4, "应该插入4个节点");
    
    for (id, _) in nodes_data.iter() {
        assert!(
            index.get_node_by_id(*id).is_some(),
            "节点{} 应该存在", id
        );
    }
    
    // 检查: 每个节点都不是悬空的（至少存在一条邻边）
    for (id, _) in nodes_data.iter() {
        let node = index.get_node_by_id(*id).expect("节点不存在");
        let neighbors = node.get_neighbors();
        
        let has_neighbors = neighbors.iter().any(|layer| !layer.is_empty());
        assert!(
            has_neighbors,
            "节点{} 应该至少有一个邻边（不应该悬空）", id
        );
    }
    
    println!("Test 3 passed: 多个节点插入成功，没有悬空节点");
}
// ========== 新的刁钻测试样例测试 ==========
// 主要针对查询进行测试,而不是插入

#[test]
fn test_tricky_identical_vectors() {
    let case = identical_vectors_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.len(), 2, "identical_vectors should return exactly k results");
    assert!(result_vector.iter().all(|id| *id <= 3), "identical_vectors should only return the tied best nodes");
}


#[test]
fn test_tricky_duplicate_distance() {
    let case = duplicate_distance_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.as_slice(), &[1, 2, 3], "duplicate_distance should keep all equally ranked nodes in ID order");
}

#[test]
fn test_tricky_collinear_points() {
    let case = collinear_points_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.as_slice(), &[1, 2], "collinear_points should rank the nearest directions first");
}

#[test]
fn test_tricky_clustered_distribution() {
    let case = clustered_distribution_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.as_slice(), &[1, 2, 3], "clustered_distribution should stay inside the nearest cluster");
}

#[test]
fn test_tricky_high_dimension() {
    let case = high_dimension_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "high_dimension should return the exact best match first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_extreme_values() {
    let case = extreme_values_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "extreme_values should return the exact best match first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_near_zero_distance() {
    let case = near_zero_distance_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "near_zero_distance should return the closest node first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_k_larger_than_dataset() {
    let case = k_larger_than_dataset_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.len(), 3, "k larger than the dataset should still return all available nodes");
}

#[test]
fn test_tricky_tightly_packed() {
    let case = tightly_packed_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "tightly_packed should return the densest center first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_extreme_ef_search() {
    let case = extreme_ef_search_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "extreme_ef_search should still find the best node");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_negative_coordinates() {
    let case = negative_coordinates_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "negative_coordinates should return the correct nearest node first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}

#[test]
fn test_tricky_sparse_vectors() {
    let case = sparse_vectors_case();
    let index = build_index(case.nodes_data);
    let result_vector = index.search_knn_v1(&case.query, case.k, case.ef_search);
    assert_eq!(result_vector.first().copied(), case.expected_result_id, "sparse_vectors should return the sparse center first");
    assert!(result_vector.len() <= case.k, "result size should not exceed k");
}
