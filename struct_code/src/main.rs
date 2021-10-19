struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

fn main() {
    /* user元组 */
    let user: (User, User) = 
        (build_user("qianxiaowei@gmail.com".to_string(), "qianxiaowei".to_string()),
        build_user_easily("qianxiaowei1@gmail.com".to_string(), "qianxiaowei1".to_string()));

    println!("{}\n{}\n{}\n{}\n", user.0.email, user.0.username, user.0.active, user.0.sign_in_count);

    println!("{}\n{}\n{}\n{}\n", user.1.email, user.1.username, user.1.active, user.1.sign_in_count);

    /* user数组 */
    let user: [User; 2] = [build_user_easily("qianxiaowei2@gmail.com".to_string(), "qianxiaowei2".to_string()),
                           build_user_easily("qianxiaowei3@gmail.com".to_string(), "qianxiaowei3".to_string())];

    println!("{}\n{}\n{}\n{}\n", user[0].email, user[0].username, user[0].active, user[0].sign_in_count);

    println!("{}\n{}\n{}\n{}\n", user[1].email, user[1].username, user[1].active, user[1].sign_in_count);


    /* 有问题的代码 */
    // cannot move out of here move occurs because `user[_].username` has type `String`, which does not implement the `Copy` trait

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user[0]
    // };

    let user = User {
        email: String::from("another@example.com"),
        username: String::from("anothe"),
        active: true,
        sign_in_count: 22
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    println!("{}\n{}\n{}\n{}\n", user2.email, user2.username, user2.active, user2.sign_in_count);

    /* 元组结构体（tuple structs）*/
    // 普通元组 let user: (User, User)
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let    test(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let test1 = test(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_easily(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}