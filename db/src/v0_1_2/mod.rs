#![doc = "GeekORM Database Migrations - 2025-01-14 23:15:30.853226847 UTC"]
#![allow(unused_variables, non_upper_case_globals)]
use super::v0_1_1 as previous;
pub struct Migration;
impl geekorm::Migration for Migration {
    fn version() -> &'static str {
        "0.1.2"
    }
    #[doc = r" The migrate function is used to apply the migration to the database."]
    async fn migrate<'a, C>(connection: &'a C) -> Result<(), geekorm::Error>
    where
        C: geekorm::GeekConnection<Connection = C> + 'a,
    {
        Ok(())
    }
    fn create_query() -> &'static str {
        include_str!("create.sql")
    }
    fn upgrade_query() -> &'static str {
        include_str!("upgrade.sql")
    }
    fn previous() -> Option<Box<dyn geekorm::Migration>>
    where
        Self: Sized,
    {
        Some(Box::new(previous::Migration))
    }
    fn database(&self) -> &geekorm::Database {
        &Database
    }
}
geekorm::lazy_static! { pub static ref Database : Box < geekorm :: Database > = Box :: new (geekorm :: Database { tables : Vec :: from ([geekorm :: Table { name : String :: from ("Users") , columns : geekorm :: Columns { columns : Vec :: from ([geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: Identifier (geekorm :: ColumnTypeOptions { primary_key : true , unique : false , not_null : false , foreign_key : String :: from ("") , auto_increment : true , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("username") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : true , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("email") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : false , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("user_type") , column_type : geekorm :: ColumnType :: Blob (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("password") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("created_at") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , }]) } } , geekorm :: Table { name : String :: from ("Sessions") , columns : geekorm :: Columns { columns : Vec :: from ([geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: Identifier (geekorm :: ColumnTypeOptions { primary_key : true , unique : false , not_null : false , foreign_key : String :: from ("") , auto_increment : true , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("token") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , }]) } } , geekorm :: Table { name : String :: from ("Posts") , columns : geekorm :: Columns { columns : Vec :: from ([geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: Identifier (geekorm :: ColumnTypeOptions { primary_key : true , unique : false , not_null : false , foreign_key : String :: from ("") , auto_increment : true , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("title") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("user") , column_type : geekorm :: ColumnType :: ForeignKey (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("Users.id") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("created_at") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , } , geekorm :: Column { name : String :: from ("updated_at") , column_type : geekorm :: ColumnType :: Text (geekorm :: ColumnTypeOptions { primary_key : false , unique : false , not_null : true , foreign_key : String :: from ("") , auto_increment : false , }) , alias : String :: from ("") , skip : false , }]) } }]) }) ; }
