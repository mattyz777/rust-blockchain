use crm_lib::contact::tag::Tag;
use crm_lib::contact::contact::Contact;

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

    #[derive(Debug)]
    struct Teacher {
        name: String,
        age: i32,
    }
    
    impl Teacher {
        fn new(name: &str, age: i32) -> Self {
            Self {
                name: String::from(name),
                age,
            }
        }

        fn is_adult(&self) -> bool {
            self.age > 18
        }
    }

    let vs:Vec<Teacher> = vec!(Teacher::new("Bob", 17), Teacher::new("Charlie", 35));

    let adults:Vec<&Teacher>  = vs.iter().filter(|v|v.is_adult()).collect();


    for t in &vs {
        println!("Teacher: {:?}", t);
    }

    // println!("Teacher: {:?}", adults);

    // &m[2]  // indexing panices if out of bounds
    // m.get(2) // returns Option<&T>, None if out of bounds
    let m: Vec<String> = vec!["aa".to_string(), "bb".to_string()];
    // let n1: str = m[0]; // Can’t move an element out of a Vec by indexing because that would leave a “hole”.
    let n2: &str = &m[0]; // panic
    
    println!("{}", n2);  // Uses Display implementation for &str to print the text content of the slice
    println!("{:p}", n2); // address of the string data on heap
    println!("{:p}", &n2); // address of the reference variable on stack

    let n: Option<&String> = m.get(2); // use index method
    println!("{:?}", n); // None
    let n2: Option<&String> = m.get(0); 
    println!("Safe get first element: {:?}", n2); // Some("hello")
    match n {
        Some(s) => println!("{}", s),
        None => println!("No value"),
    }
    
    let pt1 = Teacher::new("Bob", 17);
    // println!("{}", pt1); // error: `Teacher` doesn't implement `std::fmt::Display`
    println!("{:?}", pt1);
    println!("{:#?}", pt1);
}
