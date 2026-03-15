use serde::{Serialize, Deserialize};
use serde_json;


// 一条向量的ID
pub type Id = u64;

// 向量本体（以后可以换成别的结构）
pub type Vector = Vec<f32>;

// 一条记录：id + 向量
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]   //Rust中提供的派生宏语法，让编译器自动为这个结构体生成Clone和Debug trait的默认实现，不需要手动编码
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
    
    //回答：trait直译为特征，规定了所有VectorStore都必须有的特征, 也就是两个方法签名，insert和search。
    //     在trait当中只写方法签名，不写具体实现, 以分号结束。
    //     VectorStore是一个trait, 是一份合同, 或者说接口说明书.
}

pub struct InMemoryVectorStore{ //定义了一个struct结构体.
    data: Vec<Record>,
    // fn save(&self, path: &str) -> Result<(), Error>,  // 错误:在struct定义中,不能直接写函数签名.struct只能包含字段(数据),不能包含方法.

    
}

impl InMemoryVectorStore{       //第一个impl: 直接为结构体添加方法.
    pub fn new() -> Self{       //猜测：这里是基于上面的定义，增加了一个具体的实现：如何通过new函数新建一个InMemoryVectorStore对象。
                                //回答：Rust中struct定义数据结构，而impl块定义它的行为（函数/方法）。implement=实现。
        Self{data: Vec::new()}
    }


    //函数功能:把当前内存中所有Record写到path指定的文件中
    pub fn save(&self, path: &str) -> Result<(), std::io::Error>{
        //&self表示以只读引用的方法借用当前的实例,可以通过self访问结构体定义的唯一字段:self.data,只读.如果要修改,需要用&mut self
        //path: &str表示一个名为path的参数,变量类型是&str,也就是字符串切片.使用&str而不是String,因为前者更加通用高效.
        //Result<(), Error>是一个枚举类型,成功时返回 Ok(()).内层的()表示Unit Type(单元类型),等价于C语言的void.失败时候返回Err(Error).参见RustBook 9.2
        
        // 废稿
        // let raw_data = new(InMemoryVectorStore);
        // let serialized_data = serde_json::to_string(&raw_data).unwrap();
        // println!("{}", serialized_data);
        // std::fs::write(path, contents);

        //第一步:序列化self.data
        let json_string = serde_json::to_string_pretty(&self.data)?;
        //问号表示"如果成功就继续,若果失败就从当前函数跳出并返回错误"
        
        //第二步:写入文件
        std::fs::write(path, json_string)?;

        //第三步:返回成功
        Ok(())
    
    }

    //函数功能:从path指定的文件中读出所有Record,写入到一个新的InMemoryVectorStore中.
    pub fn load(path: &str) -> Result<Self, std::io::Error>{
        //第一步:从文件读取内容
        let json_string = std::fs::read_to_string(path)?;
        //第二步:反序列化,构造一个新的Vec<Record>
        let data: Vec<Record> = serde_json::from_str(&json_string)?;

        Ok(Self{data})
    }
}

//猜测：泛型实际上说的是模板和具体实现这两件事。有点像C++的template。
//回答：二者非常相似。编译器会根据实际使用的类型，比如i32或者f64，自动生成多份具体的代码。

impl VectorStore for InMemoryVectorStore{   //第二个impl: 为结构体实现trait.
    //人话就是, 要让 InMemoryVectorStore 这个类型遵守 VectorStore 这份契约.
    fn insert(&mut self, id: Id, vector: Vector) {
        // VectorStore的定义中包含了insert和search两个函数的声明，但是没有给出具体实现。这里应该就是为了补充实现。

        // 暂时先占位，待实现
        //unimplemented!("insert is not implemented yet"); //猜测：这里的unimplemented!是一个既定的宏或者用法，括号当中的是参数，用于输出。
                                                           //回答：这里确实是Rust标准库提供的一个宏。如果运行到这一行代码，程序会直接panic，然后打印这句话。
        
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
            //问题: 这个地方的连环调用可以说看不懂一点。
            //回答: self.data是一个Vec<Record>（见pub struct InMemoryVectorStore的定义）。是一个静态数组,里面有很多Record.
            //      .iter()表示对self.data创建一个迭代器。每次迭代产生一个&Record，是一个不可变引用。是在用这样的方式遍历Vec<Record>
            //      .map(|rec{...}|)这部分接收迭代器的每个元素rec(&Record类型).
            //      |rec|是闭包的参数,两竖线是闭包的语法,不是绝对值,用来定义匿名函数.
            //      闭包体内,使用cosine_similarity()计算相似度得分.同时,构造一个新的SearchResult.
            //      map会将每一个Record变换成一个SearchResult.
            //      .collect()收集map变换以后的结果.把所有的SearchResult收集起来,重新打包成一个全新的动态数组Vec<SearchResult>.
            
            //问题: 什么是闭包?
            //回答: 闭包是一个匿名函数,可以捕获/闭合/使用周围作用域的变量.(也就是说,它可以把前几行代码中涉及到的变量直接拿来用.)
            //      闭包的基础语法:
                // |参数1, 参数2|{
                //     // 函数体
                //     // 可以使用参数,也可以使用外部变量.
                //     // 返回值
                // }
            //      闭包拥有三种捕获方式:不可变借用(只读),可变借用(修改),获取所有权(消耗)

            //问题: map的标准用法
            //回答: 
                // iterator.map(|当前元素|{
                //     // 在这里添加你的转换逻辑
                //     // 在这里返回新元素
                // })
            //map输入:一个闭包(Closure),也就是匿名函数 |...| {...}
            //map行为:对迭代器内的每一个元素都执行一次这个闭包.
            //map输出:一个新的迭代器,里面包含转换后的元素.


        
        //类似的写法
        // let mut results = Vec::new();
        // for rec in self.data.iter(){
        //     let score = cosine_similarity(query, &rec.vector);
        //     results.push(SearchResult { id: rec.id, score});
        // }

        // let results: Vec<SearchResult> = results;


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