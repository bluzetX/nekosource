use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Ru,
    En,
    Uk,
    Be
}

impl Language {
    fn lang_id(self) -> LanguageIdentifier {
        match self {
            Language::Ru => "ru".parse().unwrap(),
            Language::En => "en".parse().unwrap(),
            Language::Uk => "uk".parse().unwrap(),
            Language::Be => "be".parse().unwrap()
        }
    }

    fn ftl_source(self) -> &'static str {
        match self {
            Language::Ru => include_str!("../../locales/ru.ftl"),
            Language::En => include_str!("../../locales/en.ftl"),
            Language::Uk => include_str!("../../locales/uk.ftl"),
            Language::Be => include_str!("../../locales/be.ftl"),
        }
    }
}

pub struct Locale {
    bundle: FluentBundle<FluentResource>
}

impl Locale {
    pub fn new(lang: Language) -> Self {
        let mut bundle = FluentBundle::new(vec![lang.lang_id()]);

        let resource = FluentResource::try_new(lang.ftl_source().to_owned())
            .expect("Failed to parse FTL source");

        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource to bundle");

        Self { bundle }
    }

    pub fn t(&self, key: &str) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(m) => m,
            None => return format!("[{key}]")
        };

        let pattern = match msg.value() {
            Some(p) => p,
            None => return format!("[{key}]")
        };

        let mut errors = Vec::new();
        self.bundle
            .format_pattern(pattern, None, &mut errors)
            .into_owned()
    }

    pub fn t_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut fluent_args = FluentArgs::new();
        for (k, v) in args {
            fluent_args.set(*k, FluentValue::from(*v));
        }
        self.format(key, Some(&fluent_args))
    }

    fn format(&self, key: &str, args: Option<&FluentArgs<'_>>) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(m) => m,
            None    => return format!("[{key}]"),
        };
 
        let pattern = match msg.value() {
            Some(p) => p,
            None    => return format!("[{key}]"),
        };
 
        let mut errors = Vec::new();
        self.bundle
            .format_pattern(pattern, args, &mut errors)
            .into_owned()
    }
}