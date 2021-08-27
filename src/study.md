##  所有权
1. 常规变量的作用域问题

```rust    
 fn main() {
    //s 不可用
    let s  = "hello";//s可用
                          //可以对s进行相关操作
}//s作用域到此结束，s不可用

``` 
2. string类型
```rust
let s = String::from("hello");
//::表示from是String类型下的函数
```
声明可变的String变量
```rust
fn main(){
    let mut s = String::from("hello");

    s.push_str(", world rust");

    println!("{}",s);
}
```
这里可以看出字符串字面值和String类型的区别字符串字面值在编译时就知道它的内容了，其文本内容被硬编码到最终的可执行文件里(速度快、高效。是因为其不可变性)。
String类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容
-操作系统必须在运行时请求内存
. 这步通过调用Sting::from来实现
-当用完String之后，需要使用某种方式将内存返回给操作系统

对于rust来说对于某个值，当他拥有的变量走出作用范围时，内存会立即自动的交还给操作系统
```rust
fn main(){
    let mut s = String::from("hello");

    s.push_str(", world rust");

    println!("{}",s);
}//s走出作用域调用的是drop函数释放内存
``` 
变量和数据交互的方式：移动(move)
+   let x = 5  
+   let y = 5
```rust
let s1 = String::from("hello");
let s2 = s1//浅拷贝不复制堆上的内存所指内容
``` 
```rust
fn main(){
    let mut s = String::from("hello");

    s.push_str(", world rust");

    let s2  = s.clone();
    println!("{} , {}",s2,s);//克隆比较消耗资源
}
```
```rust
let x = 5;
let y = x;//stack上的数据直接复制了因为是标量类型
```
实现Copy trait的类型
+ 任何简单标量的组合类型都可以是Copy的
+ 任何需要非配内存或某种资源的都不是Copy的
+ 一些拥有Copy trait的类型：   
    + 所有的整数类型例如u32
    + bool
    + char
    + 所有的浮点类型例如f64
    + Tuple（元组），如果其所有的字段都是Copy的
    (i32,u32)
### 所有权和函数
+ 在语义上，将值传递给函数和把值赋给变量是类似的：
  -将值传递给函数发生了移动或复制
```rust
fn main(){
    let mut s = String::from("hello,world");

    take_ownership(s);

    let x = 5;

    makes_copy(x);//简单标量

    println!("{}",x);//

}

fn take_ownership(someString:String){
    println!("{}",someString);
}
fn makes_copy(someNumber:i32){
    println!("{}",someNumber);
}
```
1. 返回值与作用域
+ 函数在返回值的过程中同样也会发生所有权的转移
```rust
fn main(){
    let s1 = give_string();

    let s2 = String::from("hello world");

    let s3 =tack_and_give_back(s2);

}
fn give_string()->String{
    let some = String::from("hello");
    some
}

fn tack_and_give_back(some :String)->String{
    some
}
```
+ 一个变量的所有权总是遵循同样的模式：
    + 把一个值赋给其他变量时就会发生移动
    + 把一个包含heap数据的变量离开作用域时，它的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了
```rust
fn main(){
    let s1 = String::from("hello");
    
    let (s2,len ) = calculate_len(s1);

    println!("the lenth of {} is {}",s2,len);
}

fn calculate_len(some :String)->(String,usize){
    let s = some.len();
    (some ,s)//考虑使用引用使得逻辑更加简单(reference)
}
```

##  引用和借用
```rust
fn main(){
    let s1 = String::from("hello");
    
    let len  = calculate_len(&s1);//传递参数就是引用类型

    println!("the lenth of {} is {}",s1,len);
}

fn calculate_len(some :&String)->usize{//函数参数列表也是引用类型
    let s = some.len();
    s
}
```
+ 引用的类型是&String而不是String
+ &符号就表示引用：允许你引用某些值而不取得所有权
1. 借用
+ 把引用作为函数参数的行为叫做借用
+ 是否可以修改借用的东西呢？
    + 不行
2. 可变引用
```rust
fn main(){
    let mut s1 = String::from("hello");
    
    let len  = calculate_len(&mut s1);

    println!("the lenth of {} is {}",s1,len);
}

fn calculate_len(some :&mut String)->usize{
    some.push_str(", world");
    let s = some.len();
    s
}
```
+ 可变引用的一个重要限制：在特定的作用域内，对一块数据，只能有一个可变的引用。
```rust
let mut s = String::from("hello");
let s1 = &mut s;
let s2 = &mut s;//报错
```
+   这样做的好处就是防止数据竞争
+ 以下三种行为下会发生数据竞争
    + 两个或多个指针同时访问一个数据
    + 至少有一个指针用于写入数据
    + 没有任何机制来同步对数据的访问
+ 可以通过创建新的作用域，来允许费同时的创建多个可变引用
```rust
fn main(){
    let mut s = String::from("hello"); 
    {
        let s1 = &mut s; 
    }//s1出了作用域就不在有效
    let s2 =  &mut s ;
}
```
+ 另外一个限制
    + 不可以同时拥有一个可变引用和一个不变的引用
    + 多个不可变的引用是可以的

+ 悬空引用 dangling references
    + 悬空指针(Dangling pointer):一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其它人使用了
+ 在Rust里，编译器课保证永远都不是悬空引用
###  切片(slice)
来看这样一个小栗子找到字符串当中的空格索引位置
```rust
fn main(){
    let mut s = String::from("hello , world");
    println!("the first word of {} is {}",s,first_word(&s));
}
fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();

    for (i,&iterm) in bytes.iter().enumerate(){
        if iterm ==b' ' {
            return i;
        }   
    }
    s.len()
}
```
表面上来看这样是没问题的但是这个函数和字符串本身没有关联也就是意味着即使调用了s.clear()函数这个返回值依旧是5
+ 字符串切片就是指向字符串中一部分内容的引用
```rust
fn main(){
    let s  = String::from("hello,world"); 

    let hello = &s[0..5];//[..5]
    let world = &s[6..11];//[6..]
    let whole = &s[0..11];//[..]
}
```
+ 形式：[开始索引..结束索引]
    + 开始索引就是切片起始位置的索引
    + 结束索引是切片终止位置的下一个索引值

+ 注意
    + 字符串切片的范围索引必须发生在有效的UTF-8字符边界内
    + 如果尝试从一个多字节的字符串中创建字符串切片，程序会报错退出
+ 使用字符串切片重写上述的例子
```rust
fn main(){
    let mut s = String::from("hello , world");
    println!("the first word of {} is {}",s,first_word(&s));
}
fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();

    for (i,&iterm) in bytes.iter().enumerate(){
        if iterm ==b' ' {
            return &s[..i];
        }   
    }
    &s[..]
}
```
+ 字符串字面值是切片
```rust
let s = "hello world"
```
+ 变量s的类型是&str，它是一个指向二进制程序特定位置的切片
    + &str是不可变引用，所以字符串字面值也是不可变的

## struct  
###  定义struct
+ 使用struct关键字，并为整个struct命名
+ 在花括号内，为所有字段(Filed)定义名称和类型
```rust
struct User{
    username :String,
    email:String,
    sign_in_count:i64,
    active:bool,
}
```
### 实例化struct
+ 想要使用struct，需要创建struct实例：
    + 为每个字段指定具体值
    + 无需按声明的顺序进行指定
```rust
let usr1 = User{
    email:String::from("hello,world"),
    username :String::from("someusername"),
    active:true,
    sign_in_count:1,
}
```
###  取得struct里面的某个值
+ 使用点标记法
```rust
usr1.email = String::from("anotheremail@qq.com");
```
注意
+ 一旦struct的实例是可变的那么实例中的所有字段都是可变的
### struct作为函数的返回值
```rust
fn build_user(email:String,username:String)->User{
    User{
        email:email,
        username:username,
        active:true,
        sign_in_count:1,
    }
}
```
字段名初始化简写
```rust
fn build_user(email:String,username:String)->User{
        User{
            email,
            username,
            active:true,
            sign_in_count:1,
        }
    }
```
当字段名和字段值对应的变量名相同时，就可以使用字段初始化简写的方式如上述例子
### struct更新语法
+ 当你想基于某个struct实例来创建一个新实例的时候，可以使用struct更新语法：
```rust
 let usr2 = User{
        email:String::from("hello,world rust"),
        username :String::from("someusernameqq"),
        active:usr1.active,
        sign_in_count:usr1.sign_in_count,
    };
    //等价于
    let usr2 = User{
        email:String::from("hello,world rust"),
        username :String::from("someusernameqq"),
        ..usr1
    };
```
上诉实例化中除了前面的两个字段是新赋的值剩下的事保留了原来的实例可以使用简写的形式

### Tuple struct
+ 可以定义类似tuple的struct，叫做tuple struct
    + Tuple struct 整体有个名，但里面的元素没有名
    + 适用：想给整个tuple起名，并让他不同于其他的tuple，而且又不需要给每个元素起名
+ 定义Tuple struct：使用struct关键字，后边是名字，以及里面元素的类型
```rust
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
let black = Color(0,0,0);
let origin = Point(0,0,0);
```
+ black和origin是不同的类型，是不同的tuple struct的实例

### Unit-Like Struct(没有任何字段)
+ 可以定义没有任何字段的struct，叫做Unit-Like struct(因为与(),单元类型相似)
+ 适用于需要在某个类型上实现某个trait，但是在里面有没有想要存储的数据

### struct数据的所有权
```rust
struct User{
    username :String,
    email:String,
    sign_in_count:i64,
    active:bool,
}
```
+ 这里的字段使用了String而不是&str:
    + 该struct实例拥有其所有的数据
    + 只要struct实例是有效的，那么里面的字段数据也是有效的
+ struct里也可以存放引用，但这需要是用生命周期

### struct例子
计算长方形面积
```rust
fn main(){
    let rect  = (30,50);
    println!("the area of rect is {}",area(rect));
}
fn area(dim:(u32,u32))->u32{
    dim.0 * dim.1
}
```
元组类型求解

```rust
struct  Rectangle{
    width:u32,
    length:u32,
}
fn main(){

    let rect = Rectangle{
        width:30,
        length:50,
    };
    println!("the area of rect is {}",area(&rect));//借用不会获得rect的所有权意味着函数调用后还可以继续使用

}
fn area (rect :&Rectangle)->u32{
    rect.length * rect.width
}
```
struct求解

打印结构体
```rust
#[derive(Debug)]
struct  Rectangle{
    width:u32,
    length:u32,
}
fn main(){

    let rect = Rectangle{
        width:30,
        length:50,
    };
    println!("{:#?}",rect);
}
```
使用{:?}或者{:#?}为结构体实现打印在这之前需要加上#[derive(Debug)]因为struct的打印不是std::fmt::Display而是std::fmt::Debug使用#[derive(Debug)]的意思就是让其派生于Debug

### struct 方法
+ 方法和函数类似
+ 方法是在struct(或enum、trait对象)的上下文中定义
+ 第一个参数是self，表示方法被调用的struct实例

定义方法
```rust
#[derive(Debug)]
struct  Rectangle{
    width:u32,
    length:u32,
}

impl  Rectangle{
    fn area (&self)->u32{
        self.length * self.width
    }
}

fn main(){

    let rect = Rectangle{
        width:30,
        length:50,
    };

    println!("the area of rect is {}",rect.area());

    println!("{:#?}",rect);
}
```
+ 在impl块里定义方法
+ 方法的第一个参数可以是&self，也可以获得其所有权活可变借用。和其他参数一样
+ 更良好的代码组织

方法调用的运算符
+ rust没有->运算符
+ rust会自动引用或解引用
    + 在调用方法时会发生这种行为
+ 在调用方法时，rust根据情况自动添加&、&mut或*，以便object*可以匹配方法的签名
+ 下面两行代码效果相同
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
方法参数
```rust
 fn can_hold(&self,other:&Rectangle)->bool{
        self.width > other.width && self.length > other.length
    }
```
关联函数
+ 可以在impl块里定义不把self作为第一个参数的函数，它们叫关联函数(不是方法)
    + 例如：String::from()
+ 关联函数通常用于构造器(例子)

```rust
fn quare(size:u32)->Rectangle{
        Rectangle{
            width:size,
            length:size,
        }
    }
```
调用
```rust
let s = Rectangle::squre(20);
```
::符号
+ 关联函数
+ 模块创建的命名空间

多个impl块
+ 每个struct允许拥有多个impl块
完整例子
```rust
#[derive(Debug)]
struct  Rectangle{
    width:u32,
    length:u32,
}

impl  Rectangle{
    fn area (&self)->u32{
        self.length * self.width
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width > other.width && self.length > other.length
    }
    fn square(size:u32)->Rectangle{
        Rectangle{
            width:size,
            length:size,
        }
    }
}

fn main(){

    let s = Rectangle::square(20);
    
    let rect = Rectangle{
        width:30,
        length:50,
    };

    println!("the area of rect is {}",rect.area());

    println!("{:#?}",rect);
}
```

## 枚举
定义枚举
```rust
enum IpAddKind{
    v4,
    v6,
}
```
枚举值
```rust
let four = IpAddKind::v4;
let six = IpAddKind::v6;
```
完整例子
```rust
enum IpAddKind{
    v4,
    v6,
}
fn main()
{
    let four = IpAddKind::v4;
    let six  = IpAddKind::v6;
    route(four);
    route(six);
    route(IpAddKind::v4);
}
fn route (ip:IpAddKind){
}
```
将数据附加到枚举的变体中
```rust
enum IpAddKind{
    v4,
    v6,
}

struct  Ip{
    kind :IpAddKind,
    address:String,
}
fn main(){
    
    let home = Ip{
        kind:IpAddKind::v4,
        address:String::from("127.0.0.1")
    };
    
    let  route = IP{
        kind:IpAddKind::v6,
        address:String::from("::1")
    };
}
```
更简洁的写法
```rust
enum IP{
    v4(String),
    v6(String),
}
```
+ 优点：
    + 不需要使用额外的struct
    + 每个变体可以拥有不同的类型以及关联的数据量
```rust
enum Ip{
    v4(u8,u8,u8,u8),
    v6(String),
}

fn main(){

    let home = Ip::v4(127,0,0,1);
    let route = Ip::v6("::1");
}
```
为枚举定义方法
+ 也可以使用impl关键字

Option枚举
+ 定义于标准库中
+ 在Prelude(预导入模块)中
+ 描述了：某个值可能存在(某种类型)或不存在的情况

Rust没有Null
+ 其他语言中：
    + Null是一个值，他表示“没有值”
    + 一个变量可以处于两种状态：空值(null)、非空
+ Null引用 Bilion Dollar Mistake
 rust中类似Null概念的枚举-Option&lt;T&gt;
 + 标准库中的定义：
 + eunm Option&lt;T&gt;{
     Some&lt;T&gt;,
     None,
 }
 ```rust
 fn main(){
    let some_number = Some(5);
    let some_string = Some("a string");

    let none:Option<i32> = None;
}
```
Option &lt;T&gt; 比Null好在哪
+ Option &lt;T&gt; 和T是不同的类型，不可以把Option &lt;T&gt;直接当成T
+ 若想使用 Option &lt;T&gt; 中的T必须将其转换为T

控制流运算符-match
+ 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
+ 模式可以是字面值、变量名、通配符
```rust
#[derive(Debug)]
enum Usstate{
    Alasjia,
    Aloha,
}
enum Coin{
    A,
    B,
    C,
    D(Usstate),
}

fn value(coin: Coin)->u8{
    match coin{
        Coin::A=>{
            println!("the coin from a");
            1
        }
        Coin::B => 5,
        Coin::C =>10,
        Coin::D(state) =>{
            println!("{:#?}",state);
            35
        }
    }
}

fn main(){
    let c = Coin::D(Usstate::Alasjia);
    println!("{}",value(c));
}
```
匹配Option&lt;T&gt;
```rust
fn main() {
    let five = Some(5);
}

fn option_plus(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
match匹配必须穷举所有的可能
+ 使用_通配符：替代其余没有列出的值
```rust
fn main(){
    let  v  = 0u8;
    match v{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _ =>(),
    }
}
```
if let 
+ 处理只关心一种匹配而忽略其他匹配的情况
+ 放弃了穷举的可能性
## Package Crate Module
rust的代码组织
+ 代码组织主要包括
    + 那些细节是可以暴露，那些细节是私有的
    + 作用域内那些名称有效
+ 模块系统
    + Package(包)：Cargo的特性，让你构建、测试、共享crate
    + Crate(单元包)：一个模块树，它可以产生一个library或可执行文件
    + Module(模块)： use：让你控制代码的组织、作用域、私有路径
    + Path(路径)： 为struct、function或module等命名的方式

Package和Crate
+ Crate的类型：
    + binary
    + library
+ Crate Root
    + 是源代码文件
    + rust编译器从这里开始，组成你的Crate根Module
+ 一个Package
     + 包含1个Cargo.toml，它描述了如何构建这些Crate
     + 只能包含0-1个library crate
     + 可以包含任意数量的binary crate
     + 但必须至少包含一个crate(library或binary)

+ Cargo 的惯例
    + src/main.rs:
    + -binary crate 的crate root
    + -crate名与package名相同

    + src/lib.rs：
    + -package包含一个library crate
    + -library crate的crat root
    + -crate名与package名相同
+ Cargo把crate root文件交给rustc来构建library或binary
    + 一个Package可以同时包含src/main.rs和src/lib.rs
    + 一个binary crate一个library crate
    + 名称与package名相同
    + 一个Package可以有多个binary crate：
    + 文件放在src/bin
    + 每个文件是单独的binary crate
+ Crate的作用
    + 将相关功能组合到一个作用域内，便于在项目间进行共享
    + 防止冲突
    + 例如rand crate ，访问他的功能需要通过他的名字：rand
   
定义module来控制作用域和私有性
+ Module 
    + 在一个crate内，将代码进行分组
    + 增加可读性，易于服用
    + 控制项目(item)的私有性。public、private
+ 建立module
    + mod关键字
    + 可嵌套
    + 可包含其他项(struct 、enum、常量、trait、函数等)的定义
+ 路径(path)
    + 为了在rust的模块中找到某个条目，需要使用路径
    + 路径的两种形式
    + -绝对路径：从crate root开始，使用crate名或字面值crate
    + -相对路径：从当前模块开始，使用self，super或当前模块的标识符
    + 路径至少有一个标识符组成，标识符之间使用::。 

私有边界
+ 模块不仅可以组织代码，还可以定义私有边界
+ 如果想把函数或struct等设为私有，可以将它放到某个模块中
+ rust中所有的条目(函数、方法、struct、enum、模块、常量)默认是私有的
+ 父级模块无法访问子模块中的私有条目
+ 子模块里可以使用所有祖先模块中的条目

pub关键字
+ 使用pub关键字来将某些条目标记为公共的

super关键字
+ super：用来访问父级模块中的内容，类似文件系统中的..

pub struct 
+ pub放在struct前：
    + struct是公共的
    + struct的默认字段是私有的
+ struct的字段需要单独设置pub来变成公有的

pub enum
+ pub放在enum前
    + enum是公共的
    + enum的变体也都是公共的

use关键字
+ 可以使用use关键字将路径导入到作用域内
    + 仍遵循私有性规则
+ 使用use来指定相对路径

use的习惯用法
+ 函数：将函数的父级模块引入作用域(指定到父级)
+ struct，enum，其他：指定完整路径(指定到本身)
+ 同名条目指定到父级

as关键字
+ as关键字可以为引入的路径指定本地的别名

使用pub use重新导出名称
+ 使用use将路径(名称)导入到作用域后，该名称在此作用域内是私有的。
+ pub use：重导出
    + 将条目引入作用域
    + 该条目可以被外部代码引入到他们的作用域

使用外部包(package)
1. Cargo.toml添加依赖的包(package)
dependencies下添加需要的依赖
2. use将特定的条目引入作用域

+ 标准库(std)也被当做外部包
    + 不需要修改Cargo.toml来包含std
    + 需要通过use来将特定的条目引入到当前作用域

使用嵌套路径清理大量的use语句
+ 如果使用同一个包或模块下的多个条目
```rust
use std::cmp::Ordering;
use std::io;
```
+ 可以使用嵌套路径在同一行内将上述条目进行引入：
    + 路径相同的部分::{路径差异的部分}
```rust
use std::{cmp::Ordering,io};
```
+ 如果两个use路径之一是另一个的子路径
    + 使用self

通配符*
+ 可以吧所用的公有条目都引入到作用域
+ 注意谨慎使用
+ 应用场景：
    + 测试。将所有被测试的代码引入到tests模块
    + 有时被用于预导入(prelude)模块

将模块内容移动到其他文件
+ 模块定义时，如果模块名后边是“;”,而不是代码块：
    + rust会从与模块同名的文件中加载内容
    + 模块树的结构不会变化
+ 随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中

## 常见的集合
1. vector
使用vector存储多个值
+ Vec&lt;T&gt;，叫做vector
    + 由标准库提供
    + 可存储多个值
    + 只能存储相同数据类型的值
    + 值在内存中连续存放

创建vector
+ Vec::new函数
```rust
fn main(){
    let v :Vec<i32> = Vec::new();
}
```
+ 使用初始值创建Vec&lt;T&gt;,使用vec!宏
```rust
fn main(){
    let v = vec![1,2,3];
}
```
+ 向vector中添加元素，使用push方法
```rust
fn main(){
    let mut v = Vec::new();

    v.push(3);

}
```
删除vector
+ 与其他任何struct一样，当vector离开作用域后
    + 他就被清理掉了
    + 他所有的元素也被清理掉了

读取vector的元素
+ 两种方式可以引用vector里的值
    + 索引
    + get方法
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("the third value is {}", third);

    match v.get(2) {//这种方式访问数组越界后程序不会恐慌
        Some(third) => println!("the third value is {}", third),
        None => println!("the is no third element"),
    }
}
```
索引vs get处理访问越界
+ 索引：panic
+ get：返回None

所有权和借用规则
+ 不能在同一作用域内同时拥有可变和不可变引用
```rust
fn main(){
    let v = vec![1,2,3,4];

    let s = &v[2];//不可变借用
    v.push(6);//可变借用

}
```
遍历vector中的值
+ for循环
```rust
fn main(){
    let v = vec![1,2,3,4];

    for i in &v{
        println!("{}",i);
    }
}
```
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];

    for i in &mut v {
        *i += 50;//解引用
        println!("{}",i);
    }
}
```
string
字符串是什么
+ Byte的集合
+ 一些方法
+ 能建byte解析为文本
+ rust的核心语言层面，只有一个字符串类型：字符串切片str或(&str)
+ 字符串的切片：对存储在其他地方、UTF-8编码的字符串的引用
    + 字符串字面值：存储在二进制文件中，也是字符串切片

String类型
+ 来自标准库而不是核心语言
+ 可增加、可修改、可拥有
+ UTF-8
通常说的字符串是指
+ String和&str

创建一个新的字符串
+ 很多Vec&lt;T&gt;的操作都可用于String
+ String::new()函数
```rust
let s = String::new();
```
+ 使用初始值来创建String
    + to_string()方法。可用于实现了Display trait的类型，包括了字符串字面值
    + 使用String::from()函数，从字面值创建String

更新String
+ push_str()方法：把一个字符串切片附加到String
```rust
fn main(){
    let mut s  = String::from("foo");

    s.push_str("bar");
    println!("{}",s);
}
```
+ push方法：把当个字符附加到String
```rust
fn main(){
    let mut s  = String::from("foo");

    s.push('l');
    println!("{}",s);
}
```
+ +链接字符串
```rust
fn main() {
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s = s1 + &s2;//拼接之后s2所有权保留

    println!("{}", s);
}
```
+ 使用了类似这个签名的方法fn add (self,&str)->String{...}
    + 标准库中的add方法使用了泛型
    + 只能把&str添加到String
    + 解引用强制转换

+ format!链接多个字符串
```rust
fn main() {
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s = format!("{}{}",s1,s2);
    println!("{}",s);

}
```
对String按索引的形式进行访问
+ 按索引语法访问String的某部分，会报错
+ rust的字符串不支持索引语法访问

内部表示
+ String是对Vec&lt;u8&gt;的包装
    + len()方法
```rust
fn main() {
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s = format!("{}{}",s1,s2);
    println!("{}",s);
    println!("{}",s.len());//UTF-8占6个字节

}
```
rust有三种看待字符串的方式
+ 字节(bytes)
+ 标量值(scalar values)
+ 字形簇(grapheme cluster)

rust不允许对String进行索引访问的最后一个原因
    + 索引操作应消耗一个常量时间(o(1))
    + 二String无法保证：需要遍历所有内容，来确定有多少个合法字符

切割String
+ 可以使用[]和一个范围来创建字符串的切片
```rust
fn main() {
    let s1 = "fafasfsafasfasfas";
    let s = &s1[..3];
    
    println!("{}",s);

}
```
+ 必须谨慎使用
+ 如果切割时跨越了字符边界，程序就会panic

遍历String的方法
+ 对于标量值：chars()方法
+ 对于字节：bytes()方法
+ 对于字符簇：很复杂，标准库为提供

String不简单
+ rust选择正确处理String数据作为所有rust程序的默认行为
    + 程序员必须在处理UTF-8数据之前投入更多的经历
+ 可防止在开发后期处理设计非ASCLL字符的错误

HashMap&lt;K,V&gt;
+ 键值对的形式存储数据，一个键(Key)d对于一个值(Value)
+ Hash函数：决定如何在内存中存放K和V
+ 使用场景，通过k(任何类型)来寻找数据，而不是通过索引

创建HashMap
+ 创建HashMap::new()函数
+ 添加数据：insert()方法
```rust

use  std::collections::HashMap;

fn main(){

    let mut score = HashMap::new();
    score.insert(String::from("blue"), 10);
    println!("{:#?}",score);
}
```
HashMap
+ HashMap用的较少，不在Prelude中
+ 标准库对其支持较少，没有内置的宏来创建HashMap
+ 数据存储在heap上
+ 同构的一个HashMap中
    + 所有的K必须是同一种类型
    + 所有的V必须是同一种类型

另一种创建HashMap的方式：collect方法
+ 在元素类型为Tuple的Vector上使用collect方法。可以组建一个HashMap：
    + 要求Tuple有两个值：一个作为K，一个作为V
    + collect方法可以把数据整合成很多集合类型，包括HashMap
    + 返回值需要显示指明类型
```rust

use std::collections::HashMap;
fn main() {
    let team = vec![String::from("blue"), String::from("yellow")];
    let number = vec![10, 50];
    let score:HashMap<_,_> = team.iter().zip(number.iter()).collect();//zip拉链
    println!("{:#?}",score);
}
```
HashMap和所有权
+ 对于实现了Copy trait的类型例如(i32),值会被复制到HashMap中
+ 对于拥有所有权的值(String)，值会被移动，所有权会会转移给HashMap
```rust

use std::collections::HashMap;
fn main() {
    let  user_name = String::from("hello");    
    let  user_email = String::from("world");   
    let mut hash = HashMap::new();
    hash.insert(user_name, user_email); 
}
```
+ 如果将值的引用插入到HashMap值本身不会引用
    + 在HashMap有效的期间，被引用的值必须保持有效
```rust
use std::collections::HashMap;
fn main() {
    let  user_name = String::from("hello");    
    let  user_email = String::from("world");   
    let mut hash = HashMap::new();
    hash.insert(&user_name, &user_email); 
}
```
访问HashMap中的值
+ get方法
    + 参数：k
    + 返回：Option&lt;&V&gt;

```rust
use std::collections::HashMap;
fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("bliu"), 10);
    score.insert(String::from("blu"), 40);
    let team_name = String::from("blu");
    let s = score.get(&team_name);
    match s {
        Some(s) => println!("{}", s),//
        None => println!("no value"),
    }
}
```
遍历HashMap
```rust
use std::collections::HashMap;
fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("bliu"), 10);
    score.insert(String::from("blu"), 40);
   for (k,v) in &score{
       println!("{},{}",k,v);
   }
    
}
```
更新HashMap&lt;K,V&gt;
+ HashMap大小可变
+ 每一个K同时只能对应一个V
+ 更新HashMap中的数据
    + K已经存在，对应一个V
        + 替换现有的V
        + 保留现有的V,忽略新的V
        + 合并现有的V和新的V
    + K不存在
        + 添加一对K,V
替换现有的V
+ 如果向HashMap插入一对KV,然后再插入同样的K，但是不同的V，那么原来的V会被替换掉
```rust
use std::collections::HashMap;
fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("bliu"), 10);
    score.insert(String::from("bliu"), 20);
    println!("{:#?}",score);//  v20

}
```
只在K不对应任何值的情况下，才插入V
+ entry方法：检查指定的K是否对应一个V
    + 参数为K
    + 返回enum Entry：代表值是否存在
+ Entry的or_insert()方法：
```rust
use std::collections::HashMap;
fn main(){
    let mut score  = HashMap::new();
    score.insert(String::from("blue"), 10);
    score.entry(String::from("blue")).or_insert(10);
    println!("{:#?}",score);
    score.entry(String::from("yellow"),).or_insert(10);
    println!("{:#?}",score);

}
```
+  如果K存在，返回到对应的V的一个可变引用
+ 如果K不存在，将方法参数作为K的新值插进去，返回到这个值的可变引用

基于现有的V来更新V
```rust
use std::collections::HashMap;
fn main(){
    let text = "hello world wonderful world";
    let mut map  = HashMap::new();
    for word in text.split_whitespace(){//字符串以空格分割
        let mut count = map.entry(word).or_insert(0);//如果是已经存在K返回对应的可变引用
        *count+=1; 
    }
    println!("{:#?}",map);
}
```
Hash函数
+ 默认情况下，HashMap使用加密功能强大的Hash函数，可以拒绝服务器(Dos)攻击
    + 不是可用的最快Hash算法
    + 但具有更好的安全性
+ 可以指定不同的hasher来切换到另一个函数
    + hasher是实现了BuildHasher trait的类型

## 错误处理
### panic！ 不可恢复的错误
rust错误处理概述
+ rust的可靠性：错误处理
+ 大部分情况下：在编译时提示错误，并处理

+ 错误处理的分类
+ 可恢复
    + 例如文件未找到，可再次尝试
+ 不可恢复
    + bug 例如访问的索引超出范围

+ rust没有类似异常的机制
    + 可恢复错误：Result&lt;T,E&gt;
    + 不可恢复：panic!

不可恢复的错误与panic!
+ 当panic!执行
    + 你的程序会打印一个错误信息
    + 展开(unwind)、清理调用栈(Stack)
    + 退出程序

为应对panic，展开或终止(abort)调用栈
+ 默认情况下，当panic发生：
    + 程序展开调用栈(工作量大)
        + rust沿着调用栈往回走
        + 清理每个遇到的函数中的数据
    + 或立即终止调用栈
        + 不进行清理，直接停止程序
        + 内存需要os进行清理

+ 想让二进制文件更小，把设置中的展开改为终止
    + 在Cargo.toml中适当的profile部分设置
        + panic = 'abort'

使用panic!产生的回溯信息
+ panic!可能出现在：
    + 我们所写的代码中
    + 我们所依赖的代码中

+ 可通过调用panic！的函数的回溯信息来定位引起问题的代码
+ 通过设置环境变量RUST_BACKTRACE可得到回溯信息
+ 为了获取带有调试信息的回溯，必须启用调试符号(不带--release)

Result枚举和可恢复错误
+ enum Result&lt;T,E&gt;{
    Ok(T),
    Err(E),
}
+ T 操作成功的情况下，Ok变体里返回的数据的类型
+ E 操作失败的情况下，Err变体里返回的错误的类型

处理Result的一种方式：match表达式
+ 和Option枚举一样，Result及其变体也是由predule带入作用域
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("erro openning file {}", error)
        }
    };
}

```
匹配不同的错误
+ match很有用，但是很原始
+ 闭包(closure)。Result&lt;T,E&gt;有很多方法：
    + 他们接收闭包作为参数
    + 使用match实现
    + 使用这些方法会让代码更加简洁

```rust
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error create file{}", e),
            },
            oe => panic!("error openning the file{:?}", oe),
        },
    };
}
```
unwrap
+ unwrap:match 表达式的一个快捷方法

```rust
use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("erro openning file {}", error)
//         }
//     };
// }

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
+ 如果Result结果是OK，返回Ok里面的值
+ 如果Result结果是Err，调用panic!宏

expect
+ expect :和unwrap类似，但可指定错误信息

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("无法打开文件hello.txt");
}
```

传播错误
+ 在函数中处理错误
+ 将错误返回给调用者

? 运算符
+ ？运算符：传播错误的一种快捷方式
+ 如果Result是Ok：Ok中的值就是表达式的结果，然后继续执行程序
+ 如果Result是Err：Err就是整个函数的返回值，就像使用了return

```rust

use std::fs::File;
use std::io::{self, Read};

fn read_user_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let result = read_user_file();
}

////////////////////////


use std::fs::File;
use std::io::{self,  Read};

fn read_user_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_user_file();
}
```
?from函数
+ Trait std::convert::From 上的from函数
    + 用于错误之间的转换
+ 被?所引用的错误，会隐式的被from函数处理
+ 当？调用from函数是：
    + 它所接受的错误类型会被转化为当前函数返回类型所定义的错误类型
+ 用于：针对不同错误原因，返回同一种错误类型
    + 只要每个错误类型实现了转换为所返回的错误类型的from函数

```rust
use std::fs::File;
use std::io::{self,  Read};

fn read_user_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_user_file();
}

```

? 运算符只能用于返回Result的函数
+ main函数返回特型:()
+ main返回的返回类型也可以是：Result&lt;T,E&gt;
+ Box<dyn Error>是trait对象
    + 简单理解：任何可能的错误类型

- - - 

什么时候应该用panic!
+ 在定义一个可能失败的函数时,优先考虑返回Result
+ 否则就panic!

编写实例、原型代码、测试
+ 演示某些概念：unwrap
+ 原型代码：unwrap 、expect
+ 测试 ： unwrap、expect

有时你比编译器掌握更多的信息
+ 你可以确定Result就是Ok：unwrap
```rust
use std::net::IpAddr;

fn main(){
    let ip:IpAddr = "127.0.0.1".parse().unwrap();
}
```
错误处理的指导性建议
+ 当代码最终可能处于损坏状态时，最好使用panic!
+ 损坏状态(Bad state):某些假设、保证、约定或不可变性被打破
+ 例如非法的值、矛盾的值或空缺的值被传入代码
+ 以及下列中的一条
    + 这种损坏状态并不是预期能够偶尔发生的事情
    + 在此自后，你的代码如果处于这种损坏状态就无法运行
    + 在你使用的类型中没有一个好的方法来将这些信息(处于损坏状态)进行编码

场景建议
+ 调用你的代码，传入无意义的参数值：panic!
+ 调用外部不可控代码，返回非法状态，你无法修复：panic!
+ 如果失败是可预期的：Result
+ 当你的代码对值进行操作，首先应该验证这些值：panic!

为验证创建自定义类型
+ 创建新的类型，把验证逻辑放在构造实例的函数里

getter：返回字段的数据
+ 字段是是有的外部数据无法直接对字段赋值

## 泛型、trait、生命周期
消除重复代码
```rust
fn main() {
    let number = vec![1, 2, 3, 4, 5];
    let mut largest = number[0];
    for i in number {
        if largest < i {
            largest = i;
        }
    }
    println!("the largest is {}", largest);
}
```
假如另外一个Vector需要求最大值也重复调用了这段代码

重复代码
+ 重复代码的危害    
    + 容易出错
    + 需求变更时需要在多处进行修改
+ 消除重复：提取函数

```rust
fn largest_number(list:&[i32])->i32{
    let mut largest = list[0];
    for &item in list{
        if largest < item{
            largest = item
        }
    }
 largest
}

fn main(){
    let number_list = vec![1,13,4345,4,656567,123];
    let result = largest_number(&number_list);
    println!("the largest of number list is {}", result);
    let number_list = vec![2321,4332455,56576,3213123,43423];
    let result = largest_number(&number_list);
    println!("the largest of number list is {}", result);

}
```

消除重复的步骤
+ 识别重复代码
+ 提取重复代码到函数体中，并在函数签名中指定函数的输入和返回值
+ 将重复的代码使用函数调用进行替代

泛型
+ 泛型：提高代码的复用能力
    + 处理重复代码的能力
+ 泛型是具体代码或其他属性的抽象代替：
    + 你编写的代码不是最终的代码，而是一种模板，里面有一些占位符。
    + 编译器在编译时将占位符替换为具体的类型
+ 例如： fn largest&lt;T&gt;(list:&[T])->T{...}

+ 类型参数：
    + 很短，通常带一个字母
    + CamelCase
    + T ：type的缩写

函数定义中的泛型
+ 泛型参数
    + 参数类型
    + 返回类型

struct定义中的泛型
+ 可以使用多个泛型参数
    + 太多的类型参数：你的代码需要重组为多个更小的单元

```rust

// struct Point<T>{
//     x:T,
//     y:T,
// }
struct Point<T,U>{
    x:T,
    y:U,
}
fn main(){
    let intergere = Point{x:3,y:2};
    let float = Point{x:3.0,y:2.0};
}
```
enum定义中的泛型
+ 可以让枚举的变体持有泛型数据
+ 例如Option&lt;T&gt;,result&lt;T,E&gt;

方法中定义的泛型
+ 为struct或enum实现方法的时候，可以在定义中使用泛型
+ 注意
    + 把T放在impl的后面，表示在类型T上实现方法
        + 例如impl&lt;T&gt;Point&lt;T&gt;
    + 只针对具体类型实现方法(其余类型没实现方法)
        + 例如implPoint&lt;f32&gt;
```rust

struct Point<T>{
    x:T,
    y:T,
}
impl <T> Point<T>{
    fn x (&self)->&T{
        &self.x
    }
}
impl  Point<i32>{
    fn x1 (&self)->&i32{
        &self.x
    }
}
fn main(){
    let p = Point{x:3,y:2};
    println!("the value of p.x is {}",p.x());
}
```

+ struct里的泛型参数可以和方法的泛型参数不同
```rust

struct Point<T,W>{
    x:T,
    y:W,
}
impl <T,U> Point<T,U>{
    fn mixup <V,W>(self,other:Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

fn main(){
    let p1 = Point{x:3,y:2};
    let p2 = Point{x:'c',y:'a'};
    let p3 = p1.mixup(p2);

    println!("{} ,{}",p3.x,p3.y);
}
```
泛型代码的性能
+ 使用泛型的代码和使用具体类型的代码运行速度是一样的
+ 单态化(monomorphization)
```rust
fn main(){
    let integer =Some(3);//实例为Option_i32
    let float =Some(3.0);//实例为Option_f64
}
```
Trait
+ Trait告诉rust编译器
    + 某种类型具有哪些并且可以与其它类型共享的功能
+ trait：抽象的定义共享行为
+ trait bounds(约束)：泛型类型参数指定为实现了特定行为的类型
+ trait与其他语言的接口(interface)类似，但有些区别

定义一个trait
+ trait的定义：把方法的签名放在一起，来定义实现某种目的所需的一组行为
    + 关键字：trait
    + 只有方法签名，没有具体的实现
    + trait可以有多个方法：每个方法签名各占一行，以；结尾
    + 实现该trait的类型必须提供具体的方法实现

```rust

pub trait Summary {
    fn summarize(&self)->String;
}
fn main(){

}
```
在类型上实现trait
+ 与为类型实现方法相似
+ 不同之处
    + impl xxx for Tweet{..}
    + 在impl块里，需要对Trait里的方法签名进行具体的实现

```rust
//lib.rs
pub trait Summary {
    fn summarize(&self)->String;
}
 pub struct NewsArticle{
    pub headline:String,
    pub author:String,
    pub location:String,
    pub content:String,
}

impl Summary for NewsArticle{
    fn summarize(&self) ->String {
        format!("{} by {} ({})",self.headline,self.author,self.location)
    }
}


  pub struct  Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub rettweet:bool,
}
impl Summary for Tweet{
    fn summarize(&self) ->String {
        format!("{} {}" ,self.content,self.username)
    }
}
//mian.rs
use demo::Summary;//name = "demo"在Cargo.toml中定义包名也是lib.rs和main.rs的入口
use demo::Tweet;

fn main(){
 let tweet = Tweet{
     username:String::from("wanglian"),
     content:String::from("CN OI ,BEST OI"),
     reply:false,
     rettweet:false,
 };
println!("{}",tweet.summarize());
}

```
实现trait的约束
+ 可以在某个类型上实现trait的前提条件是
    + 这个类型或这个trait是在本地crate里定义的 
+ 无法为外部类型来实现外部的trait
    + 这个限制是程序属性的一部分(也就是一致性)
    + 更具体地说是孤儿原则：之所以这样命名是因为父类型不存在
    + 此规则确保其他人的代码不能破坏你的代码 ，反之亦然
    + 如果没有这个规则，两个crate可以为同一类型实现同一个trait，rust就不知道应该使用那个实现

 默认实现
 ```rust
 pub trait Summary {
    fn summarize(&self)->String{
        format!("{} {}" ,self.content,self.username)
    }
}
```
+ 默认实现的方法可以调用trait的其他方法，即使这些方法没有默认实现
```rust
pub trait Summary {
    fn summarize_author()->String;
    fn summarize(&self)->String{
        format!("{} {}" ,self.content,self.summarize_author())
    }
}
```
+ 注意：无法从方法的重写实现里面调用默认的实现 

trait作为参数
+ impl trait 语法：适用于简单情况
```rust
use std::future::pending;

pub trait Summary {
    fn summarize(&self)->String;
}
 pub struct NewsArticle{
    pub headline:String,
    pub author:String,
    pub location:String,
    pub content:String,
}

impl Summary for NewsArticle{
    fn summarize(&self) ->String {
        format!("{} by {} ({})",self.headline,self.author,self.location)
    }
}


  pub struct  Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub rettweet:bool,
}
impl Summary for Tweet{
    fn summarize(&self) ->String {
        format!("{} {}" ,self.content,self.username)
    }
}

pub fn notify (item:impl Summary){
    println!("breaking news!{}",item.summarize());
}
```
trait bound语法：可用于复杂情况 

```rust
pub fn notify <T:Summary>(item:T){
    println!("breaking news!{}",item.summarize());
}
```
+ impl trait 语法是trait bound的语法糖
+ 使用+ 指定多个trait bound

```rust
pub fn notify (item:impl Summary + Display){
    println!("breaking news!{}",item.summarize());
}
pub fn notify1 <T:Summary+Display>(item:T){
    println!("breaking news!{}",item.summarize());
}
```

+ trait bound 使用where子句
    + 在方法签名后指定wher子句
```rust
pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: T) {
    println!("{}", a.summarize());
}

pub fn notify1<T, U>(a: T, b: U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
    println!("{}", a.summarize());
}
```

使用trait作为返回类型
+  impl trait语法
pub fn notify()->impl Sumary{...}
+ 注意：impl trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错

使用trait bound的例子
+ 例子：使用trait bound修复largest
```rust
fn largest<T:PartialOrd+ Copy>(list:&[T])->T {
    let mut largest  = list[0];//没有实现copy trait
    for &item in list.iter(){
    if largest < item{//std::cmp::PartialOrd
        largest = item
    }
}
largest
}

fn main(){

    let number = vec![1,2,3,4,56,7,88,9];
    let result = largest(&number);
    println!("{}",result);
}
```
```rust
// fn largest<T:PartialOrd+ Clone>(list:&[T])->T {
//     let mut largest  = list[0].clone();
//     for item in list.iter(){//数据没有实现copy trait无法移动
//     if largest < *item{//解引用进行比较
//         largest = item.clone()
//     }
// }
// largest
// }

fn largest<T:PartialOrd+ Clone>(list:&[T])->&T {
    let mut largest  = &list[0];
    for item in list.iter(){
    if largest < item{
        largest = item;
    }
}
largest
}

fn main(){

    let number = vec![String::from("hello"),String::from("world")];
    let result = largest(&number);
    println!("{}",result);
}
```
 使用trait bound有条件的实现方法
 + 在特定的泛型参数的impl块上使用trait bound，我们可以有条件的为实现了特定trait的类型来实现方法
 ```rust
 use std::fmt::Display;


pub struct Pair<T>{
    x:T,
    y:T,
} 

impl<T> Pair<T>{
    fn set(x:T,y:T)->Self{
       Self{ x,y}
    }
}
impl<T:Display+PartialOrd> Pair<T>{
    fn cmp(&self){
        if self.x>=self.y{
            println!("{}",self.x)
        }else{
            println!("{}",self.y)
        }
    }
}

fn main(){

}
```
+ 也可以为实现了其他trait的任意类型有条件的实现某个trait
+ 为满足trait bound的所有类型上实现trait叫做覆盖实现(blanket implmentations)
```rust
fn main(){
    let s = 3.to_string();//覆盖实现凡是实现了Display的trait的都可以使用to_string方法
}
```
生命周期
+ rust的每个引用都有自己的生命周期
+ 生命周期：引用保持有效的作用域
+ 大多数情况下： 生命周期是隐式的、可被推断的
+ 当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期

生命周期-避免悬垂引用(dangling reference)
+ 生命周期的主要目标： 避免悬垂引用(dangling reference)

```rust
fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;//x的引用到这里之后离开此作用域失效
        }
        println!("{}",r);
    }
}
```
借用检查器
+ rust编译器的借用检查器：比较作用域来判断所有的借用是否合法

```rust
fn main() {
    let x = 5;
    let  r = &x;

    println!("{}", r);
}

```

函数中的泛型生命周期
```rust
fn main(){
    let s1 = String::from("hello world");
    let s2 = "xsds";
    let result = longest(s1.as_str(),s2);
    println!("{}",result);
}

fn longest<'a>(x:&'a str,y:& 'a str)->&'a str{//'a 就是泛型生命周期
    if(x.len()>y.len()){
        x
    }
    else{
        y
    }
}
```
声明周期的标注语法
+ 生命周期的标注不会改变引用的生命周期长度
+ 当指定了泛型生命周期函数，函数可以接收带有任何生命周期的引用
+ 生命周期的标注：描述了多个引用的生命周期间的关系，但不影响生命周期

生命周期标注- 语法
+ 生命周期参数名
    + 以 '开头
    + 通常小写且非常短
    + 很多人使用'a

+ 生命周期标注的位置
    + 在引用的&符号后
    + 使用空格将标注和引用类型分开

生命周期标注-例子
+ &i32 //一个引用
+ & 'a i32 //带有显示生命周期的引用
+ & 'a mut i32 //带有显示生命周期的可变引用

+ 单个生命周期标注本身没有意义

函数签名中的生命周期标注
+ 泛型生命周期参数声明在：函数名和参数列表之间的&lt;&gt;里
+ 生命周期'a的实际生命周期是：x和y两个生命周期中较小的那个

fn longest&lt;'a &gt;(x:&'a str,y:& 'a str)->&'a str{..}

深入理解生命周期
+ 指定生命周期参数的方式依赖于函数所做的事
+ 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配
+ 如果返回的引用没有指向任何参数，那么他只能引用函数内创建的值
    + 这就是悬垂引用：该值在函数结束时就走出了作用域

```rust
fn main(){
    let s1 = String::from("hello world");
    let s2 = "xsds";
    let result = longest(s1.as_str(),s2);
    println!("{}",result);
}

fn longest<'a>(x:&'a str,y:& 'a str)->&'a str{
    let result  =String::from("hello");
    result.as_str()//在result走出作用域后就已经被清理了
}
/************************************************************/
fn main(){
    let s1 = String::from("hello world");
    let s2 = "xsds";
    let result = longest(s1.as_str(),s2);
    println!("{}",result);
}

fn longest<'a>(x:&'a str,y:& 'a str)->String{
    let result  =String::from("hello");
    result//交给函数的调用者去清理内存
}

```
struct定义中的生命周期标注
+ struct里可包括
    + 自持有的类型
    + 引用：需要在每个引用上添加生命周期标注

```rust
struct Pairname<'a> {
    pair : & 'a str,
}

fn main(){
    let noval = String::from("Call me ishmael. Some years ago ....");
    let first_sentence = noval.split('.')
        .next()
        .expect("could not found a '.'");
    
        let i  = Pairname{
            pair:first_sentence//pair的生命周期比实例的存活时间长
        }; 
}
```
生命周期的省略
+ 我们知道
    + 每个引用都有生命周期
    + 需要为使用生命周期的函数或struct指定生命周期参数

```rust
fn first_word(s:&str)->&str{
    let byte  = s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
```
生命周期省略规则
+ rust引用分析中所编入的生命周期省略规则
    + 这些规则无需开发者来遵守
    + 他们是一些特殊情况，由编译器来考虑
    + 如果你的代码符合这些情况，那么就无须显示标注生命周期

+ 生命周期省略规则不会提供完整的推断
    + 如果应用规则后，引用的生命周期任然模糊不清->编译错误
    + 解决办法：添加生命周期标注，表明引用间的相互关系

输入、输出生命周期
+ 生命周期在：
    + 函数/方法的参数：输入生命周期
    + 函数/方法的返回值：输出生命周期

生命周期省略的三个原则
+ 编译器使用3个规则在没有显示标注生命周期的情况下，来确定引用的生命周期
    + 规则一应用于输入生命周期
    + 规则二、三应用于输出生命周期
    + 如果编译器应用完三个规则之后，任然无法确定生命周期的引用->报错
    + 这些规则适用于fn定义和impl块

+ 规则一：每个引用类型的参数都有自己的生命周期
+ 规则二：如果只有一个输入生命周期函数，那么该生命周期被赋给所有的输出生命周期参数
+ 规则三：如果有多个输入生命周期参数，但其中一个是&self或&mut self(是方法)，那么self的生命周期会被赋给所有的输出生命周期参数

生命周期省略的三个规则-例子
+ 假设我们是编译器
```rust
fn first_word(s:&str)->&str{}
fn first_word<'a>;(s:&'astr)->&str{}//规则一
fn first_word<'a>;(s:&'astr)->&'a str{}//规则二

fn longest(x:str,y:str)->&str{}
fn longest<'a,'b>(x:&'a str,y:& 'b str)->str{}//规则一之后无法推断报错
```

方法定义中的生命周期标注
+ 在struct上使用生命周期标注，语法和泛型参数的语法一样
+ 在哪声明和使用生命周期参数，依赖于：
    + 生命周期是否和字段、方法的参数或返回值有关

+ struct字段的生命周期名
    + 在impl块后声明
    + 在struct名后使用
    + 这些生命周期是struct类型的一部分

+ impl块内的方法签名中：
    + 引用必须绑定在struct字段引用的生命周期，或者引用是独立的也可以
    + 生命周期省略规则经常使得方法中的生命周期标注不是必须的

```rust
struct imports<'a> {
    part:&'a str,
}

impl<'a> imports<'a>{
    fn level(&self)->i32{
        3
    }
    fn anounce(&self,anucement:i32)->&str{
        println!("anounce is  {}",anucement);
        self.part
    }
}


```
静态生命周期
+ 'static是一个特殊的生命周期：整个程序的持续时间
    + 例如：所有的字符串都拥有'static生命周期
    + let s：&'static str = " i have a static lifetime. "

+ 为引用指定'static生命周期前要三思
    + 是否需要引用在程序整个生命周期内都存活

泛型参数类型、trait bound、生命周期

```rust
use std::fmt::Display;

fn longest_str<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

fn main() {}
```

## 编写自动化测试
测试(函数)
+ 测试
    + 函数/方法的
    + 验证非测试代码的功能是否和预期一致

+ 测试函数体(通常)执行的3个操作
    + 准备数据/状态(arrange)
    + 运行被测试的代码(active)
    + 断言(assert)结果

解剖测试函数
+ 测试函数需要使用test属性(attribute)进行标注
    + attribute就是rust代码的元数据
    + 在函数上加#[test]可把函数变成测试函数

运行测试
+ 使用cargo test命令运行所有测试函数
    + rust会构建一个test runner可执行
    + 它会运行标注了test的函数，并报告其运行是否成功

+ 当使用cargo创建library项目的时候，会生成一test module,里面有一个test函数
    + 你可以添加任意数量的test module或函数

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
测试失败
+ 测试函数panic就表示失败
+ 每个测试运行在一个新线程
+ 当主线程看见某个测试线程挂掉，那个测试标记为失败

```rust
 #[test]
    fn oanic(){
        panic!("make it working");
    }
```
断言(assert)
使用assert!宏检查测试结果
+  assert!宏，来自标准库，来确定某个状态是否为true
    + true ：测试通过
    + false : 调用panic!。测试失败
```rust
 struct Rectangle{
     width:i32,
     length:i32,
 }
 impl Rectangle{
     fn can_hold(&self,other:&Rectangle) -> bool{
         if self.width > other.width && self.length > other.length{
             return true;
         }
         false
     }
 }

 #[cfg(test)]
 
 mod tests {
     use super::*;
     #[test]
     fn can_hold_rectangle() {
         let  rect = Rectangle{
             width:6,
             length:7,
         };
         let other = Rectangle{
             width:3,
             length:5,
         };
         assert!(rect.can_hold(&other));
         
     }
     #[test]
     fn can_hold_rectangle1() {
        let  rect = Rectangle{
            width:6,
            length:7,
        };
        let other = Rectangle{
            width:3,
            length:5,
        };
        assert!(!other.can_hold(&rect));
        
    }
 }
```

使用assert_eq!和assert_ne!测试相等性
+ 都来自标准库
+ 判断两个参数是否相等或不等
+ 实际上，他们使用的就是==和！= 运算符
+ 断言失败：自动打印参数的值
    + 使用debug格式打印参数
        + 要求参数实现了PatialEq和Debug Traits(所有的基本类型和标准库里大部分类型都实现了)

```rust
pub fn add_two(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(4, add_two(3,2));//测试失败可用assert_ne!
    }
}
```
自定义错误信息
添加自定义错误信息
+ 可以向assert!、assert_eq!和assert_ne!添加可选的自定义信息
    + 这些自定义信息和失败信息都会被打印出来
    + assert: 第一个参数必填，自定义消息作为第二个参数
    + assert_eq!和assert_ne!：前两个参数必填，自定义消息作为第三个参数
    + 自定义消息参数会被传递format!宏，可以使用{}占位符

```rust
pub fn greeting(name :&str) ->String {
    format!("hello {}",name)
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test(){
        let result = greeting("world");
        assert!(result.contains("world1"),
        "greeting didn't contain world,value was '{}'",
        result);
    }
}
```

should_painc检查恐慌
验证错误处理的情况
+ 测试除了验证的返回值是否正确，还需验证代码是否如预期的处理了发生错误的情况
+ 可验证代码在特定情况下是否发生了panic
+ should_painc属性(attribute)
    + 函数panic：测试通过
    + 函数没有panic：测试失败
```rust
struct Guesse{
    value:u32,
}

impl Guesse {
    fn normalize(value: u32) -> Guesse {
        if value< 1||value>100{
            panic!("the value is out of range");
        }
        Guesse{value}
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    #[should_panic]
    fn test(){
        Guesse::normalize(200);//value大于100程序发生恐慌测试失败
    }
}
```

让should_panic更精确
+ 为should_panic属性添加一个可选的expected参数：
    + 将检查失败信息中是否包含所指定的文字

```rust
struct Guesse{
    value:u32,
}

impl Guesse {
    fn normalize(value: u32) -> Guesse {
        if value>100{
            panic!("the value is out of range");
        }else{
            panic!("the value is less than 100")
        }
        Guesse{value}
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    #[should_panic(expected = "the value is out of range")]//将准确的打印panic的错误信息
    fn test(){
        Guesse::normalize(200);
    }
}
```
在测试中使用Result&lt;T,E&gt;枚举
+ 无需panic，可使用Result&lt;T,E&gt;作为返回类型作为测试
    + 返回Ok：测试通过
    + 返回Err ：测试失败
+ 注意：不要在使用Result&lt;T,E&gt;编写的测试上下标注#[should_panic]

```rust
pub fn add(a: u32) -> u32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test()->Result<(),String>{
        if  add(2)==4{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
控制测试如何进行
+ 改变cargo test的行为：添加命令行参数
+ 默认行为
    + 并行运行
    + 所有测试
    + 捕获(不显示)所有输出，使读取与测试结果相关的输出更容易

+ 命令行参数
    + 针对cargo test的参数：紧跟cargo test之后
    + 针对 测试可执行程序：放在 -- 之后

+ cargo test --help
+ cargo test -- --help

并行运行测试
+  运行多个测试：默认使用多个线程并行运行
    + 运行快

+ 确保测试之间
    + 不会互相依赖
    + 不依赖某个共享状态(环境、工作目录、环境变量等等)

+ --test-threads参数
    + 传递给二进制文件
    + 不想以并行方式运行测试，或相对线程数进行细粒度控制
    + 可以使用--test-threads参数，后边跟着线程数量
    + 例如 cargo test -- --test-thread=1

显示函数输出
+ 默认，如果测试通过，rust的test库会捕获所有打印到标准输出的内容
+ 例如，如果被测试代码中使用到了println!
    + 如果测试通过不会打印println!的内容
    + 如果测试失败就会看到println!的内容

如果想要在成功的测试中看到打印的内容：--show-output

按名称运行测试的子集
+ 选择运行的测试：将测试的名称(一个或多个)作为cargo test的参数

+ 运行单个测试指定测试名
```rust
pub fn add(a: u32) -> u32 {
    println!("i got value is {}",a);
    10

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){
        let s = add(1);
        assert_eq!(1,s)
    }
    #[test]
    fn test1(){
        let s = add(10);
        assert_eq!(10,s)//参数可以 cargo test test1以运行指定的测试
    }
}
```
+ 运行多个测试：指定测试名的一部分(模块名也可以)

```rust
pub fn add(a: u32) -> u32 {
    println!("i got value is {}",a);
    10

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1(){
        let s = add(1);
        assert_eq!(1,s)
    }
    #[test]
    fn test_21(){
        let s = add(10);
        assert_eq!(10,s)
    }//观察上面两个例子的函数名测试的时候指定参数为cargo test test会运行以上两个测试
}
```
忽略测试
+ ignore属性(attribute)

+ 运行被忽略(ignore)的测试
    + cargo test --ignored

```rust
#[cfg(test)]
mod test {
    #[test]
    fn test(){
        assert_eq!(4,3+1);
    }
    #[test]
    #[ignore]
    fn test1(){
        assert_eq!(4,3+1+1+1+1+1+1+1+1+1+1+1+1+1+1);//cargo test -- --ignored运行测试
    }
}
```
测试的组织

测试的分类
+ rust对测试的分类
    + 集成测试
    + 单元测试
+ 单元测试
    + 小、专注
    + 一次对一个模块进行隔离测试
    + 可测试private接口

+ 集成测试
    + 在库外部。和其他外部代码一样使用你的代码
    + 只能使用public接口
    + 可能在每个测试中使用到多个模块

单元测试
#[cfg(test)]标注
+ test模块上的#[cfg(test)]标注
    + 只有运行cargo test才编译和运行代码
    + 运行cargo build则不会

+ 集成测试在不同的目录，他不需要#[cfg(test)]标注

+ cfg：configuration(配置)
    + 告诉rust下面的条目只有在特定的配置选项下才会被包含
    + 配置选项test：由rust提供，用来编译和运行测试
        + 只有cargo test才会编译代码，包括模块中的helper函数和#[test]标注的函数

```rust
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}{
        assert_eq!(4,3+1);
    }
}
```
测试私有函数
+ rust允许测试私有函数
```rust
fn add_two(a:u32,b:u32) ->u32{
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4,add_two(2,2));
    }
}
```

集成测试
+ 在rust里，继承测试完全位于被测试库的外部
+ 目录：是测试被测试库的多个部分是否能正确的一起工作
+ 继承测试的覆盖率很重要

test 目录
+ 创建集成测试：tests目录
+ tests目录下的每个测试文件都是一个单独的crate
    + 需要将被测试库导入

+ 无需标注#[cfg(test)]，test目录被特殊对待
    + 只有cargo test，才会编译tests目录下的文件

运行指定的集成测试
+ 运行一个特定的集成测试：cargo test函数名
+ 运行某个测试文件内的所有测试：cargo test --test文件名
例如 cargo test --test integration_test(文件名)

集成测试中的字模块
+ test目录下每个文件被编译成单独的crate
    + 这些文件不共享行为

针对 binary crate的集成测试
+ 如果项目是binary crate，只含有src/mian.rs没有src/lib.rs
    + 不能在tests目录下创建集成测试
    + 无法把main.rs的函数导入作用域
+ 只有library crate才能暴露函数给其他crate用
+ binary crate意味着独立运行

项目实例：
- 命令行程序 

1. 接受命令行参数
```rust
use std::env;//引入命令行参数按照惯例引入父级模块

fn main() {
    let arg :Vec<String> = env::args().collect();//指定集合类型
    println!("{:?}",arg);//打印

    let query =&arg[1];//第二个参数、第一个是cargo run
    let filename = &arg[2];//第三个参数
    println!("{:?}",query);
    println!("{:?}",filename);
}      

```
2. 读取文件
```rust
use std::env;
use std::fs;//使用fs模块
fn main() {
    let arg :Vec<String> = env::args().collect();
   

    let query =&arg[1];
    let filename = &arg[2];
    println!("search for{:?}",query);
    println!("In fail{:?}",filename);

    let contents = fs::read_to_string(filename)//读取文件成功返回OK()失败打印信息
    .expect("Failed to open poem.txt");

    println!("with text:\n {}",contents);
}
```
3. 代码重构

二进制程序关注点分离的指导性原则
+ 将程序拆分main.rs和lib.rs将业务逻辑放入lib.rs
+ 当命令行解析参数较少时，将他放在main.rs也行
+ 当命令行解析逻辑变复杂时，需要将他从main.rs提取到lib.rs

经过上诉拆分留在main的功能有
+ 使用参数值调用命令行解析逻辑
+ 调用其他配置
+ 调用lib.rs中的run函数
+ 处理run函数可能出现的逻辑

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let context = fs::read_to_string(config.filename).expect("fail to read config file");
    println!("content is {:?}", context);
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(arg: &[String]) -> Config {
        let query = arg[1].clone();
        let filename = arg[2].clone();//使用clone()方法低效但是不用管理生命周期
        Config { query, filename }
    }
}

```

错误处理
```rust
use std::env;
use std::fs;
use std::process;//引入程序退出的模块
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {//闭包unwrap成功返回Config失败返回err
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let context = fs::read_to_string(config.filename).expect("fail to read config file");
    println!("content is {:?}", context);
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(arg: &[String]) -> Result<Config, &'static str> {//枚举值处理不是不可恢复错误
        if arg.len() < 3 {
            return Err("not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Ok(Config { query, filename })
    }
}

```
模块化
```rust
////////lib.rs
use std::fs;
use std::error::Error;
pub fn  run(config: Config)->Result<(),Box<dyn Error>> {
    let context = fs::read_to_string(config.filename)?;
    println!("content is {:?}", context);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(arg: &[String]) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Ok(Config { query, filename })
    }
}

****************************************************************
//main.rs
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) =minigrep::run(config) {
        println!("application: {}", e);
        process::exit(1);
    }
}
```

4. 使用TDD(测试驱动开发)开发库功能
测试驱动开发
TDD ( Test-Driven Development)
+ 编写一个会失败的测试，运行该测试，确保他是按照预期的原因失败
+ 编写或修改刚好足够的代码，让新测试通过
+ 重构刚刚添加或修改的代码，确保测试会始终通过
+ 返回步骤一，继续

使用环境变量
```rust
use std::fs;
use std::error::Error;
use std::env;
pub fn  run(config: Config)->Result<(),Box<dyn Error>> {
    let context = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query,&context)
    }else{
        search_insensitive(&config.query,&context)
    };
    for line in result{
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(arg: &[String]) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//添加环境变量选择需要的输出一个区分大小写一个不区分
        Ok(Config { query, filename ,case_sensitive})
    }
}

pub fn search<'a>(query: &str,filename:&'a str) ->Vec<&'a str> {
    let mut result = Vec::new();
    for line in filename.lines(){
        if line.contains(query){
            result.push(line)
        }
    }
    result
} 

pub fn search_insensitive<'a>(query: &str,filename:&'a str)->Vec<&'a str> {//不区分
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in filename.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line)
        }
    }
    result
}
#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_config(){
        let query = "duct";
        let filename = "\
rust:
safe,fast ,productive.
pick tree"; 
        assert_eq!(vec!["safe,fast ,productive."],search(query, filename));
    }
    #[test]
    fn test_insencitive(){
        let query = "Duct";
        let filename = "\
rust:
safe,fast ,productive.
pick tree.
duct."; 
        assert_eq!(vec!["safe,fast ,productive.","duct."],search_insensitive(query, filename));
    }
}

*******************************
//main.rs
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) =minigrep::run(config) {
        eprintln!("application: {}", e);
        process::exit(1);
    }
}

```
+ cmd命令行记得这样添加环境变量$Env::CASE_INSENSITIVE='1'
+ 去除操作则是这样 Remove-item -Path Env:CASE_INSENSITIVE

标准输出 vs 标准错误
+ 标准输出 ： stdout
+ println!

+ 标准错误： stderr
+ eprintln!

## 函数式语言特性
+ 迭代器和闭包

+ 闭包
    + 使用闭包创建抽象行为

+ 什么是闭包(closure)
+ 闭包：可以捕获其所在环境的匿名函数
+ 闭包
    + 什么是匿名函数
    + 保存为变量、作为参数
    + 可以在一个地方创建闭包、然后在另一个上下文中调用闭包来完成运算
    + 可从其定义的作用域捕获值

例子 - 生成自定义运动计划的程序
+ 算法的逻辑并不是重点，重点是算法中的计算过程需要几秒钟时间
+ 目标：不让用户发生不必要的等待
    + 仅在必要时调用该算法
    + 只调用一次

```rust
use std::thread;
use std::time::Duration;

 fn  main(){
    fn generate_workout(itensity:u32,random_number:u32){
        let expensive_result = |num|{//为了减少耗时函数的调用
            println!("work time");
            thread::sleep(Duration::from_secs(2));
            num
        };
        if itensity < 25{
            println!("today do {} pushups",expensive_result(itensity));//这里调用了两次可以提取变量也可以使用闭包特性等待改进
            println!("next do {} situps",expensive_result(itensity));
        }else{
            if random_number==3{
                println!("take a break today !remenber to stay hydrated");
            }else{
                println!("today run for {} minutes!",expensive_result(itensity));
            }
        }
    }
 }
 ```
 闭包的类型推断和标注
 + 闭包不要求标注参数和返回值类型
 + 闭包通常很短小，只在狭窄的上下文中工作，编译器通常能推断出类型
 + 可以手动添加类型标注
 ```rust
 let expensice_closure = |num:u32|->u32{};
 ```
 函数和闭包的定义语法
```rust
  let add_one_v1 (x:u32)  ->u32{x+1}
  let add_one_v1 = |x:u32|->u32{x+1};//标注返回值类型和参数类型
  let add_one_v1 = |x|         {x+1};//不标注
  let add_one_v1 = |x|          x+1;//只有一个表达式不用花括号
  ```

闭包的类型推断
  + 注意：闭包的定义最终只会作为参数/返回值推断出唯一具体的类型
  
```rust
fn main() {
    let example_closure = |x| x;
    let s  = example_closure(String::from("hello world"));
    //let s  = example_closure(5);//根据上一次的推导类型闭包的参数和返回值类型是String类型
}
```
 使用泛型参数和Fn Trait来存储闭包
   + 创建一个struct，他持有闭包及其调用结果
     + 只会在需要结果是才执行该闭包
     + 可缓存结果
   + 这个模式通常叫做记忆化(memoization)或延迟计算(lazy evaluation)

如何让struct持有闭包
+ struct的定义需要知道所有字段的类型
  + 需要指明闭包的类型
+ 每个闭包实例都有自己唯一的匿名函数，即使两个闭包签名完全一样
+ 所以需要使用：泛型和Trait Bound

Fn Trait
+ Fn Trait由标准库提供
+ 所有的闭包都至少实现了一下trait之一
    + Fn
    + FnMut
    + FnOnce
```rust
use std::thread;
use std::time::Duration;

struct Cache<T> 
where 
T:Fn(u32)->u32,
{
    caculate:T,
    value:Option<u32>,   
}
impl<T> Cache<T>
where 
T:Fn(u32)->u32,
{
    fn new(caculate:T)->Cache<T>{
        Cache {
            caculate,
            value:None,
        }
    }
    fn value(&mut self,arg:u32) -> u32{
        match self.value {
            Some(val) => val,
            None => {
                let v  = (self.caculate)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}

fn  main(){
   let itensity = 26;
   let random_number = 4;
   generate_workout(itensity, random_number);
 }
 fn generate_workout(itensity:u32,random_number:u32){
    let mut expensive_result = Cache::new(|num|{
        println!("work time");
        thread::sleep(Duration::from_secs(2));
        num});
    if itensity < 25{
        println!("today do {} pushups",expensive_result.value(itensity));
        println!("next do {} situps",expensive_result.value(itensity));
    }else{
        if random_number==3{
            println!("take a break today !remenber to stay hydrated");
        }else{
            println!("today run for {} minutes!",expensive_result.value(itensity));
        }
    }
}
```
使用缓存器(Cacher)实现的限制
1. Cacher实例假定针对不同的参数arg,value方法总会得到同样的值
    + 可以使用HashMap代替单个值
      + Key：arg参数
      + value：执行闭包的结果
2. 只能接受一个u32类型的参数和u32类型的返回值

 ```rust
 struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    caculate: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where 
T: Fn(u32) -> u32,
{
    fn new(caculate:T)->Cache<T>{
        Cache {
            caculate,
            value: None,
        }
    }
    fn value(&mut self,arg:u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.caculate)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works(){
        let mut result = Cache::new(|a| a );

        let v1 = result.value(1);//第一次传入参数的值决定了得到的闭包值只会是1
        let v2 = result.value(2);
        assert_eq!(2,v2);
    }
}
```
 闭包可以捕获它们所在的环境
 + 闭包可以访问定义他的作用域内的变量，而普通函数则不能
 + 会产生内存开销

```rust
fn main() {
    let x = 4;
    let colusure = |z| z==x;
    // fn  equal(c:i32)->bool{
    //     c==x
    // };函数不能捕获变量
    let y =4;
    assert!(equal(y));

}
```
闭包从所在环境捕获值的方式
+ 与函数获得参数的三种方式一样
  + 取得所有权：FnOnce
  + 可变借用：FnMut
  + 不可变借用: Fn

+ 创建闭包时，通过闭包对环境值的使用。rust推断出具体使用那个trait：
    + 所有的闭包都实现了FnOnce
    + 没有移动捕获变量的实现了FnMut
    + 无需可变访问捕获变量的闭包实现了Fn
  
move关键字
+ 在参数列表前使用了move关键字，可以强制闭包取得他的所有的环境值的所有权
    + 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最为有用

```rust
fn main() {
    let x = vec![1,2,3];
    let colusure = move |z| z==x;
    
    println!("x is{}",x);//x的所有权已被借用

}
```
最佳实践
+ 当指定Fn trait bound之一时，首先用Fn，基于闭包体里的情况，如果需要FnOnce或FnMut编译器会再告诉你

什么是迭代器
+ 迭代器模式：对一系列项执行某些任务
+ 迭代器负责
    + 遍历每个项
    +  确定序列(遍历)何时完成
+ rust的迭代器
  + 懒惰的：除非调用消耗迭代器的方法，否则迭代器本身没有任何效果

```rust
fn main() {
    let v = vec![1,2,3];
    let v1_iter = v.iter();//迭代器是懒惰不使用的话不会产生任何作用
    for val in v1_iter {
        println!(" Got: {}", val);
    }
}
```

iterator trait
+ 所有迭代器都实现了iterator trait
+ iterator trait定义于标准库，定义大致如下
+ pub trait Iterator{
 type Item；
 fn next(&mut self)->Option<Self::Item>;

 } 
 + type Item和Self::Item定义了与此该trait关联的类型
    + 实现Iterator trait需要你定义一个Item类型，它用于next方法的返回类型(迭代器的返回类型)

+ iterator trait仅要求实现一个方法：next
+ next
  + 每次返回迭代器的一项
  + 返回结果包裹在Some里
  + 迭代结束，返回None
+ 可直接在迭代器上调用next方法
  
```rust
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let v = vec![1, 2,4];
        let mut v1_iter = v.iter();//可变迭代器因为调用了next方法
        assert_eq!(v1_iter.next(),Some(&1));
        assert_eq!(v1_iter.next(),Some(&2));
        assert_eq!(v1_iter.next(),Some(&4));
    }
}
```
几个迭代方法
+ iter方法：在不可变引用上创建迭代器
+ into_iter方法：创建的迭代器会获得所有权
+ iter_mut方法：迭代可变的引用

消耗迭代器的方法
+ 在标准库，iterator trait有一些带默认实现的方法
+ 其中有一些方法会调用next方法
  + 实现iterator trait是必须实现next方法的原因之一
  
+ 调用next的方法叫做消耗性适配器
  + 因为调用他们会吧迭代器消耗尽

+ 例如：sum方法(就会耗尽迭代器)
  + 取得迭代器的所有权
  + 通过反复调用next，遍历所有元素
  + 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
  
```rust
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let v = vec![1, 2,4];
        let mut v1_iter = v.iter();
        let total:i32 =v1_iter.sum();
        assert_eq!(total,7);

    }
}
```
产生其他迭代器的方法
+ 定义在Iterator trait上的另外一些方法叫做“迭代器适配器”
  + 把迭代器转换为不同种类的迭代器
+  可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性较高
+  例如:map
   +  接受一个闭包，闭包作用于每个元素
   +  产生一个新的迭代器
+  collect方法：消耗型适配器，把结果收集到一个集合类型中

```rust
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let v = vec![1, 2, 3];
        let v1 :Vec<_> = v.iter().map(|x| x+2).collect();
        assert_eq!(v1,vec![3,4,5]);

    }
}
```rust
#[derive(Debug,PartialEq)]

struct Shoe{
    size:u32,
    style:String,
}

fn in_my_shoe(shoe:Vec<Shoe>,size:u32) -> Vec<Shoe>{
    shoe.into_iter().filter(|x|x.size ==size).collect()//获得所有权的迭代器filter收集鞋号一样的元素
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter_by_size(){
        let shoe =  vec![
            Shoe{size:10,style:String::from("nike"),},
            Shoe{size:12,style:String::from("adidas"),},
            Shoe{size:10,style:String::from("lining"),},
        ];
         let size  = in_my_shoe(shoe,10);
         assert_eq!(size,
        vec![ Shoe{size:10,style:String::from("nike"),},
        Shoe{size:10,style:String::from("lining"),},
        ]
        );
    }

}
```

+ 使用iterator trait来创建自定义迭代器
  + 实现next方法
```rust
struct Counter {
    count :u32,
}
impl Counter{
    fn new() -> Counter{
        Counter{count:0}
    }
}
impl  Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count<5{
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}

#[cfg(test)] 
mod tests {
    use crate::Counter;

    #[test]
    fn it_works(){
        let mut count = Counter::new();
        assert_eq!(count.next(),Some(1));
        assert_eq!(count.next(),Some(2));
        assert_eq!(count.next(),Some(3));
        assert_eq!(count.next(),Some(4));
        assert_eq!(count.next(),Some(5));
        assert_eq!(count.next(),None);
    }

    #[test]
    fn using_counter(){
        let sum:u32  = Counter::new()
        .zip(Counter::new().skip(1))//跳过有一个单位即(2,3,4,5)
        .map(|(a,b)| a*b)//闭包求两个的乘积
        .filter(|x| x % 3== 0)//过滤只取出可以整除3的结果
        .sum()//求和
        ;
        assert_eq!(18,sum);
    }
}

```

```rust
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("application: {}", e);
        process::exit(1);
    }
}
****************************************************************************
use std::fs;
use std::error::Error;
use std::env;
pub fn  run(config: Config)->Result<(),Box<dyn Error>> {
    let context = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query,&context)
    }else{
        search_insensitive(&config.query,&context)
    };
    for line in result{
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut arg: env::Args) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("not enough arguments");
        }
        arg.next();
        let query = match arg.next() {
            Some(arg) =>arg,
            None => return Err("parse error"),
        };
        let filename = match arg.next() {
            Some(arg) =>arg,
            None => return Err("parse error"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//添加环境变量选择需要的输出一个区分大小写一个不区分
        Ok(Config { query, filename ,case_sensitive})
    }
}

pub fn search<'a>(query: &str,filename:&'a str) ->Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in filename.lines(){
    //     if line.contains(query){
    //         result.push(line)
    //     }
    // }
    // result
    filename.lines()//采用闭包更加简洁
    .filter(|line| line.contains(query))
    .collect()
} 

pub fn search_insensitive<'a>(query: &str,filename:&'a str)->Vec<&'a str> {//不区分
    let query = query.to_lowercase();
    // for line in filename.lines(){
    //     if line.to_lowercase().contains(&query){
    //         result.push(line)
    //     }
    // }
    // result
    filename.lines()
    .filter(|line|line.to_lowercase().contains(&query))
    .collect()
}
#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_config(){
        let query = "duct";
        let filename = "\
rust:
safe,fast ,productive.
pick tree"; 
        assert_eq!(vec!["safe,fast ,productive."],search(query, filename));
    }
    #[test]
    fn test_insencitive(){
        let query = "Duct";
        let filename = "\
rust:
safe,fast ,productive.
pick tree.
duct."; 
        assert_eq!(vec!["safe,fast ,productive.","duct."],search_insensitive(query, filename));
    }
}
```

性能比较
循环 vs 迭代器

零开销抽象
Zero-COST Abstraction
+ 使用抽象时不会引入额外的开销

## cargo crate.io

relea profile
+ release profile:
  + 是预定义的
  + 可自定义：可使用不同的配置，对代码编译有更多的控制

+  每个profile的配置都独立于其他的profile

+ cargo 主要的两个profile
  + dev profile ：适用于开发，cargo build
  + release profile ：适用于发布： cargo build -release

+ 自定义profile
+ 针对每个profile,Cargo都提供了默认的配置
+ 如果想自定义xxxx profile的配置
  + 可以在Cargo.toml里添加[profile.xxxx]区域，在里面覆盖默认的配置子集

```
[package]
name = "minigrep"
version = "0.1.0"
authors = ["eulerf <2331249337@qq.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[profile.release]//自定义文件配置
opt-level = "3"//编译优化水平
[profile.dev]
opt-level = "1"
```

发布crate到crates.io
crate.io
+ 可以通过发布包来共享你的代码
+ crate的注册表在https://crates.io/
  + 他会分发已注册的包的源代码
  + 主要托管开源的代码

文档注释
+ 文档注释：用于生成文档
  + 生成HTML文档
  + 显示公共API的文档注释：如何使用API
  + 使用///
  + 支持MarkDown
  + 放置在被说明条目之前

生成HTML文档的命令
+ cargo doc
  + 它会运行rustdoc工具(Rust安装包自带)
  + 把它生成的HTML文档放在target/doc目录下

+ cargo doc -open
  + 构建当前crate的文档(也包含crate依赖项的文档)
  + 在浏览器打开文档

常用章节
+ #Example
+ 其他常用章节
  + Panics:函数可能发生panic的场景
  + Errors：如果函数返回Result，描述可能的错误种类，以及可导致错误的条件
  + Safety:如果函数处于unsafe调用，就应该解释unsafe的原因，以及调用者确保的使用前提
  
文档注释作为测试
+ 实例代码块的附加值
  + 运行cargo test：将把文档注释中的实例代码作为测试来运行



为包含注释的项添加文档注释
+ 符号：//!
+ 这类注释通常用crate和模块
  + crate root(按惯例src/lib.rs)
  + 一个模块内，将crate或模块作为一个整体进行记录

使用pub use导出方便使用的公共API
+ 问题：crate的程序结构在开发时对于开发者很合理，但对于它的使用者不够方便
  + 开发者会把程序结构分为很多层，使用者想找到这种深层结构中的某个类型很费劲

+ 例如
  + my_crate::some_module::anthor_module::UseFulType
  + my_crate::UseFulType

+ 解决办法
  + 不需要重新组织内部代码结构
  + 使用pub use ：可以重新导出，创建一个与内部私有结构不同的对外公共结构

```rust
lib.rs
//! # art crate
//!
//! 'art crate' for modeling artistic concepts

pub use self::kind::Color;
pub use self::kind::SecondaryColor;
pub use self::utils::mix;

pub mod kind{
/// the primary colors according to the ryb color model

pub enum Color{
    RED,
    YELLOW,
    BLUE,
}


/// the secondary colors according to the ryb color model
pub enum SecondaryColor{
    ORANGE,
    GREEN,
    PURPLE,
}
}

pub mod utils{
    use crate::kind::*;
    pub fn mix(c1:Color,c2:SecondaryColor)->SecondaryColor{
        SecondaryColor::GREEN
    }
}
****************************************************************
main.rs

use art::Color;
use art::SecondaryColor;
use art::mix;
fn main() {
  let  color = Color::BLUE;
  let color1 = SecondaryColor::GREEN;
  mix(color,color1);
}
```
创建并设置Crates.io账号
+ 发布crate前，需要在crates.io创建账号并获得API token
+ 运行命令：cargo login [你的API token]//理解为一串哈希值
  +  通知cargo 你的API token存储在本地~/.cargo/credential
+ API token 可以在https://crates.io/进行撤销
  
为新的crate添加元数据
+ 在发布crate之前，需要在Cargo.toml的[package]区域为crate添加一些元数据
  + crate需要唯一的名称：name
  + description 一两句话即可，会出现在crate搜索的结果里
  + license需提供许可证标识值(可到http://spdx.org/license/查找)
    + 可指定多个license：用OR
  + version
  + author
+ 发布： cargo publish命令( 记得切换到默认的官方网站 crm  default)

+ 发布到Crates.io
+ crate一旦发布就是永久性的，该版本无法覆盖，代码无法删除
  + 目的：依赖于该版本的项目可以继续正常工作

发布已存在crate的新版本
+ 修改crate后，需要先修改Cargo.toml 里面的version值，再重新发布
+ 参照http://semver.org/来使用你的语义版本
+ 再次执行cargo publish进行发布
  
使用cargo yank从Crates.io撤回版本
+ 不可以删除crate之前的版本
+  但可以防止其他项目把它作为新的依赖：yank(撤回)一个crate版本
   +  防止新项目依赖该版本
   +  已经存在项目可继续将其作为依赖(并可下载)
+ yank意味着
  + 所有已经产生Cargo.lock的项目都不会中断
  + 任何将来生成的Cargo.lock文件都不会使用被yank的版本
+ 命令
  + yank一个版本(不会删除任何代码)： cargo yank --vers 1.0.1
  + 取消yank cargo yank --vers 1.0.1 --undo


+ Cargo 工作空间(Workspace)
+ cargo工作空间：帮助管理多个相互关联且需要协同开发的crate
+ cargo工作空间是一套共享同一个Cargo.lock和输出文件夹的包

创建工作空间
+ 有多种方式组建工作空间 例：一个二级制crate两个库crate
  + 二进制crate：main函数依赖于其他两个库crate
  + 其中一个库crate提供add_one函数
  + 另外一个库crate提供add_two函数

在工作空间中依赖外部crate
+ 工作空间只有一个Cargo.lock文件，在工作空间的顶层目录
  + 保证工作空间所有crate使用的依赖的版本都相同
  + 工作空间所有crate相互兼容
  
为工作空间添加测试用例

从CRATES.IO安装二进制crate
+ 命令： cargo install
+ 来源：https://creates.io
+ 限制 ：只能安装具有二进制目标(binary target)的crate
+ 二进制目标binary target:是一个可运行程序
  + 由拥有src/main.rs或其他被指定为二进制文件的crate生成
+ 通常：README里有关于crate的描述
  + 拥有library target
  + 拥有binary target
  + 两者兼备

cargo install 
+ cargo install 安装的二进制程序存放在根目录的bin文件夹
+ 如果你用rustup安装的rust，没有任何自定义配置，那么二进制存放目录是$HOME/.cargo/bin/
  + 要确保该目录在环境变量&PATH中

使用自定义命令拓展cargo
+ cargo被设计成可以使用子命令来扩展
+ 例：如果$PATH的某个二进制是cargo-something，你可以向子命令一样运行
  + cargo something
+ 类似这样的自定义命令可以通过该命令列出：cargo --list

+ 优点：可使用cargo install来安装扩展，像内置工具一样来运行

## 智能指针

相关概念
+ 指针： 一个变量在内存中包含的是一个地址(指向其他数据)
+ rust中最常见的指针就是引用
+ 引用：
  + 使用*
  + 借用它指向的值
  + 没有其余的开销
  + 最常见的指针类型

智能指针
+ 智能指针是这样的一些数据结构
  + 行为和指针相似
  + 有额外的元数据和功能

引用计数(reference counting)智能指针类型
+ 通过记录所有者的数量，使一份数据被多个所有者同时持有
+ 并在没有任何所有者时自动清理数据

引用和智能指针的其他不同
+ 引用： 只借用数据
+ 智能指针 ： 很多时候都拥有它所指向的数据

智能指针的例子
+ String和Vec&lt;T&gt;
+ 都拥有一片内存区域，且允许用户对其操作
+ 还拥有元数据(例如容量等)
+ 提供额外的功能或保障(String保障其数据是合法的UTF-8编码)
  
智能指针的实现
+ 智能指针通常使用struct实现，并且实现了
  + Deref和Drop这两个trait

+ Deref trait：允许智能指针struct的实例像引用一样使用
+ Drop trait： 允许你自定义当智能指针实例走出作用域时的代码
  
本章内容
+ 介绍标准库中常见的智能指针
  + Box&lt;T&gt;在heap内存上分配值
  + Rc&lt;T&gt;启用多重所有权的引用计数类型
  + Ref&lt;T&gt;和RefMut&lt;T&gt; 通过RefCell&lt;T&gt;访问：在运行时而不是在编译时强制借用规则的类型

+ 此外
  + 内部可变模式(interior mutability pattern)：不可变类型暴露出可修改其内部值的API
  + 引用循环(reference cycles ):他们如何导致泄露内存，以及如何防止其发生

使用Box&lt;T&gt;来指向Heap上的数据 

Box&lt;T&gt;
+ Box&lt;T&gt;是最简单的智能指针
  + 允许你在heap上存储数据(而不是stack)
  + stack上是指向heap数据的指针
  + 没有性能开销
  + 没有其他额外性能
  + 实现了Deref Trait 和Drop trait 

Box&lt;T&gt;的常用场景
+ 在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道他的确切大小
+ 当你有大量数据时，想移交所有权，但需要确保在操作时数据不会被复制
+ 使用某个值时，你只关心他是否实现了特定的trait而不关心他的具体类型
  
使用Box&lt;T&gt;在heap上存储数据
```rust
fn main() {
  let b = Box::new(5);
  println!("b = {}",b);//b走出作用域时清理在satck上的指针和在heap上的数据
}
```

使用Box赋能递归类型

+ 在编译时，rust需要知道一个类型所占空间大小
+ 而递归类型的大小无法在编译时确定

+ 但Box类型的大小确定
+ 在递归类型中使用Box就可解决问题
+ 函数语言中的 Cons List

关于Cons List
+ Cons List是来自Lisp语言的一种数据结构
+ Cons List里每个成员由两个元素组成
  + 当前项的值
  + 下一个元素

+ Cons List里最后一个成员只包含一个Nil值，没有下一个元素

Cons List并不是rust的常用集合
+ 通常情况下，Vec&lt;T&gt;是更好的选择
+ 创建一个Cons List
```rust
use crate::list::{Cons,Nil};

fn main() {
  let list = Cons(1,Cons(2,Cons(3,Cons(4,Cons(5,Nil)))));
}

enum List{
  Cons(i32,List),
  Nil
}     
```
上诉例子不能编译通过

+ rust如何确定为枚举分配的空间大小
```rust
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Wtite(String),
    Change (i32,i32,i32),
}
```
存储枚举时存储最大变体的空间大小

使用Box来获得确定大小的递归类型
+ Box&lt;T&gt;是一个指针，rust知道它需要多少空间，因为
  + 指针的大小不会基于他指向的数据的大小变化而变化

```rust
use crate::List::{Cons, Nil};

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
  Cons(i32, Box<List>),
  Nil,
}
```
Box&lt;T&gt;
  + 只提供了间接存储和heap内存分配的功能
  + 没有其他额外功能
  + 没有性能开销
  + 适用于需要间接存储的场景，例如Cons List
  + 实现了Deref Trait 和Drop trait

Deref Trait 
+ 实现Deref Trait使我们可以自定义解引用运算符*的行为
+ 通过实现Deref，智能指针可像常规引用一样来处理
  
解引用运算符
+ 常规引用是一种指针
```rust
fn main(){
  let x  = 5;
  let y = &5;
  assert_eq!(5,x);
  assert_eq!(5,*y);
}
```
把Box&lt;T&gt;当做引用
+ Box&lt;T&gt;可以代替上例中的引用

```rust
fn main(){
  let x  = 5;
  let y = Box::new(x);
  assert_eq!(5,x);
  assert_eq!(5,*y);
}
```

定义自己的智能指针
+ Box&lt;T&gt;被定义成拥有一个元素的tuple struct
+ MyBox&lt;T&gt;

实现Deref Trait
+ 标准库中的Deref Trait,要求我们实现一个deref方法
  + 该方法借用self
  + 返回一个指向内部数据的引用

```rust
use std::ops::Deref;
struct MyBox<T>(T);
 impl<T> MyBox<T> {
   fn new(x:T)->MyBox<T> {
     MyBox(x)
   }
 }

 impl<T> Deref for MyBox<T>{
   type Target =  T ;
   fn deref(&self) -> &T{
     &self.0
   }
 }


fn main(){
  let x  = 5;
  let y = MyBox::new(x);
  assert_eq!(5,x);
  assert_eq!(5,*y);
  //*(y.deref())
}
```
函数和方法的隐式解引用转化(Deref Coercion)
+ 隐式解引用转化(Deref Coercion)是为函数和方法提供的一种便捷特性
+ 假设T实现了Deref Trait：
  + Deref Coercion可以把T的引用转换为T经过Deref操作后生成的引用

+ 当把某类型的引用传递给函数或方法时，但他的类型与定义的参数类型不匹配
  + Deref Coercion 就会自动发生
  + 编译器会对deref进行一系列调用，来把它转为所需的参数类型
    + 在编译时完成，没有额外性能开销

```rust
use std::ops::Deref;
struct MyBox<T>(T);
 impl<T> MyBox<T> {
   fn new(x:T)->MyBox<T> {
     MyBox(x)
   }
 }

 impl<T> Deref for MyBox<T>{
   type Target =  T ;
   fn deref(&self) -> &T{
     &self.0
   }
 }
fn hello(name :&str){
  println!("name, {}",name);
}
fn main(){
  let m  = MyBox::new(String::from("hello world"));
  hello(&m);
  hello(&(*m)[..]);
  //&m &Mybox<String>
  //deref &String
  //deref &str
  hello("rust");
}
```
解引用与可变性
+ 可使用DerefMut trait重载可变引用的*运算符
+ 在类型trait在下列情况发生时，rust会执行deref coercion
  + 当T：Deref&lt;Target=U&gt;允许&T转换为&U
  + 当T：DerefMut&lt;Target=U&gt;允许&mut T转换为&mut U
  + 当T：Deref&lt;Target=U&gt;允许&mut T转换为&U


Drop trait
+ 实现Drop trait ，可以让我们自定义当值将要离开作用域发生的动作
  + 例如，文件、网络资源释放等
  + 任何类型都可以实现Drop trait

+ Drop trait只要求你实现drop方法
  + 参数对self的可变引用

+ Drop trait在预导入模块

```rust
struct Cluster{
  data:String,
}

impl Drop for Cluster {
  fn drop(&mut self) {
    println!("drop {}", self.data);
  }
}
fn main() {
  let c1 = Cluster{data:String::from("hello")};
  let c1 = Cluster{data:String::from("hello rust")};//drop打印的顺序相反
}
```
使用std::mem::drop来提前drop值
+ 很难直接禁用自动的drop功能，也没必要
  + Drop trait的目的就是进行自动的的释放处理逻辑

+ rust不允许手动调用Drop trait的drop方法

+ 但可以调用标准库的std::mem::drop函数来提前drop值

```rust
struct Cluster{
  data:String,
}

impl Drop for Cluster {
  fn drop(&mut self) {
    println!("drop {}", self.data);
  }
}
fn main() {
  let c1 = Cluster{data:String::from("hello")};
  drop(c1);//按顺序打印而且不会二次释放
  let c2 = Cluster{data:String::from("hello rust")};
}
```
Rc&lt;T&gt;引用计数智能指针
+ 有时，一个值会有多个所有者
+ 为了支持多重所有权Rc&lt;T&gt;
  + reference counting
  + 追踪所有到值的引用
  + 0个引用：该值可以被清理掉

Rc&lt;T&gt;使用场景
+ 需要在heap上分配数据，这些数据被程序的多个部分读取(只读)，但在编译时无法确定那个部分最后使用完这些数据
+ Rc&lt;T&gt;只能用于单线程场景

+ Rc&lt;T&gt;不在预导入模块(prelude)
+ Rc::clone(&a)函数：增加引用计数
+ Rc::strong_count(&a):获得引用计数
  + 还有 Rc::weak_count函数

+ 例子两个list共享另一个list的所有权

```rust
enum List {
  Cons(i32, Rc<List>),
  Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
  //a.clone()数据的深度拷贝
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));
}
enum List {
  Cons(i32, Rc<List>),
  Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
  println!("{}", Rc::strong_count(&a));//1

  let b = Cons(3, Rc::clone(&a));
  println!("{}", Rc::strong_count(&a));//2
  {
    let c = Cons(4, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));//3离开作用域计数减一
  }
  println!("{}", Rc::strong_count(&a));//2
}

```
Rc&lt;T&gt;
+ Rc&lt;T&gt;通过不可变引用，使你可以在程序不同部分之间共享可读数据
+ 但是，如何允许数据变化

内部可变性(interio mutability)
+ 内部可变性是rust的设计模式之一
+ 它允许你在只持有不可变引用的前提下对数据进行修改
  + 数据结构中使用了unsafe代码来绕过rust正常的可变性和借用规则

RefCell&lt;T&gt;
+ 与Rc&lt;T&gt;不同RefCell&lt;T&gt;类型代表了其持有数据的唯一所有权

RefCell&lt;T&gt;与Box&lt;T&gt;的区别
Box&lt;T&gt;
+ 编译阶段强制代码遵守借用规则
+ 否则出现错误

RefCell&lt;T&gt;
+ 只会在运行时检查借用规则
+ 否则触发panic

借用规则在不同阶段进行检查的比较
编译阶段
+ 尽早暴露问题
+ 没有任何运行时开销
+ 对大多数场景是最佳选择
+ 是rust的默认行为

运行时
+ 问题暴露后延后甚至到生产环境
+ 因借用计数产生些许性能损失
+ 实现某些特定的内存安全场景(不可变环境中修改自身数据)

RefCell&lt;T&gt;
+ 只能用于单线程的场景

 选择Box&lt;T&gt;、Rc&lt;T&gt;、RefCell&lt;T&gt;的依据
Box&lt;T&gt;
+ 同一数据的所有者：一个
+ 可变性借用检查：可变、不可变借用(编译时检查)

Rc&lt;T&gt;
+ 同一数据的所有者：多个
+ 可变性借用检查：不可变借用(编译时检查)

RefCell&lt;T&gt;
+ 同一数据的所有者：一个
+ 可变性借用检查：可变、不可变借用(运行时检查)

+ 其中即便RefCell&lt;T&gt;本身不可变，但仍能修改其中存储的值

内部可变性：可变的借用一个不可变的值
```rust
fn main() {
    let x = 5;
    let y = &mut x;//报错
}
```
使用RefCell&lt;T&gt;在运行时记录借用信息
两个方法
+ borro方法
  + 返回智能指针Ref&lt;T&gt;它实现了Deref
+ borrow_mut方法
  + 返回智能指针RefMut&lt;T&gt;，它实现了Deref

使用RefCell&lt;T&gt;在运行时记录借用信息
+  RefCell&lt;T&gt;会记录当前存在多少个活跃的Ref&lt;T&gt;和RefMut&lt;T&gt;智能指针
   +  每次调用borrow：不可变借用计数加一
   +  任何一个Ref&lt;T&gt;的值离开作用域被释放时：不可变借用计数减一
   +  每次调用borrow_mut：可变借用计数加一
   +  任何一个RefMut&lt;T&gt;的值离开作用域被释放时：可变借用计数减一

+ 以此技术来维护借用检查规则
  + 任何一个给定时间里，只允许拥有多个不可变借用或一个可变借用

将Rc&lt;T&gt;和RefCell&lt;T&gt;结合使用来实现一个拥有多重所有权的可变数据
```rust


#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));
  *value.borrow_mut() +=10;
  println!("a = {:?}",a);
  println!("b = {:?}",b);
  println!("c = {:?}",c);

}
```
其他可实现内部可变性的类型
+ Cell&lt;T&gt;：通过复制来访问数据
+ Mutex&lt;T&gt;：用于实现跨线程情形下的内部可变性模式
  
循环引用可以导致内存泄露
 rust可能发生内存泄露
 + rust的内存安全机制可以保证很难发生内存泄露，但不是不可能
 + 例如使用Rc&lt;T&gt;和RefCell&lt;T&gt;就可能创造出循环引用，从而发生内存泄露
   + 每个项的引用数量不会变成0，值也不会被处理掉

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}
fn main() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("{}", Rc::strong_count(&a));
  println!("{:?}", a.tail());
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("{}", Rc::strong_count(&a));
  println!("{}", Rc::strong_count(&b));
  println!("{:?}", b.tail());
  if let Some(link) = a.tail(){
    *link.borrow_mut() = Rc::clone(&b);
  }
  println!("{}", Rc::strong_count(&b));
  println!("{}", Rc::strong_count(&a));
  //println!("{:?}", a.tail());循环链表内存泄露
}
```
防止内存泄露的解决办法

+ 依靠开发者保证，不能依靠rust
+ 重新组织数据结构：一些引用表达所有权，一些引用不表达所有权
  + 循环引用中的一部分具有所有权关系，另一部分不涉及所有权关系
  + 而只有所有权关系才影响值的清理

防止循环引用把Rc&lt;T&gt;换成Weak&lt;T&gt;
+ Rc::clone为Rc&lt;T&gt;实例的strong_count加一，Rc&lt;T&gt;的实例只有在strong_count为0的时候才会被清理
+ Rc&lt;T&gt;实例通过调用Rc::downgrade方法可以创建值的Weak Reference(弱引用)
  + 返回类型是Weak&lt;T&gt;
  + 调用Rc::downgrade会为weak_count加一

+ Rc&lt;T&gt;使用weak_count来追踪存在多少Weak&lt;T&gt;
+ weak_count不为0并不影响Rc&lt;T&gt;实例的清理

Strong vs Weak
+ strong reference(强引用)是关于如何分享Rc&lt;T&gt;实例的所有权
+ weak refere(弱引用)并不表达上述意思
+ 使用weak reference 并不会创建循环引用
  +  当strong reference数量为0的时候weak reference会自动断开

+ 在使用weak&lt;T&gt;前，需保证他指向的值仍然存在
  + 在weak&lt;T&gt;实例上调用upgrade方法，返回Option&lt;Rc&lt;T&gt;&gt;
```rust
use std::cell::RefCell;
use std::rc::{Rc,Weak};
#[derive(Debug)]
struct Node{
  value: i32,
  parent:RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}


fn main(){
 let leaf  = Rc::new(Node { 
   value: 5, 
   parent: RefCell::new(Weak::new()),
   children:RefCell::new(vec![])});
   println!(" leaf parent ={:?}",leaf.parent.borrow().upgrade());
 let branch = Rc::new(Node { 
   value:3,
   parent:RefCell::new(Weak::new()),
   children:RefCell::new(vec![Rc::clone(&leaf)])});
   *leaf.parent.borrow_mut()= Rc::downgrade(&branch);
   println!(" leaf parent ={:?}",leaf.parent.borrow().upgrade());
  }
```
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
  let leaf = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });
  println!(" leaf parent ={:?}", leaf.parent.borrow().upgrade());
  println!(
    "leaf strong  = {},weak  = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
  {
    let branch = Rc::new(Node {
      value: 3,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
      "branch strong  = {},weak  = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch)
    );
    println!(
      "leaf strong  = {},weak  = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf)
    );
  }
  println!(" leaf parent ={:?}", leaf.parent.borrow().upgrade());
  println!(
    "leaf strong  = {},weak  = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
}
/********输出如下*********/
 leaf parent =None
leaf strong  = 1,weak  = 0  
branch strong  = 1,weak  = 1
leaf strong  = 2,weak  = 0  
 leaf parent =None
leaf strong  = 1,weak  = 0  
```

## 无畏并发
并发
+ Concurrent：程序的不同部分之间独立的执行
+ Parallel：程序的不同部分同时运行

+ rust无畏并发：允许你编写没有细微bug的代码，并在不引入新bug的情况下易于重构
+ 注意本课程的并发泛指concurrent和parallel

使用线程同时运行代码

进程和线程
+ 在大部分os里，代码运行在进程(process)中，os同时管理多个进程
+ 在你的程序里，各独立部分可以同时运行，运行这些独立部分的就是线程(thread)
+ 多线程运行
  + 提升性能表现
  + 增加复杂性：无法保证各线程的执行顺序

多线程可导致的问题
+ 竞争状态，线程以不一致的顺序访问数据或资源
+ 死锁，两个线程彼此等待对方使用完所持有的资源，线程无法继续
+ 只在某些情况下发生的bug，很难可靠的复制现象和修复

实现线程的方式的方式
+ 通过调用os的api来创建线程1:1模型
  + 需要较小的运行时

+ 语言自己实现的线程(绿色线程)：M:N模型
  + 需要更大的运行时

+ rust：需要权衡运行时支持
+ rust标准库仅提供1:1模型的线程

通过spawn创建新线程
+ 通过thread::spawn函数可以创建新线程
  + 参数：一个闭包(在新线程里运行的代码)

+ thread::sleep会导致当前的线程暂停执行
```rust
use std::thread;
use std::time::Duration;
fn main() {
  thread::spawn(||{
  for i in 1..10{
    println!("hi thread is {}", i);
    thread::sleep(Duration::from_millis(1));
  }
  });
for i in 1..5{
  println!("main thread is {}", i);
    thread::sleep(Duration::from_millis(1));
}
}
```

+ 通过join handle来等待所有线程的完成
+ thread::spawn函数的返回值是joinHandle
+ joinHandle持有值的所有权
  + 调用其join方法，可以等待对应的其他线程的完成

+ join方法：调用handle的join方法会阻止当前运行的线程的运行，直到handle所表示的这些线程终结

```rust
use std::thread;
use std::time::Duration;
fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi thread is {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
   //handle.join().unwrap();分线程执行完才执行主线程
  for i in 1..5 {
    println!("main thread is {}", i);
    thread::sleep(Duration::from_millis(1));
  }
  handle.join().unwrap();//主线程执行完才执行分线程
}
```
使用move闭包
+ move闭包通常和thread::spawn函数一起使用，他允许你使用其他线程的数据
+ 创建线程时，把值的所有权从一个线程转移到另一个线程
```rust
use std::thread;

fn main() {
  let v = vec![1, 2, 3, 4];
  let handle = thread::spawn(move|| {
    println!("thread {:?}", v);
  });
  //drop(v);所有权已经移动到闭包区域里面
  handle.join();
}
```

消息传递
+ 一种很流行且能保证安全并发的技术就是：消息传递
  + 线程(或Actor)通过彼此发送信息(数据)来进行通信

+ Go语言的名言:不要用共享内存来通信，要用通信来共享内存
+  rust：channel(标准库提供)

Channel
+ Channel包含：发送端、接收端
+ 调用发送端的方法，发送数据
+ 接收端会检查和接收到达的数据
+ 如果发送端、接收端中任意一端，那么Channel就关闭了

创建Channel
+ 使用mpac::Channel函数来创建Channel
  + mpsc表示multiple producer，single consumer(多个生产者、一个消费者)
  + 返回一个tuple里面元素分别是发送端、接收端

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let values = String::from("hello risc-v ");
    tx.send(values).unwrap();
  });
  let result = rx.recv().unwrap();
  println!("{}", result);
}
```
发送端的send方法
+ 参数：想要发送的数据
+ 返回Result&lt;T,E&gt;
  + 如果有问题(例如接收端已经被丢弃)，就返回一个错误

接收端的方法
+ recv方法:阻止当前线程执行，直到Channel中有值被送来
  + 一旦有值收到，就返回Result&lt;T,E&gt;
  + 当发送端关闭，就会收到一个错误

+ try_recv方法
  + 立即返回Result&lt;T,E&gt;
    + 有数据到达：就返回Ok，里面包含着数据
    + 否则，返回错误
  + 通常会使用循环调用来检查try_recv的结果  

Channel和所有权转移
+ 所有权在消息传递中非常重要：能帮你编写安全、并发的代码

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let values = String::from("hello risc-v ");
    tx.send(values).unwrap();
    println!("{}", values);//所有权已经移交了
  });
  let result = rx.recv().unwrap();
  println!("{}", result);
}
```
发送多个值，看到接收者在等待
```rust
use std::string::ParseError;
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx,rx) = mpsc::channel();
  let tx1  = mpsc::Sender::clone(&tx);
  thread::spawn(move ||{
    let values = vec![String::from("HI"),
    String::from("world"),
    String::from("risc-v")] ;
    for i in values {
      tx.send(i).unwrap();
    }
  });
  for resive in rx{
    println!("{}",resive);
  }
}
```
通过克隆创建多个发送者
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let values = vec![
      String::from("1"),
      String::from("HI"),
      String::from("world"),
      String::from("risc-v"),
    ];
    for i in values {
      tx1.send(i).unwrap();
      thread::sleep(Duration::from_millis(200));
    }
  });
  thread::spawn(move || {
    let values = vec![
      String::from("HI"),
      String::from("world"),
      String::from("risc-v"),
    ];
    for i in values {
      tx.send(i).unwrap();
      thread::sleep(Duration::from_millis(200));
    }
  });
  for resive in rx {
    println!("{}", resive);//结果交替出现
  }
}
```

共享状态的并发

使用共享来实现并发
+ Go语言的名言：不要用共享内存来通信，要用通信来共享内存
+ rust支持通过共享状态来实现并发
+ channel类似单所有权：一旦将值的所有权移至channel就无法使用它了
+ 共享内存并发类似多所有权：多个线程可以同时访问同一块内存

使用Mutex来每次只允许一个线程访问数据
+ Mutex是mutual exclusion(互斥锁)的简写
+ 在同一时刻，Mutex只允许一个线程来访问某些数据
+ 想要访问数据
  + 线程必须首先获得互斥锁(lock)
    + lock数据结构是mutex的一部分，他能跟踪谁对数据独占访问权
  + mutex通常被描述为：通过锁定系统来保护它所持有的数据

Mutex的两条规则
+ 在使用数据之前，必须尝试获取锁(lock)
+ 使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁

Mutex&lt;T&gt;的API
+ 通过Mutex::new(数据)来创建Mutex&lt;T&gt;
  + Mutex&lt;T&gt;是一个智能指针

+ 访问数据之前，通过lock方法获取锁
  + 会阻塞当前线程
  + lock可能会失败
  + 返回的是MutexGurad(智能指针，实现Deref和Drop)

```rust
use std::sync::Mutex;

fn main() {
  let num = Mutex::new(5);
  {
    let mut  numnew = num.lock().unwrap();
    *numnew += 1;
  }
  println!("mutex is {:?} ",num);
}
```

多线程共享mutex
多线程的多重所有权
使用Arc&lt;T&gt;来进行原子计数
+ Arc&lt;T&gt;与Rc&lt;T&gt;类似，它可以用于并发场景
  + A：atomatic原子的

+ 为什么所有的基础类型都不是原子的，为什么标准库类型不默认使用Arc&lt;T&gt;？
  + 需要牺牲性能为代价
+ Arc&lt;T&gt;和Rc&lt;T&gt;的API是相同的
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let count = Arc::new(Mutex::new(0));

  let mut vec = vec![];

  for _ in 0..10 {
    let count = Arc::clone(&count);
    let handle = thread::spawn(move || {
      let mut num = count.lock().unwrap();
      *num += 1;
    });
    vec.push(handle);
  }
  for  handle in vec{
    handle.join().unwrap();
  }
  let result = *count.lock().unwrap();
  println!("{}", result);
}
```
RefCell&lt;T&gt;/Rc&lt;T&gt; VS Mutex&lt;T&gt;/Arc&lt;T&gt;
+ Mutex&lt;T&gt;提供了内部可变性。和cell家族一样
+ 我们使用RefCell&lt;T&gt;来改变Rc&lt;T&gt;里面的内容
+ 我们使用Mutex&lt;T&gt;来改变Arc&lt;T&gt;里面的内容
+ 注意：Mutex&lt;T&gt;有死锁的风险

通过Send和Sync trait来拓展并发

Send和Sync trait
+ rust语言的并发特性特别少，目前讲的并发特性都来自标准库(而不是语言本身)
+ 无需局限于标准库的并发，可以自己实现并发
+ 但在rust语言中有两个并发概念
  + std::marker::Sync和std::marker::Send这两个trait

Send :允许线程间转移所有权
+ 实现Send trait 的类型可在线程间转移所有权
+ rust中几乎所有的类型都实现了Send
  + 但Rc&lt;T&gt;没有实现Send，它只用于单线程情景
+ 任何完全由Send类型组成的类型也被标记为Send
+ 除了原始指针之外，几乎所有的基础类型都是Send

Sync：允许从多线程访问
+ 实现Sync的类型可以安全的被多个线程引用
+ 也就是说:如果T是Sync，那么&T就是Send
  + 引用可以被安全的送往另一个线程
+ 基础类型都是Sync
+ 完全由Sync类型组成的类型也是Sync
  + 但Rc&lt;T&gt;不是Sync的
  + RefCell&lt;T&gt;和Cell&lt;T&gt;家族也不是Sync的
  + 而Mutex&lt;T&gt;是Sync的

手动来实现Send和Sync是不安全的

## rust的面向对象编程特性

rust是面向对象编程语言嘛？
+ rust受到多种编程范式的影响，包括面向对象
+ 面向对象通常包含以下特征:命名对象、封装、继承

对象包含数据和行为
+ 设计模式四人帮在设计模式中给面向对象的定义
  + 面向对象的程序由对象组成
  + 对象包装了数据和操作这些数据的过程，这些过程通常被称为方法或操作

+ 基于此定义：rust是面向对象的
  + struct、enum包含数据
  + impl块为之提供了方法
  + 但带有方法的struct、enum并没有被称为对象

封装
+ 封装：调用对象外部的代码无法直接访问对象内部的细节实现细节，唯一可以与对象进行交互的方法就是通过他公开的API
+ rust：pub关键字

继承
+ 继承：使对象可以沿用另外一个对象的数据和行为，且无需重复定义相关的代码
+ rust没有继承
+ 使用继承的原因
  + 代码复用
    + rust ：默认trait方法来进行代码共享
  + 多态
    + rust：泛型和trait约束(限定参数化多态bounded parametric)

+ 很多新语言都不使用继承作为内置的程序设计方案了

使用trait对象来存储不同类型的值
为共有行为定义一个trait
+ rust避免将rust或enum称为对象，因为他们与impl快是分开的
+ trait对象有些类似与其他语言中的对象
  + 它们某种程度上组合了数据和行为
+ trait对象与传统对象不同的地方
  + 无法为trait对象添加数据
+ trait对象被专门用于抽象某些共有行为，它没其他语言中的对象那么通用

trait对象执行的是动态派发
+ 将trait约束用于泛型时，rust编译器会执行单态化
  + 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现

+ 通过单态化生成的代码会执行静态派发(static dispatch),在编译过程中确定调用的具体方法

+ 动态派发(dynamic dispatch)
  + 无法在编译过程中确定你调用的究竟是哪一种方法
  + 编译器会产生额外的代码以便在运行时找出希望调用的方法

+ 使用trait对象会执行动态派发
  + 产生运行时开销
  + 阻止编译器内联方法代码，使得部分优化操作无法进行

trait对象必须保证对象安全
+ 只能把满足对象安全(object-safe)的trait转化为trait对象
+ rust采用了一系列规则来判断某个对象是否安全，只需要记住两条
  + 方法的返回类型不是self
  + 方法不包含任何泛型参数

实现面向对象的设计模式
状态模式
+ 状态模式(state pattern)是一种面向对象设计模式
  + 一个值拥有的内部状态由数个状态对象(state object表达而成),而值的行为则随着内部状态的改变而改变

+ 使用状态模式意味着：  
  + 业务需求变化时，不需要修改持有状态的值的代码，或者使用这个值的代码
  + 只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象

状态模式的取舍权衡
+ 缺点
  + 某些状态之间是相互耦合的
  + 需要重复实现一些逻辑代码

将状态和行为编码为类型
+ 将状态编码为不同的类型
  + rust类型检查系统会通过编译时错误来阻止用户使用无效的状态

总结
+ rust不仅能够实现面向对象的设计模式，还可以支持更多的模式
+ 例如：将状态和行为编码为类型
+ 面向对象的经典模式并不总是rust编程实践中的最佳选择，因为rust具有所有权等其他面向对象语言没有的特性 

## 模式匹配
模式
+ 模式是rust中的一种特殊语法，用于匹配复杂和简单类型的结构
+ 将模式与匹配表达式和其他构造结合使用，可以更好的控制程序的控制流
+ 模式由以下元素(的一些组合)组成
  + 字面值
  + 结构的数组、struct、enum、和tuple
  + 变量
  + 通配符
  + 占位符
+ 想要使用模式，需要将与某个值进行比较
  + 如果模式匹配，就可以在代码中使用这个值的相应部分

用到模式的地方
match的Arm
+ match表达式的要求
  + 详尽(包含所有的可能性)

+ 一个特殊的模式：_(下划线)
  + 他会匹配任何东西
  + 不会绑定到变量
  + 通常用于match的最后一个Arm:或用于忽略某些值

 条件if let表达式
 + if let表达式主要是作为一种简短的方式来等价只有一个匹配项的match
 + if let可选的可以拥有else，包括：
   + else if
   + else if let
 + 但，if let不会检查穷尽性

while let条件循环
+ 只要模式继续满足匹配的条件，那他会允许while循环一直运行
```rust
fn main() {
  let mut stack = vec![];
  stack.push(1);
  stack.push(2);
  stack.push(3);
  while let Some(x) = stack.pop() {
    println!("{}", x);
  }
}
```
for循环
+ for循环是rust中最常见的值
+ for循环中，模式就是紧随for关键字后的值
```rust
fn main() {
  let v = vec![1, 2, 3, 4, 5];
  for (index,value) in v.iter().enumerate() {
    println!("{},{}",index,value);
  }
}  
```
let 语句
+ let语句也是模式
  
函数参数
+ 函数参数也可以是模式
```rust
fn point_coordinate(&(x,y):&(i32,i32)) {
  println!("{},{}",x,y);
}

fn main() {
  let point = (3,5);
  point_coordinate(&point);
}
```

可辩驳性：模式是否会无法匹配
模式的两种模式
+ 模式有两种形式：可辩驳的、无可辩驳
+ 能匹配任何可能传递值的模式：无可辩驳的
  + 例如 let x = 5
+ 对于某些可能的值，无法进行匹配的模式：可辩驳的
  + 例如if let Some(x) = a_value

+ 函数参数、let语句、for循环只接受无可辩驳的模式
+ if let和while let接受可辩驳和无可辩驳的模式

模式语法
匹配字面值
+ 模式可以直接匹配字面值
```rust
fn main() {
  let x = 1; 
  match x {
    1=>println!("one"),
    2=>println!("two"),
    3=>println!("three"),
    4=>println!("four"),
    _=>println!("anything"),
  }
}
```
匹配命名变量
+ 命名的变量是可匹配任何值的去可辩驳模式
```rust
fn main() {
  let x = Some(5);
  let  y= 10; 
  match x{
    Some(50) => println!("one"),
    Some(y)=>println!("{}", y),
    _=>println!("{:?},{:?}",x,y),
  }
  println!("{:?},{:?}",x,y);
}
```
多重模式
+ 在match表达式中，使用| 语法(就是或的意思)，可以匹配多重模式
```rust
fn main() {
  let x = 1; 
  match x {
    1 |2 =>println!("one or two"),
    3=>println!("three"),
    4=>println!("four"),
    _=>println!("anything"),
  }
}
```
使用..=来匹配某个范围的值
```rust
fn main() {
  let x = 5; 
  match x {
    1 ..=5 =>println!("one through five"),
    
    _=>println!("anything"),
  }
}
```
解构以分解值
+ 可以使用模式来解构struct、enum、tuple，从而引用这些类型值的不同部分
```rust
struct   Point{
  x:i32,
  y:i32,
}

fn main() {
  let p = Point{x:0, y:7};
  let Point{x:a, y:b} = p ;
  assert_eq!(a,0);
  assert_eq!(b,7);
}
```
在模式中忽略值
+ 有几种方式可以模式中忽略整个值或部分值
+ _
+ _配合其他模式
+ 使用以_开头的名称
+ ..(忽略值的剩余部分)


使用_
```rust
fn  foo(_:i32,y:i32) {
  println!("{}",y);
}

fn main() {
  foo(3,4);
}
```
使用嵌套的_来忽略值的一部分
```rust

fn main() {
  let z  =  (1,2,3,4,5,6,7,8,9,10);
  match z {
    (first, second, third, fourth, fifth, sixth,_,_,_,_,)=>{
      println!("{}",fifth);
    }
  }
}
```
通过使用_开头命名来忽略未使用的变量
```rust
fn main(){
    let _x =5;//编译器不会发出警报
}
```
使用..来忽略值的剩余部分
```rust

fn main() {
  let z  =  (1,2,3,4,5,6,7,8,9,10);
  match z {
    (first, ..,_,)=>{
      println!("{}",first);//不同两边都用..会发生歧义
    }
  }
}
```
使用match守卫来提供额外的条件
+ matc守卫就是match arm模式后额外的if条件，想要匹配该条件也必须满足
+ match守卫适用于比单独的模式更复杂的场景

```rust
fn main(){
  let num  = Some(5);
  match num {
    Some(x)if x<6 => println!("{}",x),
    Some(x)=> println!("{}",x),
    None=>println!("no "),
  }
}
```
@绑定
+ @符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值

## 高级特性
不安全rust
高级trait
高级类型
高级函数和闭包
宏

不安全rust
+ 隐藏着第二个语言，他没有强制内存安全保证，Unsafe rust(不安全的rust)
  + 和普通的rust一样，但提供了额外的超能力
+ unsafe rust存在的原因
  + 静态分析是保守的
    + 使用unsafe rust：我知道自己在做什么，并承担相应的风险
  + 计算机硬件本身就是不安全的，rust需要能够进行底层系统编程

unsafe超能力
+ 使用unsafe关键字来切换到unsafe rust，开启一个块，里面放着unsafe代码
+ unsafe rust里可执行的四个动作(unsafe超能力)：
  + 解引用原始指针
  + 调用unsafe函数或方法
  + 访问或修改可变的静态变量
  + 实现unsafe trait

+ 注意
  + unsafe并没有关闭借用检查或停用其他安全检查
  + 任何内存安全相关的错误必须留在unsafe块里
  + 尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全的API

 解引用原始指针
 + 原始指针
   + 可变的：*mut T 
   + 不可变的：*const T 。意味着指针在解引用后不能直接对其进行赋值
   + 注意：这里的*不是解引用符号，他是类型名的一部分

+ 与引用不同，原始指针：
  + 允许通过同时具有不可变指针和可变指针或多个指向同一位置的可变指针来忽略借用规则
  + 无法保证能指向合理的内存
  + 允许为null
  + 不实现任何自动清理

+ 放弃保证的安全，换取更好的性能/与其他语言或硬件接口的能力

```rust
fn main(){
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
  unsafe{
    println!("{}",*r1);
    println!("{}",*r2);
  }
  let address  = 0x123213443usize;
  let r = address as *const i32;
}
```
解引用原始指针
+ 为什么要使用原始指针
  + 与C语言进行接口
  + 构建借用检查器无法理解的安全抽象

调用unsafe函数或方法
+ unsafe函数或方法：在定义前加上了unsafe关键字
    + 调用前需手动满足一些条件(主要看文档),因为rust无法对这些条件进行验证
    + 需要在unsafe块里进行调用

```rust
unsafe fn dangerous(){

}
fn main(){
  unsafe {
    dangerous();
  }
}
```
创建unsafe代码的安全抽象
+ 函数包含unsafe代码并不意味着需要将整个函数标记为unsafe
+ 将unsafe代码包裹在安全函数中是一个常见的抽象

 使用extern函数调用外部代码
 + extern关键字：简化创建和使用外部函数(FFI)的过程
 + 外部函数接口(FFI.foreign function interface):他允许一种编程语言定义函数，并让其他编程语言能调用这些函数

```rust
extern "C"{//"C"应用二进制接口(application binary interface)
  fn abs(input:i32) -> i32;
}

fn main() {
  unsafe {
    println!(" absolute value of -4 according to c {}",abs(-4));
  }
}
``` 
+ 应用二进制接口(ABI,application binary interface):定义函数在汇编层的调用方式
+ "C" ABI是最常见的ABI，它遵循c语言的ABI

从其他语言调用rust函数
+ 可以使用extern创建接口，其他语言通过他们可以调用rust的函数
+ 在fn 前添加extern函数，并指定ABI
+ 还需添加#[no_mangle]注解：避免rust在编译时改变它的名称
  
```rust
#[no_mangle]

pub extern "C" fn call_from(){
  println!("just called a rust function from c");
}
fn main() {
  
}
```
访问或修改一个可变静态变量
+ rust支持全局变量，但因为所有权机制可能产生某些问题，例如数据竞争
+ 在rust里，全局变量叫做静态(static)变量

```rust
static HELLO_WORLD:&str = "hello risc-v ";
fn main() {
  println!("{}", HELLO_WORLD);
}
```
静态变量
+ 静态变量与常量相似
+ 命名：SCREAMING_SNAKE_CASE
+ 必须标注类型
+ 静态变量只能存储'static'生命周期的引用，无需显示标注
+ 访问不可变静态变量是安全的

常量和不可变静态变量的区别
+ 静态变量：有固定的内存地址。使用它的值总会访问同样的数据
+ 常量：允许使用他们的时候对数据进行复制
+ 静态变量：可以是可变的，访问和修改静态变量是不安全的(unsafe)
  
```rust
static mut COUNT: usize = 1;

fn add(count: usize) {
  unsafe {
    COUNT += count;
  }
}

fn main() {
  add(3);
  unsafe {
    println!("{}", COUNT);
  }
}
```
实现不安全(unsafe)trait
+ 当某个trait中存在的至少一个方法拥有编译器无法校验的不安全因素时，就称这个trait是不安全的
+ 声明unsafe trait在定义前加unsafe关键字
  + 该trait只能在unsafe代码块中实现

```unsafe trait  FOO {
    
}
unsafe impl FOO for i32{

}
fn main() {
  
}
```

何时使用unsafe代码
+ 编译器无法保证内存安全，保证unsafe代码正确并不简单
+ 有充足的理由使用unsafe代码时，就可以这样做
+ 通过显示标记unsafe，可以在问题出现是轻松的定位

高级trait
在trait定义中使用关联类型来指定占位类型
+ 关联类型(associated type )是trait中的类型占位符，它可以用于trait方法签名中
  + 可以定义出包含某些类型的trait，而在实现前无需知道这些类型是什么

```rust
pub trait Iterator{
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}
fn main() {

}
```
关联类型与泛型的区别
泛型
+ 每次实现trait时标注类型
+ 可以为一个泛型多次实现某个trait(不同的泛型参数)

关联类型
+ 无需标注类型
+ 无法为单个类型多次实现某个trait
  
默认泛型参数和运算符重载
+ 可以在使用泛型参数为泛型指定一个默认的具体类型
+ 语法&lt;PlaceholderType =ConcreteType&gt;
+ 这种技术常用于运算符重载(operator overloading)
+ rust不允许创建自己的运算符及重载任意的运算符
+ 但可以通过实现std::ops中列出的那些trait来重载一部分相应的运算符

```rust
use std::ops::Add;
#[derive(Debug,PartialEq)]
struct Point{
  x:i32,
  y:i32,
}

impl Add for Point{
  type Output = Point;
  fn add(self,other:Point )->Point{
    Point{
      x:self.x + other.x,
      y:self.y + other.y,
    }
  }
}
fn main() {
assert_eq!(Point{x:1,y:0}+Point{x:2,y:3}, Point{x:3,y:3})
}
```
默认泛型参数的主要应用场景
+ 扩展一个类型而不破坏现有的代码
+ 允许在大部分用户都不需要的特定场景进行自定义

完全限定语法(fully qualified syntax)
如何调用同名方法
+ 完全限定语法：&lt;Type as Trait&gt;::fuction(receiver_if_method,next_arg,..);
  + 可以在任何调用函数或方法的地方使用
  + 允许忽略那些从其他上下文能推导出来的部分
  + 当rust无法区分你期望调用哪个具体实现的时候，才需要使用这种语法

使用supertrait来要求trait附带其他的功能
+ 需要在一个trait中使用其他trait的功能
  + 需要被依赖的trait也被实现
  + 那个被间接依赖的trait就是当前trait的supertrait

```rust
use std::fmt;

trait outlinePrint:fmt::Display {
  fn outlinePrint(&self) {
    let output = self.to_string();
    let len  = output.len();
  }
}

struct Point{
  x:usize,
  y:usize,
}
impl outlinePrint for Point{}
impl fmt::Display for Point{//实现了fmt::Display后就不会报错
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "({},{})",self.x,self.y)
  }
}
fn main() {

}
```
使用newtype模式在外部类型上实现外部trait
+ 孤儿原则：只有当trait或类型定义在本地包时，才能为该类型实现这个trait
+ 可以通过newtype模式来绕过这一规则
  + 利用tuple struct(元组结构体)创建一个新的类型
  
```rust
use std::fmt;

struct Wrapper(Vec<String>); 

impl fmt::Display for Wrapper{
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
   write!(f, "[{}]", self.0.join(","), )
 }
}



fn main() {

}
```
高级类型
+ 使用newtype模式实现类型安全和抽象
+ newtype模式可以
  + 用来静态的保证各种值之间不会混淆并表明值的单位
  + 为类型的某些细节提供抽象能力
  + 通过轻量级的封装来隐藏内部实现细节

使用类型别名创建类型同义词
+ rust提供了类型别名的功能
  + 为现有的类型生产另外的名称(同义词)
  + 并不是一个独立的类型
  + 使用类型关键字type
+ 主要用途：减少代码字符重复

```rust
type kill  =i32;


fn main() {
  let x  =5;
  let y :kill = 5;
  println!("{}",x+y);
}
////////////////////////////////////////////////////////////////
type Thunk = Box<dyn Fn() + Send + 'static>;

fn take_long(f:Thunk){

}

fn return_value ()->Thunk{
  Box::new(||println!("hi"))
}



fn main() {
  let f = Box::new(||println!("hi"));
}
```
Never类型
+ 有一个名为!的特殊类型
  + 他没有任何值，行话称为空类型(empty type)
  + 我们倾向叫它never类型。因为它在不返回的函数中充当返回类型

+ 不返回值的函数也被称为发散函数(diverging function)

```rust
fn foo()->!{
  loop{
    println!("");
  }
}


fn main() {

}
```
动态大小和Sized trait
+ rust需要编译时确定为一个特定类型的值分配多少空间
+ 动态大小的类型(dynamically sized trait,dst)的概念
  + 编写代码时使用只有在运行时才能确定大小的值

+ str是动态大小的类型(注意不是&str)：只有运行时才能确定字符串的长度
  + 下列代码无法正常工作
    + let s1:str = "hello world";
    + let s2:str = "hello world risc-v";
  + 使用&str来解决
    + str的地址
    + str的长度

rust使用动态大小类型的通用方式
+ 附带一些额外的元数据来存储动态信息的大小
  + 使用动态大小类型时总会把他的值放在某种指针后边

 另外一种动态大小的类型：trait
 + 每个trait都是一个动态大小的类型，可以通过名称对其进行引用
 + 为了将trait用作trait对象，必须将他放置在某种指针之后
   + 例如&dyn trait或Box&lt;dyn trait&gt;(Rc&lt;dyn trait&gt;)之后

sized trait
+ 为了处理动态大小的类型，rust提供了一个sized trait来确定一个类型的大小在编译时是否已知
  + 编译时可计算出大小的类型会自动实现这一trait
  + rust还会为每一个泛型函数隐式的添加sized约束

+ 默认情况下，泛型函数只能用于编译时已经知道大小的类型，可以通过特殊语法解除这一限制

？sized traits约束
+ T可能是也可能不是sized
+  这个语法只能用在sized上面，不能被用于其他trait
```rust
// fn genric <T>(t:T){

// }
// fn genric<T:Sized>(t:T){

// }
// fn genric<T:?Sized>(t:&T){}


fn main() {

}
```
高级函数和闭包
函数指针
+ 可以将函数传递给其他函数
+ 函数传递过程中会被强制转换为fn类型
+ fn类型就是“函数指针”(function pointer)
  
```rust
fn add_one(x: i32) -> i32 {
  x + 1
}

fn fn_add(f:fn(i32)->i32,arg:i32)-> i32{
  f(arg) + f(arg)
}
fn main()  {
  let result = fn_add(add_one,2);
  println!("{}",result);
}
```
函数指针与闭包的不同
+ fn是一个类型，不是一个trait
  + 可以直接指定fn为类型参数，不用声明一个以fn trait为约束的泛型参数

+ 函数指针实现了全部三种闭包trait(Fn,FnMut,FnOnce):
  + 总是可以把函数指针用作参数传递给一个接受闭包的函数
  + 所以，倾向于搭配闭包trait的泛型来编写函数：可以同时接收闭包和普通函数
+ 某些情景，只想接受fn而不接收闭包
  + 与外部不支持闭包的代码交互：C函数

```rust
fn main(){
  let v = vec![1, 2, 3, 4, 5];
  let s :Vec<String> =v.iter()
  .map(|x |x.to_string())
  .collect();
  for i in 0..s.len(){
    println!("{}",s[i]);
  } 
}
////////////////////////////////
 fn main() {
  enum State {
    value(u32),
    stop,
  }
  let v = State::value(5);
  let s :Vec<State> = (0u32..20).map(State::value).collect();
}
```
返回闭包
+ 闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了trait的具体类型作为返回值
```rust
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn main() {}
```

宏macro
+ 宏在rust里指的是一组相关的集合称谓
+ 使用macro_rules!构建的声明宏(declarative  macro)
+ 三种过程宏
  + 自定义#[derive]宏，用于struct或enum，可以为其指定随derive属性添加的代码
  + 类似属性的宏，在任何条目上添加自定义属性
  + 类似函数的宏，看起来像函数调用，对其指定为参数的token进行操作

函数与宏的差别
+ 本质上，宏是用来编写可以生成其他代码的代码(元编程，mataprograming)
+ 函数在定义签名时，必须声明参数的个数和类型，宏可处理可变的参数
+ 编译器会在解释代码前展开宏
+ 宏的定义比函数复杂得多，难以阅读、理解、维护
+ 在某个文件调用宏时，必须提前定义宏或将宏引入当前作用域
+ 函数可以在任何位置定义并在任何位置使用

macro_rules!声明宏(弃用)
+ rust中最常见的宏形式：声明宏
  + 类似match的模式匹配
  + 需要使用macro_rules！

```rust
#[macro_export] //标注的宏可以被引用vec[1,2,3];
macro_rules!  vec {
    ($($x:expr),*) => {//($x:expr)匹配任何的rust表达式，*可以匹配0个或多个在他之前的项
        {
        let mut temp_vec = Vec::new();
        $(
             temp_vec.push($x);//匹配三次push进入vec
        )*
      temp_vec
    }
    };
}
```
基于属性来生成代码的过程宏
+ 这种形式更像函数(某种形式的过程)一些
  + 接收并操作输入的rust代码
  + 生成另外一些rust代码作为结果
+ 三种过程宏
  + 自定义派生
  + 属性宏
  + 函数宏
+ 创建过程宏时
  + 宏定义必须单独放在他们自己的包中，并使用特殊的包类型

自定义derive宏
+ 需求：
  + 创建一个hello_macro包，定义一个拥有关联函数hello_macro的HelloMacro trait
  + 我们提供一个能自动实现trait的过程宏
  + 在它们的类型标注#[derive(HelloMacro)],进而得到hello_macro的默认实现

类似属性的宏
+ 属性宏与自定义derive宏类似
  + 允许创建新的属性
  + 但不是为derive属性生成代码

+ 属性宏更加灵活
  + derive只能用于struct和enum
  + 属性宏以为用于任意条目，例如函数

类似函数的宏
+ 函数宏定义类似于函数调用的宏，但比普通函数更加灵活
+ 函数宏可以接受TokenStream作为参数
+ 与另外两种过程宏一样，在定义中使用rust代码操作TokenStream


## 构建多线程的web服务器
+ 在socket上监听TCP连接
+ 解析少量的HTTP强求
+ 创建一个合适的HTTP响应
+ 使用线程池改进服务器的吞吐量

+ 注意这不是最佳实践


