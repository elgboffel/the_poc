
use cynic::{GraphQlResponse, QueryFragment};
use cynic::{http::ReqwestExt, QueryBuilder};
use serde::{Serialize};

#[cynic::schema("starwars")]
mod schema {}


#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(graphql_type = "Root")]
pub struct AllFilmsQuery {
    pub all_films: Option<FilmsConnection>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct FilmsConnection {
    pub films: Option<Vec<Option<Film>>>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct Film {
    pub id: cynic::Id,
    pub title: Option<String>,
    pub producers: Option<Vec<Option<String>>>,
}

pub async fn get_all_films() -> GraphQlResponse<AllFilmsQuery> {
    let client = reqwest::Client::new();
    let operation = AllFilmsQuery::build(());

    let response= client
        .post("https://swapi-graphql.netlify.app/.netlify/functions/index")
        .run_graphql(operation)
        .await
        .expect("Failed to fetch data");

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_films_query_gql_output() {
        use cynic::QueryBuilder;

        let operation = AllFilmsQuery::build(());

        insta::assert_snapshot!(operation.query);
    }
}
