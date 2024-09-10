use async_graphql::{Context, Object};
use r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use climb_db::models;

pub struct Area(models::Area);

#[Object]
impl Area {
    async fn id(&self) -> &i32 {
        &self.0.id
    }

    async fn names(&self) -> Vec<String> {
        self.0.names
            .iter()
            .filter_map(|name| name.clone())
            .collect()
    }
}

pub struct Climb(models::Climb);

#[Object]
impl Climb {
    async fn id(&self) -> &i32 {
        &self.0.id
    }

    async fn names(&self) -> Vec<String> {
        self.0.names
            .iter()
            .filter_map(|name| name.clone())
            .collect()
    }
}

pub struct Formation(models::Formation);

#[Object]
impl Formation {
    async fn id(&self) -> &i32 {
        &self.0.id
    }

    async fn names(&self) -> Vec<String> {
        self.0.names
            .iter()
            .filter_map(|name| name.clone())
            .collect()
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns the area with the given id"
        )]
        id: i32,
    ) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::schema::areas::dsl::areas;

        let area = areas
            .find(id)
            .first::<models::Area>(&mut conn.unwrap());

        match area {
            Ok(_) => Some(Area(area.unwrap())),
            Err(_) => None,
        }
    }

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

    async fn formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns the formation with given id"
        )]
        id: i32,
    ) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::schema::formations::dsl::formations;

        let formation = formations
            .find(id)
            .first::<models::Formation>(&mut conn.unwrap());

        match formation {
            Ok(_) => Some(Formation(formation.unwrap())),
            Err(_) => None,
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Adds an area"
        )]
        names: Vec<String>,
    ) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::{ NewArea, Area };
        use climb_db::schema::areas;

        let new_area = NewArea { names: names.into_iter().map(Some).collect() };
        let result_area = diesel::insert_into(areas::table)
            .values(&new_area)
            .returning(Area::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error on saving area");

        Some(Area(result_area))
    }

    async fn remove_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes area with given id"
        )]
        id: i32,
    ) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::Area;
        use climb_db::schema::areas::dsl::{ areas, id as area_id };

        let area = diesel::delete(areas.filter(area_id.eq(id)))
            .returning(Area::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error removing area");

        Some(Area(area))
    }

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

    async fn remove_climb<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes climb with given id"
        )]
        id: i32,
    ) -> Option<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::Climb;
        use climb_db::schema::climbs::dsl::{ climbs, id as climb_id };

        let climb = diesel::delete(climbs.filter(climb_id.eq(id)))
            .returning(Climb::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error removing climb");

        Some(Climb(climb))
    }

    async fn add_formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Adds a formation"
        )]
        names: Vec<String>,
    ) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::{ NewFormation, Formation };
        use climb_db::schema::formations;

        let new_formation = NewFormation {
            names: names.into_iter().map(Some).collect(),
            ..Default::default()
        };
        let result_formation = diesel::insert_into(formations::table)
            .values(&new_formation)
            .returning(Formation::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error on saving formation");

        Some(Formation(result_formation))
    }

    async fn remove_formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes formation with given id"
        )]
        id: i32,
    ) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::Formation;
        use climb_db::schema::formations::dsl::{ formations, id as formation_id };

        let formation = diesel::delete(formations.filter(formation_id.eq(id)))
            .returning(Formation::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error removing formation");

        Some(Formation(formation))
    }
}
