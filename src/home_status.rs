use super::schema::homebool;
use diesel;
use diesel::prelude::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "homebool"]
pub struct HomeStatus {
    pub id: i32,
    pub description: String,
    pub available: bool
}


#[derive(Insertable)]
#[table_name = "homebool"]
struct InsertableHomeStatus {
    description: String,
    available: bool
}

impl InsertableHomeStatus {
    fn from_home_status(home_status: HomeStatus) -> InsertableHomeStatus {
        InsertableHomeStatus {
            description: home_status.description,
            available: home_status.available
        }
    }
}

impl HomeStatus {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<HomeStatus>> {
        homebool::table.load::<HomeStatus>(&*connection)
    }
    
    pub fn get(id: i32, connection: &PgConnection) -> QueryResult<HomeStatus> {
        homebool::table.find(id).get_result::<HomeStatus>(connection)
    }
    
    pub fn insert(home_status: HomeStatus, connection: &PgConnection) -> QueryResult<HomeStatus> {
        diesel::insert_into(homebool::table)
            .values(&InsertableHomeStatus::from_home_status(home_status))
            .get_result(connection)
    }
    
    pub fn update(id: i32, home_status: HomeStatus, connection: &PgConnection) -> QueryResult<HomeStatus> {
        diesel::update(homebool::table.find(id))
            .set(&home_status)
            .get_result(connection)
    }
    
    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(homebool::table.find(id))
            .execute(connection)
    }
}
