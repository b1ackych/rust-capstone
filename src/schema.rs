table! {
    use diesel::sql_types::*;
    use crate::diesel::sql_types::Text;

    vaults (id) {
        id -> Int4,
        user_id -> Text,
        encrypted_key -> Text,
        encrypted_data -> Text,
    }
}
