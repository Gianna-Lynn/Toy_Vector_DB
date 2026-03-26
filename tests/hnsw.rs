use std::vec;

use toy_vector_db::index::hnsw::{HnswIndex, HnswNode};

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
    assert!(test_node_1.get_neighbors()[0].contains(&2), "test_node_1 cannot view 2");
    
    let test_node_2 = index.get_node_by_id(2).expect("test_node_2 does not exist");
    assert!(test_node_2.get_neighbors()[0].contains(&1), "test_node_2 cannot view 1");
    
}

#[test]
fn test_insert_three_nodes(){
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
    assert!(test_node_1.get_neighbors()[test_lvl].contains(&2), "test_node_1 cannot view 2");
    
    let test_node_2 = index.get_node_by_id(2).expect("test_node_2 does not exist");
    assert!(test_node_2.get_neighbors()[test_lvl].contains(&1), "test_node_2 cannot view 1");
    assert!(test_node_2.get_neighbors()[test_lvl].contains(&3), "test_node_2 cannot view 3");

    let test_node_3 = index.get_node_by_id(3).expect("test_node_3 does not exist");
    assert!(test_node_3.get_neighbors()[test_lvl].contains(&2), "test_node_3 cannot view 2");

}

#[test]
fn test_search_v0(){
    // 1. 创建 index
    let mut index = HnswIndex::new();

    // 2. 创建node并插入.
    let nodes_data=[
        (1, vec![10.0, 0.0], 2),
        (2, vec![9.0, 1.0], 2),
        (3, vec![8.0, 2.0], 2),
        (4, vec![0.0, 10.0], 2),
        (5, vec![1.0 , 9.0], 2),
        (6, vec![2.0, 8.0], 2),
    ];
    for(id, data, lvl) in nodes_data.iter(){
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
    if let Some(result_id) = index.search_v0(&test_query){
        if let Some(result_node) = index.get_node_by_id(result_id){
            println!("The result is:");
            println!("{:#?}",result_node.get_id());
            println!("{:#?}",result_node.get_data());
        }
    }
    
}