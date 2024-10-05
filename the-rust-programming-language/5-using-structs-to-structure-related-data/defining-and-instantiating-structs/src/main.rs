fn main() {

    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
            }


        let mut user1 = User {
            active: true,
            username: String::from("foo@bar.com"),
            email: String::from("baz@example.com"),
            sign_in_count: 1,
            };

        user1.email = String::from("herp@derp.doo");

        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }

        let user2 = User {
            email: String::from("foo@bar.com"),
            ..user1
        };
    }
}

