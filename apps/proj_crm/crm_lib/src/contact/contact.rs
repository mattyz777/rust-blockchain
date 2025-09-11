use crate::contact::profile::Profile;
use crate::contact::tag::Tag;

pub struct Contact {
    pub id: i32,
    pub profile: Profile,
    tags: Vec<Tag>,
}

impl Contact {
    pub fn new(id: i32, first_name: &str, last_name: &str) -> Self {
        Self {
            id,
            profile: Profile::new(first_name, last_name),
            tags: vec![],
        }
    }

    //  tag ownership moves to tags vec
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn details(&self) -> String {
        let tag_names: Vec<&str> = self.tags.iter().map(|t| t.name.as_str()).collect();
        let tags_str = tag_names.join(", ");

        format!("{} {}", self.profile.full_name(), tags_str)
    }
}