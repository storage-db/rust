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
    + -相对路径：从当前木块开始，使用self，super或当前模块的标识符
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

