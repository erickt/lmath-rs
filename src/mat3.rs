use core::cast::transmute;
use core::cmp::Eq;
use core::ptr::to_unsafe_ptr;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;
use numeric::funs::*;
use numeric::types::angle::Angle;
use numeric::types::float::Float;
use numeric::types::number::Number;

use quat::Quat;
use vec::Vec3;

/**
 *  A 3 x 3 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 */
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

pub impl<T:Copy Float> Mat3<T> {
    /**
     * Construct a 3 x 3 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2` - the third column of the matrix
     *
     * ~~~
     *         c0     c1     c2
     *      +------+------+------+
     *   r0 | c0r0 | c1r0 | c2r0 |
     *      +------+------+------+
     *   r1 | c0r1 | c1r1 | c2r1 |
     *      +------+------+------+
     *   r2 | c0r2 | c1r2 | c2r2 |
     *      +------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn new(c0r0:T, c0r1:T, c0r2:T,
                       c1r0:T, c1r1:T, c1r2:T,
                       c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(move c0r0, move c0r1, move c0r2),
                        Vec3::new(move c1r0, move c1r1, move c1r2),
                        Vec3::new(move c2r0, move c2r1, move c2r2))
    }
    
    /**
     * Construct a 3 x 3 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2
     *     +------+------+------+
     *  r0 | c0.x | c1.x | c2.x |
     *     +------+------+------+
     *  r1 | c0.y | c1.y | c2.y |
     *     +------+------+------+
     *  r2 | c0.z | c1.z | c2.z |
     *     +------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_cols(c0: Vec3<T>,
                             c1: Vec3<T>,
                             c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: move c0,
               y: move c1,
               z: move c2 }
    }
    
    /**
     * Construct a 3 x 3 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2
     *     +-----+-----+-----+
     *  r0 | val |   0 |   0 |
     *     +-----+-----+-----+
     *  r1 |   0 | val |   0 |
     *     +-----+-----+-----+
     *  r2 |   0 |   0 | val |
     *     +-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat3<T> {
        let _0 = Number::from(0);
        Mat3::new(value,    _0,    _0,
                     _0, value,    _0,
                     _0,    _0, value)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat3<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat3::new(_1, _0, _0,
                  _0, _1, _0,
                  _0, _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat3<T> {
        let _0 = Number::from(0);
        Mat3::new(_0, _0, _0,
                  _0, _0, _0,
                  _0, _0, _0)
    }
    
    /**
     * Construct a matrix from an angular rotation around the `x` axis
     */
    #[inline(always)]
    static pure fn from_angle_x<A:Angle<T>>(theta: A) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(&theta.to_radians());
        let sin_theta = sin(&theta.to_radians());
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        
        Mat3::new(_1,        _0,         _0,
                  _0,  cos_theta, sin_theta,
                  _0, -sin_theta, cos_theta)
    }
    
    /**
     * Construct a matrix from an angular rotation around the `y` axis
     */
    #[inline(always)]
    static pure fn from_angle_y<A:Angle<T>>(theta: A) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(&theta.to_radians());
        let sin_theta = sin(&theta.to_radians());
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        
        Mat3::new(cos_theta, _0, -sin_theta,
                         _0, _1,         _0,
                  sin_theta, _0,  cos_theta)
    }
    
    /**
     * Construct a matrix from an angular rotation around the `z` axis
     */
    #[inline(always)]
    static pure fn from_angle_z<A:Angle<T>>(theta: A) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let cos_theta = cos(&theta.to_radians());
        let sin_theta = sin(&theta.to_radians());
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        
        Mat3::new( cos_theta, sin_theta, _0,
                  -sin_theta, cos_theta, _0,
                          _0,        _0, _1)
    }
    
    /**
     * Construct a matrix from Euler angles
     *
     * # Arguments
     *
     * * `theta_x` - the angular rotation around the `x` axis (pitch)
     * * `theta_y` - the angular rotation around the `y` axis (yaw)
     * * `theta_z` - the angular rotation around the `z` axis (roll)
     */
    #[inline(always)]
    static pure fn from_angle_xyz<A:Angle<T>>(theta_x: A, theta_y: A, theta_z: A) -> Mat3<T> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
        let cx = cos(&theta_x.to_radians());
        let sx = sin(&theta_x.to_radians());
        let cy = cos(&theta_y.to_radians());
        let sy = sin(&theta_y.to_radians());
        let cz = cos(&theta_z.to_radians());
        let sz = sin(&theta_z.to_radians());
        
        Mat3::new(            cy*cz,             cy*sz,   -sy,
                  -cx*sz + sx*sy*cz,  cx*cz + sx*sy*sz, sx*cy,
                   sx*sz + cx*sy*cz, -sx*cz + cx*sy*sz, cx*cy)
    }
    
    /**
     * Construct a matrix from an axis and an angular rotation
     */
    #[inline(always)]
    static pure fn from_axis_angle<A:Angle<T>>(axis: &Vec3<T>, theta: A) -> Mat3<T> {
        let c:  T = cos(&theta.to_radians());
        let s:  T = sin(&theta.to_radians());
        let _0: T = Number::from(0);
        let _1: T = Number::from(1);
        let _1_c:  T = _1 - c;
        
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        
        Mat3::new(_1_c*x*x + c,   _1_c*x*y + s*z, _1_c*x*z - s*y,
                  _1_c*x*y - s*z, _1_c*y*y + c,   _1_c*y*z + s*x,
                  _1_c*x*z + s*y, _1_c*y*z - s*x, _1_c*z*z + c)
    }
}

pub impl<T:Copy Float> Mat3<T>: Matrix<T, Vec3<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec3<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec3<T> {
        Vec3::new(self[0][i],
                  self[1][i],
                  self[2][i])
    }
    
    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  1 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  1 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  1 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn identity() -> Mat3<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat3::new(_1, _0, _0,
                  _0, _1, _0,
                  _0, _0, _1)
    }
    
    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  0 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn zero() -> Mat3<T> {
        let _0 = Number::from(0);
        Mat3::new(_0, _0, _0,
                  _0, _0, _0,
                  _0, _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self.row(0).dot(&other.col(0)),
                  self.row(1).dot(&other.col(0)),
                  self.row(2).dot(&other.col(0)),
            
                  self.row(0).dot(&other.col(1)),
                  self.row(1).dot(&other.col(1)),
                  self.row(2).dot(&other.col(1)),
                  
                  self.row(0).dot(&other.col(2)),
                  self.row(1).dot(&other.col(2)),
                  self.row(2).dot(&other.col(2)))
    }
    
    pure fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pure fn determinant(&self) -> T {
        self.col(0).dot(&self.col(1).cross(&self.col(2)))
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2]
    }

    // #[inline(always)]
    pure fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        if d.fuzzy_eq(&Number::from(0)) {
            None
        } else {
            Some(Mat3::from_cols(self[1].cross(&self[2]).div_t(d),
                                 self[2].cross(&self[0]).div_t(d),
                                 self[0].cross(&self[1]).div_t(d)).transpose())
        }
    }
    
    #[inline(always)]
    pure fn transpose(&self) -> Mat3<T> {
        Mat3::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = Number::from(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }

    #[inline(always)]
    pure fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(&Number::zero())
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat3<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Mat3<T>: MutableMatrix<T, Vec3<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec3<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
                   self.col_mut(b));
    }
    
    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }
    
    #[inline(always)]
    fn set(&mut self, other: &Mat3<T>) {
        (*self) = (*other);
    }
    
    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Mat3::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat3::zero();
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
        self.col_mut(2).mul_self_t(&value);
    }
    
    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
        self.col_mut(2).add_self_v(&other[2]);
    }
    
    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
        self.col_mut(2).sub_self_v(&other[2]);
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(0).index_mut(2), self.col_mut(2).index_mut(0));
        
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
        util::swap(self.col_mut(1).index_mut(2), self.col_mut(2).index_mut(1));
        
        util::swap(self.col_mut(2).index_mut(0), self.col_mut(0).index_mut(2));
        util::swap(self.col_mut(2).index_mut(1), self.col_mut(1).index_mut(2));
    }
}

pub impl<T:Copy Float> Mat3<T>: Matrix3<T, Vec3<T>> {
    #[inline(always)]
    static pure fn from_axis_angle<A:Angle<T>>(axis: &Vec3<T>, theta: A) -> Mat3<T> {
        let c:  T = cos(&theta.to_radians());
        let s:  T = sin(&theta.to_radians());
        let _0: T = Number::from(0);
        let _1: T = Number::from(1);
        let _1_c:  T = _1 - c;
        
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        
        Mat3::new(_1_c * x * x + c,       _1_c * x * y + s * z,   _1_c * x * z - s * y,
                  _1_c * x * y - s * z,   _1_c * y * y + c,       _1_c * y * z + s * x,
                  _1_c * x * z + s * y,   _1_c * y * z - s * x,   _1_c * z * z + c)
    }
    
    #[inline(always)]
    pure fn to_mat4(&self) -> Mat4<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat4::new(self[0][0], self[0][1], self[0][2], _0,
                  self[1][0], self[1][1], self[1][2], _0,
                  self[2][0], self[2][1], self[2][2], _0,
                          _0,         _0,         _0, _1)
    }
    
    pure fn to_Quat() -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s;
        let w, x, y, z;
        let trace = self.trace();
        
        let _1:   T = Number::from(1.0);
        let half: T = Number::from(0.5);
        
        if trace >= Number::from(0) {
            s = (_1 + trace).sqrt();
            w = half * s;
            s = half / s;
            x = (self[1][2] - self[2][1]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[0][1] - self[1][0]) * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (half + (self[0][0] - self[1][1] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[1][2] - self[2][1]) * s;
        } else if self[1][1] > self[2][2] {
            s = (half + (self[1][1] - self[0][0] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[2][0] - self[0][2]) * s;
        } else {
            s = (half + (self[2][2] - self[0][0] - self[1][1])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[2][0] - self[0][2]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[0][1] - self[1][0]) * s;
        }
        
        Quat::new(w, x, y, z)
    }
}

pub impl<T:Copy> Mat3<T>: Index<uint, Vec3<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec3<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat3<T>, *Vec3<T>>(
                to_unsafe_ptr(self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Float> Mat3<T>: Neg<Mat3<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat3<T> {
        Mat3::from_cols(-self[0], -self[1], -self[2])
    }
}

pub impl<T:Copy Float> Mat3<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Mat3<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Mat3<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy Float> Mat3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}