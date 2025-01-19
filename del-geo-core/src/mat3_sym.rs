fn squared_norm<Real>(sm: &[Real; 6]) -> Real
where
    Real: num_traits::Float,
{
    let two = Real::one() + Real::one();
    sm[0] * sm[0]
        + sm[1] * sm[1]
        + sm[2] * sm[2]
        + two * (sm[3] * sm[3] + sm[4] * sm[4] + sm[5] * sm[5])
}

pub fn eigen_decomp<Real>(sm: [Real; 6], nitr: usize) -> Option<([Real; 9], [Real; 3])>
where
    Real: num_traits::Float,
{
    let zero = Real::zero();
    let one = Real::one();
    let two = one + one;
    let half = one / two;
    let mut u = [zero; 9];
    // initialize u as identity matrix
    u[0] = one;
    u[4] = one;
    u[8] = one;
    let dnrm = squared_norm(&sm);
    if dnrm < Real::epsilon() {
        return None;
    } // this matrix is too small
    let scale = dnrm.sqrt();
    let invscl = one / scale;
    let mut sms = sm.map(|x| x * invscl);

    for _itr in 0..nitr {
        let m = sms;
        let v = u;
        let a12 = sms[3].abs();
        let a20 = sms[4].abs();
        let a01 = sms[5].abs();
        if a12 >= a20 && a12 >= a01 {
            // when a12 sms[3] is the biggest
            let t = half * (two * m[3]).atan2(m[2] - m[1]);
            let ct = t.cos();
            let st = t.sin();
            sms[1] = ct * ct * m[1] + st * st * m[2] - two * st * ct * m[3];
            sms[2] = ct * ct * m[2] + st * st * m[1] + two * st * ct * m[3];
            sms[3] = zero; // (ct*ct-st*st)*m[3]+st*ct*(m[1]-m[2]);
            sms[4] = st * m[5] + ct * m[4];
            sms[5] = ct * m[5] - st * m[4];
            //
            u[1] = ct * v[1] - st * v[2];
            u[2] = st * v[1] + ct * v[2];
            u[4] = ct * v[4] - st * v[5];
            u[5] = st * v[4] + ct * v[5];
            u[7] = ct * v[7] - st * v[8];
            u[8] = st * v[7] + ct * v[8];
        } else if a20 >= a01 && a20 >= a12 {
            // when a20 sms[4] is the biggest
            // the above condition statement shoud pass exactly once for each iteration.
            let t = half * (two * m[4]).atan2(m[2] - m[0]);
            let ct = t.cos();
            let st = t.sin();
            sms[0] = ct * ct * m[0] + st * st * m[2] - two * st * ct * m[4];
            sms[2] = ct * ct * m[2] + st * st * m[0] + two * st * ct * m[4];
            sms[3] = st * m[5] + ct * m[3];
            sms[4] = zero; // (ct*ct-st*st)*m[4]+st*ct*(m[0]-m[2]);
            sms[5] = ct * m[5] - st * m[3];
            //
            u[0] = ct * v[0] - st * v[2];
            u[2] = st * v[0] + ct * v[2];
            u[3] = ct * v[3] - st * v[5];
            u[5] = st * v[3] + ct * v[5];
            u[6] = ct * v[6] - st * v[8];
            u[8] = st * v[6] + ct * v[8];
        } else {
            // when a01 sms[5] is the biggest
            // the condition statement shoud pass exactly once for each iteration.
            let t = half * (two * m[5]).atan2(m[1] - m[0]);
            let ct = t.cos();
            let st = t.sin();
            sms[0] = ct * ct * m[0] + st * st * m[1] - two * st * ct * m[5];
            sms[1] = ct * ct * m[1] + st * st * m[0] + two * st * ct * m[5];
            sms[3] = st * m[4] + ct * m[3];
            sms[4] = ct * m[4] - st * m[3];
            sms[5] = zero; // (ct*ct-st*st)*m[5]+st*ct*(m[0]-m[1]);
                           //
            u[0] = ct * v[0] - st * v[1];
            u[1] = st * v[0] + ct * v[1];
            u[3] = ct * v[3] - st * v[4];
            u[4] = st * v[3] + ct * v[4];
            u[6] = ct * v[6] - st * v[7];
            u[7] = st * v[6] + ct * v[7];
        }
    }
    let l = std::array::from_fn(|i| scale * sms[i]);
    Some((u, l))
}

#[test]
fn test_eigen_decomp() {
    use crate::mat3_row_major::Mat3RowMajor;
    use rand::Rng;
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(0u64);
    // std::uniform_real_distribution < double > dist(-50.0, 50.0);
    for _itr in 0..1000 {
        let sm = std::array::from_fn(|_| rng.gen::<f64>() * 50.);
        let Some((u, _l)) = eigen_decomp(sm, 20) else {
            todo!()
        };
        {
            let ut = u.transpose();
            let utu = u.mult_mat_row_major(&ut);
            let id = crate::mat3_row_major::from_identity();
            let diff = id.sub(&utu);
            let diffnorm = diff.squared_norm();
            assert!(diffnorm < 1.0e-20);
        }
    }
}
