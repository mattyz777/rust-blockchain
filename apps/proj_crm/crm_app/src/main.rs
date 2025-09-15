use crm_lib::contact::tag::Tag;
use crm_lib::contact::contact::Contact;
use getset::{Getters, Setters};

fn main() {
    // let p = crm_lib::contact::profile::Profile::new("John", "Doe");
    // println!("Profile: {} {}", p.first_name, p.last_name);

    // let mut contact = Contact::new(1, "John", "Doe");
    // // error tags() returns &Vec<Tag> which is a borrowed view
    // // contact.tags().push(Tag::new("tag1"));  // move othership
    // contact.add_tag(Tag::new("tag1"));
    // contact.add_tag(Tag::new("tag2"));
    
    // // the last expression in a function is implicitly returned if it has no semicolon.
    // println!("{}", &contact.details());

    #[derive(Debug, Getters, Setters)]
    struct User {
        #[getset(get = "pub", set = "pub")]
        id: u32,
        #[getset(get = "pub", set = "pub")]
        username: String,
        #[getset(get = "pub", set = "pub")]
        email: String,
    }

    impl User {
        fn new(id: u32, username: String, email: String) -> Self {
            Self { id, username, email }
        }
    }

    let mut user = User::new(1, "john".to_string(), "john@email.com".to_string());
    println!("{:?}", user);
    user.set_username("john_doe".to_string());
    println!("{:?}", user);
}
