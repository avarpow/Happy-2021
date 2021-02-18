# rust start
### print
println!("{...}",xxx);
:? anything 
:b :o :h 进制转换
{name} key-value
{number} 后面元素的位置

### vars
let name: type = value
如果需要可以变化需要
let mut xxxx
可变字符串需要 String::From("string")
### arrays
let mut name :[types ; capacity]={value1,value2.....};
迭代器
for i in name.iter(){
    xxxx
}
可变迭代器
for i in name.iter_mut(){
    xxxx
}
某个切片范围
nums[a..y];
### branch
if else 和C语言一样 条件不需要括号
match 类似 switch

### loop
无限循环 loop{
    中间有break;
}
while 条件{

}
迭代器有范围的循环
for i in a..b   类似python

### function
public函数：pub关键字
fn name(value1: types,value2:types...) -> return type;
最后一个语句就是返回值

闭包函数
let name=|v1:tyes,v2:tyes| 后面是返回值 

### pointer reference
let p1=p2 其实是move了
let p1=&p2 才是reference赋值

### structs
struct name{
    subname1:type;
    subname2:type;
    subname3:type;
}

可以通过 name.0 name.1 的方式获取内部的值

### if let语法
例如
```rust
let some_u8_value=Some(0u8);
match some_u8_value {
    Some(3)=> println!("three"),
    _ => ()
}
```
等价于
```rust
if let Some(3)==some_u8_value{
    println!("three");
}
```
### 模块的引用
绝对路径：以crate名字开始
相对路径：以self，super或当前模块标识符开始
默认所有的creat，函数，结构体等都是私有的

use 类似c++，可以通过use xxx as yyy改名
use aaa::{a,b,c}同时添加多个
use aaa::*通配符，添加所有

### result语法
Ok 和 Err类型
unwarp方法：ok直接返回值，Err则panic
expect：在unwarp的基础上加入了信息。
？方法，更加省略，如果出现Err直接将返回(需要函数返回值为Result)
```rust
let mut f=File::open("1.txt")?;//失败则直接返回
```
？方法的链式调用
```rust
let mut s=String::new();
File::open("1.txt")?.read_to_string(&mut s)?;//失败则直接返回
```

### 泛型
\<T\>

### trait
1.实现trait类：直接impl trait
2.为非trait类实现trait方法：impl trait for 非trait
函数参数调用trait方法
参数声明的类型为impl tarit
用+指定多个trait
用where简化，在函数块的开头指定泛型的类型

### 生命周期

### test
\#[cfg(test)]
\#[test]为开始
1. assert
2. assert_eq
3. assert_ne

需要实现PartialEq才能断言

需要实现Debug才能断言

### 错误提示闭包
unwarp_or_else(|err|{...})
### 闭包 
cacher 在闭包中new
value值，如果有value值就不会重新计算
### cargo
三个斜杠///文档注释
//！整个crate的注释

### 智能指针
1. Box\<T\> 在堆上分配值
2. Rc\<T\> 引用计数值
3. Ref\<T\> RefMut\<T\> 通过RefCell\<T\>访问
通过指针定义list
```rust
enum List{
    Cons(i32,Box<List>),
    Nil,
}
```
Box可以像引用一样使用

自己定义的类型需要实现Deref 实现解引用

需要实现Drop 实现清理代码

Rc::clone可以增加引用计数，不会深拷贝

### 多线程
thread::spawn(闭包)

发送数据

let (tx,rx) = mpsc::channel()

### 互斥器mutex
.lock()方法获取锁

Arc\<T\> 原子引用计数，线程安全





