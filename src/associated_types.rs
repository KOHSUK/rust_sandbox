mod domain {
    pub trait GreetingRepository {
        type Message;

        fn get(&self) -> Self::Message;
        fn to_string_messege(msg: Self::Message) -> String;
        fn from_string_message<S: Into<String>>(msg: S) -> Self::Message;
    }

    pub trait GreetingRepositoryWrapper {
        fn get(&self) -> String;
    }

    impl<T: GreetingRepository> GreetingRepositoryWrapper for T {
        fn get(&self) -> String {
            let msg = self.get();
            T::to_string_messege(msg)
        }
    }
}

mod repository {
    use super::domain::GreetingRepository;

    pub struct GreetingRepositoryA {
        msg: String,
    }

    impl GreetingRepository for GreetingRepositoryA {
        type Message = String;

        fn get(&self) -> Self::Message {
            self.msg.clone()
        }

        fn to_string_messege(msg: Self::Message) -> String {
            msg
        }

        fn from_string_message<S: Into<String>>(msg: S) -> Self::Message {
            msg.into()
        }
    }

    impl GreetingRepositoryA {
        pub fn new() -> Self {
            Self {
                msg: "Hello from A".to_string(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Msg(String);

    pub struct GreetingRepositoryB {
        msg: Msg,
    }

    impl GreetingRepository for GreetingRepositoryB {
        type Message = Msg;

        fn get(&self) -> Self::Message {
            self.msg.clone()
        }

        fn to_string_messege(msg: Self::Message) -> String {
            msg.0
        }

        fn from_string_message<S: Into<String>>(msg: S) -> Self::Message {
            Msg(msg.into())
        }
    }

    impl GreetingRepositoryB {
        pub fn new() -> Self {
            Self {
                msg: Msg("Hello from B".to_string()),
            }
        }
    }
}

mod application {
    use super::domain::GreetingRepositoryWrapper;
    pub struct GreetingServiceA {
        greeting_repo: Box<dyn GreetingRepositoryWrapper>,
    }

    impl GreetingServiceA {
        pub fn new(greeting_repo: Box<dyn GreetingRepositoryWrapper>) -> Self {
            Self { greeting_repo }
        }

        pub fn say(&self) -> String {
            let msg = self.greeting_repo.get();
            format!("[Dynamic Dispatch] {}", msg)
        }
    }

    pub struct GreetingServiceB<T>
    where
        T: GreetingRepositoryWrapper,
    {
        greeting_repo: T,
    }

    impl<T: GreetingRepositoryWrapper> GreetingServiceB<T> {
        pub fn new(greeting_repo: T) -> Self {
            Self { greeting_repo }
        }

        pub fn say(&self) -> String {
            let msg = self.greeting_repo.get();
            format!("[Generics] {}", msg)
        }
    }
}

fn main() {
    use application::{GreetingServiceA, GreetingServiceB};
    use repository::{GreetingRepositoryA, GreetingRepositoryB};
    let service_a_1 = GreetingServiceA::new(Box::new(GreetingRepositoryA::new()));
    println!("{}", service_a_1.say());
    let service_a_2 = GreetingServiceA::new(Box::new(GreetingRepositoryB::new()));
    println!("{}", service_a_2.say());

    let service_b_1 = GreetingServiceB::new(GreetingRepositoryA::new());
    println!("{}", service_b_1.say());
    let service_b_2 = GreetingServiceB::new(GreetingRepositoryB::new());
    println!("{}", service_b_2.say())
}

#[cfg(test)]
mod test {
    use super::{application::GreetingServiceA, domain::GreetingRepository};
    #[test]
    fn service_a_is_working() {
        struct MockGreeting;

        impl GreetingRepository for MockGreeting {
            type Message = String;

            fn get(&self) -> Self::Message {
                "Hello from test".to_string()
            }

            fn to_string_messege(msg: Self::Message) -> String {
                msg
            }

            fn from_string_message<S: Into<String>>(msg: S) -> Self::Message {
                msg.into()
            }
        }

        let test_service = GreetingServiceA::new(Box::new(MockGreeting {}));
        let message = test_service.say();

        assert_eq!(message, "[Dynamic Dispatch] Hello from test".to_string())
    }
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

pub fn run() {
    main()
}
