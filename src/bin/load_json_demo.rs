use toy_vector_db::storage::inmemory::InMemoryVectorStore;
use std::io::Error;
fn main(){

    let store = InMemoryVectorStore::load("F://Repositories//Toy_Vector_DB//small_vectors.json").expect("加载失败");
    //尚不清楚这里的except用法.
    let mut datas = store.records();
    for data in datas{
        println!("{:?}",data);
    }

    return;
}