use druid::Rect;

pub trait Bound {
	fn bounding_box(&self) -> Rect;
}
