use std::borrow::Cow;

use hypertext::context::AttributeValue;
use hypertext::{Buffer, Renderable};

pub trait LinkSetters {
    fn href(mut self, href: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_href(href);
        self
    }

    fn target(mut self, target: Target) -> Self
    where
        Self: Sized,
    {
        self.set_target(target);
        self
    }

    fn download(mut self, download: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_download(download);
        self
    }

    fn rel(mut self, rel: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_rel(rel);
        self
    }

    fn set_href(&mut self, href: impl Into<Cow<'static, str>>);

    fn set_target(&mut self, target: Target);

    fn set_download(&mut self, download: impl Into<Cow<'static, str>>);

    fn set_rel(&mut self, rel: impl Into<Cow<'static, str>>);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Blank,
    Parent,
    Self_,
    Top,
}

impl Target {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blank => "_blank",
            Self::Parent => "_parent",
            Self::Self_ => "_self",
            Self::Top => "_top",
        }
    }
}

impl Renderable<AttributeValue> for Target {
    fn render_to(&self, buffer: &mut Buffer<AttributeValue>) {
        self.as_str().render_to(buffer);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Link {
    pub href: Option<Cow<'static, str>>,
    pub target: Option<Target>,
    pub download: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
}

impl Link {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LinkSetters for Link {
    fn set_href(&mut self, href: impl Into<Cow<'static, str>>) {
        self.href = Some(href.into());
    }

    fn set_target(&mut self, target: Target) {
        self.target = Some(target);
    }

    fn set_download(&mut self, download: impl Into<Cow<'static, str>>) {
        self.download = Some(download.into());
    }

    fn set_rel(&mut self, rel: impl Into<Cow<'static, str>>) {
        self.rel = Some(rel.into());
    }
}

impl<T: AsMut<Link>> LinkSetters for T {
    fn set_href(&mut self, href: impl Into<Cow<'static, str>>) {
        self.as_mut().set_href(href);
    }

    fn set_target(&mut self, target: Target) {
        self.as_mut().set_target(target);
    }

    fn set_download(&mut self, download: impl Into<Cow<'static, str>>) {
        self.as_mut().set_download(download);
    }

    fn set_rel(&mut self, rel: impl Into<Cow<'static, str>>) {
        self.as_mut().set_rel(rel);
    }
}
