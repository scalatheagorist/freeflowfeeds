use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use select::predicate::Predicate;

fn extract_author(article: &Node) -> Option<String> {
    let author_node = article.find(Name("a").and(Class("meta-author"))).next();
    author_node.map(|author| author.text())
}

pub fn hayek_institut_html_select_test() {
    let document = Document::from_read(html_example().as_bytes()).unwrap();

    fn get_title(node: &Node) -> String {
        let title_node = node.find(Name("a")).next().unwrap();
        title_node.attr("title").unwrap_or("N/A").to_string()
    }

    fn get_href(node: &Node) -> Option<String> {
        let link_node = node.find(Name("a")).next().unwrap();
        link_node.attr("href").map(|s| s.to_string())
    }

    for node in document.find(Class("fusion-post-card-image")) {
        get_href(&node).iter().for_each(|link| {
            let title = get_title(&node);

            println!("Titel: {}", title);
            println!("Link (href): {:?}", link.to_string());
        })
    }
}

fn html_example() -> &'static str {
    r##"
<ul class="fusion-grid fusion-grid-1 fusion-flex-align-items-flex-start fusion-grid-posts-cards">
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-13 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-0 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/innovations-landschaft-in-oesterreich/"
                               title="Innovations-Landschaft in Österreich">
                                <div class="featured-image"><img width="1280" height="853"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2.jpg"
                                                                 class="attachment-full size-full wp-post-image lazyautosizes lazyloaded"
                                                                 alt="Innovations Landschaft in Oesterreich 2"
                                                                 decoding="async"
                                                                 srcset="https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-800x533.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-1200x800.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2.jpg 1280w"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-800x533.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2-1200x800.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/09/Innovations-Landschaft-in-Oesterreich-2.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Innovations Landschaft in Oesterreich 2"
                                                                 sizes="900px"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-1 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-5 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-5 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/innovations-landschaft-in-oesterreich/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Innovations-Landschaft
                                in Österreich</a></h2></div>
                        <div class="fusion-text fusion-text-6 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Der „Transatlantic Subnational Innovation Competitiveness Index 2.0“ analysiert die
                                Innovationslandschaft von 121 Regionen in sieben Ländern.</p></div>
                        <div class="fusion-text fusion-text-7 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>14.09.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-14 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-2 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/25-jahre-deutsche-hayek-gesellschaft/"
                               title="Das Hayek Institut gratuliert der deutschen Hayek Gesellschaft zum 25jährigen Bestehen">
                                <div class="featured-image"><img width="2560" height="1707"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-scaled.jpg"
                                                                 class="attachment-full size-full wp-post-image lazyautosizes lazyloaded"
                                                                 alt="25 Jahre deutsche Hayek Gesellschaft"
                                                                 decoding="async"
                                                                 srcset="https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-800x533.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-1200x800.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-scaled.jpg 2560w"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-scaled.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-800x533.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-1200x800.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/08/25-Jahre-HG-scaled.jpg 2560w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="25 Jahre HG scaled" sizes="900px"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-3 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-8 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-6 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/25-jahre-deutsche-hayek-gesellschaft/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Das Hayek
                                Institut gratuliert der deutschen Hayek Gesellschaft zum 25jährigen Bestehen</a></h2>
                        </div>
                        <div class="fusion-text fusion-text-9 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);"></div>
                        <div class="fusion-text fusion-text-10 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>21.08.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-15 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-4 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/kryptowaehrungen/" title="Kryptowährungen">
                                <div class="featured-image"><img width="1280" height="960"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen.jpg"
                                                                 class="attachment-full size-full wp-post-image lazyautosizes lazyloaded"
                                                                 alt="Kryptowaehrungen" decoding="async"
                                                                 srcset="https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-200x150.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-400x300.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-600x450.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-800x600.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-1200x900.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen.jpg 1280w"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-200x150.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-400x300.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-600x450.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-800x600.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen-1200x900.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/07/Kryptowaehrungen.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Kryptowaehrungen" sizes="900px"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-5 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-11 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-7 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/kryptowaehrungen/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Kryptowährungen</a>
                            </h2></div>
                        <div class="fusion-text fusion-text-12 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Hayek betonte in seinen Arbeiten insbesondere den Währungswettbewerb unter privaten
                                Teilnehmern. Mit dem Erstarken von Kryptowährungen erhält diese Idee der Geldpolitik
                                einen neuen Schub.</p></div>
                        <div class="fusion-text fusion-text-13 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>12.07.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-16 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-6 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/das-bretton-woods-system/"
                               title="Das Bretton-Woods-System">
                                <div class="featured-image"><img width="1280" height="854"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System.jpg"
                                                                 class="attachment-full size-full wp-post-image lazyautosizes lazyloaded"
                                                                 alt="Das Bretton Woods System" decoding="async"
                                                                 srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-800x534.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-1200x801.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System.jpg 1280w"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-800x534.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System-1200x801.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Das-Bretton-Woods-System.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Das Bretton Woods System" sizes="900px"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-7 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-14 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-8 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/das-bretton-woods-system/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Das
                                Bretton-Woods-System</a></h2></div>
                        <div class="fusion-text fusion-text-15 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Das Bretton-Woods-System war eine Währungsordnung der Zeit nach dem Zweiten Weltkrieg,
                                welche die Schwankungen von Wechselkursen auf eine gewisse Bandbreite zum US-Dollar
                                beschränkte.</p></div>
                        <div class="fusion-text fusion-text-16 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>28.06.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-17 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-8 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/fixe-wechselkurse/" title="Fixe Wechselkurse">
                                <div class="featured-image"><img width="1280" height="809"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse.jpg"
                                                                 class="attachment-full size-full wp-post-image lazyautosizes lazyloaded"
                                                                 alt="Fixe Wechselkurse" decoding="async"
                                                                 srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-200x126.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-400x253.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-600x379.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-800x506.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-1200x758.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse.jpg 1280w"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-200x126.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-400x253.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-600x379.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-800x506.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse-1200x758.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Fixe-Wechselkurse.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Fixe Wechselkurse" sizes="900px"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-9 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-17 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-9 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/fixe-wechselkurse/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Fixe
                                Wechselkurse</a></h2></div>
                        <div class="fusion-text fusion-text-18 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Fixe Wechselkurse zwischen Währungen: Wie funktionieren sie, welche Vor- und Nachteile
                                gibt es und welche Auswirkungen hat ein solcher fixer Wechselkurs?</p></div>
                        <div class="fusion-text fusion-text-19 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>21.06.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-18 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-10 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/hochinflationsperiode-in-den-1970ern-und-die-loesungsansaetze/"
                               title="Hochinflationsperiode in den 1970ern und die Lösungsansätze">
                                <div class="featured-image"><img width="1280" height="854"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze.jpg"
                                                                 class="attachment-full size-full lazyload wp-post-image"
                                                                 alt="Hochinflationsperiode in den 1970ern und die Loesungsansaetze"
                                                                 decoding="async"
                                                                 srcset="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%271280%27%20height%3D%27854%27%20viewBox%3D%270%200%201280%20854%27%3E%3Crect%20width%3D%271280%27%20height%3D%27854%27%20fill-opacity%3D%220%22%2F%3E%3C%2Fsvg%3E"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze-800x534.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze-1200x801.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Hochinflationsperiode-in-den-1970ern-und-die-Loesungsansaetze.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Hochinflationsperiode in den 1970ern und die Loesungsansaetze">
                                </div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-11 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-20 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-10 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/hochinflationsperiode-in-den-1970ern-und-die-loesungsansaetze/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Hochinflationsperiode
                                in den 1970ern und die Lösungsansätze</a></h2></div>
                        <div class="fusion-text fusion-text-21 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Europa steht vor vielfältigen Herausforderungen wie den Auswirkungen der
                                COVID-19-Pandemie, Störungen in globalen Lieferketten und der russischen Invasion der
                                Ukraine.</p></div>
                        <div class="fusion-text fusion-text-22 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>12.06.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-19 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-12 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/hayek-und-seine-sichtweise-auf-wirtschaftliche-rezessionen/"
                               title="Hayek und seine Sichtweise auf wirtschaftliche Rezessionen">
                                <div class="featured-image"><img width="567" height="567"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Hayek-und-seine-Sichtweise-auf-wirtschaftliche-Rezessionen.jpg"
                                                                 class="attachment-full size-full lazyload wp-post-image"
                                                                 alt="Hayek und seine Sichtweise auf wirtschaftliche Rezessionen"
                                                                 decoding="async"
                                                                 srcset="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%27567%27%20height%3D%27567%27%20viewBox%3D%270%200%20567%20567%27%3E%3Crect%20width%3D%27567%27%20height%3D%27567%27%20fill-opacity%3D%220%22%2F%3E%3C%2Fsvg%3E"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Hayek-und-seine-Sichtweise-auf-wirtschaftliche-Rezessionen.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Hayek-und-seine-Sichtweise-auf-wirtschaftliche-Rezessionen-200x200.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Hayek-und-seine-Sichtweise-auf-wirtschaftliche-Rezessionen-400x400.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Hayek-und-seine-Sichtweise-auf-wirtschaftliche-Rezessionen.jpg 567w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Hayek und seine Sichtweise auf wirtschaftliche Rezessionen">
                                </div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-13 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-23 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-11 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/hayek-und-seine-sichtweise-auf-wirtschaftliche-rezessionen/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Hayek und
                                seine Sichtweise auf wirtschaftliche Rezessionen</a></h2></div>
                        <div class="fusion-text fusion-text-24 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Die Meinungsverschiedenheiten zwischen Hayek und Keynes gehören zu den bekanntesten und
                                am besten dokumentierten wirtschaftlichen Auseinandersetzungen.</p></div>
                        <div class="fusion-text fusion-text-25 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>07.06.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-20 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-14 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/nanny-state-index-2023/" title="Nanny State Index 2023">
                                <div class="featured-image"><img width="780" height="300"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023.jpg"
                                                                 class="attachment-full size-full lazyload wp-post-image"
                                                                 alt="Nanny State Index 2023" decoding="async"
                                                                 srcset="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%27780%27%20height%3D%27300%27%20viewBox%3D%270%200%20780%20300%27%3E%3Crect%20width%3D%27780%27%20height%3D%27300%27%20fill-opacity%3D%220%22%2F%3E%3C%2Fsvg%3E"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023-200x77.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023-400x154.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023-600x231.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Nanny-State-Index-2023.jpg 780w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Nanny State Index 2023"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-15 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-26 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-12 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/nanny-state-index-2023/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Nanny State
                                Index 2023</a></h2></div>
                        <div class="fusion-text fusion-text-27 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Entdecken Sie mit dem Nanny State Index 2023, der 30 Länder in vier Kategorien bewertet,
                                wie die europäischen Länder Essen, Trinken, Rauchen und Dampfen regulieren.</p></div>
                        <div class="fusion-text fusion-text-28 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>01.06.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-21 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-16 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/konjunkturzyklen-woher-sie-kommen-und-wohin-sie-gehen/"
                               title="Konjunkturzyklen – Woher sie kommen, und wohin sie gehen">
                                <div class="featured-image"><img width="1280" height="854"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen.jpg"
                                                                 class="attachment-full size-full lazyload wp-post-image"
                                                                 alt="Konjunkturzyklen – Woher sie kommen und wohin sie gehen"
                                                                 decoding="async"
                                                                 srcset="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%271280%27%20height%3D%27854%27%20viewBox%3D%270%200%201280%20854%27%3E%3Crect%20width%3D%271280%27%20height%3D%27854%27%20fill-opacity%3D%220%22%2F%3E%3C%2Fsvg%3E"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen-200x133.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen-400x267.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen-600x400.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen-800x534.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen-1200x801.jpg 1200w, https://hayek-institut.at/wp-content/uploads/2023/06/Konjunkturzyklen-–-Woher-sie-kommen-und-wohin-sie-gehen.jpg 1280w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Konjunkturzyklen – Woher sie kommen und wohin sie gehen">
                                </div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-17 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-29 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-13 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/konjunkturzyklen-woher-sie-kommen-und-wohin-sie-gehen/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Konjunkturzyklen
                                – Woher sie kommen, und wohin sie gehen</a></h2></div>
                        <div class="fusion-text fusion-text-30 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Volkswirtschaften wachsen nicht stetig und mit gleichem Tempo. Viel mehr mäandern sie auf
                                und ab und finden sich erst nach einem Tiefpunkt wieder auf noch höherem Niveau. Man
                                spricht hier von Konjunkturzyklen.</p></div>
                        <div class="fusion-text fusion-text-31 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>31.05.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
    <li class="fusion-layout-column fusion_builder_column fusion-builder-column-22 fusion-flex-column post-card fusion-grid-column fusion-post-cards-grid-column"
        style="--awb-bg-blend:overlay;--awb-bg-size:cover;">
        <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
            <div class="fusion-builder-row fusion-builder-row-inner fusion-row fusion-flex-align-items-flex-start fusion-flex-content-wrap"
                 style="width:104% !important;max-width:104% !important;margin-left: calc(-4% / 2 );margin-right: calc(-4% / 2 );">
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-18 fusion_builder_column_inner_2_5 2_5 fusion-flex-column"
                     style="--awb-bg-size:cover;--awb-width-large:40%;--awb-margin-top-large:0px;--awb-spacing-right-large:4.8%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:4.8%;--awb-width-medium:40%;--awb-order-medium:0;--awb-spacing-right-medium:4.8%;--awb-spacing-left-medium:4.8%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-flex-start fusion-content-layout-column">
                        <div class="fusion-classic-product-image-wrapper fusion-woo-product-image fusion-post-card-image fusion-post-card-image-1 product-images"
                             data-layout="crossfade"
                             style="--awb-crossfade-bg-color: #e2e2e2; --darkreader-bg--awb-crossfade-bg-color: #282c2d;">
                            <a href="https://hayek-institut.at/die-erdoelschocks-der-1970er/"
                               title="Die Erdölschocks der 1970er">
                                <div class="featured-image"><img width="1200" height="628"
                                                                 src="https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er.jpg"
                                                                 class="attachment-full size-full lazyload wp-post-image"
                                                                 alt="Die Erdoelschocks der 1970er" decoding="async"
                                                                 srcset="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%271200%27%20height%3D%27628%27%20viewBox%3D%270%200%201200%20628%27%3E%3Crect%20width%3D%271200%27%20height%3D%27628%27%20fill-opacity%3D%220%22%2F%3E%3C%2Fsvg%3E"
                                                                 data-orig-src="https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er.jpg"
                                                                 data-srcset="https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er-200x105.jpg 200w, https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er-400x209.jpg 400w, https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er-600x314.jpg 600w, https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er-800x419.jpg 800w, https://hayek-institut.at/wp-content/uploads/2023/05/Die-Erdoelschocks-der-1970er.jpg 1200w"
                                                                 data-sizes="auto" data-orig-=""
                                                                 title="Die Erdoelschocks der 1970er"></div>
                            </a></div>
                    </div>
                </div>
                <div class="fusion-layout-column fusion_builder_column_inner fusion-builder-nested-column-19 fusion_builder_column_inner_3_5 3_5 fusion-flex-column fusion-flex-align-self-stretch"
                     style="--awb-bg-size:cover;--awb-width-large:60%;--awb-margin-top-large:0px;--awb-spacing-right-large:3.2%;--awb-margin-bottom-large:0px;--awb-spacing-left-large:3.2%;--awb-width-medium:60%;--awb-order-medium:0;--awb-spacing-right-medium:3.2%;--awb-spacing-left-medium:3.2%;--awb-width-small:100%;--awb-order-small:0;--awb-spacing-right-small:1.92%;--awb-spacing-left-small:1.92%;">
                    <div class="fusion-column-wrapper fusion-column-has-shadow fusion-flex-justify-content-space-between fusion-content-layout-column">
                        <div class="fusion-separator fusion-no-medium-visibility fusion-no-large-visibility fusion-full-width-sep"
                             style="align-self: center;margin-left: auto;margin-right: auto;margin-top:20px;width:100%;"></div>
                        <div class="fusion-text fusion-text-32 fusion-text-no-margin commentary_category"
                             style="--awb-font-size: 13px; --awb-letter-spacing: 2px; --awb-text-transform: uppercase; --awb-text-color: var(--awb-custom_color_1); --awb-text-font-family: var(--awb-typography1-font-family); --awb-text-font-weight: var(--awb-typography1-font-weight); --awb-text-font-style: var(--awb-typography1-font-style); --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-custom_color_1); --darkreader-text--awb-text-color: var(--darkreader-text--awb-custom_color_1);">
                            <p><a href="https://hayek-institut.at/blog/" title="Blog">Blog</a></p></div>
                        <div class="fusion-title title fusion-title-14 fusion-sep-none fusion-title-text fusion-title-size-two"
                             style="--awb-link-color: var(--awb-color8); --awb-font-size: 20px; --darkreader-text--awb-link-color: var(--darkreader-text--awb-color8);">
                            <h2 class="fusion-title-heading title-heading-left" style="margin:0;font-size:1em;"><a
                                    href="https://hayek-institut.at/die-erdoelschocks-der-1970er/"
                                    class="awb-custom-text-color awb-custom-text-hover-color" target="_self">Die
                                Erdölschocks der 1970er</a></h2></div>
                        <div class="fusion-text fusion-text-33 fusion-text-no-margin"
                             style="--awb-font-size: 14px; --awb-text-transform: none; --awb-text-color: var(--awb-color6); --awb-margin-bottom: 10px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color6); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color6);">
                            <p>Während der 1970er Jahre stiegen die Erdölpreise aufgrund geopolitischer Spannungen im
                                Nahen Osten sprunghaft an. Ähnlich wie während der aktuellen Gaspreiskrise führten diese
                                Erdölschocks zu Verwerfungen der Weltwirtschaft.</p></div>
                        <div class="fusion-text fusion-text-34 fusion-text-no-margin commentary_date"
                             style="--awb-font-size: 12px; --awb-text-transform: none; --awb-text-color: var(--awb-color5); --awb-margin-bottom: 0px; --darkreader-bg--awb-text-color: var(--darkreader-bg--awb-color5); --darkreader-text--awb-text-color: var(--darkreader-text--awb-color5);">
                            <p>25.05.2023</p></div>
                    </div>
                </div>
            </div>
        </div>
    </li>
</ul>
    "##
}
