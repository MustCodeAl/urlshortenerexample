use crate::url::{Link, ShortLink};

pub trait UrlInterface {
    fn short_url(&self, long_url: &Link) -> Option<String>;
    fn long_url(&self, short_url: &ShortLink) -> Option<&Link>;
}


pub(crate) mod url_control {
    use crate::url::{Link, ShortLink};

      pub trait  UrlController {
        fn process(&mut self);
     fn hash_url(input_string: &str) -> String;
        fn url_shorten(&mut self, long_url: &Link);
    }
}
