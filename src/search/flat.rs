/********************************************** Search **********************************************/

use crate::distance::cosine_similarity;
use crate::storage::inmemory::{InMemoryVectorStore};
use crate::types::{SearchResult, Vector};

pub fn search_flat( 
    store: &InMemoryVectorStore, 
    query: &Vector,
    k: usize
) -> Vec<SearchResult> {
    // &self表示以只读引用的方法借用当前的实例,可以通过self访问结构体定义的唯一字段:self.data,只读.
    //如果要修改,需要用&mut self
    //unimplemented!("search is not implemented yet");
    

    let mut results: Vec<SearchResult> = store.data
        .iter()
        .map(|rec|{
            let score = cosine_similarity(query, &rec.vector);
            SearchResult {id: rec.id, score}
        })
        .collect();
        //问题: 这个地方的连环调用可以说看不懂一点。
        //回答: self.data是一个Vec<Record>（见pub struct InMemoryVectorStore的定义）。是一个静态数组,里面有很多Record.
        //      .iter()表示对self.data创建一个迭代器。每次迭代产生一个&Record，是一个不可变引用。
        //      因为返回的不是&mut T,所以不可变. 用这样的方式遍历Vec<Record>
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
    //问题: 这个sort_by的用法也不太清楚。
    //回答: sort_by是Rust标准库提供的一个方法，用于对Vec中的元素进行排序。
    //它接受一个闭包作为参数，这个闭包定义了排序的规则。
    //在这个例子中，闭包的参数a和b都是SearchResult类型的引用。
    //闭包的主体使用partial_cmp方法比较a.score和b.score的大小，
    //并且使用unwrap()来处理可能的None值（如果score是NaN的话）。
    //具体来说,unwrap()的作用是:如果partial_cmp返回Some(Ordering)，
    //就返回其中的Ordering值；如果返回None，就会导致程序panic。
    //Some(Ordering)是Rust中用于表示比较结果的枚举类型，包含Less、Equal和Greater三个变体。
    //在这个排序规则中，b.score.partial_cmp(&a.score)表示我们希望按照score从高到低排序。
    //具体来说,如果b.score > a.score，partial_cmp会返回Some(Ordering::Greater)，
    //如果b.score < a.score，partial_cmp会返回Some(Ordering::Less)，
    //如果b.score == a.score，partial_cmp会返回Some(Ordering::Equal)。
    //最后，sort_by会根据这些比较结果对results中的元素进行排序。
    results.truncate(k);
    results
}