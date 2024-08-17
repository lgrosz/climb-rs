use async_graphql::{Context, Object};
use r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use climb_db::models;

pub struct Climb(models::Climb);

#[Object]
impl Climb {
    async fn id(&self) -> &i32 {
        &self.0.id
    }

    async fn names(&self) -> &Vec<Option<String>> {
        &self.0.names
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn climb<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns climb with given id"
        )]
        id: i32,
    ) -> Option<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::schema::climbs::dsl::climbs;

        let climb = climbs
            .find(id)
            .first::<models::Climb>(&mut conn.unwrap());

        match climb {
            Ok(_) => Some(Climb(climb.unwrap())),
            Err(_) => None,
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_climb<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Adds a climb"
        )]
        names: Vec<String>,
    ) -> Option<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::{ NewClimb, Climb };
        use climb_db::schema::climbs;

        let new_climb = NewClimb { names: names.into_iter().map(Some).collect() };
        let result_climb = diesel::insert_into(climbs::table)
            .values(&new_climb)
            .returning(Climb::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error on saving climb");

        Some(Climb(result_climb))
    }
}
