#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

/*
 * Hero structure:
 * id: Is optional since consumers hitting the create endpoint won't have an id.
 * name: A string indicating the full name of our hero
 * identity: The class or profession our hero is assigned
 * hometown: Where our hero was born
 * age: The age of our hero
 */
