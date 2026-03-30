pub type NodeData = (u64, Vec<f32>, usize);

#[derive(Debug, Clone)]
pub struct HnswTestCase {
    pub nodes_data: Vec<NodeData>,
    pub query: Vec<f32>,
    pub entry_id: Option<u64>,
    pub expected_result_id: Option<u64>,
    pub level: usize,
    pub k: usize,
    pub ef_search: usize,
}

pub fn two_node_case() -> HnswTestCase {
    HnswTestCase { 
        nodes_data: vec![
            (1, vec![1.0, 2.0, 3.0], 3),
            (2, vec![2.0, 3.0, 4.0], 4),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 2,
        ef_search: 2,
    }
}

pub fn three_node_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 2.0, 3.0], 3),
            (2, vec![2.1, 3.1, 4.1], 4),
            (3, vec![2.0, 3.0, 4.0], 4),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 2,
    }
}

pub fn dense_2d_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 2),
            (2, vec![9.0, 1.0], 2),
            (3, vec![8.0, 2.0], 2),
            (4, vec![0.0, 10.0], 2),
            (5, vec![1.0, 9.0], 2),
            (6, vec![2.0, 8.0], 2),
        ],
        query: vec![6.0, 6.0],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 3,
    }
}

pub fn flat_2d_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![9.0, 1.0], 0),
            (3, vec![8.0, 2.0], 0),
            (4, vec![0.0, 10.0], 0),
            (5, vec![1.0, 9.0], 0),
            (6, vec![2.0, 8.0], 0),
        ],
        query: vec![6.0, 6.0],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 3,
    }
}

pub fn greedy_chain_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![9.0, 1.0], 0),
            (3, vec![8.0, 2.0], 0),
            (4, vec![0.0, 10.0], 0),
            (5, vec![1.0, 9.0], 0),
            (6, vec![2.0, 8.0], 0),
        ],
        query: vec![6.0, 6.0],
        entry_id: Some(1),
        expected_result_id: Some(3),
        level: 0,
        k: 3,
        ef_search: 3,
    }
}

pub fn search_layer_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![9.0, 1.0], 0),
            (3, vec![8.0, 2.0], 0),
            (4, vec![0.0, 10.0], 0),
            (5, vec![1.0, 9.0], 0),
            (6, vec![2.0, 8.0], 0),
        ],
        query: vec![6.0, 6.0],
        entry_id: Some(2),
        expected_result_id: Some(2),
        level: 0,
        k: 3,
        ef_search: 3,
    }
}

pub fn greedy_stop_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![9.0, 1.0], 0),
            (3, vec![8.0, 2.0], 0),
            (4, vec![0.0, 10.0], 0),
            (5, vec![1.0, 9.0], 0),
            (6, vec![2.0, 8.0], 0),
        ],
        query: vec![8.1, 2.1],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 3,
    }
}

pub fn multilevel_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 3),
            (2, vec![8.5, 1.0], 2),
            (3, vec![7.0, 2.0], 1),
            (4, vec![0.0, 10.0], 3),
            (5, vec![1.0, 8.5], 2),
            (6, vec![2.0, 7.0], 1),
            (7, vec![5.0, 5.0], 0),
            (8, vec![4.0, 4.5], 0),
        ],
        query: vec![4.8, 4.9],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 4,
    }
}

pub fn single_node_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![(1, vec![3.0, 4.0], 0)],
        query: vec![100.0, -100.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 1,
        ef_search: 1,
    }
}

pub fn empty_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![],
        query: vec![0.0, 1.0],
        entry_id: None,
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 4,
    }
}

pub fn unique_ranking_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![8.0, 1.0], 0),
            (3, vec![6.0, 2.0], 0),
            (4, vec![1.0, 9.0], 0),
            (5, vec![0.0, 10.0], 0),
            (6, vec![-3.0, 7.0], 0),
        ],
        query: vec![9.2, 0.3],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 3,
        ef_search: 4,
    }
}

pub fn beginning_is_the_best_case() -> HnswTestCase{
    HnswTestCase { 
        nodes_data: vec![
            (1, vec![10.0, 0.0], 0),
            (2, vec![0.0, 10.0], 0),
        ],
        query: vec![9.0, 1.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 4,
    }
}