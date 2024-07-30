// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        address -> Nullable<Text>,
        state -> Bool,
        variable -> Integer,
        room -> Text,
    }
}

diesel::table! {
    houses (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        house -> Text,
    }
}

diesel::joinable!(devices -> rooms (room));
diesel::joinable!(rooms -> houses (house));

diesel::allow_tables_to_appear_in_same_query!(devices, houses, rooms,);
