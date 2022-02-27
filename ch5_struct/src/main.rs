fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // インスタンスを生成する
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 与えられたemailとusernameでインスタンスを生成する関数
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 更新記法で他のインスタンスからインスタンスを生成する

    let user2 = User {
        email: String::from("another@exmaple.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}
