pub use crate::image::{local::*, prelude::*, remote::*};

mod local;
mod prelude;
mod remote;

#[derive(Clone)]
pub enum Image {
    Local(LocalImage),
    Remote(RemoteImage),
}
