mod fallback;
mod posts_grpc;
mod stats;
mod stats_grpc;
mod users;
pub mod util;

pub use fallback::fallback;
pub use posts_grpc::{create_post, get_post, get_posts, remove_post, update_post, PostsGrpcClient};
pub use stats::{like_post, view_post};
pub use stats_grpc::{get_post_stats, get_top_posts, get_top_users, StatsGrpcClient};
pub use users::{login, signup, update_user};
