use diesel::pg::PgConnection;
use diesel::result::Error;


/// Allows us to use natural Model::update syntax instead of diesel::update
pub trait CRUD {
  type Form;
  type IdType;

  fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error>
  where 
    Self: Sized;
  fn read(conn: &PgConnection, id: Self::IdType) -> Result<Self, Error>
  where
    Self: Sized;
  fn update(conn: &PgConnection, id: Self::IdType, form: &Self::Form) -> Result<Self, Error>
  where
    Self: Sized;
  fn delete(conn: &PgConnection, id: Self::IdType) -> Result<Self, Error>
  where
    Self: Sized;
}