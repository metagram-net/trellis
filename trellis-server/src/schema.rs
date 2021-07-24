table! {
    settings (id) {
        id -> Uuid,
        data -> Jsonb,
        user_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
