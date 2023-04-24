use crate::db_data::db;
use crate::db_data::db_error::DbError;
use crate::controller::{NewUser, Credentials};

use rocket::serde::{Serialize};
use sqlite;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserNoPassword {
    id: u8,
    firstname: String,
    lastname: String,
    age: u8,
    email: String
}

impl From<User> for UserNoPassword {
    fn from(user: User) -> Self {
        UserNoPassword{
            id: user.id.unwrap(),
            firstname: user.firstname,
            lastname: user.lastname,
            age: user.age,
            email: user.email
        }
    }
}

impl UserNoPassword {
    // pub fn get_email(&self) -> String {
    //     self.email.clone()
    // }

    pub fn get_id(&self) -> u8 {
        self.id.clone()
    }
}

#[derive(Debug)]
pub struct User {
    id: Option<u8>,
    firstname: String,
    lastname: String,
    age: u8,
    password: Option<String>,
    email: String
}

impl From<NewUser<'_>> for User {
    fn from(new_user: NewUser) -> Self {
        User {
            id: None,
            firstname: new_user.firstname.unwrap().to_string(),
            lastname: new_user.lastname.unwrap().to_string(),
            age: new_user.age.unwrap(),
            password: Some(new_user.password.unwrap().to_string()),
            email: new_user.email.unwrap().to_string(),
        }
    }
}

impl User {
    pub fn create(self) -> Result<String, DbError> {
        let db = db::db_connect()?;
        let mut statement = db.prepare("INSERT INTO users (firstname, lastname, age, password, email) 
                                        VALUES (:firstname, :lastname, :age, :password, :email);")?;
        statement.bind((":firstname", self.firstname.as_str()))?;
        statement.bind((":lastname", self.lastname.as_str()))?;
        statement.bind((":age", self.age.to_string().as_str()))?;
        statement.bind((":password", self.password.unwrap().as_str()))?;
        statement.bind((":email", self.email.as_str()))?;
        statement.next()?;
        
        let mut statement = db.prepare("SELECT id FROM users WHERE email = :email;")?;
        statement.bind((":email", self.email.as_str()))?;
        statement.next()?;
        let id = statement.read::<String, _>(0)?;
        Ok(id)
    
    }
    pub fn all() -> Result<Vec<UserNoPassword>, DbError> {
        let db = db::db_connect()?;
        let mut statement = db.prepare("SELECT * FROM users;")?;
        let mut vec: Vec<UserNoPassword> = Vec::new();
        while let sqlite::State::Row = statement.next()? {
            let user = User {
                id: Some(statement.read::<String, _>(0).unwrap().parse().unwrap()),
                firstname: statement.read::<String, _>(1).unwrap(),
                lastname: statement.read::<String, _>(2).unwrap(),
                age: statement.read::<String, _>(3).unwrap().parse().unwrap(),
                password: Some(statement.read::<String, _>(4).unwrap()),
                email: statement.read::<String, _>(5).unwrap()
            };
            let user = UserNoPassword::from(user);
            vec.push(user);
        }
        Ok(vec)
    }

    pub fn sign_in(credentials: Credentials) -> Result<UserNoPassword, DbError> {
        let db = db::db_connect()?;
        let mut statement = db.prepare("SELECT * FROM users WHERE email = :email;")?;
        statement.bind((":email", credentials.email.unwrap()))?;
        if let sqlite::State::Row = statement.next()? {
            let user = User {
                id: Some(statement.read::<String, _>(0).unwrap().parse().unwrap()),
                firstname: statement.read::<String, _>(1).unwrap(),
                lastname: statement.read::<String, _>(2).unwrap(),
                age: statement.read::<String, _>(3).unwrap().parse().unwrap(),
                password: Some(statement.read::<String, _>(4).unwrap()),
                email: statement.read::<String, _>(5).unwrap()
            };
            if credentials.password.unwrap() == user.password.clone().unwrap().as_str() {
                    let user = UserNoPassword::from(user);
                    Ok(user)
            } else {
                Err(DbError::InvalidCredentials)    
            }
        } else {
            Err(DbError::InvalidCredentials)
        }
    }

    pub fn update(user_id: &str, attribute: &str, value: &str) -> Result<(), DbError> {
            let db = db::db_connect()?;
            let query = format!("UPDATE users SET {attribute} = :value WHERE id = :id;");
            // db.execute(format!("UPDATE users SET {attribute} = {value} WHERE id = {user_id};").as_str());
            let mut statement = db.prepare(query.as_str())?;
            statement.bind((":value", value))?;
            statement.bind((":id", user_id))?;
            statement.next()?;
            Ok(())
        }
}



// pub fn find(user_id: u8) -> Result<String, DbError> {
//     let db = db::db_connect()?;
//     let mut statement = db.prepare("SELECT * FROM users WHERE id = :id;")?;
//     statement.bind((":id", user_id.to_string().as_str()))?;
//     if let sqlite::State::Row = statement.next()? {
//         let user = User::new(
//             Some(from_str::<u8>(&statement.read::<String, _>(0).unwrap()).unwrap()),
//             statement.read::<String, _>(1).unwrap(),
//             statement.read::<String, _>(2).unwrap(),
//             from_str::<u8>(&statement.read::<String, _>(3).unwrap()).unwrap(),
//             Some(statement.read::<String, _>(4).unwrap()),
//             statement.read::<String, _>(5).unwrap()
//         );
//         let user = UserNoPassword::from(user);
//         Ok(serde_json::to_string(&user).unwrap())
//     } else {
//         Err(DbError::NotFound(user_id.to_string()))
//     }
// }


// pub fn update(user_id: String, attribute: String, value: String) -> Result<String, DbError> {
//     let db = db::db_connect()?;
//     let query = format!("UPDATE users SET {attribute} = :value WHERE id = :id;");
//     // db.execute(format!("UPDATE users SET {attribute} = {value} WHERE id = {user_id};").as_str());
//     let mut statement = db.prepare(query.as_str())?;
//     statement.bind((":attribute", attribute.as_str()))?;
//     statement.bind((":value", value.as_str()))?;
//     statement.bind((":id", user_id.as_str()))?;
//     statement.next()?;

//     let mut statement = db.prepare("SELECT * FROM users WHERE id = :id;")?;
//     statement.bind((":id", user_id.as_str()))?;
//     statement.next()?;
//     let user = User::new(
//         Some(from_str::<u8>(&statement.read::<String, _>(0).unwrap()).unwrap()),
//         statement.read::<String, _>(1).unwrap(),
//         statement.read::<String, _>(2).unwrap(),
//         from_str::<u8>(&statement.read::<String, _>(3).unwrap()).unwrap(),
//         Some(statement.read::<String, _>(4).unwrap()),
//         statement.read::<String, _>(5).unwrap()
//     );
//     let user = UserNoPassword::from(user);
//     Ok(serde_json::to_string(&user).unwrap())
// }

// pub fn destroy(user_id: String) -> Result<(), DbError> {
//     let db = db::db_connect()?;
//     let mut statement = db.prepare("DELETE FROM users WHERE id = :id;")?;
//     statement.bind((":id", user_id.as_str()))?;
//     statement.next()?;
//     Ok(())
// }