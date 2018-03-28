
pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
	let mut concat: Vec<T> = vec![x[0].clone(); x.len()];

	concat.clone_from_slice(x);
	concat.extend_from_slice(y);

	concat
}
