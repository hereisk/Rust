use rocket;
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::serde::json;
use rocket::http::{CookieJar, Cookie};
//use yew::prelude::*;
use rocket::response::content::RawHtml;

use crate::db_data::user::User;
use crate::view;
// use crate::db_data::user::UserNoPassword;

#[derive(Debug)]
pub struct NewUser<'a> {
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
    pub age: Option<u8>,
    pub password: Option<&'a str>,
    pub email: Option<&'a str>
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    Io(std::io::Error),
    InvalidArguments,
    InvalidArgumentDelimiter
}

const PAYLOAD_LENGTH: u8 = 255;

impl <'a> NewUser<'a> {
    fn from_curl (curl_data: Vec<&'a str>) -> data::Outcome<'a, Self> {
        let mut new_user = NewUser {
            firstname: None,
            lastname: None,
            age: None,
            password: None,
            email: None,
        };

        for pair in curl_data {
            let (attribute, value) = match pair.find('=') {
                    Some(i) => (&pair[..i], &pair[(i + 1)..]),
                    None => return Failure((Status::UnprocessableEntity, Error::InvalidArgumentDelimiter)),
                };
            match attribute {
                "firstname" => {
                    if new_user.firstname == None && value.len() < 25 {
                        new_user.firstname = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                "lastname" => {
                    if new_user.lastname == None && value.len() < 25 {
                        new_user.lastname = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                "age" => {
                    if new_user.age == None {
                        let age: u8 = match value.parse()  {//from_str::<u8>(&value)
                            Ok(age) => age,
                            Err(_) => return Failure((Status::UnprocessableEntity, Error::InvalidArguments)),
                        };
                        new_user.age = Some(age);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                "password" => {
                    if new_user.password == None && value.len() < 25 {
                        new_user.password = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                "email" => {
                    if new_user.email == None && value.len() < 25 {
                        new_user.email = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                _ => { 
                    return Failure((Status::UnprocessableEntity, Error::InvalidArguments)); 
                }
            }
        }

        Success(new_user)
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewUser<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        

        let string = match data.open(PAYLOAD_LENGTH.bytes()).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };
        
        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        let string: Vec<&str> = string.split('&').collect();
        if string.len() != 5 {
            return Failure((Status::NotAcceptable, InvalidArguments));
        }

        NewUser::from_curl(string)
    }
}

//  curl -X POST http://127.0.0.1:8000/users -d "firstname=Jordan" -d "lastname=Richards" -d "age=22" -d "password=jr22" -d "email=jordan@richards.com"
#[rocket::post("/users", data = "<new_user>")]
fn users_post(new_user: Result<NewUser<'_>, Error>) -> Result<(Status, String), String> {
    match new_user {
        Ok(new_user) => {
            match User::create(User::from(new_user)) {
                Ok(id) => Ok((Status::Created, id)),
                Err(e) => Err(format!("{:?}", e)),
            }
        },
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    }
}

// curl -X GET http://127.0.0.1:8000/users
#[rocket::get("/users")]
fn users_get() -> Result<(Status, String), String> {
    match User::all() {
        Ok(data) => Ok((Status::Created, json::to_string(&data).unwrap())),
        Err(e) => return Err(format!("{:?}", e)),
    }
}

pub struct Credentials<'e> {
    pub password: Option<&'e str>,
    pub email: Option<&'e str>
}

impl <'e> Credentials<'e> {
    fn from_curl (curl_data: Vec<&'e str>) -> data::Outcome<'e, Self> {
        let mut credentials = Credentials {
            password: None,
            email: None,
        };

        for pair in curl_data {
            let (attribute, value) = match pair.find('=') {
                    Some(i) => (&pair[..i], &pair[(i + 1)..]),
                    None => return Failure((Status::UnprocessableEntity, Error::InvalidArgumentDelimiter)),
                };
            match attribute {
                "password" => {
                    if credentials.password == None && value.len() < 25 {
                        credentials.password = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                "email" => {
                    if credentials.email == None && value.len() < 25 {
                        credentials.email = Some(value);
                    } else {
                        return Failure((Status::UnprocessableEntity, Error::InvalidArguments));
                    }
                },
                _ => { 
                    return Failure((Status::UnprocessableEntity, Error::InvalidArguments)); 
                }
            }
        }

        Success(credentials)
    }
}

#[rocket::async_trait]
impl<'c> FromData<'c> for Credentials<'c> {
    type Error = Error;

    async fn from_data(req: &'c Request<'_>, data: Data<'c>) -> data::Outcome<'c, Self> {
        use Error::*;
        

        let string = match data.open(PAYLOAD_LENGTH.bytes()).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };
        
        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        let string: Vec<&str> = string.split('&').collect();
        if string.len() != 2 {
            return Failure((Status::NotAcceptable, InvalidArguments));
        }

        Credentials::from_curl(string)
    }
}

// curl -c cookie.txt -X POST http://127.0.0.1:8000/sign_in -d "password=jr22" -d "email=jordan@rich.com"
#[rocket::post("/sign_in", data = "<credentials>")]
fn sign_in_post(credentials: Result<Credentials<'_>, Error>, jar: &CookieJar<'_>) -> Result<(Status, String), String> {
    if let Some(_) = jar.get("user_id").map(|cookie| cookie.value()) {
        return Err("User is already logged in".to_string());
    }
    
    match credentials {
        Ok(credentials) => {
            match User::sign_in(credentials) {
                Ok(user) => {
                    //let user = json::to_string(&user).unwrap();
                    jar.add(Cookie::new("user_id", user.get_id().to_string()));
                    match jar.get("user_id") {
                        Some(user) => println!("{:?}", user),
                        None =>  println!("no cookies in a jar"),
                    }

                    Ok((Status::Created, json::to_string(&user).unwrap()))
                },
                Err(e) => Err(format!("{:?}", e)),
            }
        },
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    }
}

#[derive(Debug)]
struct Password<'p> (&'p str, &'p str);

#[rocket::async_trait]
impl<'p> FromData<'p> for Password<'p> {
    type Error = Error;

    async fn from_data(req: &'p Request<'_>, data: Data<'p>) -> data::Outcome<'p, Self> {
        use Error::*;
        

        let string = match data.open(PAYLOAD_LENGTH.bytes()).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };
        
        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        let string: Vec<&str> = string.split('=').collect();
        if string.len() != 2 {
            return Failure((Status::NotAcceptable, InvalidArguments));
        }

        if string[0] == "password" && string[1].len() < 25 {
            return Success(Password(string[0], string[1]));
        }

        Failure((Status::UnprocessableEntity, Error::InvalidArguments))
    }
}

// curl -b cookie.txt -X PUT http://127.0.0.1:8000/sign_in -d "password=rj22"
#[rocket::put("/sign_in", data = "<password>")]
fn sign_in_put(jar: &CookieJar<'_>, password: Password) -> Status {
    let (attribute, value) = (password.0.clone(), password.1.clone());
    match jar.get("user_id").map(|cookie| cookie.value()) {
        Some(id) => match User::update(id, attribute, value) {
            Ok(_) => Status::NoContent,
            Err(_) => Status::BadRequest,
        },
        None => Status::NoContent,
    }
}

// curl -b cookie.txt -X DELETE http://127.0.0.1:8000/sign_out
#[rocket::delete("/sign_out")]
fn sign_out_delete(jar: &CookieJar <'_>) -> Status {
    match jar.get("user_id").map(|cookie| cookie.value()) {
        Some(_) => {
            jar.remove(Cookie::named("user_id"));
            Status::NoContent
        },
        None => Status::BadRequest,
    }
}

#[rocket::get("/")]
fn views() -> &'static str {
    "users"
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![users_get, users_post, sign_in_post, sign_in_put, sign_out_delete, views]
}