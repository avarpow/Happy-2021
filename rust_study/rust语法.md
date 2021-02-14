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
let mut name :[types : capacity]={value1,value2.....};
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


