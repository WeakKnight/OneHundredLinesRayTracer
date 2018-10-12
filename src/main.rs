extern crate rand;

use rand::prelude::*;
use std::f32;
use std::ops;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

#[derive(Copy, Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }
    fn normalize(self) -> Self {
        self * (1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }
    fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Copy, Clone)]
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: direction.normalize(),
        }
    }
}

enum ReflectionType {
    Diffuse,
    Specular,
    Refractable,
}
trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f32;
}
struct Sphere {
    radius: f32,
    position: Vector3,
    emission: Vector3,
    color: Vector3,
    reflection_type: ReflectionType,
}
impl Sphere {
    fn new(
        radius: f32,
        position: Vector3,
        emission: Vector3,
        color: Vector3,
        reflection_type: ReflectionType,
    ) -> Sphere {
        Sphere {
            radius: radius,
            position: position,
            emission: emission,
            color: color,
            reflection_type: reflection_type,
        }
    }
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> f32 {
        let origin_to_position: Vector3 = self.position - ray.origin;
        let project_to_direction: f32 = origin_to_position.dot(ray.direction);
        let project_point: Vector3 = ray.origin + ray.direction * project_to_direction;
        let length_to_project_point: f32 = (project_point - self.position).length();
        if (length_to_project_point + f32::EPSILON) > self.radius {
            return 0.0;
        } else {
            return (self.radius * self.radius - length_to_project_point * length_to_project_point)
                .sqrt();
        }
    }
}

struct Scene {
    objects: Vec<Box<Intersectable>>,
}

impl Scene {
    fn new() -> Scene {
        Scene {
            objects: Vec::<Box<Intersectable>>::new(),
        }
    }
    fn push(&mut self, object: Box<Intersectable>) {
        self.objects.push(object);
    }

    fn intersect(&self, _ray: &Ray, _t: &mut f32, _id: &mut usize) -> bool {
        let count = self.objects.len();
        let mut distance: f32;
        *_t = f32::INFINITY;
        for i in count - 1..0 {
            let object = &self.objects[i];
            distance = object.intersect(_ray);
            if (distance - 0.0).abs() > f32::EPSILON && distance < (*_t) {
                *_id = i;
                *_t = distance;
            }
        }
        *_t < f32::INFINITY
    }

    fn shade(&self, ray: &Ray, depth: i32) -> Vector3 {
        let mut t: f32 = 0.0;
        let mut id: usize = 0;
        if !self.intersect(ray, &mut t, &mut id) {
            return Vector3::new(1.0, 1.0, 1.0);
        }
        Vector3::new(0.0, 0.0, 0.0)
    }
}

fn clamp(x: f32) -> f32 {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    }
    x
}
fn to_int(x: f32) -> i32 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32
}

fn main() {
    let width = 1024;
    let height = 768;
    let samps = 1;

    let mut scene = Scene::new();
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(1e5 + 1.0, 40.8, 81.6),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.75, 0.25, 0.25),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(-1e5 + 99.0, 40.8, 81.6),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.25, 0.25, 0.75),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(50.0, 40.8, 1e5),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.75, 0.75, 0.75),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(50.0, 40.8, -1e5 + 170.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(50.0, 1e5, 81.6),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.75, 0.75, 0.75),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        1e5,
        Vector3::new(50.0, -1e5 + 81.6, 81.6),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.75, 0.75, 0.75),
        ReflectionType::Diffuse,
    )));
    scene.push(Box::new(Sphere::new(
        16.5,
        Vector3::new(27.0, 16.5, 47.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        ReflectionType::Specular,
    )));
    scene.push(Box::new(Sphere::new(
        16.5,
        Vector3::new(73.0, 16.5, 78.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        ReflectionType::Refractable,
    )));
    scene.push(Box::new(Sphere::new(
        600.0,
        Vector3::new(50.0, 681.6 - 0.27, 81.6),
        Vector3::new(12.0, 12.0, 12.0),
        Vector3::new(0.0, 0.0, 0.0),
        ReflectionType::Diffuse,
    )));

    let camera_ray = Ray::new(
        Vector3::new(50.0, 52.0, 295.6),
        Vector3::new(0.0, -0.042612, -1.0).normalize(),
    );
    let cx = Vector3::new(
        width as f32 * 0.5135 / height as f32,
        width as f32 * 0.5135 / height as f32,
        width as f32 * 0.5135 / height as f32,
    );
    let cy = cx.cross(camera_ray.direction).normalize() * 0.5135;
    let mut result = Vector3::new(0.0, 0.0, 0.0);
    let mut context = vec![Vector3::new(0.0, 0.0, 0.0); width * height];
    let mut rng = thread_rng();
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i = (height - y - 1) * width + x;
            for sy in 0..1 {
                for sx in 0..1 {
                    result = Vector3::new(0.0, 0.0, 0.0);
                    for _ in 0..samps - 1 {
                        let mut dx: f32 = rng.gen_range::<f32>(-1.0, 1.0);
                        let mut dy: f32 = rng.gen_range::<f32>(-1.0, 1.0);
                        let d: Vector3 = cx
                            * (((sx as f32 + 0.5 + dx) * 0.5 + x as f32) / width as f32 - 0.5)
                            + cy * (((sy as f32 + 0.5 + dy) * 0.5 + y as f32) / height as f32 - 0.5) + camera_ray.direction;
                        result = result + scene.shade(&Ray::new(camera_ray.origin + d * 140.0, d.normalize()), 0)*(1.0/(samps as f32));
                    }
                    context[i] = context[i] + Vector3::new(clamp(result.x), clamp(result.y), clamp(result.z));
                }
            }
        }
    }
    let path = Path::new("result.ppm");
    let file = File::create(path);
    let mut buffer_writter = BufWriter::new(file.unwrap());
    buffer_writter.write(format!("P3\n{} {}\n{}\n",width,height,255).as_bytes()).unwrap();
    for i in 0..context.len() - 1{
        buffer_writter.write(format!("{} {} {} ", to_int(context[i].x), to_int(context[i].y), to_int(context[i].z)).as_bytes()).unwrap();
    }
}
