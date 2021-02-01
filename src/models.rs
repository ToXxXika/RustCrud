use diesel;
use diesel:: prelude::*;
use diesel::mysql::MysqlConnection;
use Schema::user;
//DSL : dynamic specific language
use Schema::user::dsl::user as all_users;

#[derive(Queryable)]
pub struct User{
    pub id: i32,
    pub name:String,
    pub cin:String ,
}
// this structor will be used when we create new User insideDB
#[derive(Insertable)]
#[table_name="user"]
pub struct Newuser{
    pub id: i32,
    pub name:String,
    pub cin:String ,
}
impl User{
    pub fn show(id:i32,conn: &MysqlConnection) -> Vec<User>{
        all_users.find(id)
            .load::<User>(conn)
            .expect("Error Loading User")
    }
    pub fn all(conn: &MysqlConnection)-> Vec<User>{
        all_users.order(users::id.desc())
            .load::<User>(conn)
            .expect("error loading books")
    }
    pub fn createUser(u:Newuser,conn:&MysqlConnection)->bool{
        diesel::insert_into(user::table)
            .values(&u)
            .execute(conn)
            .is_ok()
    }
}