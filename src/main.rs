use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::Extension,
    response::IntoResponse,
    routing::{get},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

struct Query;

// 1. Описываем наш объект.
// #[Object] делает структуру видимой для GraphQL.
#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Привет! Это твой первый GraphQL запрос на Rust!"
    }

    async fn howdy(&self, name: String) -> String {
        format!("Привет, {}! Как дела в Rust мире?", name)
    }
}

// Тип для нашей схемы (у нас пока нет мутаций и подписок)
type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

// 2. Обработчик для POST запросов
// async fn graphql_handler(schema: axum::Extension<MySchema>, req: GraphQLRequest)-> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// 3. Обработчик для GraphiQL (интерфейс в браузере)
async fn graphiql() -> impl IntoResponse {
    axum::response::Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    // Создаем схему данных
    let schema: Schema<Query, EmptyMutation, EmptySubscription> =
        Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(axum::Extension(schema)); // Прокидываем схему в обработчики
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);

    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

