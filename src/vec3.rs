
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn length(self) -> f32 {
        (
            self.e[0] * self.e[0]
            +self.e[1] * self.e[1]
            +self.e[2] * self.e[2]
        ).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] +self.e[1] * self.e[1] +self.e[2] * self.e[2]
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
            ]
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ]
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, lambda: f32) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * lambda,
                self.e[1] * lambda,
                self.e[2] * lambda,
            ]
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            e: [
                v.e[0] * self,
                v.e[1] * self,
                v.e[2] * self,
            ]
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, x: f32) -> Self::Output {
        let lambda = 1.0 / x;

        Vec3 {
            e: [
                self.e[0] * lambda, 
                self.e[1] * lambda, 
                self.e[2] * lambda
            ]
        }
    }
}