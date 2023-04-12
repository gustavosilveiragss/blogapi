
use actix::{Actor, Addr, SyncContext};
use diesel::{
  PgConnection,
  r2d2::{ConnectionManager, Pool}
};

pub struct AppState {
  pub db: Addr<DbActor>
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
  type Context = SyncContext<Self>;
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
  let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
  
  Pool::builder().build(manager).expect("Error building a connection pool")
}

#[macro_export]
macro_rules! derive_with_relations {
    (
        $holder_type:ty,
        $struct_name:ident {
            $($field_name:ident : $field_ty:ty),* $(,)?
        },
        {
            $($relational_name:ident : $relational_ty:ty),* $(,)?
        }
    ) => {
        #[derive(Queryable, Debug, Serialize)]
        pub struct $struct_name {
            $($field_name : $field_ty),*,
            $($relational_name : $relational_ty),*
        }

        impl $struct_name {
            pub fn build(holder: $holder_type, $($relational_name: $relational_ty,)*) -> Self {
                Self {
                    $($field_name : holder.$field_name),*,
                    $($relational_name : $relational_name),*
                }
            }
        }
    };
}