#![doc = "GeekORM Database Migrations - v0.2.0"]
#![allow(unused_variables, non_upper_case_globals, missing_docs)]
use super::v0_1_1 as previous;
pub struct Migration;
impl geekorm::Migration for Migration {
    fn version() -> &'static str {
        "0.2.0"
    }
    fn previous() -> Option<Box<dyn geekorm::Migration>>
    where
        Self: Sized,
    {
        Some(Box::new(previous::Migration))
    }
    fn create_query() -> &'static str {
        include_str!("create.sql")
    }
    fn upgrade_query() -> &'static str {
        include_str!("upgrade.sql")
    }
    fn database(&self) -> &geekorm::Database {
        &Database
    }
}
geekorm::lazy_static! { geekorm :: Database { tables : vec ! [geekorm :: Table { name : String :: from ("Users") , columns : geekorm :: Columns :: from (vec ! [geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : true , unique : false , not_null : false , auto_increment : true } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("username") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : true , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("email") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : false , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("user_type") , column_type : geekorm :: ColumnType :: "Blob" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("password") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("created_at") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : }]) , database : Some (String :: from ("Database")) , } , geekorm :: Table { name : String :: from ("Sessions") , columns : geekorm :: Columns :: from (vec ! [geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : true , unique : false , not_null : false , auto_increment : true } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("token") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : }]) , database : Some (String :: from ("Database")) , } , geekorm :: Table { name : String :: from ("Posts") , columns : geekorm :: Columns :: from (vec ! [geekorm :: Column { name : String :: from ("id") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : true , unique : false , not_null : false , auto_increment : true } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("title") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("user") , column_type : geekorm :: ColumnType :: "ForeignKey" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "Users.id" , table_name : } , geekorm :: Column { name : String :: from ("created_at") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : } , geekorm :: Column { name : String :: from ("updated_at") , column_type : geekorm :: ColumnType :: "Text" , column_options : geekorm :: ColumnOptions { primary_key : false , unique : false , not_null : true , auto_increment : false } , alias : "" , foreign_key : "" , table_name : }]) , database : Some (String :: from ("Database")) , }] } }
