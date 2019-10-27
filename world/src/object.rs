use crate::intersections::Intersection;
use crate::materials::Material;
use crate::shapes::Shape;
use matrices::{inverse, matrix_tuple_multiply, transpose, IDENTITY};
use rays::Ray;
use std::rc::Rc;
use tuples::{normalize, Tuple};
use uuid::Uuid;

pub struct Object {
    pub id: Uuid,
    pub transform: [[f64; 4]; 4],
    pub material: Rc<Material>,
    pub shape: Box<dyn Shape>,
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]> {
        let local_ray = ray.transform(&inverse(&self.transform));
        let intersections = self.shape.intersects_at(&local_ray);
        intersections.map(move |xs| {
            [
                Intersection {
                    t: xs[0],
                    object: self,
                },
                Intersection {
                    t: xs[1],
                    object: self,
                },
            ]
        })
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let inverse_transform = inverse(&self.transform);
        let transpose_inverse_transform = transpose(&inverse_transform);
        let local_point = matrix_tuple_multiply(&inverse_transform, world_point);
        let local_normal = self.shape.local_normal_at(&local_point);
        let mut world_normal = matrix_tuple_multiply(&transpose_inverse_transform, &local_normal);
        world_normal.w = 0.0;
        normalize(&world_normal)
    }
}

impl Object {
    pub fn new(shape: Box<dyn Shape>) -> Object {
        Object {
            id: Uuid::new_v4(),
            transform: IDENTITY,
            material: Rc::new(Material::default()),
            shape,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Object {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::object::Object;
    use crate::shapes::test::TestShape;
    use matrices::IDENTITY;
    use rays::Ray;
    use std::f64::consts::PI;
    use std::rc::Rc;
    use transformations::MatrixTransformations;
    use tuples::{point, vector};

    #[test]
    fn default_transformation() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        assert_eq!(o.transform, IDENTITY);
    }

    #[test]
    fn assigning_a_transformation() {
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.translate(2.0, 3.0, 4.0);
        assert_eq!(o.transform, IDENTITY.translate(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let m = Material::default();
        assert_eq!((*o.material).color, m.color);
        assert_eq!((*o.material).ambient, m.ambient);
        assert_eq!((*o.material).diffuse, m.diffuse);
        assert_eq!((*o.material).specular, m.specular);
        assert_eq!((*o.material).shininess, m.shininess);
        assert!((*o.material).pattern.is_none());
    }

    #[test]
    fn assigning_a_material() {
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        let mut m = Material::default();
        m.diffuse = 1.0;
        let m = Rc::new(m);
        o.material = Rc::clone(&m);
        assert_eq!(o.material, m);
    }

    #[test]
    fn intersecting_a_scaled_object_with_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        o.intersect(&r);
        let shape = o.shape.as_any().downcast_ref::<TestShape>().unwrap();
        assert_eq!(
            shape.saved_ray.borrow().as_ref().unwrap().origin,
            point(0.0, 0.0, -2.5)
        );
        assert_eq!(
            shape.saved_ray.borrow().as_ref().unwrap().direction,
            vector(0.0, 0.0, 0.5)
        );
    }

    #[test]
    fn intersecting_a_translated_object_with_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.translate(5.0, 0.0, 0.0);
        o.intersect(&r);
        let shape = o.shape.as_any().downcast_ref::<TestShape>().unwrap();
        assert_eq!(
            shape.saved_ray.borrow().as_ref().unwrap().origin,
            point(-5.0, 0.0, -5.0)
        );
        assert_eq!(
            shape.saved_ray.borrow().as_ref().unwrap().direction,
            vector(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_object() {
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.translate(0.0, 1.0, 0.0);
        let n = o.normal_at(&point(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_object() {
        let shape = TestShape::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        let n = o.normal_at(&point(
            0.0,
            (2.0 as f64).sqrt() / 2.0,
            -((2.0 as f64).sqrt() / 2.0),
        ));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
