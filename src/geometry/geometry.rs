// Basic 3D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// 3D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > f64::EPSILON {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[derive(Debug, Clone)]
pub enum CurveType {
    Line {
        start: Point3D,
        end: Point3D,
    },
    Circle {
        center: Point3D,
        normal: Vector3D,
        radius: f64,
    },
    Ellipse {
        center: Point3D,
        normal: Vector3D,
        major_axis: Vector3D,
        minor_axis: Vector3D,
        major_radius: f64,
        minor_radius: f64,
    },
    BSpline {
        control_points: Vec<Point3D>,
        knots: Vec<f64>,
        degree: u32,
        weights: Option<Vec<f64>>, // For NURBS
    },
    Parabola {
        apex: Point3D,
        axis: Vector3D,
        focal_length: f64,
    },
    Hyperbola {
        center: Point3D,
        axis: Vector3D,
        major_radius: f64,
        minor_radius: f64,
    },
}

// Surface types
#[derive(Debug, Clone)]
pub enum SurfaceType {
    Plane {
        origin: Point3D,
        normal: Vector3D,
    },
    Cylinder {
        axis_start: Point3D,
        axis_end: Point3D,
        radius: f64,
    },
    Cone {
        apex: Point3D,
        axis: Vector3D,
        half_angle: f64,
    },
    Sphere {
        center: Point3D,
        radius: f64,
    },
    Torus {
        center: Point3D,
        axis: Vector3D,
        major_radius: f64,
        minor_radius: f64,
    },
    BSplineSurface {
        control_points: Vec<Vec<Point3D>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_degree: u32,
        v_degree: u32,
        weights: Option<Vec<Vec<f64>>>, // For NURBS
    },
}