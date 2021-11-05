use std::rc::Rc;

// 一个值只能被一个变量所拥有，这个变量被称为所有者（Each value in Rust has a variable that’s called its owner）。
// 一个值同一时刻只能有一个所有者（There can only be one owner at a time），也就是说不能有两个变量拥有相同的值。
// 所以对应刚才说的变量赋值、参数传递、函数返回等行为，旧的所有者会把值的所有权转移给新的所有者，以便保证单一所有者的约束。
// 当所有者离开作用域，其拥有的值被丢弃（When the owner goes out of scope, the value will be dropped），内存得到释放。

// 1. 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
// 2. 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
fn main() {
    println!("==============================&str but string in the static memory==================================");
    // normal fat pointer initialization
    // 这里的“13131”存在静态储存区。
    let a = "13131";
    let b = a;
    let c = a;
    // to_string会申请一个新的“13131”在堆上面，相当于特殊的Copy，因此之前的a还是可以使用。
    let mut d = a.to_string();

    // 1. 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
    // 2. 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
    let  f = &mut d;

    f.push_str("你是谁？");

    let e = &d;
    
    println!("e {}: stack {:p} -> heap {:p}", e, &e, e.as_ptr());
    // X 报错: 因为在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
    //----------------------------------------------------------------


    // use the & operator to get the address of any variable.
    println!("a {}: stack {:p} -> heap {:p}", a, &a, a);
    println!("b {}: stack {:p} -> heap {:p}", b, &b, b);
    println!("c {}: stack {:p} -> heap {:p}", c, &c, c);

    println!("d {}: stack {:p} -> heap {:p}", d, &d, d.as_ptr());
    // Summary: 只读引用实现了 Copy trait，也就意味着引用的赋值、传参都会产生新的浅拷贝。
    println!("==============================String + clone==================================");
    // normal String initialization
    let a = String::from("13131");
    let b = a.clone();
    let c = a.clone();

    // use the & operator to get the address of any variable.
    // as_ptr() to get address at the heap
    println!("a {}: stack {:p} -> heap {:p}", a, &a, a.as_ptr());
    println!("b {}: stack {:p} -> heap {:p}", b, &b, b.as_ptr());
    println!("c {}: stack {:p} -> heap {:p}", c, &c, c.as_ptr());
    // Summary: String没有实现Copy trait, 只能通过clone出新的在堆上面，相当于 堆&栈都复制了一份。
    println!("==============================String + reference==================================");
    // normal String initialization
    let a = String::from("13131");
    let b = &a;
    let c = &a;
    let d = a.as_str();

    // use the & operator to get the address of any variable.
    // as_ptr() to get address at the heap
    println!("a {}: stack {:p} -> heap {:p}", a, &a, a.as_ptr());
    println!("b {}: stack {:p} -> heap {:p}", b, &b, b.as_ptr());
    println!("c {}: stack {:p} -> heap {:p}", c, &c, c.as_ptr());

    println!("d {}: stack {:p} -> heap {:p}", d, &d, d);
    // Summary: 多个栈内存引用了同一个堆的内存，但是这些都是可读的引用，符合规则。
    println!("==============================Rc==================================");
    // 相比于String的clone, Rc的clone
    // 对一个 Rc 结构进行 clone()，不会将其内部的数据复制，只会增加引用计数。
    let a = Rc::new(5);
    let b = a.clone();
    let c = a.clone();

    // use the & operator to get the address of any variable.
    println!("a {}: stack {:p} -> heap {:p}", a, &a, a);
    println!("b {}: stack {:p} -> heap {:p}", b, &b, b);
    println!("c {}: stack {:p} -> heap {:p}", c, &c, c);

    // Summary: 多个栈引用了同一个堆的内存
}
