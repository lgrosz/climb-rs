use std::str::FromStr;

use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, Enum};
use climbing_grades::verm;
use r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use climb_db::models;

pub struct Area(i32);

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum GradeType {
    Vermin,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "GradeInput")]
pub struct Grade {
    #[graphql(name="type")]
    pub grade_type: GradeType,
    pub value: String,
}

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

        names.into_iter().flatten().collect()
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

        data.into_iter().map(Area).collect()
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

        data.into_iter().map(Formation).collect()
    }

    async fn climbs<'a>(&self, ctx: &Context<'a>) -> Vec<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::climb_belongs_to;

        let data = match climb_belongs_to::table
            .filter(climb_belongs_to::area_id.eq(&self.0))
            .select(climb_belongs_to::climb_id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(Climb).collect()
    }
}

pub struct Ascent(i32);

#[Object]
impl Ascent {
    async fn id(&self) -> &i32 {
        &self.0
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

        names.into_iter().flatten().collect()
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

    async fn grades<'a>(&self, ctx: &Context<'a>) -> Option<Vec<Grade>> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::climb_vermin_grades;

        climb_vermin_grades::table
            .filter(climb_vermin_grades::climb_id.eq(&self.0))
            .select(climb_vermin_grades::value)
            .load::<i32>(&mut conn)
            .ok()?
            .into_iter()
            .map(|value| verm::Grade::new(value as u8))
            .map(|grade| Some(Grade {
                grade_type: GradeType::Vermin,
                value: grade.to_string()
            }))
            .collect()
    }

    async fn area<'a>(&self, ctx: &Context<'a>) -> Option<Area> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::climb_belongs_to;

        climb_belongs_to::table
            .filter(climb_belongs_to::climb_id.eq(&self.0))
            .select(climb_belongs_to::area_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?
            .map(Area)
    }

    async fn formation<'a>(&self, ctx: &Context<'a>) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::climb_belongs_to;

        climb_belongs_to::table
            .filter(climb_belongs_to::climb_id.eq(&self.0))
            .select(climb_belongs_to::formation_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?
            .map(Formation)
    }

    async fn ascents<'a>(&self, ctx: &Context<'a>) -> Vec<Ascent> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::ascents;

        let data = match ascents::table
            .filter(ascents::climb_id.eq(&self.0))
            .select(ascents::id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(Ascent).collect()
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

        names.into_iter().flatten().collect()
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

        formation_belongs_to::table
            .filter(formation_belongs_to::formation_id.eq(&self.0))
            .select(formation_belongs_to::area_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?
            .map(Area)
    }

    async fn super_formation<'a>(&self, ctx: &Context<'a>) -> Option<Formation> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().ok()?;

        use climb_db::schema::formation_belongs_to;

        formation_belongs_to::table
            .filter(formation_belongs_to::formation_id.eq(&self.0))
            .select(formation_belongs_to::super_formation_id)
            .first::<Option<i32>>(&mut conn)
            .ok()?
            .map(Formation)
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

        data.into_iter().map(Formation).collect()
    }

    async fn climbs<'a>(&self, ctx: &Context<'a>) -> Vec<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };

        use climb_db::schema::climb_belongs_to;

        let data = match climb_belongs_to::table
            .filter(climb_belongs_to::formation_id.eq(&self.0))
            .select(climb_belongs_to::climb_id)
            .load::<i32>(&mut conn)
        {
            Ok(ids) => ids,
            Err(_) => Vec::new(),
        };

        data.into_iter().map(Climb).collect()
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

        let areas = result.into_iter().map(Area).collect();

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

        let climbs = result.into_iter().map(Climb).collect();

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

        let formations = result.into_iter().map(Formation).collect();

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
            desc = "Names to associate with the climb"
        )]
        names: Option<Vec<String>>,
        #[graphql(
            desc = "Descriptions to associate with the climb"
        )]
        descriptions: Option<Vec<KVPair>>,
        #[graphql(
            desc = "Grades to associate with the climb"
        )]
        grades: Option<Vec<Grade>>,
        #[graphql(
            desc = "Parent area id of the climb"
        )]
        area_id: Option<i32>,
        #[graphql(
            desc = "Parent formation id of the climb"
        )]
        formation_id: Option<i32>,
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let mut conn = pool.get().map_err(|e| e.to_string())?;

        conn.transaction(|conn| {
            use climb_db::models::NewClimb;
            use climb_db::schema::climbs;

            let new_climb = NewClimb {
                names: names.map_or_else(std::vec::Vec::new, |vec| vec.into_iter().map(Some).collect()),
            };

            let climb_id = diesel::insert_into(climbs::table)
                .values(&new_climb)
                .returning(climbs::id)
                .get_result::<i32>(conn)?;

            if let Some(descriptions) = descriptions {
                use crate::queries::set_climb_descriptions;
                set_climb_descriptions(conn, climb_id, descriptions)?;
            }

            if let Some(grades) = grades {
                use crate::queries::set_climb_grades;
                set_climb_grades(conn, climb_id, grades)?;
            }

            if let Some(area_id) = area_id {
                use crate::queries::set_climb_area_id;
                set_climb_area_id(conn, climb_id, area_id)?;
            }

            if let Some(formation_id) = formation_id {
                use crate::queries::set_climb_formation_id;
                set_climb_formation_id(conn, climb_id, formation_id)?;
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

    async fn add_climb_grade<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Climb id to add grade to"
        )]
        id: i32,
        #[graphql(
            desc = "Grade which to add"
        )]
        grade: Grade
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climb_vermin_grades;

        match grade.grade_type {
            GradeType::Vermin => {
                use climb_db::models::NewClimbVerminGrade;

                let grade = verm::Grade::from_str(grade.value.as_str()).map_err(|_|"Failed to parse grade")?;
                let db_grade = NewClimbVerminGrade { climb_id: id, value: grade.value() as i32 };

                let _ = diesel::insert_into(climb_vermin_grades::table)
                    .values(db_grade)
                    .execute(&mut conn)?;
            },
        };

        Ok(Climb(id))
    }

    async fn remove_climb_grade<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Climb id to remove grade from")] id: i32,
        #[graphql(desc = "Grade to remove")] grade: Grade,
    ) -> FieldResult<Climb> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::climb_vermin_grades::dsl::*;

        match grade.grade_type {
            GradeType::Vermin => {
                let parsed_grade = verm::Grade::from_str(grade.value.as_str())
                    .map_err(|_| "Failed to parse grade")?;

                let num_deleted = diesel::delete(
                    climb_vermin_grades
                        .filter(climb_id.eq(id).and(value.eq(parsed_grade.value() as i32))),
                )
                .execute(&mut conn)?;

                if num_deleted == 0 {
                    return Err("Grade not found for the specified climb".into());
                }
            }
        }

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

    async fn add_ascent<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Id of the ascended climb"
        )]
        climb_id: i32,
    ) -> FieldResult<Ascent> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();

        let mut conn = pool.get().map_err(|e| e.to_string())?;

        conn.transaction(|conn| {
            use climb_db::models::NewAscent;
            use climb_db::schema::ascents;

            let new_ascent = NewAscent {
                climb_id,
                ascent_date: None,
            };

            let ascent_id = diesel::insert_into(ascents::table)
                .values(&new_ascent)
                .returning(ascents::id)
                .get_result::<i32>(conn)?;

            Ok(Ascent(ascent_id))
        })
    }

    async fn remove_ascent<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "Id of the ascent"
        )]
        id: i32,
    ) -> FieldResult<Ascent> {
        let pool = ctx.data_unchecked::<Pool<ConnectionManager<PgConnection>>>();
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        use climb_db::schema::ascents;

        let _ = diesel::delete(ascents::table.filter(ascents::id.eq(id)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(Ascent(id))
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
                names: names.map_or_else(std::vec::Vec::new, |vec| vec.into_iter().map(Some).collect()),
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
