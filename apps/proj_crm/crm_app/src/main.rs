use crm_lib::contact::tag::Tag;
use crm_lib::contact::contact::Contact;

fn main() {
    // let p = crm_lib::contact::profile::Profile::new("John", "Doe");
    // println!("Profile: {} {}", p.first_name, p.last_name);

    let mut contact = Contact::new(1, "John", "Doe");
    // error tags() returns &Vec<Tag> which is a borrowed view, it cannot add items
    // contact.tags().push(Tag::new("tag1")); 
    contact.add_tag(Tag::new("tag1"));
    contact.add_tag(Tag::new("tag2"));
    
    // the last expression in a function is implicitly returned if it has no semicolon.

    println!("{}", &contact.details());
}
