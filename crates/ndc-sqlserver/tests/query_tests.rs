pub mod common;

mod basic {
    use super::common::run_query;
    #[tokio::test]
    async fn select_by_pk() {
        let result = run_query("select_by_pk").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_5() {
        let result = run_query("select_5").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_int_and_string() {
        let result = run_query("select_int_and_string").await;
        insta::assert_json_snapshot!(result);
    }
}

mod predicates {
    use super::common::run_query;

    #[tokio::test]
    async fn select_where_name_like() {
        let result = run_query("select_where_name_like").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_name_not_like() {
        let result = run_query("select_where_name_not_like").await;
        insta::assert_json_snapshot!(result);
    }

    /*
    // this fails because empty responses don't return `{ rows: [] }` and instead return `{}`
    #[tokio::test]
    async fn select_where_album_id_less_than() {
        let result = run_query("select_where_album_id_less_than").await;
        insta::assert_json_snapshot!(result);
    }
    */
    #[tokio::test]
    async fn select_where_album_id_less_than_or_equal_to() {
        let result = run_query("select_where_album_id_less_than_or_equal_to").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_album_id_greater_than() {
        let result = run_query("select_where_album_id_greater_than").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_album_id_greater_than_or_equal_to() {
        let result = run_query("select_where_album_id_greater_than_or_equal_to").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_album_id_is_not_null() {
        let result = run_query("select_where_album_id_is_not_null").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_album_id_equals_self() {
        let result = run_query("select_where_album_id_equals_self").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_name_in() {
        let result = run_query("select_where_name_in").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_where_name_not_in() {
        let result = run_query("select_where_name_not_in").await;
        insta::assert_json_snapshot!(result);
    }

    /*
    // need to run query for each set of variables
    #[tokio::test]
    async fn select_where_variable() {
        let result = run_query("select_where_variable").await;
        insta::assert_json_snapshot!(result);
    }

    // need to run query for each set of variables
    #[tokio::test]
    async fn select_where_variable_int() {
        let result = run_query("select_where_variable_int").await;
        insta::assert_json_snapshot!(result);
    }

    // fix exists implementation
    #[tokio::test]
    async fn select_where_unrelated_exists() {
        let result = run_query("select_where_unrelated_exists").await;
        insta::assert_json_snapshot!(result);
    }

    // fix exists implementation
    #[tokio::test]
    async fn select_where_related_exists() {
        let result = run_query("select_where_related_exists").await;
        insta::assert_json_snapshot!(result);
    }
    */
}

/*
mod sorting {
    use super::common::run_query;
        #[tokio::test]
        // off by one error
        async fn select_order_by_name() {
            let result = run_query("select_order_by_name").await;
            insta::assert_json_snapshot!(result);
        }

    // syntax error, probably lateral joins
    #[tokio::test]
    async fn select_order_by_artist_name() {
        let result = run_query("select_order_by_artist_name").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_order_by_album_artist_name() {
        let result = run_query("select_order_by_album_artist_name").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_track_order_by_artist_id_and_album_title() {
        let result = run_query("select_track_order_by_artist_id_and_album_title").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_order_by_artist_name_with_name() {
        let result = run_query("select_order_by_artist_name_with_name").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_order_by_artist_album_count() {
        let result = run_query("select_order_by_artist_album_count").await;
        insta::assert_json_snapshot!(result);
    }
}
    */

mod aggregation {
    use super::common::run_query;

    #[tokio::test]
    async fn aggregate_count_albums() {
        let result = run_query("aggregate_count_albums").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn aggregate_count_albums_plus_field() {
        let result = run_query("aggregate_count_albums_plus_field").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn aggregate_count_artist_albums() {
        let result = run_query("aggregate_count_artist_albums").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn aggregate_count_artist_albums_plus_field() {
        let result = run_query("aggregate_count_artist_albums_plus_field").await;
        insta::assert_json_snapshot!(result);
    }
}

mod relationships {
    use super::common::run_query;
    #[tokio::test]
    async fn select_album_object_relationship_to_artist() {
        let result = run_query("select_album_object_relationship_to_artist").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn select_artist_array_relationship_to_album() {
        let result = run_query("select_artist_array_relationship_to_album").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn nested_array_relationships() {
        let result = run_query("nested_array_relationships").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn nested_object_relationships() {
        let result = run_query("nested_object_relationships").await;
        insta::assert_json_snapshot!(result);
    }

    #[tokio::test]
    async fn dup_array_relationship() {
        let result = run_query("dup_array_relationship").await;
        insta::assert_json_snapshot!(result);
    }
}
