//! 2D Continuous Collision Detection (CCD)

use num_traits::AsPrimitive;

pub struct ThreePoints<'a, T> {
    pub p0: &'a nalgebra::Vector2<T>,
    pub p1: &'a nalgebra::Vector2<T>,
    pub p2: &'a nalgebra::Vector2<T>,
}

/// compute time when four points gets co-planar
pub fn coplanar_time<T>(s: ThreePoints<T>, e: ThreePoints<T>) -> Option<[T; 2]>
where
    T: nalgebra::RealField + Copy + num_traits::Float,
    i64: AsPrimitive<T>,
{
    let x1 = s.p1 - s.p0;
    let x2 = s.p2 - s.p0;
    let v1 = e.p1 - e.p0 - x1;
    let v2 = e.p2 - e.p0 - x2;
    // a = (x1 + t*v1).cross(x2+t*v2)
    //   = x1.cross(x2) + t * { v1.cross(x2) + x1.cross(v2) } + t*t v1.cross(v2)

    // compute coefficient for cubic function
    use crate::vec2::area_quadrilateral;
    let c0 = area_quadrilateral(&x1, &x2); // constant
    let c1 = area_quadrilateral(&x1, &x2) + area_quadrilateral(&v1, &x2); // linear
    let c2 = area_quadrilateral(&v1, &v2);
    // cubic function is f(x) = c0 + c1*t + c2*t^2
    del_geo_core::polynomial_root::quadratic_root(c0, c1, c2)
}

pub struct EdgeVertex<'a, T> {
    pub e0: &'a nalgebra::Vector2<T>,
    pub e1: &'a nalgebra::Vector2<T>,
    pub v: &'a nalgebra::Vector2<T>,
}

pub fn intersecting_time_ev<T>(s: EdgeVertex<T>, e: EdgeVertex<T>) -> Option<T>
where
    T: nalgebra::RealField + Copy + num_traits::Float,
    i64: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{
    let list_te = coplanar_time(
        ThreePoints {
            p0: s.e0,
            p1: s.e1,
            p2: s.v,
        },
        ThreePoints {
            p0: e.e0,
            p1: e.e1,
            p2: e.v,
        },
    );
    let list_te= list_te?; // return None if none
    assert!(list_te[0] <= list_te[1]);
    for te in list_te {
        let ts = T::one() - te;
        let e0 = s.e0.scale(ts) + e.e0.scale(te);
        let e1 = s.e1.scale(ts) + e.e1.scale(te);
        let v = s.v.scale(ts) + e.v.scale(te);
        let coord = crate::edge2::barycentric(&e0, &e1, &v);
        if coord >= T::zero() && coord <= T::one() {
            return Some(te);
        }
    }
    None
}