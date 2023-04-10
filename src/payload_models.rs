use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::Serialize;

use crate::db_models::{Category, Post, User};

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

derive_with_relations!(
    Post,
    PostWithAuthorCategory {
        id: i64,
        created_at: DateTime<Utc>,
        title: String,
        subtitle: String,
        content: String,
        published: bool,
        author_id: i32,
        category_id: i32,
    },
    {
        author: User,
        category: Category
    }
);

