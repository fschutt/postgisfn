use Error;
use Table;
use Raster;

use traits::{_2d, _3d, Dimension};
use geometry::*;
use index::Index;
use raster::Raster;
use spheroid::Spheroid;

pub struct Schema {
    tables: Vec<Table>,
}

/// API predefines
pub type _2278      = ();
pub type _89856     = ();
pub type _89860     = ();
pub type _17        = ();
pub type _604       = ();
pub type _701       = ();
pub type _89887     = ();
pub type _89891     = ();
pub type _89895     = ();
pub type _2281      = ();
pub type _25        = ();
pub type _21        = ();
pub type _2279      = ();
pub type _89868     = ();
pub type _90146     = ();
pub type _603       = ();
pub type _2249      = ();
pub type _90260     = ();
pub type _90299     = ();
pub type _28        = ();
pub type _90480     = ();
pub type _1043      = ();
pub type _90744     = ();
pub type _90797     = ();
pub type _20        = ();
pub type _90828     = ();
pub type _1022      = ();
pub type _90757     = ();
pub type _91169     = ();
pub type _1009      = ();
pub type _1231      = ();
pub type _1000      = ();
pub type _2205      = ();

/// Public API for the schema
impl Schema {

    #[inline]
    pub fn box2df_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn gidx_in(&mut self) -> Result<Index, Error> { }

    #[inline]
    pub fn gidx_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn raster_hash(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn geometry_lt<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _postgis_deprecate(&mut self,  oldname: &str, newname: &str, version: &str) -> Result<_2278, Error> { }

    #[inline]
    pub fn spheroid_in(&mut self) -> Result<_89856, Error> { }

    #[inline]
    pub fn spheroid_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn geometry_in(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn geometry_typmod_in(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn geometry_typmod_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn geometry_analyze(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_recv(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry_send(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn point(&mut self) -> Result<Point<Dimension>, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn path(&mut self) -> Result<Path<Dimension>, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn polygon(&mut self) -> Result<_604, Error> { }

    #[inline]
    pub fn st_x(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_y(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_z(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_m(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn box3d_in(&mut self) -> Result<Geography<_3d>, Error> { }

    #[inline]
    pub fn box3d_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn box2d_in(&mut self) -> Result<Geography<_2d>, Error> { }

    #[inline]
    pub fn box2d_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn box2df_in(&mut self) -> Result<_89895, Error> { }

    #[inline]
    pub fn geometry_le<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_gt<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_ge<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_eq<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_cmp<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<u32, Error> { }

    #[inline]
    pub fn geometry_gist_distance_2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_gist_consistent_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_gist_compress_2d(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_penalty_2d(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_picksplit_2d(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_union_2d(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_same_2d<T>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, catalog: _2281) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_decompress_2d(&mut self) -> Result<_2281, Error> { }
/*
    #[inline]
    pub fn _postgis_selectivity(&mut self, tbl, att_name, geom: GeometryType<T>,  mode) -> Result<_701, Error> { }
*/
    #[inline]
    pub fn _postgis_join_selectivity(&mut self) -> Result<_701, Error> { }
/*
    #[inline]
    pub fn _postgis_stats(&mut self, tbl, att_name) -> Result<_25, Error> { }
*/
    #[inline]
    pub fn gserialized_gist_sel_2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn gserialized_gist_sel_nd(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn gserialized_gist_joinsel_2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn gserialized_gist_joinsel_nd(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_overlaps<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_same<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_distance_centroid<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_distance_box<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_contains<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_within<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_left<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_overleft<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_below<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_overbelow<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_overright<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_right<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_overabove<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_above<T: Dimension>(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_gist_consistent_nd(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_gist_compress_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_penalty_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_picksplit_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_union_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_same_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_gist_decompress_nd(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geometry_overlaps_nd(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_distance_centroid_nd(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_distance_cpa(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn geometry_gist_distance_nd(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_shiftlongitude(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_wrapx(&mut self, geom: GeometryType<T>, wrap,move) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_shift_longitude(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_xmin(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_ymin(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_zmin(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_xmax(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_ymax(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_zmax(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_expand(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_expand(&mut self, box,dx,dy) -> Result<_89891, Error> { }

    #[inline]
    pub fn postgis_getbbox(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_makebox2d(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_estimatedextent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_estimatedextent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_estimated_extent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_estimatedextent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_estimated_extent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_findextent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_find_extent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_findextent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_find_extent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn postgis_addbbox(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_dropbbox(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_hasbbox(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_memsize(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_mem_size(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_summary(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_npoints(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_nrings(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_3dlength(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_lengthspheroid(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_3dlength_spheroid(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length_spheroid(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length2dspheroid(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length2d_spheroid(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_3dperimeter(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_perimeter2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_perimeter(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_area2d(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_area(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distancespheroid(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, spheroid: Spheroid) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distance_spheroid(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, spheroid: Spheroid) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_pointinsidecircle(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_point_inside_circle(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_azimuth(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_force2d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_2d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force3dz(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_3dz(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force3d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_3d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force3dm(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_3dm(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force4d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_4d(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_forcecollection(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_force_collection(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_collectionextract(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_collectionhomogenize(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multi(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_forcecurve(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_forcesfs(&mut self, version: Option<&str>) -> Result<_89860, Error> { }

/*
    #[inline]
    pub fn st_expand<T: Dimension>(&mut self) -> Result<Geography<T>, Error> { }

    #[inline]
    pub fn st_expand(&mut self, box,dx,dy,dz) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_expand(&mut self) -> Result<_89860, Error> { }
*/

    #[inline]
    pub fn st_expand(&mut self, geom: GeometryType<T>, dx,dy,dz,dm) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_envelope(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_boundingdiagonal(&mut self, geom: GeometryType<T>, fits) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_reverse(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_forcerhr(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_noop(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_normalize(&mut self, geom) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_zmflag(&mut self) -> Result<_21, Error> { }

    #[inline]
    pub fn st_ndims(&mut self) -> Result<_21, Error> { }

    #[inline]
    pub fn st_asewkt(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_astwkb(&mut self, geom: GeometryType<T>, prec,prec_z,prec_m,with_sizes,with_boxes) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astwkb(&mut self, geom: GeometryType<T>, ids,prec,prec_z,prec_m,with_sizes,with_boxes) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asewkb(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_ashexewkb(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_ashexewkb(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asewkb(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_aslatlontext(&mut self, geom: GeometryType<T>, tmpl) -> Result<_25, Error> { }

    #[inline]
    pub fn geomfromewkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromewkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromtwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geomfromewkt(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromewkt(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_cache_bbox(&mut self) -> Result<_2279, Error> { }

    #[inline]
    pub fn st_makepoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makepoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makepoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makepointm(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_3dmakebox(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_makeline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linefrommultipoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makeline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_addpoint(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, index: Option<u32>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_removepoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_setpoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makeenvelope(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makepolygon(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makepolygon(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buildarea(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygonize(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_clusterintersecting(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn st_clusterwithin(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn st_clusterdbscan(&mut self, ",eps,minpoints) -> Result<u32, Error> { }

    #[inline]
    pub fn st_linemerge(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_affine(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_affine(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotate(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotate(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotate(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotatez(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotatex(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_rotatey(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_translate(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_translate(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_scale(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_scale(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_scale(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_transscale(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_dump(&mut self) -> Result<_90146, Error> { }

    #[inline]
    pub fn st_dumprings(&mut self) -> Result<_90146, Error> { }

    #[inline]
    pub fn _st_dumppoints(&mut self, the_geom: GeometryType<T>, cur_path) -> Result<_90146, Error> { }

    #[inline]
    pub fn st_dumppoints(&mut self) -> Result<_90146, Error> { }

    #[inline]
    pub fn st_line_locate_point(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_locate_between_measures(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_locate_along_measure(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_addmeasure(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn populate_geometry_columns(&mut self, use_typmod) -> Result<_25, Error> { }

    #[inline]
    pub fn populate_geometry_columns(&mut self, tbl_oid,use_typmod) -> Result<u32, Error> { }

    #[inline]
    pub fn addgeometrycolumn(&mut self, catalog_name,schema_name,table_name,column_name,new_srid_in,new_type,new_dim,use_typmod) -> Result<_25, Error> { }

    #[inline]
    pub fn addgeometrycolumn(&mut self, schema_name,table_name,column_name,new_srid,new_type,new_dim,use_typmod) -> Result<_25, Error> { }

    #[inline]
    pub fn addgeometrycolumn(&mut self, table_name,column_name,new_srid,new_type,new_dim,use_typmod) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrycolumn(&mut self, catalog_name,schema_name,table_name,column_name) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrycolumn(&mut self, schema_name,table_name,column_name) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrycolumn(&mut self, table_name,column_name) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrytable(&mut self, catalog_name,schema_name,table_name) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrytable(&mut self, schema_name,table_name) -> Result<_25, Error> { }

    #[inline]
    pub fn dropgeometrytable(&mut self, table_name) -> Result<_25, Error> { }

    #[inline]
    pub fn st_closestpointofapproach(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distancecpa(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_cpawithin(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_isvalidtrajectory(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn updategeometrysrid(&mut self, catalogn_name,schema_name,table_name,column_name,new_srid_in) -> Result<_25, Error> { }

    #[inline]
    pub fn updategeometrysrid(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn updategeometrysrid(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn find_srid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn get_proj4_from_srid(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_setsrid(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_srid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn postgis_transform_geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_transform(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_transform(&mut self, geom: GeometryType<T>, to_proj) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_transform(&mut self, geom: GeometryType<T>, from_proj,to_proj) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_transform(&mut self, geom: GeometryType<T>, from_proj,to_srid) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_liblwgeom_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_proj_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_scripts_installed(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_lib_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_scripts_released(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_geos_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_svn_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_libxml_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_scripts_build_date(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_lib_build_date(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_full_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn box2d(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn box3d(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn box(&mut self) -> Result<_603, Error> { }

    #[inline]
    pub fn box2d(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn box3d(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn box(&mut self) -> Result<_603, Error> { }

    #[inline]
    pub fn text(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn box3dtobox(&mut self) -> Result<_603, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn bytea(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_simplify(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_simplify(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_simplifyvw(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_seteffectivearea(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_snaptogrid<T>(&mut self, geom1: GeometryType<T>,) -> Result<GeometryType<T>, Error> { }

/*

    #[inline]
    pub fn st_snaptogrid(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_snaptogrid(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_snaptogrid(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,,",",") -> Result<_89860, Error> { }
*/

    #[inline]
    pub fn st_segmentize(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_lineinterpolatepoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_line_interpolate_point(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linesubstring(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_line_substring(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linelocatepoint(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_minimumboundingradius(&mut self, ",center,radius) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_minimumboundingcircle(&mut self, inputgeom: GeometryType<T>, segs_per_quarter) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_offsetcurve(&mut self, line,distance,params) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_generatepoints(&mut self, area,npoints) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_convexhull(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_linecrossingdirection(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<u32, Error> { }

    #[inline]
    pub fn st_linecrossingdirection(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<u32, Error> { }

    #[inline]
    pub fn st_simplifypreservetopology(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_isvalidreason(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_isvaliddetail(&mut self) -> Result<_90260, Error> { }

    #[inline]
    pub fn st_isvaliddetail(&mut self) -> Result<_90260, Error> { }

    #[inline]
    pub fn st_isvalidreason(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_isvalid(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_hausdorffdistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_hausdorffdistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<_701, Error> { }

    #[inline]
    pub fn st_difference(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_boundary(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_points(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_symdifference(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_symmetricdifference(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_union(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_unaryunion(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_removerepeatedpoints(&mut self, geom: GeometryType<T>, tolerance) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_clipbybox2d(&mut self, geom: GeometryType<T>, box) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_subdivide(&mut self, geom: GeometryType<T>, maxvertices) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makevalid(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_cleangeometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_split(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_sharedpaths(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_snap(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<_89860, Error> { }

    #[inline]
    pub fn st_relatematch(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_node(&mut self, g) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_delaunaytriangles(&mut self, g1,tolerance,flags) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_voronoi(&mut self, g1,clip,tolerance,return_polygons) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_voronoipolygons(&mut self, g1,tolerance,extend_to) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_voronoilines(&mut self, g1,tolerance,extend_to) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_combinebbox(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_combinebbox(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_combine_bbox(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_combinebbox(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_combine_bbox(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_extent(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_3dextent(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_collect(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_memcollect(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_collect(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_memunion(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn pgis_abs_in(&mut self) -> Result<_90299, Error> { }

    #[inline]
    pub fn pgis_abs_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn pgis_geometry_accum_transfn(&mut self) -> Result<_90299, Error> { }

    #[inline]
    pub fn pgis_geometry_accum_transfn(&mut self) -> Result<_90299, Error> { }

    #[inline]
    pub fn pgis_geometry_accum_transfn(&mut self) -> Result<_90299, Error> { }

    #[inline]
    pub fn pgis_geometry_accum_finalfn(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn pgis_geometry_union_finalfn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn pgis_geometry_collect_finalfn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn pgis_geometry_polygonize_finalfn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn pgis_geometry_clusterintersecting_finalfn(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn pgis_geometry_clusterwithin_finalfn(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn pgis_geometry_makeline_finalfn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_accum(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_collect(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_clusterintersecting(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn st_clusterwithin(&mut self) -> Result<_89868, Error> { }

    #[inline]
    pub fn st_polygonize(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_makeline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_clusterkmeans(&mut self, geom: GeometryType<T>, k) -> Result<u32, Error> { }

    #[inline]
    pub fn st_relate(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_25, Error> { }

    #[inline]
    pub fn st_relate(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<_25, Error> { }

    #[inline]
    pub fn st_relate(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<bool, Error> { }

    #[inline]
    pub fn st_disjoint(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_touches(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_touches(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_dwithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<bool, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>,") -> Result<bool, Error> { }

    #[inline]
    pub fn _st_intersects(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_crosses(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_crosses(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_contains(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_contains(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_coveredby(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coveredby(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_covers(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_covers(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_containsproperly(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_containsproperly(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_overlaps(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_within(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_within(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_overlaps(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_isvalid(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_minimumclearance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_minimumclearanceline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_centroid(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geometricmedian(&mut self, g,tolerance,max_iter,fail_if_not_converged) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_isring(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_pointonsurface(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_issimple(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_iscollection(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_equals(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_equals(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn equals(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_geomfromgml(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromgml(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromgml(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_gmltosql(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_gmltosql(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromkml(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromgeojson(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn postgis_libjson_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_linefromencodedpolyline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_asencodedpolyline(&mut self, geom: GeometryType<T>, precison: u32) -> Result<_25, Error> { }

    #[inline]
    pub fn st_numinteriorring(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_assvg(&mut self, geom: GeometryType<T>, rel,maxdecimaldigits) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_asgml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgml(&mut self, geom: GeometryType<T>, maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgml(&mut self, version,geom: GeometryType<T>, maxdecimaldigits,options,nprefix,id) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_askml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_askml(&mut self, geom: GeometryType<T>, maxdecimaldigits) -> Result<_25, Error> { }

    #[inline]
    pub fn st_askml(&mut self, version,geom: GeometryType<T>, maxdecimaldigits,nprefix) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgeojson(&mut self, geom: GeometryType<T>, maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_asgeojson(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgeojson(&mut self, gj_version,geom: GeometryType<T>, maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn st_geohash(&mut self, geom: GeometryType<T>, maxchars) -> Result<_25, Error> { }

    #[inline]
    pub fn st_box2dfromgeohash(&mut self) -> Result<_89891, Error> { }

    #[inline]
    pub fn st_pointfromgeohash(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromgeohash(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_numpoints(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_numgeometries(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_geometryn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_dimension(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_exteriorring(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_numinteriorrings(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_interiorringn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geometrytype(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_geometrytype(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_pointn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_numpatches(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_patchn(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_startpoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_endpoint(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_isclosed(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_isempty(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_asbinary(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asbinary(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astext(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_geometryfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geometryfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_wkttosql(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_pointfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_pointfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linefromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linefromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygonfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygonfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mlinefromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mlinefromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multilinestringfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multilinestringfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpointfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpointfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipointfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpolyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpolyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipolygonfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipolygonfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomcollfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomcollfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_pointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_pointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linefromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linefromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linestringfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_linestringfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygonfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygonfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipointfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multilinefromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mlinefromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mlinefromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpolyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_mpolyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipolyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_multipolyfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomcollfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_geomcollfromwkb(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_maxdistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_maxdistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_closestpoint(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_shortestline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_longestline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_longestline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_dfullywithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dfullywithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn st_swapordinates(&mut self, geom: GeometryType<T>, ords) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_flipcoordinates(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_bdpolyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_bdmpolyfromtext(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn unlockrows(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn lockrow(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn lockrow(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn lockrow(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn lockrow(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn addauth(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn checkauth(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn checkauth(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn checkauthtrigger(&mut self) -> Result<_2279, Error> { }

    #[inline]
    pub fn gettransactionid(&mut self) -> Result<_28, Error> { }

    #[inline]
    pub fn enablelongtransactions(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn longtransactionsenabled(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn disablelongtransactions(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn geography_typmod_in(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn geography_typmod_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn geography_in(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn geography_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn geography_recv(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn geography_send(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn geography_analyze(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn geography(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn bytea(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astext(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_astext(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_geographyfromtext(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_geogfromtext(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_geogfromwkb(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn postgis_typmod_dims(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn postgis_typmod_srid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn postgis_typmod_type(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn geography(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn geometry(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn geography_gist_consistent(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_gist_compress(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_gist_penalty(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_gist_picksplit(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_gist_union(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_gist_same(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_gist_decompress(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn geography_overlaps(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_distance_knn(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn geography_gist_distance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn overlaps_geog(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_geog(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_geog(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geog_brin_inclusion_add_value(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_lt(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_le(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_gt(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_ge(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_eq(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geography_cmp(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_assvg(&mut self, geog,rel,maxdecimaldigits) -> Result<_25, Error> { }

    #[inline]
    pub fn st_assvg(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_asgml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgml(&mut self, geog,maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn st_area(&mut self, geog,use_spheroid) -> Result<_701, Error> { }

    #[inline]
    pub fn st_area(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_asgml(&mut self, version,geog,maxdecimaldigits,options,nprefix,id) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_askml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_askml(&mut self, geog,maxdecimaldigits) -> Result<_25, Error> { }

    #[inline]
    pub fn st_askml(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_askml(&mut self, version,geog,maxdecimaldigits,nprefix) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_asgeojson(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgeojson(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgeojson(&mut self, geog,maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asgeojson(&mut self, gj_version,geog,maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn _st_distance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_dwithin(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_distance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distance(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_expand(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_distanceuncached(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_distanceuncached(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_distanceuncached(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_distancetree(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_distancetree(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_dwithinuncached(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_dwithinuncached(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_length(&mut self, geog,use_spheroid) -> Result<_701, Error> { }

    #[inline]
    pub fn st_length(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_project(&mut self, geog,distance,azimuth) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_azimuth(&mut self, geog1,geog2) -> Result<_701, Error> { }

    #[inline]
    pub fn st_perimeter(&mut self, geog,use_spheroid) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_pointoutside(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn _st_covers(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_covers(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_covers(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coveredby(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coveredby(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_segmentize(&mut self, geog,max_segment_length) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_intersects(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_bestsrid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn _st_bestsrid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_buffer(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_intersection(&mut self) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_intersection(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_asbinary(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asbinary(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asewkt(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asewkt(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn geometrytype(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_summary(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_geohash(&mut self, geog,maxchars) -> Result<_25, Error> { }

    #[inline]
    pub fn st_srid(&mut self, geog) -> Result<u32, Error> { }

    #[inline]
    pub fn st_setsrid(&mut self, geog,srid) -> Result<_90480, Error> { }

    #[inline]
    pub fn st_distancesphere(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distance_sphere(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn postgis_type_name(&mut self, geomname,coord_dimension,use_new_name) -> Result<_1043, Error> { }

    #[inline]
    pub fn postgis_constraint_srid(&mut self, geomschema,geomtable,geomcolumn) -> Result<u32, Error> { }

    #[inline]
    pub fn postgis_constraint_dims(&mut self, geomschema,geomtable,geomcolumn) -> Result<u32, Error> { }

    #[inline]
    pub fn postgis_constraint_type(&mut self, geomschema,geomtable,geomcolumn) -> Result<_1043, Error> { }

    #[inline]
    pub fn st_3ddistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_3dmaxdistance(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_701, Error> { }

    #[inline]
    pub fn st_3dclosestpoint(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_3dshortestline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_3dlongestline(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_3ddwithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn st_3ddwithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_3ddfullywithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn st_3ddfullywithin(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>, precision: f64) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_3dintersects(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_3dintersects(&mut self, geom1: GeometryType<T>, geom2: GeometryType<T>) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coorddim(&mut self, geometry) -> Result<_21, Error> { }

    #[inline]
    pub fn st_curvetoline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_curvetoline(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_hasarc(&mut self, geometry) -> Result<bool, Error> { }

    #[inline]
    pub fn st_linetocurve(&mut self, geometry) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_orderingequals(&mut self, geometrya,geometryb) -> Result<bool, Error> { }

    #[inline]
    pub fn st_orderingequals(&mut self, geometrya,geometryb) -> Result<bool, Error> { }

    #[inline]
    pub fn st_point(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_polygon(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_wkbtosql(&mut self, wkb) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_locatebetween(&mut self, geometry,frommeasure,tomeasure,leftrightoffset) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_locatealong(&mut self, geometry,measure,leftrightoffset) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_locatebetweenelevations(&mut self, geometry,fromelevation,toelevation) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_interpolatepoint(&mut self, line,point) -> Result<_701, Error> { }

    #[inline]
    pub fn contains_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn is_contained_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn contains_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn is_contained_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn contains_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn is_contained_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_2d(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_nd(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_nd(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn overlaps_nd(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn  geom2: GeometryType<T>d_brin_inclusion_add_value(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geom3d_brin_inclusion_add_value(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geom4d_brin_inclusion_add_value(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_metadata(&mut self, rast,upperleftx,upperlefty,width,height,scalex,scaley,skewx,skewy,srid,numbands) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_band(&mut self, rast,nband) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_summarystats(&mut self, rastertable,rastercolumn,exclude_nodata_value) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_concavehull(&mut self, param_inputgeom) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_concavehull(&mut self, param_geom: GeometryType<T>, param_pctconvex,param_allow_holes) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_asx3d(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_asx3d(&mut self, geom: GeometryType<T>, maxdecimaldigits,options) -> Result<_25, Error> { }

    #[inline]
    pub fn raster_in(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn raster_out(&mut self) -> Result<String, Error> { }

    #[inline]
    pub fn postgis_raster_lib_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_raster_scripts_installed(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_raster_lib_build_date(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn postgis_gdal_version(&mut self) -> Result<_25, Error> { }

    #[inline]
    pub fn st_envelope(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_convexhull(&mut self) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_minconvexhull(&mut self, rast,nband) -> Result<_89860, Error> { }

    #[inline]
    pub fn box3d(&mut self) -> Result<_89887, Error> { }

    #[inline]
    pub fn st_height(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_numbands(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_scalex(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_scaley(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_skewx(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_skewy(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_srid(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_upperleftx(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_upperlefty(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_width(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_pixelwidth(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_pixelheight(&mut self) -> Result<_701, Error> { }

/*
public.raster, out imag double precision, out jmag double precision, out theta_i double precision, out theta_ij double precision, out xoffset double precision, out yoffset double precision
*/
    #[inline]
    pub fn st_geotransform(&mut self, raster: Raster<_2d>, imag: f64, jmag: f64, theta_i: f64, theta_ij: f64, xoffset: f64, yoffset: f64) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_rotation(&mut self) -> Result<_701, Error> { }

    #[inline]
    pub fn st_summary(&mut self, rast) -> Result<_25, Error> { }

    #[inline]
    pub fn st_memsize(&mut self) -> Result<u32, Error> { }

    #[inline]
    pub fn st_makeemptyraster(&mut self, width,height,upperleftx,upperlefty,scalex,scaley,skewx,skewy,srid) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_makeemptyraster(&mut self, width,height,upperleftx,upperlefty,pixelsize) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_makeemptyraster(&mut self, rast) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, rast,addbandargset) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, rast,index,pixeltype,initialvalue,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, rast,pixeltype,initialvalue,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, torast,fromrast,fromband,torastindex) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, torast,fromrasts,fromband,torastindex) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, rast,index,outdbfile,outdbindex,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_addband(&mut self, rast,outdbfile,outdbindex,index,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_band(&mut self, rast,nbands) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_band(&mut self, rast,nbands,delimiter) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_summarystats(&mut self, rast,nband,exclude_nodata_value,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_summarystats(&mut self, rast,nband,exclude_nodata_value) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_summarystats(&mut self, rast,exclude_nodata_value) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rast,nband,exclude_nodata_value,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rast,nband,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rast,exclude_nodata_value,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rast,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_summarystats_finalfn(&mut self) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_summarystats_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_summarystatsagg(&mut self) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_summarystats_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_summarystatsagg(&mut self) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_summarystats_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_summarystatsagg(&mut self) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_summarystats(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_summarystats(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value) -> Result<_90797, Error> { }

    #[inline]
    pub fn raster_eq(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rastertable,rastercolumn,nband,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rastertable,rastercolumn,exclude_nodata_value) -> Result<_90797, Error> { }

    #[inline]
    pub fn st_approxsummarystats(&mut self, rastertable,rastercolumn,sample_percent) -> Result<_90797, Error> { }

    #[inline]
    pub fn _st_count(&mut self, rast,nband,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_count(&mut self, rast,nband,exclude_nodata_value) -> Result<_20, Error> { }

    #[inline]
    pub fn st_count(&mut self, rast,exclude_nodata_value) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rast,nband,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rast,nband,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rast,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rast,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn _st_countagg_finalfn(&mut self, agg) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rastertable,rastercolumn,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,nband,sample_percent,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,nband,sample_percent,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn __st_countagg_transfn(&mut self, agg,rast,nband,exclude_nodata_value,sample_percent) -> Result<_90828, Error> { }

    #[inline]
    pub fn _st_countagg_transfn(&mut self, agg,rast,nband,exclude_nodata_value,sample_percent) -> Result<_90828, Error> { }

    #[inline]
    pub fn st_countagg(&mut self) -> Result<_20, Error> { }

    #[inline]
    pub fn _st_countagg_transfn(&mut self, agg,rast,nband,exclude_nodata_value) -> Result<_90828, Error> { }

    #[inline]
    pub fn st_countagg(&mut self) -> Result<_20, Error> { }

    #[inline]
    pub fn _st_countagg_transfn(&mut self, agg,rast,exclude_nodata_value) -> Result<_90828, Error> { }

    #[inline]
    pub fn st_countagg(&mut self) -> Result<_20, Error> { }

    #[inline]
    pub fn _st_count(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_count(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value) -> Result<_20, Error> { }

    #[inline]
    pub fn st_count(&mut self, rastertable,rastercolumn,exclude_nodata_value) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rastertable,rastercolumn,nband,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn st_approxcount(&mut self, rastertable,rastercolumn,exclude_nodata_value,sample_percent) -> Result<_20, Error> { }

    #[inline]
    pub fn raster_overleft(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_histogram(&mut self, rast,nband,exclude_nodata_value,sample_percent,bins,width,right,min,max,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rast,nband,exclude_nodata_value,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rast,nband,exclude_nodata_value,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rast,nband,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rast,nband,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,nband,exclude_nodata_value,sample_percent,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,nband,exclude_nodata_value,sample_percent,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,nband,sample_percent,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rast,sample_percent,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn raster_overright(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_histogram(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rastertable,rastercolumn,nband,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_histogram(&mut self, rastertable,rastercolumn,nband,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,nband,sample_percent,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,sample_percent,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,nband,sample_percent,bins,width,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxhistogram(&mut self, rastertable,rastercolumn,nband,sample_percent,bins,right,min,max,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn _st_quantile(&mut self, rast,nband,exclude_nodata_value,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,nband,exclude_nodata_value,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,nband,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,nband,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,nband,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rast,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,nband,exclude_nodata_value,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,nband,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn raster_left(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,nband,exclude_nodata_value,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,nband,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rast,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_quantile(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,nband,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,nband,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_quantile(&mut self, rastertable,rastercolumn,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_aspng(&mut self, rast,nband,options) -> Result<_17, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,nband,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,sample_percent,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,quantiles,quantile,value) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,nband,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,sample_percent,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,exclude_nodata_value,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn st_approxquantile(&mut self, rastertable,rastercolumn,quantile) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_valuecount(&mut self, rast,nband,exclude_nodata_value,searchvalues,roundto,value,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,nband,exclude_nodata_value,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,nband,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,nband,exclude_nodata_value,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,nband,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rast,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,nband,exclude_nodata_value,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,nband,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,nband,exclude_nodata_value,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,nband,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rast,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_valuecount(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,searchvalues,roundto,value,count,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn st_aspng(&mut self, rast,nband,compression) -> Result<_17, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,nband,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,searchvalues,roundto,value,count) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,nband,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuecount(&mut self, rastertable,rastercolumn,searchvalue,roundto) -> Result<u32, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,nband,exclude_nodata_value,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,nband,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,searchvalues,roundto,value,percent) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast,pixeltype,expression,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,nband,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn st_valuepercent(&mut self, rastertable,rastercolumn,searchvalue,roundto) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_reclass(&mut self, rast,reclassargset) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_reclass(&mut self, rast,reclassargset) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_reclass(&mut self, rast,nband,reclassexpr,pixeltype,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_reclass(&mut self, rast,reclassexpr,pixeltype) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_colormap(&mut self, rast,nband,colormap,method) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_colormap(&mut self, rast,nband,colormap,method) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_colormap(&mut self, rast,colormap,method) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_fromgdalraster(&mut self, gdaldata,srid) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_gdaldrivers(&mut self, idx,short_name,long_name,create_options) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_asgdalraster(&mut self, rast,format,options,srid) -> Result<_17, Error> { }

    #[inline]
    pub fn st_aspng(&mut self, rast,nbands,compression) -> Result<_17, Error> { }

    #[inline]
    pub fn raster_right(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_astiff(&mut self, rast,options,srid) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astiff(&mut self, rast,nbands,options,srid) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astiff(&mut self, rast,compression,srid) -> Result<_17, Error> { }

    #[inline]
    pub fn st_astiff(&mut self, rast,nbands,compression,srid) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asjpeg(&mut self, rast,options) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asjpeg(&mut self, rast,nbands,options) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asjpeg(&mut self, rast,nbands,quality) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asjpeg(&mut self, rast,nband,options) -> Result<_17, Error> { }

    #[inline]
    pub fn st_asjpeg(&mut self, rast,nband,quality) -> Result<_17, Error> { }

    #[inline]
    pub fn st_aspng(&mut self, rast,options) -> Result<_17, Error> { }

    #[inline]
    pub fn st_aspng(&mut self, rast,nbands,options) -> Result<_17, Error> { }

    #[inline]
    pub fn raster_overabove(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_asraster(&mut self, geom: GeometryType<T>, scalex,scaley,width,height,pixeltype,value,nodataval,upperleftx,upperlefty,gridx,gridy,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, scalex,scaley,gridx,gridy,pixeltype,value,nodataval,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, scalex,scaley,pixeltype,value,nodataval,upperleftx,upperlefty,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, width,height,gridx,gridy,pixeltype,value,nodataval,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, width,height,pixeltype,value,nodataval,upperleftx,upperlefty,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, scalex,scaley,gridx,gridy,pixeltype,value,nodataval,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, scalex,scaley,pixeltype,value,nodataval,upperleftx,upperlefty,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_bandisnodata(&mut self, rast,band,forcechecking) -> Result<bool, Error> { }

    #[inline]
    pub fn raster_overbelow(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, width,height,gridx,gridy,pixeltype,value,nodataval,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, width,height,pixeltype,value,nodataval,upperleftx,upperlefty,skewx,skewy,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, ref,pixeltype,value,nodataval,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_asraster(&mut self, geom: GeometryType<T>, ref,pixeltype,value,nodataval,touched) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_gdalwarp(&mut self, rast,algorithm,maxerr,srid,scalex,scaley,gridx,gridy,skewx,skewy,width,height) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_resample(&mut self, rast,scalex,scaley,gridx,gridy,skewx,skewy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_bandisnodata(&mut self, rast,forcechecking) -> Result<bool, Error> { }

    #[inline]
    pub fn st_bandpath(&mut self, rast,band) -> Result<_25, Error> { }

    #[inline]
    pub fn raster_above(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_resample(&mut self, rast,width,height,gridx,gridy,skewx,skewy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_resample(&mut self, rast,ref,algorithm,maxerr,usescale) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_resample(&mut self, rast,ref,usescale,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_transform(&mut self, rast,srid,algorithm,maxerr,scalex,scaley) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_transform(&mut self, rast,srid,scalex,scaley,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_transform(&mut self, rast,srid,scalexy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_bandpixeltype(&mut self, rast,band) -> Result<_25, Error> { }

    #[inline]
    pub fn st_transform(&mut self, rast,alignto,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_rescale(&mut self, rast,scalex,scaley,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_rescale(&mut self, rast,scalexy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_reskew(&mut self, rast,skewx,skewy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_reskew(&mut self, rast,skewxy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_snaptogrid(&mut self, rast,gridx,gridy,algorithm,maxerr,scalex,scaley) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_snaptogrid(&mut self, rast,gridx,gridy,scalex,scaley,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_snaptogrid(&mut self, rast,gridx,gridy,scalexy,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn raster_below(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_resize(&mut self, rast,width,height,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_resize(&mut self, rast,width,height,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_resize(&mut self, rast,percentwidth,percentheight,algorithm,maxerr) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebraexpr(&mut self, rast,band,pixeltype,expression,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebraexpr(&mut self, rast,pixeltype,expression,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,band,pixeltype,onerastuserfunc,args) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,band,pixeltype,onerastuserfunc) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,band,onerastuserfunc,args) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,band,onerastuserfunc) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,pixeltype,onerastuserfunc,args) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,pixeltype,onerastuserfunc) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,onerastuserfunc,args) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast,onerastuserfunc) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_bandmetadata(&mut self, rast,band,bandnum,pixeltype,nodatavalue,isoutdb,path) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_mapalgebraexpr(&mut self, rast1,band1,rast2,band2,expression,pixeltype,extenttype,nodata1expr,nodata2expr,nodatanodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebraexpr(&mut self, rast1,rast2,expression,pixeltype,extenttype,nodata1expr,nodata2expr,nodatanodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast1,band1,rast2,band2,tworastuserfunc,pixeltype,extenttype,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafct(&mut self, rast1,rast2,tworastuserfunc,pixeltype,extenttype,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebrafctngb(&mut self, rast,band,pixeltype,ngbwidth,ngbheight,onerastngbuserfunc,nodatamode,args) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_max4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_min4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_sum4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_bandmetadata(&mut self, rast,band,pixeltype,nodatavalue,isoutdb,path) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_mean4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_range4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distinct4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn st_stddev4ma(&mut self, matrix,nodatamode,args) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_mapalgebra(&mut self, rastbandargset,callbackfunc,pixeltype,distancex,distancey,extenttype,customextent,mask,weighted,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rastbandargset,callbackfunc,pixeltype,extenttype,customextent,distancex,distancey,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast1,band1,rast2,band2,expression,pixeltype,extenttype,nodata1expr,nodata2expr,nodatanodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_value(&mut self, rast,band,x,y,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast,nband,callbackfunc,pixeltype,extenttype,customextent,distancex,distancey,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast,nband,callbackfunc,pixeltype,extenttype,customextent,distancex,distancey,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast1,nband1,rast2,nband2,callbackfunc,pixeltype,extenttype,customextent,distancex,distancey,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast,nband,callbackfunc,mask,weighted,pixeltype,extenttype,customextent,userargs) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_mapalgebra(&mut self, rastbandargset,expression,pixeltype,extenttype,nodata1expr,nodata2expr,nodatanodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast,nband,pixeltype,expression,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_rastertoworldcoordy(&mut self, rast,xr,yr) -> Result<_701, Error> { }

    #[inline]
    pub fn st_mapalgebra(&mut self, rast1,rast2,expression,pixeltype,extenttype,nodata1expr,nodata2expr,nodatanodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_convertarray4ma(&mut self, value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_max4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_min4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_sum4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_mean4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn raster_same(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_range4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_distinct4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_stddev4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_invdistweight4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_mindist4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_slope4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_slope(&mut self, rast,nband,customextent,pixeltype,units,scale,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_slope(&mut self, rast,nband,pixeltype,units,scale,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_aspect4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_aspect(&mut self, rast,nband,customextent,pixeltype,units,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_aspect(&mut self, rast,nband,pixeltype,units,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_hillshade4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_rastertoworldcoordy(&mut self, rast,yr) -> Result<_701, Error> { }

    #[inline]
    pub fn st_hillshade(&mut self, rast,nband,customextent,pixeltype,azimuth,altitude,max_bright,scale,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_hillshade(&mut self, rast,nband,pixeltype,azimuth,altitude,max_bright,scale,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_tpi4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_tpi(&mut self, rast,nband,customextent,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_tpi(&mut self, rast,nband,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_tri(&mut self, rast,nband,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_isempty(&mut self, rast) -> Result<bool, Error> { }

    #[inline]
    pub fn st_hasnoband(&mut self, rast,nband) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_roughness4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_roughness(&mut self, rast,nband,customextent,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_roughness(&mut self, rast,nband,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_tri4ma(&mut self, value,pos,userargs) -> Result<_701, Error> { }

    #[inline]
    pub fn st_tri(&mut self, rast,nband,customextent,pixeltype,interpolate_nodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_bandnodatavalue(&mut self, rast,band) -> Result<_701, Error> { }

    #[inline]
    pub fn st_value(&mut self, rast,x,y,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_value(&mut self, rast,band,pt,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_value(&mut self, rast,pt,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_pixelofvalue(&mut self, rast,nband,search,exclude_nodata_value,val,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelofvalue(&mut self, rast,search,exclude_nodata_value,val,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelofvalue(&mut self, rast,nband,search,exclude_nodata_value,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelofvalue(&mut self, rast,search,exclude_nodata_value,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_georeference(&mut self, rast,format) -> Result<_25, Error> { }

    #[inline]
    pub fn st_setscale(&mut self, rast,scale) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setscale(&mut self, rast,scalex,scaley) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setskew(&mut self, rast,skew) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setskew(&mut self, rast,skewx,skewy) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setsrid(&mut self, rast,srid) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setupperleft(&mut self, rast,upperleftx,upperlefty) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setrotation(&mut self, rast,rotation) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setgeotransform(&mut self, rast,imag,jmag,theta_i,theta_ij,xoffset,yoffset) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setgeoreference(&mut self, rast,georef,format) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setgeoreference(&mut self, rast,upperleftx,upperlefty,scalex,scaley,skewx,skewy) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_tile(&mut self, rast,width,height,nband,padwithnodata,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_tile(&mut self, rast,nband,width,height,padwithnodata,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_tile(&mut self, rast,nband,width,height,padwithnodata,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_tile(&mut self, rast,width,height,padwithnodata,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setbandnodatavalue(&mut self, rast,band,nodatavalue,forcechecking) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setbandnodatavalue(&mut self, rast,nodatavalue) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setbandisnodata(&mut self, rast,band) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_setvalues(&mut self, rast,nband,x,y,newvalueset,noset,hasnosetvalue,nosetvalue,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalues(&mut self, rast,nband,x,y,newvalueset,noset,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalues(&mut self, rast,nband,x,y,newvalueset,nosetvalue,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_minpossiblevalue(&mut self, pixeltype) -> Result<_701, Error> { }

    #[inline]
    pub fn raster_contained(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_setvalues(&mut self, rast,nband,x,y,width,height,newvalue,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalues(&mut self, rast,x,y,width,height,newvalue,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalues(&mut self, rast,nband,geomvalset,keepnodata) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalue(&mut self, rast,band,x,y,newvalue) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalue(&mut self, rast,x,y,newvalue) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalue(&mut self, rast,nband,geom: GeometryType<T>, newvalue) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_setvalue(&mut self, rast,geom: GeometryType<T>, newvalue) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_dumpaspolygons(&mut self, rast,band,exclude_nodata_value) -> Result<_90757, Error> { }

    #[inline]
    pub fn st_dumpvalues(&mut self, rast,nband,exclude_nodata_value,nband,valarray) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_dumpvalues(&mut self, rast,nband,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_polygon(&mut self, rast,band) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_pixelaspolygons(&mut self, rast,band,columnx,rowy,exclude_nodata_value,geom: GeometryType<T>, val,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelaspolygons(&mut self, rast,band,exclude_nodata_value,geom: GeometryType<T>, val,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelaspolygon(&mut self, rast,x,y) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_asbinary(&mut self, outasin) -> Result<_17, Error> { }

    #[inline]
    pub fn bytea(&mut self) -> Result<_17, Error> { }

    #[inline]
    pub fn st_pixelaspoints(&mut self, rast, band, exclude_nodata_value, geom: GeometryType<T>,  val, x, y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelaspoint(&mut self, rast,x,y) -> Result<_89860, Error> { }

    #[inline]
    pub fn st_pixelascentroids(&mut self, rast,band,exclude_nodata_value,geom: GeometryType<T>, val,x,y) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_pixelascentroid(&mut self, rast,x,y) -> Result<_89860, Error> { }

    #[inline]
    pub fn _st_worldtorastercoord(&mut self, rast,longitude,latitude,columnx,rowy) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_worldtorastercoord(&mut self, rast,longitude,latitude,columnx,rowy) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_worldtorastercoord(&mut self, rast,pt,columnx,rowy) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_worldtorastercoordx(&mut self, rast,xw,yw) -> Result<u32, Error> { }

    #[inline]
    pub fn st_worldtorastercoordx(&mut self, rast,xw) -> Result<u32, Error> { }

    #[inline]
    pub fn st_worldtorastercoordx(&mut self, rast,pt) -> Result<u32, Error> { }

    #[inline]
    pub fn st_worldtorastercoordy(&mut self, rast,xw,yw) -> Result<u32, Error> { }

    #[inline]
    pub fn st_worldtorastercoordy(&mut self, rast,yw) -> Result<u32, Error> { }

    #[inline]
    pub fn st_worldtorastercoordy(&mut self, rast,pt) -> Result<u32, Error> { }

    #[inline]
    pub fn _st_rastertoworldcoord(&mut self, rast,columnx,rowy,longitude,latitude) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_rastertoworldcoord(&mut self, rast,columnx,rowy,longitude,latitude) -> Result<_2249, Error> { }

    #[inline]
    pub fn st_rastertoworldcoordx(&mut self, rast,xr,yr) -> Result<_701, Error> { }

    #[inline]
    pub fn st_rastertoworldcoordx(&mut self, rast,xr) -> Result<_701, Error> { }

    #[inline]
    pub fn raster_contain(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn raster_overlap(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn raster_geometry_contain(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn raster_contained_by_geometry(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn raster_geometry_overlap(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_raster_contain(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_contained_by_raster(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn geometry_raster_overlap(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_samealignment(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_samealignment(&mut self, ulx1,uly1,scalex1,scaley1,skewx1,skewy1,ulx2,uly2,scalex2,scaley2,skewx2,skewy2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_samealignment_transfn(&mut self, agg,rast) -> Result<_91169, Error> { }

    #[inline]
    pub fn _st_samealignment_finalfn(&mut self, agg) -> Result<bool, Error> { }

    #[inline]
    pub fn st_samealignment(&mut self) -> Result<bool, Error> { }

    #[inline]
    pub fn st_notsamealignmentreason(&mut self, rast1,rast2) -> Result<_25, Error> { }

    #[inline]
    pub fn st_iscoveragetile(&mut self, rast,coverage,tilewidth,tileheight) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_intersects(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_intersects(&mut self, geom: GeometryType<T>, rast,nband) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, geom: GeometryType<T>, rast,nband) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, rast,geom: GeometryType<T>, nband) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersects(&mut self, rast,nband,geom) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_overlaps(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_overlaps(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_overlaps(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_touches(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_touches(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_touches(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_contains(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_contains(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_contains(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_containsproperly(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_containsproperly(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_containsproperly(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_covers(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_covers(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_covers(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_coveredby(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coveredby(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_coveredby(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_within(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_within(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_within(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_dwithin(&mut self, rast1,nband1,rast2,nband2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self, rast1,nband1,rast2,nband2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dwithin(&mut self, rast1,rast2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn _st_dfullywithin(&mut self, rast1,nband1,rast2,nband2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dfullywithin(&mut self, rast1,nband1,rast2,nband2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn st_dfullywithin(&mut self, rast1,rast2,distance) -> Result<bool, Error> { }

    #[inline]
    pub fn st_disjoint(&mut self, rast1,nband1,rast2,nband2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_disjoint(&mut self, rast1,rast2) -> Result<bool, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, geomin,rast,band) -> Result<_90757, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast,band,geomin) -> Result<_90757, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast,geomin) -> Result<_90757, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,band1,rast2,band2,returnband,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,band1,rast2,band2,returnband,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,band1,rast2,band2,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,band1,rast2,band2,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,rast2,returnband,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,rast2,returnband,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,rast2,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_intersection(&mut self, rast1,rast2,nodataval) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_finalfn(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_union_transfn(&mut self) -> Result<_2281, Error> { }

    #[inline]
    pub fn st_union(&mut self) -> Result<_90744, Error> { }

    #[inline]
    pub fn _st_clip(&mut self, rast,nband,geom: GeometryType<T>, nodataval,crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,nband,geom: GeometryType<T>, nodataval,crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,nband,geom: GeometryType<T>, nodataval,crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,nband,geom: GeometryType<T>, crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,geom: GeometryType<T>, nodataval,crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,geom: GeometryType<T>, nodataval,crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_clip(&mut self, rast,geom: GeometryType<T>, crop) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_nearestvalue(&mut self, rast,band,pt,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_nearestvalue(&mut self, rast,pt,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_nearestvalue(&mut self, rast,band,columnx,rowy,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn st_nearestvalue(&mut self, rast,columnx,rowy,exclude_nodata_value) -> Result<_701, Error> { }

    #[inline]
    pub fn _st_neighborhood(&mut self, rast,band,columnx,rowy,distancex,distancey,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_neighborhood(&mut self, rast,band,columnx,rowy,distancex,distancey,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_neighborhood(&mut self, rast,columnx,rowy,distancex,distancey,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_neighborhood(&mut self, rast,band,pt,distancex,distancey,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn st_neighborhood(&mut self, rast,pt,distancex,distancey,exclude_nodata_value) -> Result<_1022, Error> { }

    #[inline]
    pub fn _add_raster_constraint(&mut self, cn,sql) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint(&mut self, rastschema,rasttable,cn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_srid(&mut self, rastschema,rasttable,rastcolumn) -> Result<u32, Error> { }

    #[inline]
    pub fn _add_raster_constraint_srid(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_srid(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_scale(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<_701, Error> { }

    #[inline]
    pub fn _add_raster_constraint_scale(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_scale(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_blocksize(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<u32, Error> { }

    #[inline]
    pub fn _add_raster_constraint_blocksize(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_blocksize(&mut self, rastschema,rasttable,rastcolumn,axis) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_extent(&mut self, rastschema,rasttable,rastcolumn) -> Result<_89860, Error> { }

    #[inline]
    pub fn _add_raster_constraint_extent(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_extent(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_alignment(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _add_raster_constraint_alignment(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_alignment(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_spatially_unique(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _add_raster_constraint_spatially_unique(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_spatially_unique(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_coverage_tile(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _add_raster_constraint_coverage_tile(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_coverage_tile(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_regular_blocking(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_regular_blocking(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_num_bands(&mut self, rastschema,rasttable,rastcolumn) -> Result<u32, Error> { }

    #[inline]
    pub fn _add_raster_constraint_num_bands(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_num_bands(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_pixel_types(&mut self, rastschema,rasttable,rastcolumn) -> Result<_1009, Error> { }

    #[inline]
    pub fn _raster_constraint_pixel_types(&mut self, rast) -> Result<_1009, Error> { }

    #[inline]
    pub fn _add_raster_constraint_pixel_types(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_pixel_types(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_nodata_values(&mut self, rastschema,rasttable,rastcolumn) -> Result<_1022, Error> { }

    #[inline]
    pub fn _raster_constraint_nodata_values(&mut self, rast) -> Result<_1231, Error> { }

    #[inline]
    pub fn _add_raster_constraint_nodata_values(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_nodata_values(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_out_db(&mut self, rastschema,rasttable,rastcolumn) -> Result<_1000, Error> { }

    #[inline]
    pub fn _raster_constraint_out_db(&mut self, rast) -> Result<_1000, Error> { }

    #[inline]
    pub fn _add_raster_constraint_out_db(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_raster_constraint_out_db(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _raster_constraint_info_index(&mut self, rastschema,rasttable,rastcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn addrasterconstraints(&mut self, rastschema,rasttable,rastcolumn,constraints) -> Result<bool, Error> { }

    #[inline]
    pub fn addrasterconstraints(&mut self, rasttable,rastcolumn,constraints) -> Result<bool, Error> { }

    #[inline]
    pub fn addrasterconstraints(&mut self, rastschema,rasttable,rastcolumn,srid,scale_x,scale_y,blocksize_x,blocksize_y,same_alignment,regular_blocking,num_bands,pixel_types,nodata_values,out_db,extent) -> Result<bool, Error> { }

    #[inline]
    pub fn addrasterconstraints(&mut self, rasttable,rastcolumn,srid,scale_x,scale_y,blocksize_x,blocksize_y,same_alignment,regular_blocking,num_bands,pixel_types,nodata_values,out_db,extent) -> Result<bool, Error> { }

    #[inline]
    pub fn droprasterconstraints(&mut self, rastschema,rasttable,rastcolumn,constraints) -> Result<bool, Error> { }

    #[inline]
    pub fn droprasterconstraints(&mut self, rasttable,rastcolumn,constraints) -> Result<bool, Error> { }

    #[inline]
    pub fn droprasterconstraints(&mut self, rastschema,rasttable,rastcolumn,srid,scale_x,scale_y,blocksize_x,blocksize_y,same_alignment,regular_blocking,num_bands,pixel_types,nodata_values,out_db,extent) -> Result<bool, Error> { }

    #[inline]
    pub fn droprasterconstraints(&mut self, rasttable,rastcolumn,srid,scale_x,scale_y,blocksize_x,blocksize_y,same_alignment,regular_blocking,num_bands,pixel_types,nodata_values,out_db,extent) -> Result<bool, Error> { }

    #[inline]
    pub fn _overview_constraint(&mut self, ov,factor,refschema,reftable,refcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _overview_constraint_info(&mut self, ovschema,ovtable,ovcolumn,refschema,reftable,refcolumn,factor) -> Result<_2249, Error> { }

    #[inline]
    pub fn _add_overview_constraint(&mut self, ovschema,ovtable,ovcolumn,refschema,reftable,refcolumn,factor) -> Result<bool, Error> { }

    #[inline]
    pub fn _drop_overview_constraint(&mut self, ovschema,ovtable,ovcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn addoverviewconstraints(&mut self, ovschema,ovtable,ovcolumn,refschema,reftable,refcolumn,ovfactor) -> Result<bool, Error> { }

    #[inline]
    pub fn addoverviewconstraints(&mut self, ovtable,ovcolumn,reftable,refcolumn,ovfactor) -> Result<bool, Error> { }

    #[inline]
    pub fn dropoverviewconstraints(&mut self, ovschema,ovtable,ovcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn dropoverviewconstraints(&mut self, ovtable,ovcolumn) -> Result<bool, Error> { }

    #[inline]
    pub fn _updaterastersrid(&mut self, schema_name,table_name,column_name,new_srid) -> Result<bool, Error> { }

    #[inline]
    pub fn updaterastersrid(&mut self, schema_name,table_name,column_name,new_srid) -> Result<bool, Error> { }

    #[inline]
    pub fn updaterastersrid(&mut self, table_name,column_name,new_srid) -> Result<bool, Error> { }

    #[inline]
    pub fn st_retile(&mut self, tab,col,ext,sfx,sfy,tw,th,algo) -> Result<_90744, Error> { }

    #[inline]
    pub fn st_createoverview(&mut self, tab,col,factor,algo) -> Result<_2205, Error> { }

    #[inline]
    pub fn postgis_noop(&mut self) -> Result<_89860, Error> { }
}
