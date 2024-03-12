// use tower::{layer::util::Stack, ServiceBuilder};
// use tower_http::{compression::CompressionLayer, trace::TraceLayer};

// async fn middleware_stack () ->  {
//     let middleware_stack = ServiceBuilder::new()
//         // add high level tracing of requests and responses
//         .layer(TraceLayer::new_for_http())
//         // compression responses
//         .layer(CompressionLayer::new())
//         // convert the `ServiceBuilder` into a `tower::Layer`;
//         .into_inner();

//         middleware_stack
// }