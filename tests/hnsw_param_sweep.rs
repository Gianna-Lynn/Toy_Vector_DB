mod datasets;
use datasets::*;

use toy_vector_db::{
    index::hnsw::HnswIndex,
    types::{Vector, Id},
    distance::cosine_similarity
};



// 参数说明:
// - id: 新节点的唯一标识符
// - data: 新节点的向量数据
// - ef_construction: 在构建过程中搜索层时使用的参数，控制搜索的宽度, 也就是论文中的ef_construction
//                    至于所谓的ef_search不会出现在这里, 是用于搜索函数的.
// - m: 每层连接的最大邻居数. 在插入新节点时, 每层最多连接m个邻居.
// - m_max: 每层允许的最大邻居数. 在插入新节点时, 如果某个节点的邻居数超过m_max, 就需要进行邻居选择和替换.
    
/// id: u64, data: Vec<f32>, lvl: usize
/// v1 版本的构图 helper, 复制自hnsw_test_v1.rs
fn build_index_v1(nodes_data: Vec<(u64, Vec<f32>, usize)>, ef_construction: usize, m: usize, m_max: usize) -> HnswIndex {
    let mut index = HnswIndex::new();

    for (id, data, _) in nodes_data.iter() {
        index.insert_v1(*id, data.clone(), ef_construction, m, m_max);
    }
    index
}

// debug: 这个地方的expected_list最好完全独立于build_index_v1.
//        也就是说, 不要从index中取结果, 而是直接从HnswTestCase中取结果.
//     
fn donotuse_ideal_answer_generator(index: &HnswIndex, query: &Vector, top_k: usize) -> Vec<Id>{

    // 按暴力 cosine similarity 排序后的理想 id 列表
    // - 遍历 case 的 nodes_data
    // - 算 query 和每个点的 cosine similarity
    // - 排序
    // - 取前 k
    // - 你必须给 tie 一个稳定规则，不然实验结果会漂

    assert!(top_k > 0, "in function ideal_answer_generator, expected top_k > 0");

    let mut nodes_vec = index.get_nodes().clone();
    nodes_vec.sort_by(|node1, node2|{
        let score1 = cosine_similarity(query, node1.get_data());
        let score2 = cosine_similarity(query, node2.get_data());
        match score2.partial_cmp(&score1).unwrap(){
            std::cmp::Ordering::Equal => node1.get_id().cmp(node2.get_id()),
            other => other,
        }
    });
    let mut id_vec: Vec<Id> = Vec::new();
    for( idx, node) in nodes_vec.iter().enumerate(){
        if idx == top_k {break;}
        id_vec.push(*node.get_id());
    }
    return id_vec;
}


fn ideal_answer_generator(case: &HnswTestCase, query: &Vector, top_k: usize) -> Vec<Id>{
    // 按暴力 cosine similarity 排序后的理想 id 列表
    // - 遍历 case 的 nodes_data
    // - 算 query 和每个点的 cosine similarity
    // - 排序
    // - 取前 k
    // - 你必须给 tie 一个稳定规则，不然实验结果会漂
    let mut nodes_data_vec = case.nodes_data.clone();
            // 数据示例
            // nodes_data: vec![
            //     (1, vec![1.0, 2.0, 3.0], 3),
            //     (2, vec![2.0, 3.0, 4.0], 4),
            // ],
    // note: 访问元组的第一个元素,需要用.1, 第二个元素.2
    nodes_data_vec.sort_by(|node1, node2|{
        let score1 = cosine_similarity(query, &node1.1);
        let score2 = cosine_similarity(query, &node2.1);
        match score2.partial_cmp(&score1).unwrap(){
            std::cmp::Ordering::Equal => node1.0.cmp(&node2.0),
            other => other,
        }
    });
    let mut id_vec: Vec<Id> = Vec::new();
    for( idx, node) in nodes_data_vec.iter().enumerate(){
        if idx == top_k {break;}
        id_vec.push(node.0);
    }
    return id_vec;
}

// debug: 输出的元组中第二个写成了 Vec<&str>会怎样? 
//        &str 是“借用字符串”，但函数签名里没写清楚它借的是谁，所以编译器报：missing lifetime specifier
//        由于cases是字符串字面量, 本身是 'static的, 写成Vec<& 'static str>, 或者加上.to_string(), 返回 Vec<String>
fn selected_cases() -> (Vec<datasets::HnswTestCase>, Vec<String>){
// 返回一个小列表, 内含要使用的全部试验样例case.
    let cases=vec![
        unique_ranking_case(),
        multilevel_case(),
        high_dimension_case(),
        negative_coordinates_case(),
        
        // =============== 第二轮添加的case ===============
        // two_node_case(),
        // three_node_case(),
        // dense_2d_case(),
        // flat_2d_case(),
        // greedy_chain_case(),
        // search_layer_case(),
        // greedy_stop_case(),
        // single_node_case(),
        // beginning_is_the_best_case(),
        // identical_vectors_case(),
        // duplicate_distance_case(),
        // collinear_points_case(),
        // clustered_distribution_case(),
        // extreme_values_case(),
        // near_zero_distance_case(),
        // k_larger_than_dataset_case(),
        // tightly_packed_case(),
        // ===================================================
    ];
    let case_names= vec![
        "unique_ranking_case".to_string(),
        "multilevel_case".to_string(),
        "high_dimension_case".to_string(),
        "negative_coordinates_case".to_string(),
        
        // =============== 第二轮添加的case ===============
        // "two_node_case".to_string(),
        // "three_node_case".to_string(),
        // "dense_2d_case".to_string(),
        // "flat_2d_case".to_string(),
        // "greedy_chain_case".to_string(),
        // "search_layer_case".to_string(),
        // "greedy_stop_case".to_string(),
        // "single_node_case".to_string(),
        // "beginning_is_the_best_case".to_string(),
        // "identical_vectors_case".to_string(),
        // "duplicate_distance_case".to_string(),
        // "collinear_points_case".to_string(),
        // "clustered_distribution_case".to_string(),
        // "extreme_values_case".to_string(),
        // "near_zero_distance_case".to_string(),
        // "k_larger_than_dataset_case".to_string(),
        // "tightly_packed_case".to_string(),
        // ===================================================
    ];
    return (cases, case_names);
    // TODO:
    // 更自然的设计是直接返回 Vec<(case_name, case)>
}

#[test] #[ignore] 
fn experiment_param_sweep(){
    // 固定一些参数
    let ef_construction = 10;
    let ms = [2, 4];
    let ef_searches = [4, 8, 16];

    // 然后两层循环：
    // 外层扫 m
    for m in ms{
        // 中间自己算 m_max
        let m_max = 2 * m;
        // 内层扫 ef_search
        for ef_search in ef_searches{
            // improve: 在这里补一个参数标题打印.
            println!("\n========== ef_construction={}, m={}, m_max={}, ef_search={} ==========", ef_construction, m, 2*m, ef_search);

            // debug: 统计量的初始化应出现在这里.对于每一组参数(m, ef_search),对应一些统计量.

            let mut total = 0;      //这组参数 (m, ef_search) 一共跑了多少个 case。
            let mut top1_hit= 0;    //第一个结果命中的 case 个数
            let mut exact_hit= 0; //有多少个 case 完全匹配
            
            
            
            
            let (cases, cases_names) = selected_cases(); //这一步是解构
            
            // debug: 不能直接对 tuple 迭代，要先解构再 zip 配对.
            // for (case,case_names) in selected_cases() {

            // debug: zip:是拉链的意思, 把两个序列一一配对.
            for (case, case_name) in cases.into_iter().zip(cases_names.into_iter()) {

                total += 1; //这组参数 (m, ef_search) 一共跑了多少个 case。
                // Task 6：在每组参数里跑所有 selected cases
                // 1
                // 初始化统计量：
                // debug: 统计量的初始化并不在这里.对于每一组参数(m, ef_search),对应一些统计量.


                // 2
                // 对每个 case：
                // 算 expected
                // 用当前参数建图
                // 用当前 ef_search 调 search_knn_v1(...)
                // 比较结果

                // debug: 如果先let index, 后let expected_list: 前者利用值传递拿走了case.nodes_data, 后者想要整体借用case, rust不允许"部分字段已经被搬走后还想借用整个结构体"
                //        解决方法是交换这两行, 先let expected_list借用, 后拿走case.nodes_data
                //        解决方法更正: 第二步拿走case.nodes_data还不行,改成了clone()
                let expected_list = ideal_answer_generator(&case, &case.query, case.k);
                
                let index = build_index_v1(case.nodes_data.clone(), ef_construction, m, m_max);
                // debug: 这个地方的expected_list最好完全独立于build_index_v1.
                // let expected_list = ideal_answer_generator(&index, &case.query, case.k);
                let search_result_list = index.search_knn_v1(&case.query, case.k, ef_search);
            
                // 3
                // 定义两个判断：
                let mut top1_ok: bool = false;    //这个 case 的第一个结果是否命中
                let mut exact_ok: bool = false;   //这个 case 的整个结果列表是否完全一致

                // debug: 改成宽容的逻辑，不用 assert! 打断整趟实验
                //        空列表时，top1_ok = false；长度不等时，exact_ok = false；继续打印
                if !expected_list.is_empty() && !search_result_list.is_empty() {
                    if expected_list[0] == search_result_list[0] {
                        top1_ok = true; 
                        top1_hit += 1;
                    }
                }

                // 4
                // 更新统计量
                // debug: rust 允许使用 == 直接对比两个list, 返回一个布尔值, 不需要一位一位对比.
                // debug: exact_hit 和 exact_ok 一开始因为变量作用不清晰所以完全会错了意.
                // let len = expected_list.len();
                //  for i in 0 ..= len-1 {
                //    if expected_list[i] == search_result[i]{
                //         exact_hit += 1;
                //     }
                // }
                // if exact_hit == len {exact_ok = true;}

                if expected_list == search_result_list{
                    exact_ok = true;
                    exact_hit += 1;
                }

                // 5
                // 打印：case 名字, top1 是否对,exact 是否对, expected 什么, got 什么
                println!("  [{}]", case_name);
                println!("    top1_ok={}, exact_ok={}", top1_ok, exact_ok);
                println!("    expected={:?}", expected_list);
                println!("    got={:?}", search_result_list);
                
            }
        
        // Task 7：打印每组参数的 summary, 一共 <m x ef_search> 这么多组.
        // m, m_max, ef_construction, ef_search, top1_hit / total, exact_hit / total
        println!("---");
        println!("SUMMARY: ef_construction={}, m={}, m_max={}, ef_search={}", ef_construction, m, m_max, ef_search);
        println!("  top1_hit / total = [{}/{}]", top1_hit, total);
        println!("  exact_hit / total = [{}/{}]", exact_hit, total);
        println!("===\n");
        }
    }
}