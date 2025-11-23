// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        nazov -> Text,
        popis -> Text,
        priorita -> Integer,
        planovany_zaciatok -> Text,
        skutocny_zaciatok -> Nullable<Text>,
        planovane_trvanie -> Integer,
        skutocne_trvanie -> Nullable<Integer>,
    }
}
