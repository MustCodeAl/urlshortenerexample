use std::hash::Hasher;
// use crate::trait_mod::url_control::UrlController;
use crate::trait_mod::{url_control, UrlInterface};

#[derive(Clone, Eq, Debug)]
pub struct Link {
    url: String,
}

impl Link {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
        // todo!()
    }
}

impl std::hash::Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);

        // todo!()
    }
}

#[derive(Clone, Eq, Debug)]
pub struct ShortLink(Link);

impl ShortLink {
    pub fn new(field0: Link) -> Self {
        Self(field0)
    }
}


impl PartialEq for ShortLink {
    fn eq(&self, other: &Self) -> bool {
        self.0.url == other.0.url
        // todo!()
    }
}

impl std::hash::Hash for ShortLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.url.hash(state)
        // todo!()
    }
}

#[derive(Default, Debug)]
struct UrlMapper {
    queue: Vec<Link>,
    // make 100k

    link_size: usize,
    url_table: std::collections::HashMap<ShortLink, Link>,

    hash_long_link_table: std::collections::HashMap<Link, String>,

}


impl crate::trait_mod::url_control::UrlController for UrlMapper {
    fn process(&mut self) {
        for url in self.queue.clone() {
            self.url_shorten(&url);
        }
    }
    fn hash_url(input_string: &str) -> String {
        // use sha2::{Digest, Sha256};
        //
        // let mut hasher = Sha256::new();
        // hasher.update(input_string.as_bytes());
        // let hash = hasher.finalize();
        // format!("{:x}", hash)
        todo!()
    }
    fn url_shorten(&mut self, long_url: &Link) {

        // hash the link
        // append some to short urls front
        // let short_url = hash(link.clone());


        let long_hash = Self::hash_url(
            long_url.
                url.as_str()).chars()
            .take(self.link_size).
            collect::<String>();
        self.hash_long_link_table.insert(long_url.clone(), long_hash);

        self.url_table.insert(ShortLink(
            Link {
                url: self.short_url(&long_url.clone()).unwrap()
            }),
            long_url.clone());


        // short url will be key
        // todo!()
    }
}


impl UrlInterface for UrlMapper {
    fn short_url(&self, long_url: &Link) -> Option<String> {
        let mut url = String::from("Dm/");
        // let link = Self::hash(&long_url);
        // url.push_str(link.0.long_url.as_str());
        //
        // url

        let hash_value = self.hash_long_link_table.get(long_url);
        url.push_str(match hash_value {
            Some(x) => x,
            None => return None,
        }.as_str());
        Some(url)
    }
    fn long_url(&self, short_url: &ShortLink) -> Option<&Link> {
        // Some(short_url.0)
        self.url_table.get(short_url)
        // if we want to get short_url
    }
}

pub struct UrlMapperExp(UrlMapper);

impl UrlInterface for UrlMapperExp {
    fn short_url(&self, long_url: &Link) -> Option<String> {
        self.0.short_url(long_url)
    }

    fn long_url(&self, short_url: &ShortLink) -> Option<&Link> {
        self.0.long_url(short_url)
    }
}

impl UrlMapperExp {
    pub fn new() -> Self {
        Self(UrlMapper::new())
    }
}

impl UrlMapper {
    // https://stackoverflow.com/questions/63025313/rust-calling-mutable-self-function-from-mutable-self-function


    fn url_redirect(&self, short_link: &ShortLink) {
        if !self.url_table.is_empty() {
            let client = Client::new();

            let url_long = self.url_table.get(short_link).unwrap();
            // go to original url if hashed url is true


            client.get(url_long.url.as_str());
        }
    }

    fn new() -> Self {
        Self { link_size: 7,..Default::default() }
    }
}


struct Client(/*client: reqwest::Client*/);

impl Client {
    pub fn get(&self, link: &str) {


        // HttpResponse::new().Status("301_REDIRECT").body(url.long_url());
        //status 301 means permanent move means all requests are now sent to long url

        //status 302 means temporary moved and all requests are sent to short url service first
    }
    pub fn new() -> Self {
        Self {}
    }
}
