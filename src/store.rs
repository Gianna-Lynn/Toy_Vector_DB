// 一条向量的ID
pub type Id = u64;

// 向量本体（以后可以换成别的结构）
pub type Vector = Vec<f32>;

// 一条记录：id + 向量
#[derive(Clone, Debug)]   //Rust中提供的派生宏语法，让编译器自动为这个结构体生成Clone和Debug trait的默认实现，不需要手动编码
pub struct Record{
    pub id: Id,
    pub vector: Vector,
}

// 搜索结果：搜索的是哪一个id，匹配程度有多好
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub id: Id,
    pub score: f32,    //比如余弦相似度
}

//统一的向量存储接口
pub trait VectorStore{
    // 插入或者更新一条向量
    fn insert(&mut self, id: Id, vector: Vector);
    
    // 基于相似度搜索前K个
    fn search(&self, query: &Vector, k: usize) -> Vec<SearchResult>;  //问题：目前对于trait和泛型这一块的知识还不清晰。
}

pub struct InMemoryVectorStore{
    data: Vec<Record>,
}

impl InMemoryVectorStore{
    pub fn new() -> Self{       //猜测：这里是基于上面的定义，增加了一个具体的实现：如何通过new函数新建一个InMemoryVectorStore对象。
        Self{data: Vec::new()}
    }
}

//猜测：泛型实际上说的是模板和具体实现这两件事。有点像C++的template。

impl VectorStore for InMemoryVectorStore{
    // 猜测：这里是基于上面的模版，增加了trait VectorStore的具体实现。
    fn insert(&mut self, id: Id, vector: Vector) {
        // 猜测：VectorStore的定义中包含了insert和search两个函数的声明，但是没有给出具体实现。这里应该就是为了补充实现。
        
        // 暂时先占位，待实现
        //unimplemented!("insert is not implemented yet"); //猜测：这里的unimplemented!是一个既定的宏或者用法，括号当中的是参数，用于输出。
        
        // 如果已经存在，那么就覆盖；如果不存在，那么添加新记录。
        if let Some(rec) = self.data.iter_mut().find(|r| r.id == id){
            //问题：看不懂这个if条件。后面的两条竖线是表示绝对值的意思吗？
            rec.vector = vector;
        }
        else{
            self.data.push(Record { id, vector });
            //猜测：此处应该是为了添加一条全新的记录。
        }

    }

    fn search(&self, query: &Vector, k: usize) -> Vec<SearchResult> {

        //unimplemented!("search is not implemented yet");
        let mut results: Vec<SearchResult> = self.data
            .iter()
            .map(|rec|{
                let score = cosine_similarity(query, &rec.vector);
                SearchResult {id: rec.id, score}
            })
            .collect();
            //问题：这个地方的连环调用可以说看不懂一点。

        //按照相似度从高到低排序，截断到前面k个。
        results.sort_by(|a,b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(k);
        results
    }
}

fn cosine_similarity(a: &Vector, b: &Vector) -> f32{
    // 计算余弦相似度
    assert_eq!(a.len(), b.len(), "dimension mismatch");  //猜测：这里的字符串参数应该是assert断言不生效时候，屏幕上会输出的话。

    let mut dot = 0.0f32;       //dot: 矢量a和矢量b的点积
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;  // 循环完毕后，norm_a中记录了矢量a中各分量值的平方和。
        norm_b += y * y;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();  //数值上看，denom的值等于a的二范数乘以b的二范数
    if denom == 0.0{
        0.0
    }
    else{
        dot / denom
    }
}