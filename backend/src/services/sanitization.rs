use ammonia::Builder;

pub fn clean_all_html(src: &str) -> String {
    Builder::default().clean(src).to_string()
}