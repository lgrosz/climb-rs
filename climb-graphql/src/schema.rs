use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use climb_db::models;

pub struct Area(models::Area);

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "KVPairInput")]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

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

    async fn descriptions<'a>(&self, ctx: &Context<'a>) -> Option<Vec<KVPair>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::{climb_descriptions, climb_description_types};

        // TODO This would be better if it was async
        let data = climb_descriptions::table
            .inner_join(
                climb_description_types::table.on(
                    climb_descriptions::climb_description_type_id.eq(climb_description_types::id)
                    )
                )
            .filter(climb_descriptions::climb_id.eq(&self.0.id))
            .select((climb_description_types::name, climb_descriptions::value))
            .load::<(String, String)>(&mut conn)
            .ok()?;

        Some(data.into_iter().map(|(key, value)| KVPair { key, value }).collect())
    }

    async fn grades<'a>(&self, ctx: &Context<'a>) -> Option<Vec<KVPair>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::{climb_grades, grade_types, grades};

        // TODO This would be better if it was async
        let data = climb_grades::table
            .inner_join(
                grades::table.on(
                    climb_grades::grade_id.eq(grades::id)
                    )
                )
            .inner_join(
                grade_types::table.on(
                    grades::grade_type_id.eq(grade_types::id)
                )
            )
            .filter(climb_grades::climb_id.eq(&self.0.id))
            .select((grade_types::name, grades::value))
            .load::<(String, String)>(&mut conn)
            .ok()?;

        Some(data.into_iter().map(|(key, value)| KVPair { key, value }).collect())
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
    async fn areas<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Parent area id"
        )]
        area_id: Option<i32>,
    ) -> FieldResult<Vec<Area>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::{areas,area_belongs_to};

        let query = areas::table
            .left_join(area_belongs_to::table.on(area_belongs_to::area_id.eq(areas::id)))
            .into_boxed();

        let query = if let Some(id) = area_id {
            query.filter(area_belongs_to::area_id.eq(id))
        } else {
            query
        };

        let result = query
            .select(areas::all_columns)
            .load::<models::Area>(&mut conn)
            .map_err(|e| e.to_string())?;

        let areas = result.into_iter().map(|area| Area(area)).collect();

        Ok(areas)
    }

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

    async fn climbs<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Parent area id"
        )]
        area_id: Option<i32>,
        #[graphql(
            desc = "Parent formation id"
        )]
        formation_id: Option<i32>
    ) -> FieldResult<Vec<Climb>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::{climbs,climb_belongs_to};

        let query = climbs::table
            .left_join(climb_belongs_to::table.on(climb_belongs_to::climb_id.eq(climbs::id)))
            .into_boxed();

        let query = if let Some(id) = area_id {
            query.filter(climb_belongs_to::area_id.eq(id))
        } else {
            query
        };

        let query = if let Some(id) = formation_id {
            query.filter(climb_belongs_to::formation_id.eq(id))
        } else {
            query
        };

        let result = query
            .select(climbs::all_columns)
            .load::<models::Climb>(&mut conn)
            .map_err(|e| e.to_string())?;

        let climbs = result.into_iter().map(|climb| Climb(climb)).collect();

        Ok(climbs)
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

    async fn formations<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Parent area id"
        )]
        area_id: Option<i32>,
        #[graphql(
            desc = "Parent formation id"
        )]
        formation_id: Option<i32>
    ) -> FieldResult<Vec<Formation>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::{formations,formation_belongs_to};

        let query = formations::table
            .left_join(formation_belongs_to::table.on(formation_belongs_to::formation_id.eq(formations::id)))
            .into_boxed();

        let query = if let Some(id) = area_id {
            query.filter(formation_belongs_to::area_id.eq(id))
        } else {
            query
        };

        let query = if let Some(id) = formation_id {
            query.filter(formation_belongs_to::formation_id.eq(id))
        } else {
            query
        };

        let result = query
            .select(formations::all_columns)
            .load::<models::Formation>(&mut conn)
            .map_err(|e| e.to_string())?;

        let formations = result.into_iter().map(|formation| Formation(formation)).collect();

        Ok(formations)
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
    ) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::{ NewArea, Area };
        use climb_db::schema::areas;

        let new_area = NewArea { names: vec!() };
        let result_area = diesel::insert_into(areas::table)
            .values(&new_area)
            .returning(Area::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error on saving area");

        Some(Area(result_area))
    }

    async fn add_area_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Area id to add name to"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to add"
        )]
        name: String
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::areas;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(areas::table)
            .filter(areas::id.eq(id))
            .set(areas::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_append(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_area = areas::table
            .find(id)
            .first::<models::Area>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(updated_area))
    }

    async fn remove_area_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Area id to remove name from"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to remove"
        )]
        name: String
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::areas;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(areas::table)
            .filter(areas::id.eq(id))
            .set(areas::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_area = areas::table
            .find(id)
            .first::<models::Area>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(updated_area))
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
        descriptions: Option<Vec<KVPair>>,
        #[graphql(
            desc = "Grades to associate with the climb"
        )]
        grades: Option<Vec<KVPair>>,
    ) -> Option<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        // Start a new transaction
        let mut conn = pool.get().ok()?;
        conn.transaction(|conn| {
            use climb_db::models::{NewClimb, Climb};
            use climb_db::schema::climbs;

            // Insert the new climb
            let new_climb = NewClimb { names: vec!() };
            let result_climb = diesel::insert_into(climbs::table)
                .values(&new_climb)
                .returning(Climb::as_returning())
                .get_result::<Climb>(conn)?;

            let climb_id = result_climb.id; // Assuming `id` is the primary key field in `Climb`

            use climb_db::schema::climb_description_types;

            let descriptions = descriptions.unwrap_or_default();

            // Map keys to climb_description_type_id
            let description_keys: Vec<String> = descriptions.iter().map(|kv| kv.key.clone()).collect();
            let type_ids_map: std::collections::HashMap<String, i32> = climb_description_types::table
                .filter(climb_description_types::name.eq_any(description_keys))
                .select((climb_description_types::name, climb_description_types::id))
                .load::<(String, i32)>(conn)?
                .into_iter()
                .collect::<std::collections::HashMap<_, _>>();

            use climb_db::models::NewClimbDescription;
            use climb_db::schema::climb_descriptions;

            // Insert descriptions
            let new_descriptions: Vec<NewClimbDescription> = descriptions.into_iter()
                .filter_map(|kv| {
                    type_ids_map.get(&kv.key).map(|type_id| NewClimbDescription {
                        climb_id,
                        climb_description_type_id: *type_id,
                        value: kv.value,
                    })
                })
            .collect();

            diesel::insert_into(climb_descriptions::table)
                .values(&new_descriptions)
                .execute(conn)?;

            let grades = grades.unwrap_or_default();

            use climb_db::schema::grade_types;

            // Map keys to grade_types::id
            let grade_keys: Vec<String> = grades.iter().map(|kv| kv.key.clone()).collect();
            let grade_type_ids_map: std::collections::HashMap<String, i32> = grade_types::table
                .filter(grade_types::name.eq_any(grade_keys))
                .select((grade_types::name, grade_types::id))
                .load::<(String, i32)>(conn)?
                .into_iter()
                .collect::<std::collections::HashMap<_, _>>();

            for kv in grades {
                let grade_key = kv.key;
                let grade_value = kv.value;

                use climb_db::schema::grades;

                if let Some(&grade_type_id) = grade_type_ids_map.get(&grade_key) {
                    let grade_id = diesel::insert_into(grades::table)
                        .values((
                                grades::value.eq(grade_value.clone()),
                                grades::grade_type_id.eq(grade_type_id),
                        ))
                        .on_conflict((grades::value, grades::grade_type_id))
                        .do_update()
                        .set(grades::value.eq(grade_value.clone()))
                        .returning(grades::id)
                        .get_result::<i32>(conn)?;

                    use climb_db::schema::climb_grades;

                    diesel::insert_into(climb_grades::table)
                        .values((
                                climb_grades::climb_id.eq(climb_id),
                                climb_grades::grade_id.eq(grade_id),
                        ))
                        .execute(conn)?;
                }
            }

            diesel::result::QueryResult::Ok(Climb(result_climb))
        }).ok()
    }

    async fn add_climb_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Climb id to add name to"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to add"
        )]
        name: String
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climbs;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(climbs::table)
            .filter(climbs::id.eq(id))
            .set(climbs::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_append(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_climb = climbs::table
            .find(id)
            .first::<models::Climb>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(updated_climb))
    }

    async fn remove_climb_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Climb id to remove name from"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to remove"
        )]
        name: String
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climbs;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(climbs::table)
            .filter(climbs::id.eq(id))
            .set(climbs::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_climb = climbs::table
            .find(id)
            .first::<models::Climb>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(updated_climb))
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
    ) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let conn = pool.get();
        if conn.is_err() {
            // How can I propogate errors?
            return None;
        }

        use climb_db::models::{ NewFormation, Formation };
        use climb_db::schema::formations;

        let new_formation = NewFormation { ..Default::default() };
        let result_formation = diesel::insert_into(formations::table)
            .values(&new_formation)
            .returning(Formation::as_returning())
            .get_result(&mut conn.unwrap())
            .expect("Error on saving formation");

        Some(Formation(result_formation))
    }

    async fn add_formation_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to add name to"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to add"
        )]
        name: String
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_append(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_formation = formations::table
            .find(id)
            .first::<models::Formation>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(updated_formation))
    }

    async fn remove_formation_name<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to remove name from"
        )]
        id: i32,
        #[graphql(
            desc = "Name which to remove"
        )]
        name: String
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;
        use diesel::dsl::sql;
        use diesel::sql_types::{Array,Nullable,Text};

        diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        let updated_formation = formations::table
            .find(id)
            .first::<models::Formation>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(updated_formation))
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
