table! {
    issues (id) {
        id -> Int8,
        project_id -> Int8,
        author -> Varchar,
        url -> Varchar,
        title -> Varchar,
        content -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Int8,
        name -> Varchar,
        url -> Varchar,
        watchers -> Nullable<Int8>,
        forks -> Nullable<Int8>,
        stars -> Nullable<Int8>,
    }
}

joinable!(issues -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    issues,
    projects,
);
