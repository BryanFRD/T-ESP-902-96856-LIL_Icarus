mod inference;
mod route_pattern;

pub use inference::{infer_delete_event, infer_get_event, infer_patch_event, infer_post_event};
pub use route_pattern::RoutePattern;
