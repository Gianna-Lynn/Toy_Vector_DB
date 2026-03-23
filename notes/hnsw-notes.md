# 区分Vec, vec!, Vector
## Vec
Vec是Rust标准库内的动态数组类型名.Vec是一种容器类型.下面的说法都是正确的:
Vec<f32>;
Vec<Id>;
Vec<Record>;
Vec<(f32, Id)>; # 这是一个包含f32和Id的元组的Vec
分别表示一个包含f32类型元素的Vec,一个包含Id类型元素的Vec,一个包含Record类型元素的Vec,一个包含(f32, Id)类型元素的Vec.
## vec!
vec!是Rust中的一个宏,可以快速创建一个Vec.它的语法是vec![元素1, 元素2, ...].例如:
let v = vec![1, 2, 3];
## Vector
Vector通常不是Rust中的一个具体类型,而是我们自己用type alias定义的一个类型别名.例如:
type Vector = Vec<f32>;
这里我们定义了一个名为Vector的类型别名,它实际上是Vec<f32>.
data:Vector;
等价于
data:Vec<f32>;