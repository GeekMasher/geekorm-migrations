use super::Migration;

#[doc = r" The migrate function is used to apply the migration to the database."]
pub(super) async fn migrate<'a, C>(connection: &'a C) -> Result<(), geekorm::Error>
where
    C: geekorm::GeekConnection<Connection = C> + 'a,
{
    todo!("Migrate the database to version 0.1.3")
}
