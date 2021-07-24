use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

pub fn authenticate(token: &str) -> Result<User, ()> {
    if token == "ABCDEF" {
        Ok(User {
            // XXX: No static IDs!
            user_id: "698d7e28-76ee-4b43-9744-deda822cf109".to_owned(),
        })
    } else {
        // XXX: Real error
        Err(())
    }
}
