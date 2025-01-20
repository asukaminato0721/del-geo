//! method common in 2D or 3D Axis-Aligned Bounding Box (AABB)

/// compute intersection against ray
/// * `line_org` - origin of ray
/// * `line_dir` - direction of ray (general non-zero vector, not necessarily unitary)
///
/// * Return
///     * `None`: if there is no intersection
///     * `(t_min: Real, t_max: Real)` min and max of the depth at intersections.
///         `t_*` is a ratio of `line_dir` not distance.
///         For example `p_min = line_org + t_min * line_dir`
pub fn intersections_against_line<Real, const NDIM: usize, const SIZE_AABB: usize>(
    aabb: &[Real; SIZE_AABB],
    line_org: &[Real; NDIM],
    line_dir: &[Real; NDIM],
) -> Option<(Real, Real)>
where
    Real: num_traits::Float,
{
    assert_eq!(NDIM * 2, SIZE_AABB);
    let mut tmin = Real::min_value();
    let mut tmax = Real::max_value();
    for i_dim in 0..NDIM {
        if !line_dir[i_dim].abs().is_zero() {
            let t1 = (aabb[i_dim] - line_org[i_dim]) / line_dir[i_dim];
            let t2 = (aabb[i_dim + NDIM] - line_org[i_dim]) / line_dir[i_dim];
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        } else if line_org[i_dim] < aabb[i_dim] || line_org[i_dim] > aabb[i_dim + NDIM] {
            return None;
        }
    }
    if tmax >= tmin {
        Some((tmin, tmax))
    } else {
        None
    }
}

/// compute intersection against ray
/// * `ray_org` - origin of ray
/// * `ray_dir` - direction of ray (general non-zero vector, not necessarily unitary)
///
/// * Return
///     * `None`: if there is no intersection
///     * `(t_min: Real, t_max: Real)` min and max of the depth at intersections.
///         `t_*` is a ratio of `ray_dir` not distance.
///         For example `p_min = ray_org + t_min * ray_dir`
pub fn intersections_against_ray<Real, const NDIM: usize, const SIZE_AABB: usize>(
    aabb: &[Real; SIZE_AABB],
    ray_org: &[Real; NDIM],
    ray_dir: &[Real; NDIM],
) -> Option<(Real, Real)>
where
    Real: num_traits::Float,
{
    intersections_against_line(aabb, ray_org, ray_dir).filter(|(_tmin, tmax)| *tmax >= Real::zero())
}

pub fn is_include_point<Real, const NDIM: usize, const SIZE_AABB: usize>(
    aabb: &[Real; SIZE_AABB],
    point: &[Real; NDIM],
) -> bool
where
    Real: num_traits::Float,
{
    assert_eq!(NDIM * 2, SIZE_AABB);
    point
        .iter()
        .zip(&aabb[..NDIM])
        .zip(&aabb[NDIM..])
        .all(|((p, &min), &max)| *p >= min && *p <= max)
}

pub fn center<Real, const NDIM: usize, const SIZE_AABB: usize>(
    aabb: &[Real; SIZE_AABB],
) -> [Real; NDIM]
where
    Real: num_traits::Float,
{
    let half = Real::one() / (Real::one() + Real::one());
    std::array::from_fn::<_, NDIM, _>(|i| (aabb[i] + aabb[i + NDIM]) * half)
}

// -----------------------------
#[derive(Debug, Clone, Copy)]
pub struct AABB<Real, const NDIM: usize, const SIZE_AABB: usize> {
    pub aabb: [Real; SIZE_AABB],
}

impl<Real, const NDIM: usize, const SIZE_AABB: usize> AABB<Real, NDIM, SIZE_AABB>
where
    Real: num_traits::Float,
{
    pub fn is_include_point(&self, point: &[Real; NDIM]) -> bool {
        is_include_point::<Real, NDIM, SIZE_AABB>(&self.aabb, point)
    }

    pub fn intersections_against_ray(
        &self,
        ray_org: &[Real; NDIM],
        ray_dir: &[Real; NDIM],
    ) -> Option<(Real, Real)> {
        intersections_against_ray::<Real, NDIM, SIZE_AABB>(&self.aabb, ray_org, ray_dir)
    }

    pub fn intersections_against_line(
        &self,
        line_org: &[Real; NDIM],
        line_dir: &[Real; NDIM],
    ) -> Option<(Real, Real)> {
        intersections_against_line::<Real, NDIM, SIZE_AABB>(&self.aabb, line_org, line_dir)
    }

    pub fn center(&self) -> [Real; NDIM] {
        center(&self.aabb)
    }
}
