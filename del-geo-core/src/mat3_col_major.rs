//! methods for 3x3 matrix where storage is column major order

/// trait for 3x3 matrix where storage is column major order
pub trait Mat3ColMajor<T: num_traits::Float>
where
    Self: Sized,
{
    fn from_diagonal(diagonal: &[T; 3]) -> Self;
    fn from_identity() -> Self;
    fn add(&self, b: &Self) -> Self;
    fn sub(&self, b: &Self) -> Self;
    fn scale(&self, s: T) -> Self;
    fn determinant(&self) -> T;
    fn try_inverse(&self) -> Option<Self>;
    fn transpose(&self) -> Self;
    fn mult_mat_col_major(&self, other: &Self) -> Self;
    fn mult_vec(&self, vec: &[T; 3]) -> [T; 3];
    fn transform_homogeneous(&self, x: &[T; 2]) -> Option<[T; 2]>;
    fn squared_norm(&self) -> T;
    fn norm(&self) -> T;
}

impl<Real> Mat3ColMajor<Real> for [Real; 9]
where
    Real: num_traits::Float,
{
    fn from_diagonal(diagonal: &[Real; 3]) -> Self {
        from_diagonal(diagonal)
    }
    fn from_identity() -> Self {
        from_identity()
    }
    fn add(&self, b: &Self) -> Self {
        add(self, b)
    }
    fn sub(&self, b: &Self) -> Self {
        sub(self, b)
    }
    fn scale(&self, s: Real) -> Self {
        scale(self, s)
    }
    fn determinant(&self) -> Real {
        determinant(self)
    }
    fn try_inverse(&self) -> Option<Self> {
        try_inverse(self)
    }
    fn transpose(&self) -> Self {
        transpose(self)
    }
    fn mult_mat_col_major(&self, other: &Self) -> Self {
        mult_mat_col_major(self, other)
    }
    fn mult_vec(&self, vec: &[Real; 3]) -> [Real; 3] {
        mult_vec(self, vec)
    }
    fn transform_homogeneous(&self, x: &[Real; 2]) -> Option<[Real; 2]> {
        transform_homogeneous(self, x)
    }
    fn squared_norm(&self) -> Real {
        crate::mat3_row_major::squared_norm(self)
    }
    fn norm(&self) -> Real {
        crate::mat3_row_major::norm(self)
    }
}

// --------------------------------------------------
// below from methods

pub fn from_diagonal<Real>(s: &[Real; 3]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    [s[0], zero, zero, zero, s[1], zero, zero, zero, s[2]]
}

pub fn from_columns<Real>(x: &[Real; 3], y: &[Real; 3], z: &[Real; 3]) -> [Real; 9]
where
    Real: Copy,
{
    [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]]
}

pub fn from_identity<T>() -> [T; 9]
where
    T: num_traits::Float,
{
    let zero = T::zero();
    let one = T::one();
    [one, zero, zero, zero, one, zero, zero, zero, one]
}

pub fn from_translate<Real>(v: &[Real; 2]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    let one = Real::one();
    [one, zero, zero, zero, one, zero, v[0], v[1], one]
}

pub fn from_rotate_x<Real>(theta: Real) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    let one = Real::one();
    let c = theta.cos();
    let s = theta.sin();
    [one, zero, zero, zero, c, s, zero, -s, c]
}

pub fn from_rotate_y<Real>(theta: Real) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    let one = Real::one();
    let c = theta.cos();
    let s = theta.sin();
    [c, zero, -s, zero, one, zero, s, zero, c]
}

pub fn from_rotate_z<Real>(theta: Real) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    let one = Real::one();
    let c = theta.cos();
    let s = theta.sin();
    [c, s, zero, -s, c, zero, zero, zero, one]
}

/// rotation matrix where x-rotation, y-rotation and z-rotation is applied sequentially
pub fn from_bryant_angles<Real>(rx: Real, ry: Real, rz: Real) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let x = from_rotate_x(rx);
    let y = from_rotate_y(ry);
    let z = from_rotate_z(rz);
    let yx = mult_mat_col_major(&y, &x);
    mult_mat_col_major(&z, &yx)
}

/// transformation converting normalized device coordinate (NDC) `[-1,+1]^2` to pixel coordinate
/// * `image_shape` - (width, height)
pub fn from_transform_ndc2pix(img_shape: (usize, usize)) -> [f32; 9] {
    [
        0.5 * (img_shape.0 as f32),
        0.,
        0.,
        0.,
        -0.5 * (img_shape.1 as f32),
        0.,
        0.5 * (img_shape.0 as f32),
        0.5 * (img_shape.1 as f32),
        1.,
    ]
}

/// transformation converting unit coodinate (NDC) `[0,+1]^2` to pixel coordinate
/// * `image_shape` - (width, height)
pub fn from_transform_unit2pix(img_shape: (usize, usize)) -> [f32; 9] {
    [
        img_shape.0 as f32,
        0.,
        0.,
        0.,
        -(img_shape.1 as f32),
        0.,
        0.,
        img_shape.1 as f32,
        1.,
    ]
}

pub fn from_scaled_outer_product<T>(s: T, a: &[T; 3], b: &[T; 3]) -> [T; 9]
where
    T: num_traits::Float,
{
    [
        s * a[0] * b[0],
        s * a[1] * b[0],
        s * a[2] * b[0],
        s * a[0] * b[1],
        s * a[1] * b[1],
        s * a[2] * b[1],
        s * a[0] * b[2],
        s * a[1] * b[2],
        s * a[2] * b[2],
    ]
}

pub fn from_vec3_to_skew_mat<T>(v: &[T; 3]) -> [T; 9]
where
    T: num_traits::Float,
{
    [
        T::zero(),
        v[2],
        -v[1],
        -v[2],
        T::zero(),
        v[0],
        v[1],
        -v[0],
        T::zero(),
    ]
}

// above: from methods
// ---------------------------------------------
// below: to methods

pub fn to_vec3_from_skew_mat<T>(m: &[T; 9]) -> [T; 3]
where
    T: num_traits::Float,
{
    let one = T::one();
    let half = one / (one + one);
    [
        (m[5] - m[7]) * half,
        (m[6] - m[2]) * half,
        (m[1] - m[3]) * half,
    ]
}

#[test]
fn test_skew() {
    use crate::mat3_col_major::Mat3ColMajor;
    use crate::vec3::Vec3;
    let v0 = [1.1f64, 3.1, 2.5];
    let m0 = from_vec3_to_skew_mat(&v0);
    {
        let v1 = [2.1, 0.1, 4.5];
        let c0 = v0.cross(&v1);
        let c1 = m0.mult_vec(&v1);
        assert!(c0.sub(&c1).norm() < 1.0e-10);
    }
    let v0a = to_vec3_from_skew_mat(&m0);
    assert!(v0.sub(&v0a).norm() < 1.0e-10);
}

/// Return a quaternion with `[i,j,k,w]` storage
/// the input must be a rotation matrix
pub fn to_quaternion<Real>(p: &[Real; 9]) -> [Real; 4]
where
    Real: num_traits::Float + std::fmt::Debug,
{
    let one = Real::one();
    let one4th = one / (one + one + one + one);
    let smat = [
        one + p[0] - p[4] - p[8], // 00
        p[3] + p[1],              // 01
        p[6] + p[2],              // 02
        p[5] - p[7],              // 03
        p[1] + p[3],              // 10
        one - p[0] + p[4] - p[8], // 11
        p[7] + p[5],              // 12
        p[6] - p[2],              // 13
        p[6] + p[2],              // 20
        p[7] + p[5],              // 21
        one - p[0] - p[4] + p[8], // 22
        p[1] - p[3],              // 23
        p[5] - p[7],              // 30
        p[6] - p[2],              // 31
        p[1] - p[3],              // 32
        one + p[0] + p[4] + p[8], // 33
    ];

    let dias = [smat[0], smat[5], smat[10], smat[15]];
    use itertools::Itertools;
    let imax = dias
        .iter()
        .position_max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    assert!(dias[0] <= dias[imax], "{:?} {}", dias, imax);
    assert!(dias[1] <= dias[imax]);
    assert!(dias[2] <= dias[imax]);
    assert!(dias[3] <= dias[imax]);

    let mut quat = [Real::zero(); 4];
    quat[imax] = smat[imax * 4 + imax].sqrt() / (one + one);
    for k in 0..4 {
        if k == imax {
            continue;
        } else {
            quat[k] = smat[imax * 4 + k] * one4th / quat[imax];
        }
    }
    quat
}

#[test]
fn test_to_quaternion() {
    use crate::quaternion::Quaternion;
    let quats = [
        [-3., -2., 0., -1.],
        [3., -2., 0., -1.],
        [-1., 3., -2., -1.],
        [-1., -3., -2., -1.],
        [-1., -2., 3., -1.],
        [-1., -2., -3., -1.],
        [-1., -2., 1., -4.],
        [-1., -2., -1., -4.],
    ];
    for quat0 in quats {
        let quat0 = quat0.normalized();
        let r_mat = quat0.to_mat3_col_major();
        let quat1 = to_quaternion(&r_mat);
        let quat0 = nalgebra::Vector4::<f32>::from_row_slice(&quat0);
        let quat1 = nalgebra::Vector4::<f32>::from_row_slice(&quat1);
        assert!((quat0 - quat1).norm().min((quat0 + quat1).norm()) < 1.0e-7);
    }
}

// https://en.wikipedia.org/wiki/Axis%E2%80%93angle_representation
pub fn to_vec3_axisangle_from_rot_mat<T>(m: &[T; 9]) -> [T; 3]
where
    T: num_traits::Float,
{
    let one = T::one();
    let half = one / (one + one);
    let cos_t0 = (m[0] + m[4] + m[8] - one) * half;
    if (cos_t0 - one).abs() <= T::epsilon() {
        // very small rotation
        return [
            (m[5] - m[7]) * half,
            (m[6] - m[2]) * half,
            (m[1] - m[3]) * half,
        ];
    }
    let t0 = cos_t0.acos();
    let c0 = t0 * half / t0.sin();
    [c0 * (m[5] - m[7]), c0 * (m[6] - m[2]), c0 * (m[1] - m[3])]
}

/// Return a 2x3 matrix with column major storage by throwing away the last row
pub fn to_mat2x3_col_major_xy(m: &[f32; 9]) -> [f32; 6] {
    [m[0], m[1], m[3], m[4], m[6], m[7]]
}

pub fn to_columns<T>(a: &[T; 9]) -> ([T; 3], [T; 3], [T; 3])
where
    T: num_traits::Float,
{
    ([a[0], a[1], a[2]], [a[3], a[4], a[5]], [a[6], a[7], a[8]])
}

// above: to methods
// ---------------------------------------------

pub fn add_in_place_scaled_outer_product<T>(m: &mut [T; 9], s: T, a: &[T; 3], b: &[T; 3])
where
    T: num_traits::Float,
{
    m[0] = m[0] + s * a[0] * b[0];
    m[1] = m[1] + s * a[1] * b[0];
    m[2] = m[2] + s * a[2] * b[0];
    m[3] = m[3] + s * a[0] * b[1];
    m[4] = m[4] + s * a[1] * b[1];
    m[5] = m[5] + s * a[2] * b[1];
    m[6] = m[6] + s * a[0] * b[2];
    m[7] = m[7] + s * a[1] * b[2];
    m[8] = m[8] + s * a[2] * b[2];
}

pub fn add<T>(a: &[T; 9], b: &[T; 9]) -> [T; 9]
where
    T: num_traits::Float,
{
    std::array::from_fn(|i| a[i] + b[i])
}

pub fn sub<T>(a: &[T; 9], b: &[T; 9]) -> [T; 9]
where
    T: num_traits::Float,
{
    std::array::from_fn(|i| a[i] - b[i])
}

pub fn try_inverse<T>(b: &[T; 9]) -> Option<[T; 9]>
where
    T: num_traits::Float,
{
    let det = b[0] * b[4] * b[8] + b[3] * b[7] * b[2] + b[6] * b[1] * b[5]
        - b[0] * b[7] * b[5]
        - b[6] * b[4] * b[2]
        - b[3] * b[1] * b[8];
    if det.is_zero() {
        return None;
    }
    let inv_det = T::one() / det;
    Some([
        inv_det * (b[4] * b[8] - b[5] * b[7]),
        inv_det * (b[2] * b[7] - b[1] * b[8]),
        inv_det * (b[1] * b[5] - b[2] * b[4]),
        inv_det * (b[5] * b[6] - b[3] * b[8]),
        inv_det * (b[0] * b[8] - b[2] * b[6]),
        inv_det * (b[2] * b[3] - b[0] * b[5]),
        inv_det * (b[3] * b[7] - b[4] * b[6]),
        inv_det * (b[1] * b[6] - b[0] * b[7]),
        inv_det * (b[0] * b[4] - b[1] * b[3]),
    ])
}

#[test]
fn test_try_inverse() {
    let m: [f32; 9] = [1.7, 3., 2.3, 4.5, 5., 1.5, 3.3, 2., 4.2];
    let mi = m.try_inverse().unwrap();
    let mmi = m.mult_mat_col_major(&mi);
    let mim = mi.mult_mat_col_major(&m);
    for i in 0..3 {
        for j in 0..3 {
            let v = if i == j { 1. } else { 0. };
            assert!((v - mmi[i + j * 3]).abs() < 1.0e-6);
            assert!((v - mim[i + j * 3]).abs() < 1.0e-6);
        }
    }
}

pub fn transform_homogeneous<Real>(transform: &[Real; 9], x: &[Real; 2]) -> Option<[Real; 2]>
where
    Real: num_traits::Float,
{
    let y2 = transform[2] * x[0] + transform[5] * x[1] + transform[8];
    if y2.is_zero() {
        return None;
    }
    //
    let y0 = transform[0] * x[0] + transform[3] * x[1] + transform[6];
    let y1 = transform[1] * x[0] + transform[4] * x[1] + transform[7];
    Some([y0 / y2, y1 / y2])
}

pub fn transform_direction<Real>(transform: &[Real; 9], x: &[Real; 2]) -> [Real; 2]
where
    Real: num_traits::Float,
{
    [
        transform[0] * x[0] + transform[3] * x[1] + transform[6],
        transform[1] * x[0] + transform[4] * x[1] + transform[7],
    ]
}

pub fn mult_vec<Real>(a: &[Real; 9], b: &[Real; 3]) -> [Real; 3]
where
    Real: num_traits::Float,
{
    [
        a[0] * b[0] + a[3] * b[1] + a[6] * b[2],
        a[1] * b[0] + a[4] * b[1] + a[7] * b[2],
        a[2] * b[0] + a[5] * b[1] + a[8] * b[2],
    ]
}

pub fn transpose<Real>(m: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    [m[0], m[3], m[6], m[1], m[4], m[7], m[2], m[5], m[8]]
}

pub fn scale<Real>(m: &[Real; 9], scale: Real) -> [Real; 9]
where
    Real: num_traits::Float,
{
    std::array::from_fn(|i| m[i] * scale)
}

pub fn mult_mat_col_major<Real>(a: &[Real; 9], b: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let mut r = [Real::zero(); 9];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                r[i + 3 * j] = r[i + 3 * j] + a[i + 3 * k] * b[k + 3 * j]
            }
        }
    }
    r
}

pub fn mult_mat_row_major<Real>(a: &[Real; 9], b: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    let mut r = [Real::zero(); 9];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                r[i + 3 * j] = r[i + 3 * j] + a[i + 3 * k] * b[j + 3 * k]
            }
        }
    }
    r
}

/// Determinant of a 3x3 matrix
pub fn determinant<Real>(b: &[Real; 9]) -> Real
where
    Real: num_traits::Float,
{
    b[0] * b[4] * b[8] + b[3] * b[7] * b[2] + b[6] * b[1] * b[5]
        - b[0] * b[7] * b[5]
        - b[6] * b[4] * b[2]
        - b[3] * b[1] * b[8]
}

/// # Argument
/// * `n` - world 3D vector that corresponds local z (no need to be unit vector)
pub fn transform_lcl2world_given_local_z(n: &[f32; 3]) -> [f32; 9] {
    use crate::vec3;
    let n = vec3::normalize(n);
    let t = if n[0].abs() > 0.1 {
        [0., 1., 0.]
    } else {
        [1., 0., 0.]
    };
    let u = vec3::normalize(&vec3::cross(&t, &n));
    let v = vec3::cross(&n, &u);
    [u[0], u[1], u[2], v[0], v[1], v[2], n[0], n[1], n[2]]
}

/// Return 3x3 rotation matrix as a column major storage.
/// That rotation matrix rotate `v0: &[T;3]` to `v1: &[T;3]`.
pub fn minimum_rotation_matrix<T>(v0: &[T; 3], v1: &[T; 3]) -> [T; 9]
where
    T: num_traits::Float,
{
    use crate::vec3::Vec3;
    let one = T::one();
    let half = one / (one + one);
    let ep = v0.normalize();
    let eq = v1.normalize();
    let n = ep.cross(&eq);
    let st2 = n.dot(&n);
    let ct = ep.dot(&eq);

    if st2 < T::epsilon() {
        // very small angle or n is zero
        // inifinitesimal rotation
        if ct > one - T::epsilon() {
            return [
                T::one() + half * (n[0] * n[0] - st2),
                -n[2] + half * (n[0] * n[1]),
                n[1] + half * (n[0] * n[2]),
                n[2] + half * (n[1] * n[0]),
                T::one() + half * (n[1] * n[1] - st2),
                -n[0] + half * (n[1] * n[2]),
                -n[1] + half * (n[2] * n[0]),
                n[0] + half * (n[2] * n[1]),
                T::one() + half * (n[2] * n[2] - st2),
            ];
        } else {
            let (epx, epy) = crate::vec3::basis_xy_from_basis_z(&ep);
            let eqx = epx.sub(&eq.scale(eq.dot(&epx))); // vector orthogonal to eq
            let eqy = eq.cross(&eqx);
            return [
                eqx.dot(&epx),
                eqy.dot(&epx),
                eq.dot(&epx),
                eqx.dot(&epy),
                eqy.dot(&epy),
                eq.dot(&epy),
                eqx.dot(&ep),
                eqy.dot(&ep),
                eq.dot(&ep),
            ];
        }
    }
    let st = st2.sqrt();
    let n = n.normalize();
    // Rodoriguez's rotation formula
    [
        ct + (T::one() - ct) * n[0] * n[0],
        -n[2] * st + (T::one() - ct) * n[0] * n[1],
        n[1] * st + (T::one() - ct) * n[0] * n[2],
        n[2] * st + (T::one() - ct) * n[1] * n[0],
        ct + (T::one() - ct) * n[1] * n[1],
        -n[0] * st + (T::one() - ct) * n[1] * n[2],
        -n[1] * st + (T::one() - ct) * n[2] * n[0],
        n[0] * st + (T::one() - ct) * n[2] * n[1],
        ct + (T::one() - ct) * n[2] * n[2],
    ]
}

// -----------------------------------
// Below: SVD related

/// Singular Value Decomposition (SVD)
/// input = U * S * V^t
///
/// # Returns
/// (U, S, V)
pub fn svd<Real>(
    f: &[Real; 9],
    mode: crate::mat3_sym::EigenDecompositionModes,
) -> Option<([Real; 9], [Real; 3], [Real; 9])>
where
    Real: num_traits::Float + num_traits::FloatConst,
{
    let (u, s, v) = crate::mat3_row_major::svd(f, mode)?;
    Some((transpose(&v), s, transpose(&u)))
}

pub fn enforce_rotation_matrix_for_svd<Real>(
    u: &[Real; 9],
    l: &[Real; 3],
    v: &[Real; 9],
) -> ([Real; 9], [Real; 3], [Real; 9])
where
    Real: num_traits::Float + std::fmt::Debug,
{
    if determinant(v) < Real::zero() || determinant(u) < Real::zero() {
        let mut u = *u;
        let mut l = *l;
        let mut v = *v;
        if determinant(&v) < Real::zero() {
            v[6] = -v[6]; // v[0,2] = v[0*3+2]
            v[7] = -v[7]; // v[1,2] = v[1*3+2]
            v[8] = -v[8];
            l[2] = -l[2];
        }
        if determinant(&u) < Real::zero() {
            u[6] = -u[6]; // v[0,2] = v[0*3+2]
            u[7] = -u[7]; // v[1,2] = v[1*3+2]
            u[8] = -u[8];
            l[2] = -l[2];
        }
        (u, l, v)
    } else {
        (*u, *l, *v)
    }
}

#[test]
fn test_svd() {
    use rand::Rng;
    use rand::SeedableRng;
    use Mat3ColMajor;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    for (_iter, i_mode_eigen, is_rot) in itertools::iproduct!(0..100, 0..2, 0..2) {
        let m: [f64; 9] = std::array::from_fn(|_| rng.random_range(-1f64..1f64));
        let (u, s, v) = {
            let mode = match i_mode_eigen {
                0 => crate::mat3_sym::EigenDecompositionModes::JacobiNumIter(100),
                1 => crate::mat3_sym::EigenDecompositionModes::Analytic,
                _ => unreachable!(),
            };
            svd(&m, mode).unwrap()
        };
        let (u, s, v) = if is_rot == 1 {
            enforce_rotation_matrix_for_svd(&u, &s, &v)
        } else {
            (u, s, v)
        };
        if is_rot == 1 {
            let det_v = determinant(&v);
            assert!((det_v - 1.).abs() < 1.0e-10);
            let det_u = determinant(&u);
            assert!((det_u - 1.).abs() < 1.0e-10);
        }
        {
            // test u
            let diff = transpose(&u)
                .mult_mat_col_major(&u)
                .sub(&from_identity())
                .squared_norm();
            assert!(diff < 1.0e-20f64, "{}", diff);
        }
        {
            // test V V^t = I
            let diff = transpose(&v)
                .mult_mat_col_major(&v)
                .sub(&from_identity())
                .squared_norm();
            assert!(diff < 1.0e-20f64, "{}", diff);
        }
        {
            // test A = USV^t
            let s = from_diagonal(&s);
            let m1 = u.mult_mat_col_major(&s).mult_mat_col_major(&transpose(&v));
            let diff = m1.sub(&m).squared_norm();
            assert!(diff < 1.0e-20f64, "{} {:?} {:?}", diff, m, m1);
        }
    }
}

/// when SVD of 3x3 matrix a is U*S*V^T, compute U*V^T
/// determinant of the result is one
pub fn rotational_component<T>(a: &[T; 9]) -> [T; 9]
where
    T: num_traits::Float + num_traits::FloatConst + std::fmt::Debug,
{
    use crate::mat3_sym::EigenDecompositionModes;
    let (u, _s, v) = svd(a, EigenDecompositionModes::JacobiNumIter(20)).unwrap();
    let v_t = transpose(&v);
    let u_vt = mult_mat_col_major(&u, &v_t);
    dbg!(determinant(&u_vt));
    if determinant(&u_vt) > T::zero() {
        u_vt
    } else {
        let v_t = [
            -v_t[0], v_t[1], v_t[2], -v_t[3], v_t[4], v_t[5], -v_t[6], v_t[7], v_t[8],
        ];
        mult_mat_col_major(&u, &v_t)
    }
}

#[test]
fn test_rotational_component() {
    use rand::Rng;
    use rand::SeedableRng;
    use Mat3ColMajor;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    for _iter in 0..100 {
        let m: [f64; 9] = std::array::from_fn(|_| rng.random_range(-1f64..1f64));
        // let (u, s, v) = svd(&m, crate::mat3_sym::EigenDecompositionModes::Analytic).unwrap();
        let r = rotational_component(&m);
        assert!((r.determinant() - 1.).abs() < 1.0e-8);
    }
}

/// Jacobian of singular value decomposition
///
/// # Reference
/// Papadopoulo, Théodore & Lourakis, Manolis. (2000). "
/// Estimating the Jacobian of the Singular Value Decomposition"
/// Theory and Applications. 554-570.
///
/// # Returns `(diff_u, diff_s, diff:v)`
/// - `diff_u[k][i*3+j]` : differentiation of u is a skew matrix, represented by a 3D vector
/// - `diff_v[k][i*3+j]` : differentiation of u is a skew matrix, represented by a 3D vector
#[allow(clippy::type_complexity)]
pub fn svd_differential(
    u: [f64; 9],
    s: [f64; 3],
    v: [f64; 9],
) -> ([[f64; 9]; 3], [[f64; 9]; 3], [[f64; 9]; 3]) {
    use Mat3ColMajor;
    let (du, ds, dv) = crate::mat3_row_major::svd_differential(v.transpose(), s, u.transpose());
    (dv, ds, du)
}

#[test]
fn test_svd_differential() {
    use crate::vec3::Vec3;
    use rand::Rng;
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let eps = 1.0e-6;
    for _iter in 0..100 {
        let m0: [f64; 9] = std::array::from_fn(|_| rng.random::<f64>());
        let (u0, s0, v0) = svd(
            &m0,
            crate::mat3_sym::EigenDecompositionModes::JacobiNumIter(100),
        )
        .unwrap();
        let (diff_u, diff_s, diff_v) = svd_differential(u0, s0, v0);
        for (i, j) in itertools::iproduct!(0..3, 0..3) {
            let m1 = {
                let mut m1 = m0;
                m1[i + 3 * j] += eps;
                m1
            };
            let (u1, s1, v1) = svd(
                &m1,
                crate::mat3_sym::EigenDecompositionModes::JacobiNumIter(100),
            )
            .unwrap();
            {
                let du_num = transpose(&u1).mult_mat_col_major(&u0).scale(1. / eps);
                let du_num = to_vec3_from_skew_mat(&du_num);
                let du_ana = [
                    diff_u[0][i + 3 * j],
                    diff_u[1][i + 3 * j],
                    diff_u[2][i + 3 * j],
                ];
                // println!("du: {} {} {:?} {:?}", i, j, du_ana, du_num);
                assert!(
                    du_num.sub(&du_ana).norm() < 1.0e-4 * (1.0 + du_ana.norm()),
                    "du: {} {} {:?} {:?}",
                    i,
                    j,
                    du_ana,
                    du_num
                );
            }
            {
                let ds_num = [
                    (s1[0] - s0[0]) / eps,
                    (s1[1] - s0[1]) / eps,
                    (s1[2] - s0[2]) / eps,
                ];
                let ds_ana = [
                    diff_s[0][i + 3 * j],
                    diff_s[1][i + 3 * j],
                    diff_s[2][i + 3 * j],
                ];
                // println!("ds: {} {} {:?} {:?}", i, j, ds_num, ds_ana);
                assert!(
                    ds_num.sub(&ds_ana).norm() < 1.0e-5 * (1.0 + ds_ana.norm()),
                    "{} {} {:?} {:?}",
                    i,
                    j,
                    ds_ana,
                    ds_num
                );
            }
            {
                let dv_num = transpose(&v1).mult_mat_col_major(&v0).scale(1. / eps);
                let dv_num = to_vec3_from_skew_mat(&dv_num);
                let dv_ana = [
                    diff_v[0][i + 3 * j],
                    diff_v[1][i + 3 * j],
                    diff_v[2][i + 3 * j],
                ];
                // println!("d0: {} {} {:?} {:?}", i, j, dv_ana, dv_num);
                assert!(
                    dv_num.sub(&dv_ana).norm() < 1.0e-3 * (1.0 + dv_ana.norm()),
                    "{:?}",
                    dv_ana
                );
            }
        }
    }
}

// Above: SVD related
// -------------------------------------------------

/// Add three vectors
pub fn add_three<T>(a: &[T; 9], b: &[T; 9], c: &[T; 9]) -> [T; 9]
where
    T: num_traits::Float,
{
    [
        a[0] + b[0] + c[0],
        a[1] + b[1] + c[1],
        a[2] + b[2] + c[2],
        a[3] + b[3] + c[3],
        a[4] + b[4] + c[4],
        a[5] + b[5] + c[5],
        a[6] + b[6] + c[6],
        a[7] + b[7] + c[7],
        a[8] + b[8] + c[8],
    ]
}
