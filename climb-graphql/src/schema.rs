use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use climb_db::models;

pub struct Area(i32);

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "KVPairInput")]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CoordinateInput")]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[Object]
impl Area {
    async fn id(&self) -> &i32 {
        &self.0
    }

    async fn names<'a>(&self, ctx: &Context<'a>) -> Vec<String> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::areas;

        let names = match areas::table
            .filter(areas::id.eq(&self.0))
            .select(areas::names)
            .first::<Vec<Option<String>>>(&mut conn)
        {
            Ok(names) => names,
            Err(_) => return Vec::new(),
        };

        names.into_iter().filter_map(|name| name).collect()
    }

    async fn super_area<'a>(&self, ctx: &Context<'a>) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::area_belongs_to;

        let data = area_belongs_to::table
            .filter(area_belongs_to::area_id.eq(&self.0))
            .select(area_belongs_to::super_area_id)
            .first::<i32>(&mut conn)
            .ok()?;

        Some(Area(data))
    }

    async fn sub_areas<'a>(&self, ctx: &Context<'a>) -> Vec<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::area_belongs_to;

        let data = match area_belongs_to::table
            .filter(area_belongs_to::super_area_id.eq(&self.0))
            .select(area_belongs_to::area_id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(|id| Area(id)).collect()
    }

    async fn formations<'a>(&self, ctx: &Context<'a>) -> Vec<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::formation_belongs_to;

        let data = match formation_belongs_to::table
            .filter(formation_belongs_to::area_id.eq(&self.0))
            .select(formation_belongs_to::formation_id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(|id| Formation(id)).collect()
    }
}

pub struct Climb(i32);

#[Object]
impl Climb {
    async fn id(&self) -> &i32 {
        &self.0
    }

    async fn names<'a>(&self, ctx: &Context<'a>) -> Vec<String> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::climbs;

        let names = match climbs::table
            .filter(climbs::id.eq(&self.0))
            .select(climbs::names)
            .first::<Vec<Option<String>>>(&mut conn)
        {
            Ok(names) => names,
            Err(_) => return Vec::new(),
        };

        names.into_iter().filter_map(|name| name).collect()
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
            .filter(climb_descriptions::climb_id.eq(&self.0))
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
            .filter(climb_grades::climb_id.eq(&self.0))
            .select((grade_types::name, grades::value))
            .load::<(String, String)>(&mut conn)
            .ok()?;

        Some(data.into_iter().map(|(key, value)| KVPair { key, value }).collect())
    }
}

pub struct Formation(i32);

#[Object]
impl Formation {
    async fn id(&self) -> &i32 {
        &self.0
    }

    async fn names<'a>(&self, ctx: &Context<'a>) -> Vec<String> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::formations;

        let names = match formations::table
            .filter(formations::id.eq(&self.0))
            .select(formations::names)
            .first::<Vec<Option<String>>>(&mut conn)
        {
            Ok(names) => names,
            Err(_) => return Vec::new(),
        };

        names.into_iter().filter_map(|name| name).collect()
    }

    async fn location<'a>(&self, ctx: &Context<'a>) -> Option<Coordinate> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return None,
        };

        use climb_db::schema::formations;
        use postgis_diesel::types::Point;

        let location = match formations::table
            .filter(formations::id.eq(&self.0))
            .select(formations::location)
            .first::<Option<Point>>(&mut conn)
        {
            Ok(location) => location,
            Err(_) => return None,
        };

        location.map(|loc| Coordinate { latitude: loc.x, longitude: loc.y })
    }

    async fn area<'a>(&self, ctx: &Context<'a>) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::formation_belongs_to;

        let data = formation_belongs_to::table
            .filter(formation_belongs_to::formation_id.eq(&self.0))
            .select(formation_belongs_to::area_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?;

        data.map(|id| Area(id))
    }

    async fn super_formation<'a>(&self, ctx: &Context<'a>) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::formation_belongs_to;

        let data = formation_belongs_to::table
            .filter(formation_belongs_to::formation_id.eq(&self.0))
            .select(formation_belongs_to::super_formation_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?;

        data.map(|id| Formation(id))
    }

    async fn sub_formations<'a>(&self, ctx: &Context<'a>) -> Vec<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::formation_belongs_to;

        let data = match formation_belongs_to::table
            .filter(formation_belongs_to::super_formation_id.eq(&self.0))
            .select(formation_belongs_to::formation_id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(|id| Formation(id)).collect()
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
            .select(areas::id)
            .load(&mut conn)
            .map_err(|e| e.to_string())?;

        let areas = result.into_iter().map(|id| Area(id)).collect();

        Ok(areas)
    }

    async fn area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns the area with the given id"
        )]
        id: i32,
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::areas;

        let area_id = areas::table
            .find(id)
            .select(areas::id)
            .first(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
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
            .select(climbs::id)
            .load::<i32>(&mut conn)
            .map_err(|e| e.to_string())?;

        let climbs = result.into_iter().map(|id| Climb(id)).collect();

        Ok(climbs)
    }

    async fn climb<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns climb with given id"
        )]
        id: i32,
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climbs;

        let climb_id = climbs::table
            .find(id)
            .select(climbs::id)
            .first::<i32>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(climb_id))
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
            .select(formations::id)
            .load::<i32>(&mut conn)
            .map_err(|e| e.to_string())?;

        let formations = result.into_iter().map(|id| Formation(id)).collect();

        Ok(formations)
    }

    async fn formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Returns the formation with given id"
        )]
        id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;

        let formation_id = formations::table
            .find(id)
            .select(formations::id)
            .first::<i32>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(formation_id))
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_area<'a>(
        &self,
        ctx: &Context<'a>,
        names: Option<Vec<String>>,
        super_area_id: Option<i32>,
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let mut conn = pool.get().map_err(|e| e.to_string())?;

        conn.transaction(|conn| {
            use climb_db::models::NewArea;
            use climb_db::schema::areas;

            let new_area = NewArea { names: vec!() };
            let area_id = diesel::insert_into(areas::table)
                .values(&new_area)
                .returning(areas::id)
                .get_result::<i32>(conn)
                .map_err(|e| e.to_string())?;

            if let Some(names) = names {
                use crate::queries::set_area_names;
                set_area_names(conn, area_id, names)?;
            }

            if let Some(super_area_id) = super_area_id {
                use crate::queries::set_area_super_area_id;
                set_area_super_area_id(conn, area_id, super_area_id)?;
            }

            Ok(Area(area_id))
        })
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

        let area_id = areas::table
            .find(id)
            .select(areas::id)
            .first(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
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

        let area_id = diesel::update(areas::table)
            .filter(areas::id.eq(id))
            .set(areas::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .returning(areas::id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
    }

    async fn set_super_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Area id to set 'super area' of"
        )]
        id: i32,
        #[graphql(
            desc = "Super area id"
        )]
        super_area_id: i32
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::area_belongs_to;
        use diesel::upsert::excluded;

        let new_area_belongs_to = models::NewAreaBelongsTo {
            area_id: id,
            super_area_id
        };

        diesel::insert_into(area_belongs_to::table)
            .values(new_area_belongs_to)
            .on_conflict(area_belongs_to::area_id)
            .do_update()
            .set(area_belongs_to::super_area_id.eq(excluded(area_belongs_to::super_area_id)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        use climb_db::schema::areas;

        let area_id = areas::table
            .find(id)
            .select(areas::id)
            .first(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
    }
    async fn clear_super_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Area id to clear 'super area' of"
        )]
        id: i32,
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::area_belongs_to;

        let area_id = diesel::delete(
            area_belongs_to::table
                .filter(area_belongs_to::area_id.eq(id)))
            .returning(area_belongs_to::area_id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
    }

    async fn remove_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes area with given id"
        )]
        id: i32,
    ) -> FieldResult<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::areas;

        let area_id = diesel::delete(areas::table.filter(areas::id.eq(id)))
            .returning(areas::id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Area(area_id))
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
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let mut conn = pool.get().map_err(|e| e.to_string())?;

        conn.transaction(|conn| {
            use climb_db::models::NewClimb;
            use climb_db::schema::climbs;

            let new_climb = NewClimb { names: vec!() };

            let climb_id = diesel::insert_into(climbs::table)
                .values(&new_climb)
                .returning(climbs::id)
                .get_result::<i32>(conn)?;

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

            Ok(Climb(climb_id))
        })
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

        let _ = diesel::update(climbs::table)
            .filter(climbs::id.eq(id))
            .set(climbs::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_append(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(id))
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

        let _ = diesel::update(climbs::table)
            .filter(climbs::id.eq(id))
            .set(climbs::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(id))
    }

    async fn remove_climb<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes climb with given id"
        )]
        id: i32,
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climbs;

        let _ = diesel::delete(climbs::table.filter(climbs::id.eq(id)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Climb(id))
    }

    async fn add_formation<'a>(
        &self,
        ctx: &Context<'a>,
        names: Option<Vec<String>>,
        area_id: Option<i32>,
        super_formation_id: Option<i32>,
        location: Option<Coordinate>,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let mut conn = pool.get().map_err(|e| e.to_string())?;

        conn.transaction(|conn| {
            use climb_db::models::NewFormation;
            use climb_db::schema::formations;
            use postgis_diesel::types::Point;

            let new_formation = NewFormation {
                names: names.map_or_else(|| vec![], |vec| vec.into_iter().map(Some).collect()),
                location: location.map(|loc| Point { x: loc.latitude, y: loc.longitude, srid: None }),
            };

            let formation_id = diesel::insert_into(formations::table)
                .values(&new_formation)
                .returning(formations::id)
                .get_result::<i32>(conn)
                .map_err(|e| e.to_string())?;

            if let Some(area_id) = area_id {
                use crate::queries::set_formation_area_id;
                set_formation_area_id(conn, formation_id, area_id)?;
            }

            if let Some(super_formation_id) = super_formation_id {
                use crate::queries::set_formation_super_formation_id;
                set_formation_super_formation_id(conn, formation_id, super_formation_id)?;
            }

            Ok(Formation(formation_id))
        })
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

        let _ = diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_append(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
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

        let _ = diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::names.eq(sql::<Array<Nullable<Text>>>(
                &format!("array_remove(names, '{}')", name)
            )))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
    }

    async fn set_formation_location<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to set location of"
        )]
        id: i32,
        #[graphql(
            desc = "Location of the formation"
        )]
        location: Coordinate
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;
        use postgis_diesel::types::Point;

        let _ = diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::location.eq(Point {
                x: location.latitude,
                y: location.longitude,
                srid: None,
            }))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
    }

    async fn clear_formation_location<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to set location of"
        )]
        id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;
        use postgis_diesel::types::Point;

        let _ = diesel::update(formations::table)
            .filter(formations::id.eq(id))
            .set(formations::location.eq(None::<Point>))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
    }

    async fn set_formation_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to area of"
        )]
        id: i32,
        #[graphql(
            desc = "Area id"
        )]
        area_id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formation_belongs_to;
        use diesel::upsert::excluded;

        let new_formation_belongs_to = models::NewFormationBelongsTo {
            formation_id: id,
            area_id: Some(area_id),
            super_formation_id: None,
        };

        diesel::insert_into(formation_belongs_to::table)
            .values(new_formation_belongs_to)
            .on_conflict(formation_belongs_to::formation_id)
            .do_update()
            .set((
                formation_belongs_to::area_id.eq(excluded(formation_belongs_to::area_id)),
                formation_belongs_to::super_formation_id.eq(None::<i32>),
            ))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
    }

    async fn set_formation_super_formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to super-formation of"
        )]
        id: i32,
        #[graphql(
            desc = "Super formation id"
        )]
        super_formation_id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formation_belongs_to;
        use diesel::upsert::excluded;

        let new_formation_belongs_to = models::NewFormationBelongsTo {
            formation_id: id,
            area_id: None,
            super_formation_id: Some(super_formation_id),
        };

        diesel::insert_into(formation_belongs_to::table)
            .values(new_formation_belongs_to)
            .on_conflict(formation_belongs_to::formation_id)
            .do_update()
            .set((
                formation_belongs_to::area_id.eq(excluded(formation_belongs_to::area_id)),
                formation_belongs_to::super_formation_id.eq(None::<i32>),
            ))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(id))
    }

    async fn clear_formation_area<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to area of"
        )]
        id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formation_belongs_to;

        let formation_id = diesel::delete(
            formation_belongs_to::table
                .filter(formation_belongs_to::formation_id.eq(id)))
            .returning(formation_belongs_to::formation_id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(formation_id))
    }

    // TODO This same thing as `clear_formation_area`. Is there a common name I can use to avoid
    // this duplication? Or from an outside view, does it make sense to keep them separate?
    async fn clear_formation_super_formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Formation id to super-formation of"
        )]
        id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formation_belongs_to;

        let formation_id = diesel::delete(
            formation_belongs_to::table
                .filter(formation_belongs_to::formation_id.eq(id)))
            .returning(formation_belongs_to::formation_id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(formation_id))
    }

    async fn remove_formation<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Removes formation with given id"
        )]
        id: i32,
    ) -> FieldResult<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::formations;

        let formation_id = diesel::delete(formations::table.filter(formations::id.eq(id)))
            .returning(formations::id)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Formation(formation_id))
    }
}
