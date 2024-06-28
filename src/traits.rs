pub trait Transform<T>{
    fn transform(&mut self, transform_fn: fn(&mut [T]));
    fn async_transform(&mut self, transform_fn: fn(&mut [T]));
}