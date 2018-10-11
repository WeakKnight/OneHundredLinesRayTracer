use std::ops;
#[derive(Copy, Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3{
    fn new(x:f32, y:f32, z:f32)->Vector3{
        Vector3{x:x,y:y,z:z}
    }
    fn normalize(self)->Self{
        self*(1.0/(self.x*self.x+self.y*self.y+self.z*self.z).sqrt())
    }
    fn dot(self, other:Vector3)->f32{
        self.x * other.x + self.y * other.y + self.z *other.z
    }
    fn cross(self, other:Vector3)->Vector3{
        Vector3::new(self.y * other.z - self.z * other.y, self.z*other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
}
impl ops::Add<Vector3> for Vector3{
    type Output = Vector3;
    fn add(self, rhs:Vector3)->Vector3{
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl ops::Mul<Vector3> for Vector3{
    type Output = Vector3;
    fn mul(self, rhs:Vector3)->Vector3{
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl ops::Mul<f32> for Vector3{
    type Output = Vector3;
    fn mul(self, rhs:f32)->Vector3{
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl ops::Mul<Vector3> for f32{
    type Output = Vector3;
    fn mul(self, rhs:Vector3)->Vector3{
        Vector3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl ops::Sub<Vector3> for Vector3{
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3{
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
struct Ray{
    origin:Vector3,
    direction:Vector3
}
impl Ray{
    fn new(origin:Vector3, direction:Vector3)->Ray{
        Ray{origin:origin, direction:direction}
    }
}
enum ReflectionType{
    diffuse,
    specular,
    refractable
}
struct Sphere{
    radius:f32,
    position:Vector3,
    emission:Vector3,
    color:Vector3,
    reflectionType:ReflectionType
}
//  struct Ray { Vec o, d; Ray(Vec o_, Vec d_) : o(o_), d(d_) {} };
//  enum Refl_t { DIFF, SPEC, REFR };  // material types, used in radiance()
//  struct Sphere {
//    double rad;       // radius
//    Vec p, e, c;      // position, emission, color
//    Refl_t refl;      // reflection type (DIFFuse, SPECular, REFRactive)
//    Sphere(double rad_, Vec p_, Vec e_, Vec c_, Refl_t refl_):
//      rad(rad_), p(p_), e(e_), c(c_), refl(refl_) {}
//    double intersect(const Ray &r) const { // returns distance, 0 if nohit
//      Vec op = p-r.o; // Solve t^2*d.d + 2*t*(o-p).d + (o-p).(o-p)-R^2 = 0
//      double t, eps=1e-4, b=op.dot(r.d), det=b*b-op.dot(op)+rad*rad;
//      if (det<0) return 0; else det=sqrt(det);
//      return (t=b-det)>eps ? t : ((t=b+det)>eps ? t : 0);
//    }
//  };

fn main() {
    println!("Hello, world!");
}
