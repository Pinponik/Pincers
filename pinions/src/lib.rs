#[cfg_attr(feature = "no_std", no_std)]
#[cfg(feature = "no_std")]
pub use heapless;
use num::Num;
pub use pinions_macros;
pub use winit;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[cfg(feature = "no_std")]
type Str<const N: usize> = heapless::String<N>;

#[cfg(not(feature = "no_std"))]
type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
type Vect<T, const N: usize = 0> = Vec<T>;

struct Point {
    x: f32,
    y: f32,
}

type Mouse = Option<Point>;

pub struct Wid<const L: usize, I: Num, const S: usize> {
    pub label: Str<L>,
    pub icon: Vect<I, S>,
    pub mouse: Mouse,
}

impl<const L: usize, I: Num, const S: usize> Wid<L, I, S> {
    pub fn new() -> Self {
        let mut lbl = Str::<L>::new();
        let icon = Vect::<I, S>::new();
        Self {
            label: lbl,
            icon,
            mouse: None,
        }
    }
}

pub struct Win<
    const T: usize, // title length
    const E: usize, // event length
    const V: usize, // widget count
    const L: usize, // label length -\
    I: Num,         // icon type      | <- from struct Wid
    const S: usize, // icon size    -/
> {
    window: Option<Window>,
    title: Str<T>,
    poll: bool,
    widgets: Vect<Wid<L, I, S>, V>,
}

impl<const T: usize, const E: usize, const V: usize, const L: usize, I: Num, const S: usize>
    Win<T, E, V, L, I, S>
{
    pub fn new() -> Self {
        Self {
            window: None,
            title: Str::<T>::new(),
            poll: false,
            widgets: Vect::<Wid<L, I, S>, V>::new(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
