use rays::Ray;
use std::any::Any;
use tuples::Tuple;

pub trait Shape: Any {
    fn intersects_at(&self, ray: &Ray) -> Option<[f64; 2]>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
    fn as_any(&self) -> &dyn Any;
}

pub mod test {
    use crate::shape::Shape;
    use rays::Ray;
    use std::any::Any;
    use std::cell::RefCell;
    use tuples::{vector, Tuple};

    pub struct TestShape {
        pub saved_ray: RefCell<Option<Ray>>,
    }

    impl Shape for TestShape {
        fn intersects_at(&self, ray: &Ray) -> Option<[f64; 2]> {
            *self.saved_ray.borrow_mut() = Some(Ray {
                origin: ray.origin,
                direction: ray.direction,
            });
            None
        }

        fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
            vector(local_point.x, local_point.y, local_point.z)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl Default for TestShape {
        fn default() -> TestShape {
            TestShape {
                saved_ray: RefCell::new(None),
            }
        }
    }
}
