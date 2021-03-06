use mat::*;
use vec::*;

// TODO

#[test]
fn test_mat2() {
    let a = Mat2 { x: Vec2 { x: 1f, y: 3f },
                   y: Vec2 { x: 2f, y: 4f } };
    let b = Mat2 { x: Vec2 { x: 2f, y: 4f },
                   y: Vec2 { x: 3f, y: 5f } };
    
    let v1 = Vec2::new(1f, 2f);
    let f1 = 0.5f;
    
    assert a == Mat2::new(1f, 3f,
                          2f, 4f);
    
    assert a == Mat2::from_cols(Vec2::new(1f, 3f),
                                Vec2::new(2f, 4f));
    
    assert Mat2::from_value(4f64) == Mat2::new(4f64, 0f64,
                                               0f64, 4f64);
    
    assert a[0] == Vec2::new(1f, 3f);
    assert a[1] == Vec2::new(2f, 4f);
    
    assert a.row(0) == Vec2::new(1f, 2f);
    assert a.row(1) == Vec2::new(3f, 4f);
    
    assert a.col(0) == Vec2::new(1f, 3f);
    assert a.col(1) == Vec2::new(2f, 4f);
    
    assert Mat2::identity() == Mat2::new(1f, 0f,
                                         0f, 1f);
    
    assert Mat2::zero() == Mat2::new(0f, 0f,
                                     0f, 0f);

    assert a.determinant() == -2f;
    assert a.trace() == 5f;
    
    assert a.neg() == Mat2::new(-1f, -3f,
                                -2f, -4f);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == Mat2::new(0.5f, 1.5f,
                                    1.0f, 2.0f);
    assert a.mul_v(&v1) == Vec2::new(5f, 11f);
    
    assert a.add_m(&b) == Mat2::new(3f, 7f,
                                    5f, 9f);
    assert a.sub_m(&b) == Mat2::new(-1f, -1f,
                                    -1f, -1f);
    assert a.mul_m(&b) == Mat2::new(10.0, 22.0,
                                    13.0, 29.0);
    assert a.dot(&b) == 40f; 
    
    assert a.transpose() == Mat2::new(1f, 2f,
                                      3f, 4f);

    assert option::unwrap(a.inverse()) == Mat2::new(-2f,  1.5f,
                                                    1f, -0.5f);

    assert Mat2::new(0f, 2f,
                     0f, 5f).inverse().is_none();
    
    // let ident: Mat2<float> = Matrix::identity();     // FIXME: there's something wrong with static functions here!
    let ident: Mat2<float> = Mat2::identity();
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert a.is_invertible();
    
    let c = Mat2::new(2f, 1f,
                      1f, 2f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert Mat2::from_value(6f).is_diagonal();
    
    assert a.to_mat3() == Mat3::new(1f, 3f, 0f,
                                    2f, 4f, 0f,
                                    0f, 0f, 1f);
    
    assert a.to_mat4() == Mat4::new(1f, 3f, 0f, 0f,
                                    2f, 4f, 0f, 0f,
                                    0f, 0f, 1f, 0f,
                                    0f, 0f, 0f, 1f);
}

fn test_mat2_mut() {
    let a = Mat2 { x: Vec2 { x: 1f, y: 3f },
                   y: Vec2 { x: 2f, y: 4f } };
    let b = Mat2 { x: Vec2 { x: 2f, y: 4f },
                   y: Vec2 { x: 3f, y: 5f } };
    
    let f1 = 0.5f;
    
    let mut mut_a = a;
    
    mut_a.swap_cols(0, 1);
    assert mut_a.col(0) == a.col(1);
    assert mut_a.col(1) == a.col(0);
    mut_a = a;
    
    mut_a.swap_rows(0, 1);
    assert mut_a.row(0) == a.row(1);
    assert mut_a.row(1) == a.row(0);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == Mat2::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_a.invert_self();
    assert mut_a == option::unwrap(a.inverse());
    mut_a = a;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat2_fuzzy_eq() {
    assert !Mat2::new(0.000001, 0.000001,
                      0.000001, 0.000001).fuzzy_eq(&Mat2::zero());
    assert Mat2::new(0.0000001, 0.0000001,
                     0.0000001, 0.0000001).fuzzy_eq(&Mat2::zero());
}

#[test]
fn test_mat3() {
    let a = Mat3 { x: Vec3 { x: 1f, y: 4f, z:  7f },
                   y: Vec3 { x: 2f, y: 5f, z:  8f },
                   z: Vec3 { x: 3f, y: 6f, z:  9f } };
    let b = Mat3 { x: Vec3 { x: 2f, y: 5f, z:  8f },
                   y: Vec3 { x: 3f, y: 6f, z:  9f },
                   z: Vec3 { x: 4f, y: 7f, z: 10f } };
    
    let v1 = Vec3::new(1f, 2f, 3f);
    let f1 = 0.5f;
    
    assert a == Mat3::new(1f, 4f, 7f,
                          2f, 5f, 8f,
                          3f, 6f, 9f);
    
    assert a == Mat3::from_cols(Vec3::new(1f, 4f, 7f),
                                Vec3::new(2f, 5f, 8f),
                                Vec3::new(3f, 6f, 9f));
    
    assert a[0] == Vec3::new(1f, 4f, 7f);
    assert a[1] == Vec3::new(2f, 5f, 8f);
    assert a[2] == Vec3::new(3f, 6f, 9f);
    
    assert a.row(0) == Vec3::new(1f, 2f, 3f);
    assert a.row(1) == Vec3::new(4f, 5f, 6f);
    assert a.row(2) == Vec3::new(7f, 8f, 9f);
    
    assert a.col(0) == Vec3::new(1f, 4f, 7f);
    assert a.col(1) == Vec3::new(2f, 5f, 8f);
    assert a.col(2) == Vec3::new(3f, 6f, 9f);
    
    assert Mat3::identity() == Mat3::new(1f, 0f, 0f,
                                         0f, 1f, 0f,
                                         0f, 0f, 1f);
    
    assert Mat3::zero() == Mat3::new(0f, 0f, 0f,
                                     0f, 0f, 0f,
                                     0f, 0f, 0f);

    assert a.determinant() == 0f;
    assert a.trace() == 15f;
    
    assert a.neg() == Mat3::new(-1f, -4f, -7f,
                                -2f, -5f, -8f,
                                -3f, -6f, -9f);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == Mat3::new(0.5f, 2.0f, 3.5f,
                                    1.0f, 2.5f, 4.0f,
                                    1.5f, 3.0f, 4.5f);
    assert a.mul_v(&v1) == Vec3::new(14f, 32f, 50f);
    
    assert a.add_m(&b) == Mat3::new(3f,  9f, 15f,
                                    5f, 11f, 17f,
                                    7f, 13f, 19f);
    assert a.sub_m(&b) == Mat3::new(-1f, -1f, -1f,
                                    -1f, -1f, -1f,
                                    -1f, -1f, -1f);
    assert a.mul_m(&b) == Mat3::new(36f,  81f, 126f,
                                    42f,  96f, 150f,
                                    48f, 111f, 174f);
    assert a.dot(&b) == 330f;
    
    assert a.transpose() == Mat3::new(1f, 2f, 3f,
                                      4f, 5f, 6f,
                                      7f, 8f, 9f);

    assert a.inverse().is_none();
    
    assert option::unwrap(Mat3::new(2f, 4f, 6f,
                                    0f, 2f, 4f,
                                    0f, 0f, 1f).inverse())
        == Mat3::new(0.5f,  -1f,  1f,
                       0f, 0.5f, -2f,
                       0f,   0f,  1f);
    
    // let ident: Mat3<float> = Matrix::identity();     // FIXME: there's something wrong with static functions here!
    let ident: Mat3<float> = Mat3::identity();

    assert option::unwrap(ident.inverse()) == ident;
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert !a.is_invertible();
    
    let c = Mat3::new(3f, 2f, 1f,
                      2f, 3f, 2f,
                      1f, 2f, 3f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert Mat3::from_value(6f).is_diagonal();
    
    assert a.to_mat4() == Mat4::new(1f, 4f, 7f, 0f,
                                    2f, 5f, 8f, 0f,
                                    3f, 6f, 9f, 0f,
                                    0f, 0f, 0f, 1f);
    
    // to_Quaternion
}

fn test_mat3_mut() {
    let a = Mat3 { x: Vec3 { x: 1f, y: 4f, z:  7f },
                   y: Vec3 { x: 2f, y: 5f, z:  8f },
                   z: Vec3 { x: 3f, y: 6f, z:  9f } };
    let b = Mat3 { x: Vec3 { x: 2f, y: 5f, z:  8f },
                   y: Vec3 { x: 3f, y: 6f, z:  9f },
                   z: Vec3 { x: 4f, y: 7f, z: 10f } };
    let c = Mat3 { x: Vec3 { x: 2f, y: 4f, z:  6f },
                   y: Vec3 { x: 0f, y: 2f, z:  4f },
                   z: Vec3 { x: 0f, y: 0f, z:  1f } };
    
    let f1 = 0.5f;
    
    let mut mut_a = a;
    let mut mut_c = c;
    
    mut_a.swap_cols(0, 2);
    assert mut_a.col(0) == a.col(2);
    assert mut_a.col(2) == a.col(0);
    mut_a = a;
    
    mut_a.swap_cols(1, 2);
    assert mut_a.col(1) == a.col(2);
    assert mut_a.col(2) == a.col(1);
    mut_a = a;
    
    mut_a.swap_rows(0, 2);
    assert mut_a.row(0) == a.row(2);
    assert mut_a.row(2) == a.row(0);
    mut_a = a;
    
    mut_a.swap_rows(1, 2);
    assert mut_a.row(1) == a.row(2);
    assert mut_a.row(2) == a.row(1);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == Mat3::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_c.invert_self();
    assert mut_c == option::unwrap(c.inverse());
    // mut_c = c;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat3_fuzzy_eq() {
    assert !Mat3::new(0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001).fuzzy_eq(&Mat3::zero());
    assert Mat3::new(0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001).fuzzy_eq(&Mat3::zero());
}

#[test]
fn test_mat4() {
    let a = Mat4 { x: Vec4 { x: 1f, y: 5f, z:  9f, w: 13f },
                   y: Vec4 { x: 2f, y: 6f, z: 10f, w: 14f },
                   z: Vec4 { x: 3f, y: 7f, z: 11f, w: 15f },
                   w: Vec4 { x: 4f, y: 8f, z: 12f, w: 16f } };
    let b = Mat4 { x: Vec4 { x: 2f, y: 6f, z: 10f, w: 14f },
                   y: Vec4 { x: 3f, y: 7f, z: 11f, w: 15f },
                   z: Vec4 { x: 4f, y: 8f, z: 12f, w: 16f },
                   w: Vec4 { x: 5f, y: 9f, z: 13f, w: 17f } };
    let c = Mat4 { x: Vec4 { x: 3f, y: 2f, z:  1f, w:  1f },
                   y: Vec4 { x: 2f, y: 3f, z:  2f, w:  2f },
                   z: Vec4 { x: 1f, y: 2f, z:  3f, w:  3f },
                   w: Vec4 { x: 0f, y: 1f, z:  1f, w:  0f } };
    
    let v1 = Vec4::new(1f, 2f, 3f, 4f);
    let f1 = 0.5f;
    
    assert a == Mat4::new(1f, 5f,  9f, 13f,
                          2f, 6f, 10f, 14f,
                          3f, 7f, 11f, 15f,
                          4f, 8f, 12f, 16f);
    
    assert a == Mat4::from_cols(Vec4::new(1f, 5f,  9f, 13f),
                                Vec4::new(2f, 6f, 10f, 14f),
                                Vec4::new(3f, 7f, 11f, 15f),
                                Vec4::new(4f, 8f, 12f, 16f));
    
    assert Mat4::from_value(4f64) == Mat4::new(4f64, 0f64, 0f64, 0f64,
                                               0f64, 4f64, 0f64, 0f64,
                                               0f64, 0f64, 4f64, 0f64,
                                               0f64, 0f64, 0f64, 4f64);
    
    assert a[0] == Vec4::new(1f, 5f,  9f, 13f);
    assert a[1] == Vec4::new(2f, 6f, 10f, 14f);
    assert a[2] == Vec4::new(3f, 7f, 11f, 15f);
    assert a[3] == Vec4::new(4f, 8f, 12f, 16f);
    
    assert a.row(0) == Vec4::new( 1f,  2f,  3f,  4f);
    assert a.row(1) == Vec4::new( 5f,  6f,  7f,  8f);
    assert a.row(2) == Vec4::new( 9f, 10f, 11f, 12f);
    assert a.row(3) == Vec4::new(13f, 14f, 15f, 16f);
    
    assert a.col(0) == Vec4::new(1f, 5f,  9f, 13f);
    assert a.col(1) == Vec4::new(2f, 6f, 10f, 14f);
    assert a.col(2) == Vec4::new(3f, 7f, 11f, 15f);
    assert a.col(3) == Vec4::new(4f, 8f, 12f, 16f);
    
    assert Mat4::identity() == Mat4::new(1f, 0f, 0f, 0f,
                                         0f, 1f, 0f, 0f,
                                         0f, 0f, 1f, 0f,
                                         0f, 0f, 0f, 1f);
    
    assert Mat4::zero() == Mat4::new(0f, 0f, 0f, 0f,
                                     0f, 0f, 0f, 0f,
                                     0f, 0f, 0f, 0f,
                                     0f, 0f, 0f, 0f);

    assert a.determinant() == 0f;
    assert a.trace() == 34f;
    
    assert a.neg() == Mat4::new(-1f, -5f,  -9f, -13f,
                                -2f, -6f, -10f, -14f,
                                -3f, -7f, -11f, -15f,
                                -4f, -8f, -12f, -16f);
    assert -a == a.neg();
    
    assert a.mul_t(f1) == Mat4::new(0.5f, 2.5f, 4.5f, 6.5f,
                                    1.0f, 3.0f, 5.0f, 7.0f,
                                    1.5f, 3.5f, 5.5f, 7.5f,
                                    2.0f, 4.0f, 6.0f, 8.0f);
    assert a.mul_v(&v1) == Vec4::new(30.0, 70.0, 110.0, 150.0);
    
    assert a.add_m(&b) == Mat4::new(3f, 11f, 19f, 27f,
                                    5f, 13f, 21f, 29f,
                                    7f, 15f, 23f, 31f,
                                    9f, 17f, 25f, 33f);
    assert a.sub_m(&b) == Mat4::new(-1f, -1f, -1f, -1f,
                                    -1f, -1f, -1f, -1f,
                                    -1f, -1f, -1f, -1f,
                                    -1f, -1f, -1f, -1f);
    assert a.mul_m(&b) == Mat4::new(100f, 228f, 356f, 484f,
                                    110f, 254f, 398f, 542f,
                                    120f, 280f, 440f, 600f,
                                    130f, 306f, 482f, 658f);
    assert a.dot(&b) == 1632f;
    
    assert a.transpose() == Mat4::new( 1f,  2f,  3f,  4f,
                                       5f,  6f,  7f,  8f,
                                       9f, 10f, 11f, 12f,
                                      13f, 14f, 15f, 16f);

    assert option::unwrap(c.inverse())
        .fuzzy_eq(&Mat4::new( 5f, -4f,  1f,  0f,
                             -4f,  8f, -4f,  0f,
                              4f, -8f,  4f,  8f,
                             -3f,  4f,  1f, -8f).mul_t(0.125f));
    
    // let ident: Mat4<float> = Matrix::identity();
    let ident: Mat4<float> = Mat4::identity();

    assert option::unwrap(ident.inverse()) == ident;
    
    assert ident.is_identity();
    assert ident.is_symmetric();
    assert ident.is_diagonal();
    assert !ident.is_rotated();
    assert ident.is_invertible();
    
    assert !a.is_identity();
    assert !a.is_symmetric();
    assert !a.is_diagonal();
    assert a.is_rotated();
    assert !a.is_invertible();
    
    let c = Mat4::new(4f, 3f, 2f, 1f,
                      3f, 4f, 3f, 2f,
                      2f, 3f, 4f, 3f,
                      1f, 2f, 3f, 4f);
    assert !c.is_identity();
    assert c.is_symmetric();
    assert !c.is_diagonal();
    assert c.is_rotated();
    assert c.is_invertible();
    
    assert Mat4::from_value(6f).is_diagonal();
}

fn test_mat4_mut() {
    let a = Mat4 { x: Vec4 { x: 1f, y: 5f, z:  9f, w: 13f },
                   y: Vec4 { x: 2f, y: 6f, z: 10f, w: 14f },
                   z: Vec4 { x: 3f, y: 7f, z: 11f, w: 15f },
                   w: Vec4 { x: 4f, y: 8f, z: 12f, w: 16f } };
    let b = Mat4 { x: Vec4 { x: 2f, y: 6f, z: 10f, w: 14f },
                   y: Vec4 { x: 3f, y: 7f, z: 11f, w: 15f },
                   z: Vec4 { x: 4f, y: 8f, z: 12f, w: 16f },
                   w: Vec4 { x: 5f, y: 9f, z: 13f, w: 17f } };
    let c = Mat4 { x: Vec4 { x: 3f, y: 2f, z:  1f, w:  1f },
                   y: Vec4 { x: 2f, y: 3f, z:  2f, w:  2f },
                   z: Vec4 { x: 1f, y: 2f, z:  3f, w:  3f },
                   w: Vec4 { x: 0f, y: 1f, z:  1f, w:  0f } };
    
    let f1 = 0.5f;
    
    let mut mut_a = a;
    let mut mut_c = c;
    
    mut_a.swap_cols(0, 3);
    assert mut_a.col(0) == a.col(3);
    assert mut_a.col(3) == a.col(0);
    mut_a = a;
    
    mut_a.swap_cols(1, 2);
    assert mut_a.col(1) == a.col(2);
    assert mut_a.col(2) == a.col(1);
    mut_a = a;
    
    mut_a.swap_rows(0, 3);
    assert mut_a.row(0) == a.row(3);
    assert mut_a.row(3) == a.row(0);
    mut_a = a;
    
    mut_a.swap_rows(1, 2);
    assert mut_a.row(1) == a.row(2);
    assert mut_a.row(2) == a.row(1);
    mut_a = a;
    
    mut_a.set(&b);
    assert mut_a == b;
    mut_a = a;
    
    mut_a.to_identity();
    assert mut_a.is_identity();
    mut_a = a;
    
    mut_a.to_zero();
    assert mut_a == Mat4::zero();
    mut_a = a;
    
    mut_a.mul_self_t(f1);
    assert mut_a == a.mul_t(f1);
    mut_a = a;
    
    mut_a.add_self_m(&b);
    assert mut_a == a.add_m(&b);
    mut_a = a;
    
    mut_a.sub_self_m(&b);
    assert mut_a == a.sub_m(&b);
    mut_a = a;
    
    mut_c.invert_self();
    assert mut_c == option::unwrap(c.inverse());
    // mut_c = c;
    
    mut_a.transpose_self();
    assert mut_a == a.transpose();
    // mut_a = a;
}

#[test]
fn test_mat4_fuzzy_eq() {
    assert !Mat4::new(0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001,
                      0.000001, 0.000001, 0.000001, 0.000001).fuzzy_eq(&Mat4::zero());
    assert Mat4::new(0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001,
                     0.0000001, 0.0000001, 0.0000001, 0.0000001).fuzzy_eq(&Mat4::zero());
}