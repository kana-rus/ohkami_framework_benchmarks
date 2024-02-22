mod postgres;
pub use postgres::Postgres;

mod models;
pub use models::{Fortune, Message, World, WorldsQuery};

mod templates;
pub use templates::FortunesTemplate;

use ohkami::{Ohkami, Route, Memory};


#[tokio::main]
async fn main() {
    struct SetServer;
    impl ohkami::BackFang for SetServer {
        type Error = std::convert::Infallible;

        #[inline(always)]
        async fn bite(&self, res: &mut ohkami::Response, _req: &ohkami::Request) -> Result<(), Self::Error> {
            res.headers.set().Server("ohkami");
            Ok(())
        }
    }

    Ohkami::with((SetServer, Postgres::init().await), (
        "/json"     .GET(json_serialization),
        "/db"       .GET(single_database_query),
        "/queries"  .GET(multiple_database_query),
        "/fortunes" .GET(fortunes),
        "/updates"  .GET(database_updates),
        "/plaintext".GET(plaintext),
    )).howl("0.0.0.0:8000").await
}

#[inline]
async fn json_serialization() -> Message {
    Message {
        message: "Hello, World!"
    }
}

#[inline]
async fn single_database_query(p: Memory<'_, Postgres>) -> World {
    p.select_random_world().await
}

#[inline]
async fn multiple_database_query(q: WorldsQuery<'_>, p: Memory<'_, Postgres>) -> Vec<World> {
    let n = q.parse();
    p.select_n_random_worlds(n).await
}

#[inline]
async fn fortunes(p: Memory<'_, Postgres>) -> FortunesTemplate {
    let mut fortunes = p.select_all_fortunes().await;
    fortunes.push(Fortune {
        id:      0,
        message: String::from("Additional fortune added at request time."),
    });
    fortunes.sort_unstable_by(|a, b| str::cmp(&a.message, &b.message));

    FortunesTemplate { fortunes }
}

#[inline]
async fn database_updates(q: WorldsQuery<'_>, p: Memory<'_, Postgres>) -> Vec<World> {
    let worlds = p.select_n_random_worlds(q.parse()).await;
    p.update_random_ids_of_worlds(worlds).await
}

#[inline]
async fn plaintext() -> &'static str {
    "Hello, World"
}