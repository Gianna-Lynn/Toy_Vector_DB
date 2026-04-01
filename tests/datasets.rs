#![allow(dead_code)]

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
            (5, vec![0.0, 10.0], 0),
            (2, vec![8.0, 1.0], 0),
            (8, vec![4.0, 8.0], 0),
            (1, vec![10.0, 0.0], 0),
            (6, vec![-3.0, 7.0], 0),
            (4, vec![1.0, 9.0], 0),
            (7, vec![7.0, 3.0], 0),
            (3, vec![6.0, 2.0], 0),
        ],
        query: vec![9.2, 0.3],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 4,
    }
}

pub fn beginning_is_the_best_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![(1, vec![10.0, 0.0], 0), (2, vec![0.0, 10.0], 0)],
        query: vec![9.0, 1.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 4,
    }
}

pub fn identical_vectors_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![5.0, 5.0], 0),
            (2, vec![5.0, 5.0], 0),
            (3, vec![5.0, 5.0], 0),
            (4, vec![10.0, 10.0], 0),
        ],
        query: vec![5.0, 5.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 2,
        ef_search: 3,
    }
}

pub fn duplicate_distance_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 0.0], 0),
            (2, vec![0.0, 1.0], 0),
            (3, vec![0.0, -1.0], 0),
            (4, vec![0.0, 2.0], 0),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 5,
    }
}

pub fn collinear_points_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 0.0], 0),
            (2, vec![0.95, 0.10], 0),
            (3, vec![0.90, 0.20], 0),
            (4, vec![0.85, 0.30], 0),
            (5, vec![0.80, 0.40], 0),
            (6, vec![0.75, 0.50], 0),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 2,
        ef_search: 4,
    }
}

pub fn clustered_distribution_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 0.0], 0),
            (2, vec![0.99, 0.01], 0),
            (3, vec![0.98, 0.02], 0),
            (4, vec![0.97, 0.03], 0),
            (5, vec![-1.0, 0.0], 0),
            (6, vec![-0.99, -0.01], 0),
            (7, vec![-0.98, -0.02], 0),
            (8, vec![-0.97, -0.03], 0),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 3,
        ef_search: 6,
    }
}

pub fn high_dimension_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0),
            (2, vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0),
            (3, vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0),
            (4, vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0),
            (5, vec![0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0),
        ],
        query: vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        entry_id: Some(1),
        expected_result_id: Some(5),
        level: 0,
        k: 2,
        ef_search: 4,
    }
}

pub fn extreme_values_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1e5, 1e5], 0),
            (2, vec![100001.0, 99999.0], 0),
            (3, vec![1e-5, 2e-5], 0),
            (4, vec![0.0001, 0.0002], 0),
            (5, vec![1e10, 0.0], 0),
        ],
        query: vec![1e5, 1e5],
        entry_id: Some(1),
        expected_result_id: Some(1),
        level: 0,
        k: 2,
        ef_search: 4,
    }
}

pub fn near_zero_distance_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![1.0, 0.0], 0),
            (2, vec![1.0000001, 0.0000001], 0),
            (3, vec![1.0000002, 0.0000002], 0),
            (4, vec![10.0, 10.0], 0),
        ],
        query: vec![1.0, 0.0],
        entry_id: Some(4),
        expected_result_id: Some(1),
        level: 0,
        k: 2,
        ef_search: 3,
    }
}

pub fn k_larger_than_dataset_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![0.0, 0.0], 0),
            (2, vec![1.0, 0.0], 0),
            (3, vec![0.0, 1.0], 0),
        ],
        query: vec![0.5, 0.5],
        entry_id: Some(1),
        expected_result_id: None,
        level: 0,
        k: 10,
        ef_search: 10,
    }
}

pub fn tightly_packed_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![0.0, 0.0], 0),
            (2, vec![0.01, 0.0], 0),
            (3, vec![0.0, 0.01], 0),
            (4, vec![0.012, 0.009], 0),
            (5, vec![0.005, 0.005], 0),
            (6, vec![0.007, 0.003], 0),
        ],
        query: vec![0.005, 0.005],
        entry_id: Some(1),
        expected_result_id: Some(5),
        level: 0,
        k: 3,
        ef_search: 5,
    }
}

pub fn extreme_ef_search_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![0.0, 0.0], 0),
            (2, vec![1.0, 0.0], 0),
            (3, vec![0.0, 1.0], 0),
            (4, vec![1.0, 1.0], 0),
        ],
        query: vec![0.5, 0.5],
        entry_id: Some(1),
        expected_result_id: Some(4),
        level: 0,
        k: 2,
        ef_search: 1000,
    }
}

pub fn negative_coordinates_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![-10.0, -11.0], 0),
            (2, vec![-4.0, -4.0], 0),
            (3, vec![0.0, 0.0], 0),
            (4, vec![5.0, 5.0], 0),
            (5, vec![10.0, 9.0], 0),
        ],
        query: vec![-2.5, -2.5],
        entry_id: Some(2),
        expected_result_id: Some(2),
        level: 0,
        k: 2,
        ef_search: 4,
    }
}

pub fn sparse_vectors_case() -> HnswTestCase {
    HnswTestCase {
        nodes_data: vec![
            (1, vec![10.0, 0.0, 0.0, 0.0], 0),
            (2, vec![0.0, 10.0, 0.0, 0.0], 0),
            (3, vec![0.0, 0.0, 10.0, 0.0], 0),
            (4, vec![0.0, 0.0, 0.0, 10.0], 0),
            (5, vec![5.0, 5.0, 0.0, 0.0], 0),
        ],
        query: vec![1.0, 1.0, 0.0, 0.0],
        entry_id: Some(5),
        expected_result_id: Some(5),
        level: 0,
        k: 2,
        ef_search: 4,
    }
}
