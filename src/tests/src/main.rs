use tests::eigentuemlichfrei_html_select_test::eigentuemlichfrei_html_select_test;
use tests::redis_test::redis_test;
use tests::html_render_test::html_render_test;

#[tokio::main]
async fn main() {
    //eigentuemlichfrei_html_select_test();
    //redis_test().await
    html_render_test()
}
