//! The `graphql` module implements the GraphQL types, queries and mutations
//! that are available to users.

use crate::repository::Repository;

/// The query object defines all queries that the schema supports.
pub struct Query;

juniper::graphql_object!(Query: Repository |&self| {
    field apiVersion() -> & str {
        "1.0"
    }
});

/// The mutation object defines all mutations that the schema supports.
pub struct Mutation;

juniper::graphql_object!(Mutation: Repository | &self | {});

/// The GraphQL schema can be queries by users.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

/// Create a schema.
///
/// This method initializes the schema with the default query and mutation
/// objects.
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::graphql::create_schema;
    use crate::repository::Repository;
    use juniper::Variables;

    fn repo() -> Repository {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };

        Repository::with_test_transactions(&config.database_url().as_str())
    }

    #[test]
    fn execute_schema() {
        let (result, _errors) = juniper::execute(
            "query { apiVersion }",
            None,
            &create_schema(),
            &Variables::new(),
            &repo(),
        )
        .unwrap();

        assert_eq!(result, graphql_value!({ "apiVersion" : "1.0" }));
    }
}
