use std::borrow::Cow;

pub trait CommonAttributeSetters {
    fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_id(id);
        self
    }

    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_class(class);
        self
    }

    fn style(mut self, style: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_style(style);
        self
    }

    fn set_id(&mut self, id: impl Into<Cow<'static, str>>);

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>);

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>);

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>);

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>);
}

pub trait CommonAttributeGetters {
    fn get_id(&self) -> &Cow<'static, str>;

    fn get_not_empty_id(&self) -> Option<&Cow<'static, str>> {
        let id = self.get_id();
        if id.is_empty() { None } else { Some(id) }
    }

    fn get_classes(&self) -> &[Cow<'static, str>];

    fn get_class_line(&self) -> Option<String> {
        let line = self.get_classes().join(" ");
        if line.is_empty() { None } else { Some(line) }
    }

    fn get_styles(&self) -> &[Cow<'static, str>];

    fn get_style_line(&self) -> Option<String> {
        let line = self.get_styles().join("; ");
        if line.is_empty() { None } else { Some(line) }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommonAttrs {
    pub id: Cow<'static, str>,
    pub classes: Vec<Cow<'static, str>>,
    pub styles: Vec<Cow<'static, str>>,
}

impl CommonAttrs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CommonAttributeSetters for CommonAttrs {
    fn set_id(&mut self, id: impl Into<Cow<'static, str>>) {
        self.id = id.into();
    }

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>) {
        self.classes = classes;
    }

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>) {
        self.styles = styles;
    }

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>) {
        self.classes.push(class.into());
    }

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>) {
        self.styles.push(style.into());
    }
}

impl CommonAttributeGetters for CommonAttrs {
    fn get_id(&self) -> &Cow<'static, str> {
        &self.id
    }

    fn get_classes(&self) -> &[Cow<'static, str>] {
        &self.classes
    }

    fn get_styles(&self) -> &[Cow<'static, str>] {
        &self.styles
    }
}

impl<T: AsMut<CommonAttrs>> CommonAttributeSetters for T {
    fn set_id(&mut self, id: impl Into<Cow<'static, str>>) {
        self.as_mut().set_id(id);
    }

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>) {
        self.as_mut().set_classes(classes);
    }

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>) {
        self.as_mut().set_styles(styles);
    }

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>) {
        self.as_mut().add_class(class);
    }

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>) {
        self.as_mut().add_style(style);
    }
}

impl<T: AsRef<CommonAttrs>> CommonAttributeGetters for T {
    fn get_id(&self) -> &Cow<'static, str> {
        self.as_ref().get_id()
    }

    fn get_classes(&self) -> &[Cow<'static, str>] {
        self.as_ref().get_classes()
    }

    fn get_styles(&self) -> &[Cow<'static, str>] {
        self.as_ref().get_styles()
    }
}
