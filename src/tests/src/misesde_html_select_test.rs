use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

pub fn misesde_html_select_test() {
    let document: Document = Document::from_read(html_example().as_bytes()).unwrap();

    for article in document.find(Name("div")) {
       if let Some(class) = article.attr("class") {
           if class.contains("pt-cv-content-item") {
               let author = article
                   .find(Name("span").and(Attr("class", "author")))
                   .next()
                   .and_then(|node| node.find(Name("span")).next())
                   .map(|node| node.text())
                   .unwrap_or("".to_string())
                   .trim()
                   .to_owned();

               let title = article
                   .find(Name("h4"))
                   .next()
                   .and_then(|node| node.find(Name("a")).next())
                   .map(|node| node.text())
                   .unwrap_or("".to_string())
                   .trim()
                   .to_owned();

               let href = article
                   .find(Name("h4"))
                   .next()
                   .and_then(|node| node.find(Name("a")).next())
                   .and_then(|node| node.attr("href"))
                   .unwrap_or("")
                   .trim()
                   .to_owned();

               println!("Author: {}", author);
               println!("Title: {}", title);
               println!("Href-Link: {}", href);
               println!("---");
           }
       }
    }
}

fn html_example() -> &'static str {
    r#"
<html lang="de-DE" class="csstransforms csstransforms3d csstransitions js csstransforms csstransforms3d csstransitions Firefox Firefox118 cssanimations csstransitions no-touchevents js_active vc_desktop vc_transform vc_transform vc_transform wf-opensans-i4-active wf-opensans-n4-active wf-opensans-n6-active wf-opensans-n7-active wf-opensans-n1-active wf-opensans-n2-active wf-opensans-n3-active wf-opensans-i1-active wf-opensans-i2-active wf-opensans-i3-active wf-opensans-i6-active wf-opensans-n8-active wf-opensans-n9-active wf-opensans-n5-active wf-opensans-i5-active wf-opensans-i8-active wf-opensans-i9-active wf-opensans-i7-active wf-active" data-lt-installed="true"><head><style class="darkreader darkreader--fallback" media="screen"></style><style class="darkreader darkreader--text" media="screen"></style><style class="darkreader darkreader--invert" media="screen">.jfk-bubble.gtx-bubble, .captcheck_answer_label > input + img, span#closed_text > img[src^="https://www.gstatic.com/images/branding/googlelogo"], span[data-href^="https://www.hcaptcha.com/"] > #icon, #bit-notification-bar-iframe, ::-webkit-calendar-picker-indicator {
    filter: invert(100%) hue-rotate(180deg) contrast(90%) !important;
}</style><style class="darkreader darkreader--inline" media="screen">[data-darkreader-inline-bgcolor] {
  background-color: var(--darkreader-inline-bgcolor) !important;
}
[data-darkreader-inline-bgimage] {
  background-image: var(--darkreader-inline-bgimage) !important;
}
[data-darkreader-inline-border] {
  border-color: var(--darkreader-inline-border) !important;
}
[data-darkreader-inline-border-bottom] {
  border-bottom-color: var(--darkreader-inline-border-bottom) !important;
}
[data-darkreader-inline-border-left] {
  border-left-color: var(--darkreader-inline-border-left) !important;
}
[data-darkreader-inline-border-right] {
  border-right-color: var(--darkreader-inline-border-right) !important;
}
[data-darkreader-inline-border-top] {
  border-top-color: var(--darkreader-inline-border-top) !important;
}
[data-darkreader-inline-boxshadow] {
  box-shadow: var(--darkreader-inline-boxshadow) !important;
}
[data-darkreader-inline-color] {
  color: var(--darkreader-inline-color) !important;
}
[data-darkreader-inline-fill] {
  fill: var(--darkreader-inline-fill) !important;
}
[data-darkreader-inline-stroke] {
  stroke: var(--darkreader-inline-stroke) !important;
}
[data-darkreader-inline-outline] {
  outline-color: var(--darkreader-inline-outline) !important;
}
[data-darkreader-inline-stopcolor] {
  stop-color: var(--darkreader-inline-stopcolor) !important;
}</style><style class="darkreader darkreader--variables" media="screen">:root {
   --darkreader-neutral-background: #131516;
   --darkreader-neutral-text: #d8d4cf;
   --darkreader-selection-background: #004daa;
   --darkreader-selection-text: #e8e6e3;
}</style><style class="darkreader darkreader--root-vars" media="screen"></style><style class="darkreader darkreader--user-agent" media="screen">html {
    background-color: #181a1b !important;
}
html {
    color-scheme: dark !important;
}
html, body {
    background-color: #181a1b;
}
html, body {
    border-color: #736b5e;
    color: #e8e6e3;
}
a {
    color: #3391ff;
}
table {
    border-color: #545b5e;
}
::placeholder {
    color: #b2aba1;
}
input:-webkit-autofill,
textarea:-webkit-autofill,
select:-webkit-autofill {
    background-color: #404400 !important;
    color: #e8e6e3 !important;
}
::selection {
    background-color: #004daa !important;
    color: #e8e6e3 !important;
}
::-moz-selection {
    background-color: #004daa !important;
    color: #e8e6e3 !important;
}</style>
		
<!-- Author Meta Tags by Molongui Authorship, visit: https://wordpress.org/plugins/molongui-authorship/ -->
<meta name="author" content="LvMID">
<!-- /Molongui Authorship -->

<meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=0"><meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1"><meta name="format-detection" content="telephone=no"><title>Ludwig von Mises Institut Deutschland</title>
<script type="text/javascript" async="" src="https://www.gstatic.com/recaptcha/releases/lLirU0na9roYU3wDDisGJEVT/recaptcha__de.js" crossorigin="anonymous" integrity="sha384-GdV15gXliOPfpfOpqFhqY9dS3FCI2YwTGtVcyzj5ab0yiuc+vZVJm97rcCa1WL2G"></script><script id="facebook-jssdk" src="//connect.facebook.net/de_DE/sdk.js#xfbml=1&amp;version=v2.10"></script><script type="text/javascript">var ajaxurl = "https://www.misesde.org/wp-admin/admin-ajax.php";</script><meta name="darkreader" content="d82a417eec954712846bc62110f86375"><style class="darkreader darkreader--override" media="screen">.vimvixen-hint {
    background-color: #7b5300 !important;
    border-color: #d8b013 !important;
    color: #f3e8c8 !important;
}
#vimvixen-console-frame {
    color-scheme: light !important
}
::placeholder {
    opacity: 0.5 !important;
}
#edge-translate-panel-body,
.MuiTypography-body1,
.nfe-quote-text {
    color: var(--darkreader-neutral-text) !important;
}
gr-main-header {
    background-color: #0f3a48 !important;
}
.tou-z65h9k,
.tou-mignzq,
.tou-1b6i2ox,
.tou-lnqlqk {
    background-color: var(--darkreader-neutral-background) !important;
}
.tou-75mvi {
    background-color: #032029 !important;
}
.tou-ta9e87,
.tou-1w3fhi0,
.tou-1b8t2us,
.tou-py7lfi,
.tou-1lpmd9d,
.tou-1frrtv8,
.tou-17ezmgn {
    background-color: #0a0a0a !important;
}
.tou-uknfeu {
    background-color: #231603 !important;
}
.tou-6i3zyv {
    background-color: #19576c !important;
}
div.mermaid-viewer-control-panel .btn {
  fill: var(--darkreader-neutral-text);
  background-color: var(--darkreader-neutral-background);
}
svg g rect.er {
  fill: var(--darkreader-neutral-background) !important;
}
svg g rect.er.entityBox {
  fill: var(--darkreader-neutral-background) !important;
}
svg g rect.er.attributeBoxOdd {
  fill: var(--darkreader-neutral-background) !important;
}
svg g rect.er.attributeBoxEven {
  fill-opacity: 0.8 !important;
  fill: var(--darkreader-selection-background);
}
svg rect.er.relationshipLabelBox {
  fill: var(--darkreader-neutral-background) !important;
}
svg g g.nodes rect, svg g g.nodes polygon {
  fill: var(--darkreader-neutral-background) !important;
}
svg g rect.task {
  fill: var(--darkreader-selection-background) !important;
}
svg line.messageLine0, svg line.messageLine1 {
  stroke: var(--darkreader-neutral-text) !important;
}
div.mermaid .actor {
  fill: var(--darkreader-neutral-background) !important;
}
embed[type="application/pdf"] { filter: invert(100%) contrast(90%); }</style><meta name="robots" content="max-image-preview:large">

		<style id="critical-path-css" type="text/css">
			body,html{width:100%;height:100%;margin:0;padding:0}.page-preloader{top:0;left:0;z-index:999;position:fixed;height:100%;width:100%;text-align:center}.preloader-preview-area{animation-delay:-.2s;top:50%;-ms-transform:translateY(100%);transform:translateY(100%);margin-top:10px;max-height:calc(50% - 20px);opacity:1;width:100%;text-align:center;position:absolute}.preloader-logo{max-width:90%;top:50%;-ms-transform:translateY(-100%);transform:translateY(-100%);margin:-10px auto 0 auto;max-height:calc(50% - 20px);opacity:1;position:relative}.ball-pulse>div{width:15px;height:15px;border-radius:100%;margin:2px;animation-fill-mode:both;display:inline-block;animation:ball-pulse .75s infinite cubic-bezier(.2,.68,.18,1.08)}.ball-pulse>div:nth-child(1){animation-delay:-.36s}.ball-pulse>div:nth-child(2){animation-delay:-.24s}.ball-pulse>div:nth-child(3){animation-delay:-.12s}@keyframes ball-pulse{0%{transform:scale(1);opacity:1}45%{transform:scale(.1);opacity:.7}80%{transform:scale(1);opacity:1}}.ball-clip-rotate-pulse{position:relative;-ms-transform:translateY(-15px) translateX(-10px);transform:translateY(-15px) translateX(-10px);display:inline-block}.ball-clip-rotate-pulse>div{animation-fill-mode:both;position:absolute;top:0;left:0;border-radius:100%}.ball-clip-rotate-pulse>div:first-child{height:36px;width:36px;top:7px;left:-7px;animation:ball-clip-rotate-pulse-scale 1s 0s cubic-bezier(.09,.57,.49,.9) infinite}.ball-clip-rotate-pulse>div:last-child{position:absolute;width:50px;height:50px;left:-16px;top:-2px;background:0 0;border:2px solid;animation:ball-clip-rotate-pulse-rotate 1s 0s cubic-bezier(.09,.57,.49,.9) infinite;animation-duration:1s}@keyframes ball-clip-rotate-pulse-rotate{0%{transform:rotate(0) scale(1)}50%{transform:rotate(180deg) scale(.6)}100%{transform:rotate(360deg) scale(1)}}@keyframes ball-clip-rotate-pulse-scale{30%{transform:scale(.3)}100%{transform:scale(1)}}@keyframes square-spin{25%{transform:perspective(100px) rotateX(180deg) rotateY(0)}50%{transform:perspective(100px) rotateX(180deg) rotateY(180deg)}75%{transform:perspective(100px) rotateX(0) rotateY(180deg)}100%{transform:perspective(100px) rotateX(0) rotateY(0)}}.square-spin{display:inline-block}.square-spin>div{animation-fill-mode:both;width:50px;height:50px;animation:square-spin 3s 0s cubic-bezier(.09,.57,.49,.9) infinite}.cube-transition{position:relative;-ms-transform:translate(-25px,-25px);transform:translate(-25px,-25px);display:inline-block}.cube-transition>div{animation-fill-mode:both;width:15px;height:15px;position:absolute;top:-5px;left:-5px;animation:cube-transition 1.6s 0s infinite ease-in-out}.cube-transition>div:last-child{animation-delay:-.8s}@keyframes cube-transition{25%{transform:translateX(50px) scale(.5) rotate(-90deg)}50%{transform:translate(50px,50px) rotate(-180deg)}75%{transform:translateY(50px) scale(.5) rotate(-270deg)}100%{transform:rotate(-360deg)}}.ball-scale>div{border-radius:100%;margin:2px;animation-fill-mode:both;display:inline-block;height:60px;width:60px;animation:ball-scale 1s 0s ease-in-out infinite}@keyframes ball-scale{0%{transform:scale(0)}100%{transform:scale(1);opacity:0}}.line-scale>div{animation-fill-mode:both;display:inline-block;width:5px;height:50px;border-radius:2px;margin:2px}.line-scale>div:nth-child(1){animation:line-scale 1s -.5s infinite cubic-bezier(.2,.68,.18,1.08)}.line-scale>div:nth-child(2){animation:line-scale 1s -.4s infinite cubic-bezier(.2,.68,.18,1.08)}.line-scale>div:nth-child(3){animation:line-scale 1s -.3s infinite cubic-bezier(.2,.68,.18,1.08)}.line-scale>div:nth-child(4){animation:line-scale 1s -.2s infinite cubic-bezier(.2,.68,.18,1.08)}.line-scale>div:nth-child(5){animation:line-scale 1s -.1s infinite cubic-bezier(.2,.68,.18,1.08)}@keyframes line-scale{0%{transform:scaley(1)}50%{transform:scaley(.4)}100%{transform:scaley(1)}}.ball-scale-multiple{position:relative;-ms-transform:translateY(30px);transform:translateY(30px);display:inline-block}.ball-scale-multiple>div{border-radius:100%;animation-fill-mode:both;margin:2px;position:absolute;left:-30px;top:0;opacity:0;margin:0;width:50px;height:50px;animation:ball-scale-multiple 1s 0s linear infinite}.ball-scale-multiple>div:nth-child(2){animation-delay:-.2s}.ball-scale-multiple>div:nth-child(3){animation-delay:-.2s}@keyframes ball-scale-multiple{0%{transform:scale(0);opacity:0}5%{opacity:1}100%{transform:scale(1);opacity:0}}.ball-pulse-sync{display:inline-block}.ball-pulse-sync>div{width:15px;height:15px;border-radius:100%;margin:2px;animation-fill-mode:both;display:inline-block}.ball-pulse-sync>div:nth-child(1){animation:ball-pulse-sync .6s -.21s infinite ease-in-out}.ball-pulse-sync>div:nth-child(2){animation:ball-pulse-sync .6s -.14s infinite ease-in-out}.ball-pulse-sync>div:nth-child(3){animation:ball-pulse-sync .6s -70ms infinite ease-in-out}@keyframes ball-pulse-sync{33%{transform:translateY(10px)}66%{transform:translateY(-10px)}100%{transform:translateY(0)}}.transparent-circle{display:inline-block;border-top:.5em solid rgba(255,255,255,.2);border-right:.5em solid rgba(255,255,255,.2);border-bottom:.5em solid rgba(255,255,255,.2);border-left:.5em solid #fff;transform:translateZ(0);animation:transparent-circle 1.1s infinite linear;width:50px;height:50px;border-radius:50%}.transparent-circle:after{border-radius:50%;width:10em;height:10em}@keyframes transparent-circle{0%{transform:rotate(0)}100%{transform:rotate(360deg)}}.ball-spin-fade-loader{position:relative;top:-10px;left:-10px;display:inline-block}.ball-spin-fade-loader>div{width:15px;height:15px;border-radius:100%;margin:2px;animation-fill-mode:both;position:absolute;animation:ball-spin-fade-loader 1s infinite linear}.ball-spin-fade-loader>div:nth-child(1){top:25px;left:0;animation-delay:-.84s;-webkit-animation-delay:-.84s}.ball-spin-fade-loader>div:nth-child(2){top:17.05px;left:17.05px;animation-delay:-.72s;-webkit-animation-delay:-.72s}.ball-spin-fade-loader>div:nth-child(3){top:0;left:25px;animation-delay:-.6s;-webkit-animation-delay:-.6s}.ball-spin-fade-loader>div:nth-child(4){top:-17.05px;left:17.05px;animation-delay:-.48s;-webkit-animation-delay:-.48s}.ball-spin-fade-loader>div:nth-child(5){top:-25px;left:0;animation-delay:-.36s;-webkit-animation-delay:-.36s}.ball-spin-fade-loader>div:nth-child(6){top:-17.05px;left:-17.05px;animation-delay:-.24s;-webkit-animation-delay:-.24s}.ball-spin-fade-loader>div:nth-child(7){top:0;left:-25px;animation-delay:-.12s;-webkit-animation-delay:-.12s}.ball-spin-fade-loader>div:nth-child(8){top:17.05px;left:-17.05px;animation-delay:0s;-webkit-animation-delay:0s}@keyframes ball-spin-fade-loader{50%{opacity:.3;transform:scale(.4)}100%{opacity:1;transform:scale(1)}}		</style><style class="darkreader darkreader--sync" media="screen"></style>

		<link rel="dns-prefetch" href="//www.misesde.org">
<link rel="dns-prefetch" href="//use.fontawesome.com">
<link rel="alternate" type="application/rss+xml" title="Ludwig von Mises Institut Deutschland » Feed" href="https://www.misesde.org/feed/">
<link rel="alternate" type="application/rss+xml" title="Ludwig von Mises Institut Deutschland » Kommentar-Feed" href="https://www.misesde.org/comments/feed/">

<link rel="shortcut icon" href="https://www.misesde.org/wp-content/uploads/2013/10/logo_inkl_mises.png">
<script type="text/javascript">window.abb = {};php = {};window.PHP = {};PHP.ajax = "https://www.misesde.org/wp-admin/admin-ajax.php";PHP.wp_p_id = "18800";var mk_header_parallax, mk_banner_parallax, mk_page_parallax, mk_footer_parallax, mk_body_parallax;var mk_images_dir = "https://www.misesde.org/wp-content/themes/jupiter/assets/images",mk_theme_js_path = "https://www.misesde.org/wp-content/themes/jupiter/assets/js",mk_theme_dir = "https://www.misesde.org/wp-content/themes/jupiter",mk_captcha_placeholder = "Captcha eingeben",mk_captcha_invalid_txt = "Ungültig. Versuchen Sie es erneut.",mk_captcha_correct_txt = "Captcha richtig.",mk_responsive_nav_width = 1140,mk_vertical_header_back = "Zurück",mk_vertical_header_anim = "1",mk_check_rtl = true,mk_grid_width = 1300,mk_ajax_search_option = "fullscreen_search",mk_preloader_bg_color = #ffffff",mk_accent_color = #1e73be",mk_go_to_top =  "true",mk_smooth_scroll =  "true",mk_show_background_video =  "true",mk_preloader_bar_color = #1e73be",mk_preloader_logo = "";var mk_header_parallax = false,mk_banner_parallax = false,mk_footer_parallax = false,mk_body_parallax = false,mk_no_more_posts = "Keine weiteren Beiträge",mk_typekit_id   = "",mk_google_fonts = ["Open Sans:100italic,200italic,300italic,400italic,500italic,600italic,700italic,800italic,900italic,100,200,300,400,500,600,700,800,900"],mk_global_lazyload = false;</script><link rel="stylesheet" id="pt-cv-public-style-css" href="https://www.misesde.org/wp-content/plugins/content-views-query-and-display-post-page/public/assets/css/cv.css?ver=3.5.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
    <link rel="stylesheet" id="pt-cv-public-pro-style-css" href="https://www.misesde.org/wp-content/plugins/pt-content-views-pro/public/assets/css/cvpro.min.css?ver=5.9.2.2" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="wp-block-library-css" href="https://www.misesde.org/wp-includes/css/dist/block-library/style.min.css?ver=6.3.1" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <style id="wp-block-library-theme-inline-css" type="text/css">
        .wp-block-audio figcaption{color:#555;font-size:13px;text-align:center}.is-dark-theme .wp-block-audio figcaption{color:hsla(0,0%,100%,.65)}.wp-block-audio{margin:0 0 1em}.wp-block-code{border:1px solid #ccc;border-radius:4px;font-family:Menlo,Consolas,monaco,monospace;padding:.8em 1em}.wp-block-embed figcaption{color:#555;font-size:13px;text-align:center}.is-dark-theme .wp-block-embed figcaption{color:hsla(0,0%,100%,.65)}.wp-block-embed{margin:0 0 1em}.blocks-gallery-caption{color:#555;font-size:13px;text-align:center}.is-dark-theme .blocks-gallery-caption{color:hsla(0,0%,100%,.65)}.wp-block-image figcaption{color:#555;font-size:13px;text-align:center}.is-dark-theme .wp-block-image figcaption{color:hsla(0,0%,100%,.65)}.wp-block-image{margin:0 0 1em}.wp-block-pullquote{border-bottom:4px solid;border-top:4px solid;color:currentColor;margin-bottom:1.75em}.wp-block-pullquote cite,.wp-block-pullquote footer,.wp-block-pullquote__citation{color:currentColor;font-size:.8125em;font-style:normal;text-transform:uppercase}.wp-block-quote{border-left:.25em solid;margin:0 0 1.75em;padding-left:1em}.wp-block-quote cite,.wp-block-quote footer{color:currentColor;font-size:.8125em;font-style:normal;position:relative}.wp-block-quote.has-text-align-right{border-left:none;border-right:.25em solid;padding-left:0;padding-right:1em}.wp-block-quote.has-text-align-center{border:none;padding-left:0}.wp-block-quote.is-large,.wp-block-quote.is-style-large,.wp-block-quote.is-style-plain{border:none}.wp-block-search .wp-block-search__label{font-weight:700}.wp-block-search__button{border:1px solid #ccc;padding:.375em .625em}:where(.wp-block-group.has-background){padding:1.25em 2.375em}.wp-block-separator.has-css-opacity{opacity:.4}.wp-block-separator{border:none;border-bottom:2px solid;margin-left:auto;margin-right:auto}.wp-block-separator.has-alpha-channel-opacity{opacity:1}.wp-block-separator:not(.is-style-wide):not(.is-style-dots){width:100px}.wp-block-separator.has-background:not(.is-style-dots){border-bottom:none;height:1px}.wp-block-separator.has-background:not(.is-style-wide):not(.is-style-dots){height:2px}.wp-block-table{margin:0 0 1em}.wp-block-table td,.wp-block-table th{word-break:normal}.wp-block-table figcaption{color:#555;font-size:13px;text-align:center}.is-dark-theme .wp-block-table figcaption{color:hsla(0,0%,100%,.65)}.wp-block-video figcaption{color:#555;font-size:13px;text-align:center}.is-dark-theme .wp-block-video figcaption{color:hsla(0,0%,100%,.65)}.wp-block-video{margin:0 0 1em}.wp-block-template-part.has-background{margin-bottom:0;margin-top:0;padding:1.25em 2.375em}
        </style><style class="darkreader darkreader--sync" media="screen"></style>
        <style id="classic-theme-styles-inline-css" type="text/css">
        /*! This file is auto-generated */
        .wp-block-button__link{color:#fff;background-color:#32373c;border-radius:9999px;box-shadow:none;text-decoration:none;padding:calc(.667em + 2px) calc(1.333em + 2px);font-size:1.125em}.wp-block-file__button{background:#32373c;color:#fff;text-decoration:none}
        </style><style class="darkreader darkreader--sync" media="screen"></style>
        <style id="global-styles-inline-css" type="text/css">
        body{--wp--preset--color--black: #000000;--wp--preset--color--cyan-bluish-gray: #abb8c3;--wp--preset--color--white: #ffffff;--wp--preset--color--pale-pink: #f78da7;--wp--preset--color--vivid-red: #cf2e2e;--wp--preset--color--luminous-vivid-orange: #ff6900;--wp--preset--color--luminous-vivid-amber: #fcb900;--wp--preset--color--light-green-cyan: #7bdcb5;--wp--preset--color--vivid-green-cyan: #00d084;--wp--preset--color--pale-cyan-blue: #8ed1fc;--wp--preset--color--vivid-cyan-blue: #0693e3;--wp--preset--color--vivid-purple: #9b51e0;--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple: linear-gradient(135deg,rgba(6,147,227,1) 0%,rgb(155,81,224) 100%);--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan: linear-gradient(135deg,rgb(122,220,180) 0%,rgb(0,208,130) 100%);--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange: linear-gradient(135deg,rgba(252,185,0,1) 0%,rgba(255,105,0,1) 100%);--wp--preset--gradient--luminous-vivid-orange-to-vivid-red: linear-gradient(135deg,rgba(255,105,0,1) 0%,rgb(207,46,46) 100%);--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray: linear-gradient(135deg,rgb(238,238,238) 0%,rgb(169,184,195) 100%);--wp--preset--gradient--cool-to-warm-spectrum: linear-gradient(135deg,rgb(74,234,220) 0%,rgb(151,120,209) 20%,rgb(207,42,186) 40%,rgb(238,44,130) 60%,rgb(251,105,98) 80%,rgb(254,248,76) 100%);--wp--preset--gradient--blush-light-purple: linear-gradient(135deg,rgb(255,206,236) 0%,rgb(152,150,240) 100%);--wp--preset--gradient--blush-bordeaux: linear-gradient(135deg,rgb(254,205,165) 0%,rgb(254,45,45) 50%,rgb(107,0,62) 100%);--wp--preset--gradient--luminous-dusk: linear-gradient(135deg,rgb(255,203,112) 0%,rgb(199,81,192) 50%,rgb(65,88,208) 100%);--wp--preset--gradient--pale-ocean: linear-gradient(135deg,rgb(255,245,203) 0%,rgb(182,227,212) 50%,rgb(51,167,181) 100%);--wp--preset--gradient--electric-grass: linear-gradient(135deg,rgb(202,248,128) 0%,rgb(113,206,126) 100%);--wp--preset--gradient--midnight: linear-gradient(135deg,rgb(2,3,129) 0%,rgb(40,116,252) 100%);--wp--preset--font-size--small: 13px;--wp--preset--font-size--medium: 20px;--wp--preset--font-size--large: 36px;--wp--preset--font-size--x-large: 42px;--wp--preset--spacing--20: 0.44rem;--wp--preset--spacing--30: 0.67rem;--wp--preset--spacing--40: 1rem;--wp--preset--spacing--50: 1.5rem;--wp--preset--spacing--60: 2.25rem;--wp--preset--spacing--70: 3.38rem;--wp--preset--spacing--80: 5.06rem;--wp--preset--shadow--natural: 6px 6px 9px rgba(0, 0, 0, 0.2);--wp--preset--shadow--deep: 12px 12px 50px rgba(0, 0, 0, 0.4);--wp--preset--shadow--sharp: 6px 6px 0px rgba(0, 0, 0, 0.2);--wp--preset--shadow--outlined: 6px 6px 0px -3px rgba(255, 255, 255, 1), 6px 6px rgba(0, 0, 0, 1);--wp--preset--shadow--crisp: 6px 6px 0px rgba(0, 0, 0, 1);}:where(.is-layout-flex){gap: 0.5em;}:where(.is-layout-grid){gap: 0.5em;}body .is-layout-flow > .alignleft{float: left;margin-inline-start: 0;margin-inline-end: 2em;}body .is-layout-flow > .alignright{float: right;margin-inline-start: 2em;margin-inline-end: 0;}body .is-layout-flow > .aligncenter{margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained > .alignleft{float: left;margin-inline-start: 0;margin-inline-end: 2em;}body .is-layout-constrained > .alignright{float: right;margin-inline-start: 2em;margin-inline-end: 0;}body .is-layout-constrained > .aligncenter{margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained > :where(:not(.alignleft):not(.alignright):not(.alignfull)){max-width: var(--wp--style--global--content-size);margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained > .alignwide{max-width: var(--wp--style--global--wide-size);}body .is-layout-flex{display: flex;}body .is-layout-flex{flex-wrap: wrap;align-items: center;}body .is-layout-flex > *{margin: 0;}body .is-layout-grid{display: grid;}body .is-layout-grid > *{margin: 0;}:where(.wp-block-columns.is-layout-flex){gap: 2em;}:where(.wp-block-columns.is-layout-grid){gap: 2em;}:where(.wp-block-post-template.is-layout-flex){gap: 1.25em;}:where(.wp-block-post-template.is-layout-grid){gap: 1.25em;}.has-black-color{color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-color{color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-color{color: var(--wp--preset--color--white) !important;}.has-pale-pink-color{color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-color{color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-color{color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-color{color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-color{color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-color{color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-color{color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-color{color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-color{color: var(--wp--preset--color--vivid-purple) !important;}.has-black-background-color{background-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-background-color{background-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-background-color{background-color: var(--wp--preset--color--white) !important;}.has-pale-pink-background-color{background-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-background-color{background-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-background-color{background-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-background-color{background-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-background-color{background-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-background-color{background-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-background-color{background-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-background-color{background-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-background-color{background-color: var(--wp--preset--color--vivid-purple) !important;}.has-black-border-color{border-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-border-color{border-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-border-color{border-color: var(--wp--preset--color--white) !important;}.has-pale-pink-border-color{border-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-border-color{border-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-border-color{border-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-border-color{border-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-border-color{border-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-border-color{border-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-border-color{border-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-border-color{border-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-border-color{border-color: var(--wp--preset--color--vivid-purple) !important;}.has-vivid-cyan-blue-to-vivid-purple-gradient-background{background: var(--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple) !important;}.has-light-green-cyan-to-vivid-green-cyan-gradient-background{background: var(--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan) !important;}.has-luminous-vivid-amber-to-luminous-vivid-orange-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange) !important;}.has-luminous-vivid-orange-to-vivid-red-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-orange-to-vivid-red) !important;}.has-very-light-gray-to-cyan-bluish-gray-gradient-background{background: var(--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray) !important;}.has-cool-to-warm-spectrum-gradient-background{background: var(--wp--preset--gradient--cool-to-warm-spectrum) !important;}.has-blush-light-purple-gradient-background{background: var(--wp--preset--gradient--blush-light-purple) !important;}.has-blush-bordeaux-gradient-background{background: var(--wp--preset--gradient--blush-bordeaux) !important;}.has-luminous-dusk-gradient-background{background: var(--wp--preset--gradient--luminous-dusk) !important;}.has-pale-ocean-gradient-background{background: var(--wp--preset--gradient--pale-ocean) !important;}.has-electric-grass-gradient-background{background: var(--wp--preset--gradient--electric-grass) !important;}.has-midnight-gradient-background{background: var(--wp--preset--gradient--midnight) !important;}.has-small-font-size{font-size: var(--wp--preset--font-size--small) !important;}.has-medium-font-size{font-size: var(--wp--preset--font-size--medium) !important;}.has-large-font-size{font-size: var(--wp--preset--font-size--large) !important;}.has-x-large-font-size{font-size: var(--wp--preset--font-size--x-large) !important;}
        .wp-block-navigation a:where(:not(.wp-element-button)){color: inherit;}
    :where(.wp-block-post-template.is-layout-flex){gap: 1.25em;}:where(.wp-block-post-template.is-layout-grid){gap: 1.25em;}
    :where(.wp-block-columns.is-layout-flex){gap: 2em;}:where(.wp-block-columns.is-layout-grid){gap: 2em;}
        .wp-block-pullquote{font-size: 1.5em;line-height: 1.6;}
        </style><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="contact-form-7-css" href="https://www.misesde.org/wp-content/plugins/contact-form-7/includes/css/styles.css?ver=5.8.1" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="cookie-law-info-css" href="https://www.misesde.org/wp-content/plugins/cookie-law-info/legacy/public/css/cookie-law-info-public.css?ver=3.1.4" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="cookie-law-info-gdpr-css" href="https://www.misesde.org/wp-content/plugins/cookie-law-info/legacy/public/css/cookie-law-info-gdpr.css?ver=3.1.4" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="email-subscribers-css" href="https://www.misesde.org/wp-content/plugins/email-subscribers/lite/public/css/email-subscribers-public.css?ver=5.6.23" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="SFSIPLUSmainCss-css" href="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/css/sfsi-style.css?ver=3.6.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="theme-styles-css" href="https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/min/full-styles.6.10.2.css?ver=1656075786" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <style id="theme-styles-inline-css" type="text/css">

    #wpadminbar {
        -webkit-backface-visibility: hidden;
        backface-visibility: hidden;
        -webkit-perspective: 1000;
        -ms-perspective: 1000;
        perspective: 1000;
        -webkit-transform: translateZ(0px);
        -ms-transform: translateZ(0px);
        transform: translateZ(0px);
    }
    @media screen and (max-width: 600px) {
        #wpadminbar {
            position: fixed !important;
        }
    }

    body { background-color:#fff; } .hb-custom-header #mk-page-introduce, .mk-header { background-color:#b7b7b7;background-position:center top;background-attachment:fixed;background-size:cover;-webkit-background-size:cover;-moz-background-size:cover; } .hb-custom-header > div, .mk-header-bg { background-color:#0a3159;background-position:center top;background-size:cover;-webkit-background-size:cover;-moz-background-size:cover; } .mk-classic-nav-bg { background-color:#0a3159;background-position:center top;background-size:cover;-webkit-background-size:cover;-moz-background-size:cover; } .master-holder-bg { background-color:#fff; } #mk-footer { background-color:#3d4045; } #mk-boxed-layout { -webkit-box-shadow:0 0 0px rgba(0, 0, 0, 0); -moz-box-shadow:0 0 0px rgba(0, 0, 0, 0); box-shadow:0 0 0px rgba(0, 0, 0, 0); } .mk-news-tab .mk-tabs-tabs .is-active a, .mk-fancy-title.pattern-style span, .mk-fancy-title.pattern-style.color-gradient span:after, .page-bg-color { background-color:#fff; } .page-title { font-size:15px; color:#4d4d4d; text-transform:uppercase; font-weight:900; letter-spacing:2px; } .page-subtitle { font-size:14px; line-height:100%; color:#4d4d4d; font-size:14px; text-transform:none; } .mk-header { border-bottom:1px solid #ededed; } .header-style-1 .mk-header-padding-wrapper, .header-style-2 .mk-header-padding-wrapper, .header-style-3 .mk-header-padding-wrapper { padding-top:154px; } .mk-process-steps[max-width~="950px"] ul::before { display:none !important; } .mk-process-steps[max-width~="950px"] li { margin-bottom:30px !important; width:100% !important; text-align:center; } .mk-event-countdown-ul[max-width~="750px"] li { width:90%; display:block; margin:0 auto 15px; } body { font-family:Open Sans } @font-face { font-family:'star'; src:url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/star/font.eot'); src:url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/star/font.eot?#iefix') format('embedded-opentype'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/star/font.woff') format('woff'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/star/font.ttf') format('truetype'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/star/font.svg#star') format('svg'); font-weight:normal; font-style:normal; } @font-face { font-family:'WooCommerce'; src:url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/woocommerce/font.eot'); src:url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/woocommerce/font.eot?#iefix') format('embedded-opentype'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/woocommerce/font.woff') format('woff'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/woocommerce/font.ttf') format('truetype'), url('https://www.misesde.org/wp-content/themes/jupiter/assets/stylesheet/fonts/woocommerce/font.svg#WooCommerce') format('svg'); font-weight:normal; font-style:normal; }#podcast-container { position:relative; width:100%; height:100%; } .podcasts { position:relative; left:0; width:100%; height:100%; overflow:hidden; background-color:#ddd; border:3px solid #ccc; border-radius:10px; } .podcasts .mk-video-container { width:90%; line-height:242px; height:160px; margin:1px 0 0 10px; padding:0; padding-bottom:0; position:relative; } #video-container .mk-video-container { padding-bottom:56.25%; } aside { box-shadow:4px 4px 3px rgba(0,0,0,0.08); background-color:#f8f8f8; border-radius:4px; max-width:270px; } .mises-sidebar { padding:20px; } .mises-button { border-radius:4px; background:#0a3159; color:#fff; padding:8px; box-shadow:3px 3px 4px 4px rgba(0,0,0,0.1); margin:10px; text-align:center; } .mises-button a { color:#eee !important; } .mises-button:hover { background:#1a4169; color:#fff; box-shadow:1px 1px 4px 4px rgba(0,0,0,0.2); } .mises-button:hover a { color:#fff !important; } div.wpcf7-response-output { position:relative; display:none; margin:0; padding:20px; border-radius:4px; } .mk-love-this, .mk-love-holder { display:none; } #div-slider-box { height:350px; } .ms-container * { font-family:"Open Sans", sans-serif; } #div-slider-left { width:60%; height:100%; float:left; } #div-slider-right { width:35%; height:100%; float:left; margin-left:6px; } #div-slider-right-top { width:100%; height:200px; } #div-slider-right-b1 { width:48%; height:150px; float:left; margin:3% 0 0 0; } #div-slider-right-b2 { width:48%; height:150px; float:left; margin:3% 0 0 4%; } .fb-like { outline:1px solid #888; } @media only screen and (max-width:800px) { #div-slider-box { } }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         </style><style class="darkreader darkreader--sync" media="screen"></style>
            <link rel="stylesheet" id="mkhb-render-css" href="https://www.misesde.org/wp-content/themes/jupiter/header-builder/includes/assets/css/mkhb-render.css?ver=6.10.2" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="mkhb-row-css" href="https://www.misesde.org/wp-content/themes/jupiter/header-builder/includes/assets/css/mkhb-row.css?ver=6.10.2" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="mkhb-column-css" href="https://www.misesde.org/wp-content/themes/jupiter/header-builder/includes/assets/css/mkhb-column.css?ver=6.10.2" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="js_composer_front-css" href="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/css/js_composer.min.css?ver=7.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="theme-options-css" href="https://www.misesde.org/wp-content/uploads/mk_assets/theme-options-production-1696923017.css?ver=1696923008" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="ms-main-css" href="https://www.misesde.org/wp-content/plugins/masterslider/public/assets/css/masterslider.main.css?ver=3.6.5" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="ms-custom-css" href="https://www.misesde.org/wp-content/uploads/masterslider/custom.css?ver=28" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="bfa-font-awesome-css" href="https://use.fontawesome.com/releases/v5.15.4/css/all.css?ver=2.0.3" type="text/css" media="all"><style class="darkreader darkreader--cors" media="screen">.fa,.fab,.fad,.fal,.far,.fas{-moz-osx-font-smoothing:grayscale;-webkit-font-smoothing:antialiased;display:inline-block;font-style:normal;font-variant:normal;text-rendering:auto;line-height:1}.fa-lg{font-size:1.33333em;line-height:.75em;vertical-align:-.0667em}.fa-xs{font-size:.75em}.fa-sm{font-size:.875em}.fa-1x{font-size:1em}.fa-2x{font-size:2em}.fa-3x{font-size:3em}.fa-4x{font-size:4em}.fa-5x{font-size:5em}.fa-6x{font-size:6em}.fa-7x{font-size:7em}.fa-8x{font-size:8em}.fa-9x{font-size:9em}.fa-10x{font-size:10em}.fa-fw{text-align:center;width:1.25em}.fa-ul{list-style-type:none;margin-left:2.5em;padding-left:0}.fa-ul>li{position:relative}.fa-li{left:-2em;position:absolute;text-align:center;width:2em;line-height:inherit}.fa-border{border:.08em solid #eee;border-radius:.1em;padding:.2em .25em .15em}.fa-pull-left{float:left}.fa-pull-right{float:right}.fa.fa-pull-left,.fab.fa-pull-left,.fal.fa-pull-left,.far.fa-pull-left,.fas.fa-pull-left{margin-right:.3em}.fa.fa-pull-right,.fab.fa-pull-right,.fal.fa-pull-right,.far.fa-pull-right,.fas.fa-pull-right{margin-left:.3em}.fa-spin{-webkit-animation:fa-spin 2s linear infinite;animation:fa-spin 2s linear infinite}.fa-pulse{-webkit-animation:fa-spin 1s steps(8) infinite;animation:fa-spin 1s steps(8) infinite}@-webkit-keyframes fa-spin{0%{-webkit-transform:rotate(0deg);transform:rotate(0deg)}to{-webkit-transform:rotate(1turn);transform:rotate(1turn)}}@keyframes fa-spin{0%{-webkit-transform:rotate(0deg);transform:rotate(0deg)}to{-webkit-transform:rotate(1turn);transform:rotate(1turn)}}.fa-rotate-90{-ms-filter:"progid:DXImageTransform.Microsoft.BasicImage(rotation=1)";-webkit-transform:rotate(90deg);transform:rotate(90deg)}.fa-rotate-180{-ms-filter:"progid:DXImageTransform.Microsoft.BasicImage(rotation=2)";-webkit-transform:rotate(180deg);transform:rotate(180deg)}.fa-rotate-270{-ms-filter:"progid:DXImageTransform.Microsoft.BasicImage(rotation=3)";-webkit-transform:rotate(270deg);transform:rotate(270deg)}.fa-flip-horizontal{-ms-filter:"progid:DXImageTransform.Microsoft.BasicImage(rotation=0, mirror=1)";-webkit-transform:scaleX(-1);transform:scaleX(-1)}.fa-flip-vertical{-webkit-transform:scaleY(-1);transform:scaleY(-1)}.fa-flip-both,.fa-flip-horizontal.fa-flip-vertical,.fa-flip-vertical{-ms-filter:"progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)"}.fa-flip-both,.fa-flip-horizontal.fa-flip-vertical{-webkit-transform:scale(-1);transform:scale(-1)}:root .fa-flip-both,:root .fa-flip-horizontal,:root .fa-flip-vertical,:root .fa-rotate-90,:root .fa-rotate-180,:root .fa-rotate-270{-webkit-filter:none;filter:none}.fa-stack{display:inline-block;height:2em;line-height:2em;position:relative;vertical-align:middle;width:2.5em}.fa-stack-1x,.fa-stack-2x{left:0;position:absolute;text-align:center;width:100%}.fa-stack-1x{line-height:inherit}.fa-stack-2x{font-size:2em}.fa-inverse{color:#fff}.fa-500px:before{content:"\f26e"}.fa-accessible-icon:before{content:"\f368"}.fa-accusoft:before{content:"\f369"}.fa-acquisitions-incorporated:before{content:"\f6af"}.fa-ad:before{content:"\f641"}.fa-address-book:before{content:"\f2b9"}.fa-address-card:before{content:"\f2bb"}.fa-adjust:before{content:"\f042"}.fa-adn:before{content:"\f170"}.fa-adversal:before{content:"\f36a"}.fa-affiliatetheme:before{content:"\f36b"}.fa-air-freshener:before{content:"\f5d0"}.fa-airbnb:before{content:"\f834"}.fa-algolia:before{content:"\f36c"}.fa-align-center:before{content:"\f037"}.fa-align-justify:before{content:"\f039"}.fa-align-left:before{content:"\f036"}.fa-align-right:before{content:"\f038"}.fa-alipay:before{content:"\f642"}.fa-allergies:before{content:"\f461"}.fa-amazon:before{content:"\f270"}.fa-amazon-pay:before{content:"\f42c"}.fa-ambulance:before{content:"\f0f9"}.fa-american-sign-language-interpreting:before{content:"\f2a3"}.fa-amilia:before{content:"\f36d"}.fa-anchor:before{content:"\f13d"}.fa-android:before{content:"\f17b"}.fa-angellist:before{content:"\f209"}.fa-angle-double-down:before{content:"\f103"}.fa-angle-double-left:before{content:"\f100"}.fa-angle-double-right:before{content:"\f101"}.fa-angle-double-up:before{content:"\f102"}.fa-angle-down:before{content:"\f107"}.fa-angle-left:before{content:"\f104"}.fa-angle-right:before{content:"\f105"}.fa-angle-up:before{content:"\f106"}.fa-angry:before{content:"\f556"}.fa-angrycreative:before{content:"\f36e"}.fa-angular:before{content:"\f420"}.fa-ankh:before{content:"\f644"}.fa-app-store:before{content:"\f36f"}.fa-app-store-ios:before{content:"\f370"}.fa-apper:before{content:"\f371"}.fa-apple:before{content:"\f179"}.fa-apple-alt:before{content:"\f5d1"}.fa-apple-pay:before{content:"\f415"}.fa-archive:before{content:"\f187"}.fa-archway:before{content:"\f557"}.fa-arrow-alt-circle-down:before{content:"\f358"}.fa-arrow-alt-circle-left:before{content:"\f359"}.fa-arrow-alt-circle-right:before{content:"\f35a"}.fa-arrow-alt-circle-up:before{content:"\f35b"}.fa-arrow-circle-down:before{content:"\f0ab"}.fa-arrow-circle-left:before{content:"\f0a8"}.fa-arrow-circle-right:before{content:"\f0a9"}.fa-arrow-circle-up:before{content:"\f0aa"}.fa-arrow-down:before{content:"\f063"}.fa-arrow-left:before{content:"\f060"}.fa-arrow-right:before{content:"\f061"}.fa-arrow-up:before{content:"\f062"}.fa-arrows-alt:before{content:"\f0b2"}.fa-arrows-alt-h:before{content:"\f337"}.fa-arrows-alt-v:before{content:"\f338"}.fa-artstation:before{content:"\f77a"}.fa-assistive-listening-systems:before{content:"\f2a2"}.fa-asterisk:before{content:"\f069"}.fa-asymmetrik:before{content:"\f372"}.fa-at:before{content:"\f1fa"}.fa-atlas:before{content:"\f558"}.fa-atlassian:before{content:"\f77b"}.fa-atom:before{content:"\f5d2"}.fa-audible:before{content:"\f373"}.fa-audio-description:before{content:"\f29e"}.fa-autoprefixer:before{content:"\f41c"}.fa-avianex:before{content:"\f374"}.fa-aviato:before{content:"\f421"}.fa-award:before{content:"\f559"}.fa-aws:before{content:"\f375"}.fa-baby:before{content:"\f77c"}.fa-baby-carriage:before{content:"\f77d"}.fa-backspace:before{content:"\f55a"}.fa-backward:before{content:"\f04a"}.fa-bacon:before{content:"\f7e5"}.fa-bacteria:before{content:"\e059"}.fa-bacterium:before{content:"\e05a"}.fa-bahai:before{content:"\f666"}.fa-balance-scale:before{content:"\f24e"}.fa-balance-scale-left:before{content:"\f515"}.fa-balance-scale-right:before{content:"\f516"}.fa-ban:before{content:"\f05e"}.fa-band-aid:before{content:"\f462"}.fa-bandcamp:before{content:"\f2d5"}.fa-barcode:before{content:"\f02a"}.fa-bars:before{content:"\f0c9"}.fa-baseball-ball:before{content:"\f433"}.fa-basketball-ball:before{content:"\f434"}.fa-bath:before{content:"\f2cd"}.fa-battery-empty:before{content:"\f244"}.fa-battery-full:before{content:"\f240"}.fa-battery-half:before{content:"\f242"}.fa-battery-quarter:before{content:"\f243"}.fa-battery-three-quarters:before{content:"\f241"}.fa-battle-net:before{content:"\f835"}.fa-bed:before{content:"\f236"}.fa-beer:before{content:"\f0fc"}.fa-behance:before{content:"\f1b4"}.fa-behance-square:before{content:"\f1b5"}.fa-bell:before{content:"\f0f3"}.fa-bell-slash:before{content:"\f1f6"}.fa-bezier-curve:before{content:"\f55b"}.fa-bible:before{content:"\f647"}.fa-bicycle:before{content:"\f206"}.fa-biking:before{content:"\f84a"}.fa-bimobject:before{content:"\f378"}.fa-binoculars:before{content:"\f1e5"}.fa-biohazard:before{content:"\f780"}.fa-birthday-cake:before{content:"\f1fd"}.fa-bitbucket:before{content:"\f171"}.fa-bitcoin:before{content:"\f379"}.fa-bity:before{content:"\f37a"}.fa-black-tie:before{content:"\f27e"}.fa-blackberry:before{content:"\f37b"}.fa-blender:before{content:"\f517"}.fa-blender-phone:before{content:"\f6b6"}.fa-blind:before{content:"\f29d"}.fa-blog:before{content:"\f781"}.fa-blogger:before{content:"\f37c"}.fa-blogger-b:before{content:"\f37d"}.fa-bluetooth:before{content:"\f293"}.fa-bluetooth-b:before{content:"\f294"}.fa-bold:before{content:"\f032"}.fa-bolt:before{content:"\f0e7"}.fa-bomb:before{content:"\f1e2"}.fa-bone:before{content:"\f5d7"}.fa-bong:before{content:"\f55c"}.fa-book:before{content:"\f02d"}.fa-book-dead:before{content:"\f6b7"}.fa-book-medical:before{content:"\f7e6"}.fa-book-open:before{content:"\f518"}.fa-book-reader:before{content:"\f5da"}.fa-bookmark:before{content:"\f02e"}.fa-bootstrap:before{content:"\f836"}.fa-border-all:before{content:"\f84c"}.fa-border-none:before{content:"\f850"}.fa-border-style:before{content:"\f853"}.fa-bowling-ball:before{content:"\f436"}.fa-box:before{content:"\f466"}.fa-box-open:before{content:"\f49e"}.fa-box-tissue:before{content:"\e05b"}.fa-boxes:before{content:"\f468"}.fa-braille:before{content:"\f2a1"}.fa-brain:before{content:"\f5dc"}.fa-bread-slice:before{content:"\f7ec"}.fa-briefcase:before{content:"\f0b1"}.fa-briefcase-medical:before{content:"\f469"}.fa-broadcast-tower:before{content:"\f519"}.fa-broom:before{content:"\f51a"}.fa-brush:before{content:"\f55d"}.fa-btc:before{content:"\f15a"}.fa-buffer:before{content:"\f837"}.fa-bug:before{content:"\f188"}.fa-building:before{content:"\f1ad"}.fa-bullhorn:before{content:"\f0a1"}.fa-bullseye:before{content:"\f140"}.fa-burn:before{content:"\f46a"}.fa-buromobelexperte:before{content:"\f37f"}.fa-bus:before{content:"\f207"}.fa-bus-alt:before{content:"\f55e"}.fa-business-time:before{content:"\f64a"}.fa-buy-n-large:before{content:"\f8a6"}.fa-buysellads:before{content:"\f20d"}.fa-calculator:before{content:"\f1ec"}.fa-calendar:before{content:"\f133"}.fa-calendar-alt:before{content:"\f073"}.fa-calendar-check:before{content:"\f274"}.fa-calendar-day:before{content:"\f783"}.fa-calendar-minus:before{content:"\f272"}.fa-calendar-plus:before{content:"\f271"}.fa-calendar-times:before{content:"\f273"}.fa-calendar-week:before{content:"\f784"}.fa-camera:before{content:"\f030"}.fa-camera-retro:before{content:"\f083"}.fa-campground:before{content:"\f6bb"}.fa-canadian-maple-leaf:before{content:"\f785"}.fa-candy-cane:before{content:"\f786"}.fa-cannabis:before{content:"\f55f"}.fa-capsules:before{content:"\f46b"}.fa-car:before{content:"\f1b9"}.fa-car-alt:before{content:"\f5de"}.fa-car-battery:before{content:"\f5df"}.fa-car-crash:before{content:"\f5e1"}.fa-car-side:before{content:"\f5e4"}.fa-caravan:before{content:"\f8ff"}.fa-caret-down:before{content:"\f0d7"}.fa-caret-left:before{content:"\f0d9"}.fa-caret-right:before{content:"\f0da"}.fa-caret-square-down:before{content:"\f150"}.fa-caret-square-left:before{content:"\f191"}.fa-caret-square-right:before{content:"\f152"}.fa-caret-square-up:before{content:"\f151"}.fa-caret-up:before{content:"\f0d8"}.fa-carrot:before{content:"\f787"}.fa-cart-arrow-down:before{content:"\f218"}.fa-cart-plus:before{content:"\f217"}.fa-cash-register:before{content:"\f788"}.fa-cat:before{content:"\f6be"}.fa-cc-amazon-pay:before{content:"\f42d"}.fa-cc-amex:before{content:"\f1f3"}.fa-cc-apple-pay:before{content:"\f416"}.fa-cc-diners-club:before{content:"\f24c"}.fa-cc-discover:before{content:"\f1f2"}.fa-cc-jcb:before{content:"\f24b"}.fa-cc-mastercard:before{content:"\f1f1"}.fa-cc-paypal:before{content:"\f1f4"}.fa-cc-stripe:before{content:"\f1f5"}.fa-cc-visa:before{content:"\f1f0"}.fa-centercode:before{content:"\f380"}.fa-centos:before{content:"\f789"}.fa-certificate:before{content:"\f0a3"}.fa-chair:before{content:"\f6c0"}.fa-chalkboard:before{content:"\f51b"}.fa-chalkboard-teacher:before{content:"\f51c"}.fa-charging-station:before{content:"\f5e7"}.fa-chart-area:before{content:"\f1fe"}.fa-chart-bar:before{content:"\f080"}.fa-chart-line:before{content:"\f201"}.fa-chart-pie:before{content:"\f200"}.fa-check:before{content:"\f00c"}.fa-check-circle:before{content:"\f058"}.fa-check-double:before{content:"\f560"}.fa-check-square:before{content:"\f14a"}.fa-cheese:before{content:"\f7ef"}.fa-chess:before{content:"\f439"}.fa-chess-bishop:before{content:"\f43a"}.fa-chess-board:before{content:"\f43c"}.fa-chess-king:before{content:"\f43f"}.fa-chess-knight:before{content:"\f441"}.fa-chess-pawn:before{content:"\f443"}.fa-chess-queen:before{content:"\f445"}.fa-chess-rook:before{content:"\f447"}.fa-chevron-circle-down:before{content:"\f13a"}.fa-chevron-circle-left:before{content:"\f137"}.fa-chevron-circle-right:before{content:"\f138"}.fa-chevron-circle-up:before{content:"\f139"}.fa-chevron-down:before{content:"\f078"}.fa-chevron-left:before{content:"\f053"}.fa-chevron-right:before{content:"\f054"}.fa-chevron-up:before{content:"\f077"}.fa-child:before{content:"\f1ae"}.fa-chrome:before{content:"\f268"}.fa-chromecast:before{content:"\f838"}.fa-church:before{content:"\f51d"}.fa-circle:before{content:"\f111"}.fa-circle-notch:before{content:"\f1ce"}.fa-city:before{content:"\f64f"}.fa-clinic-medical:before{content:"\f7f2"}.fa-clipboard:before{content:"\f328"}.fa-clipboard-check:before{content:"\f46c"}.fa-clipboard-list:before{content:"\f46d"}.fa-clock:before{content:"\f017"}.fa-clone:before{content:"\f24d"}.fa-closed-captioning:before{content:"\f20a"}.fa-cloud:before{content:"\f0c2"}.fa-cloud-download-alt:before{content:"\f381"}.fa-cloud-meatball:before{content:"\f73b"}.fa-cloud-moon:before{content:"\f6c3"}.fa-cloud-moon-rain:before{content:"\f73c"}.fa-cloud-rain:before{content:"\f73d"}.fa-cloud-showers-heavy:before{content:"\f740"}.fa-cloud-sun:before{content:"\f6c4"}.fa-cloud-sun-rain:before{content:"\f743"}.fa-cloud-upload-alt:before{content:"\f382"}.fa-cloudflare:before{content:"\e07d"}.fa-cloudscale:before{content:"\f383"}.fa-cloudsmith:before{content:"\f384"}.fa-cloudversify:before{content:"\f385"}.fa-cocktail:before{content:"\f561"}.fa-code:before{content:"\f121"}.fa-code-branch:before{content:"\f126"}.fa-codepen:before{content:"\f1cb"}.fa-codiepie:before{content:"\f284"}.fa-coffee:before{content:"\f0f4"}.fa-cog:before{content:"\f013"}.fa-cogs:before{content:"\f085"}.fa-coins:before{content:"\f51e"}.fa-columns:before{content:"\f0db"}.fa-comment:before{content:"\f075"}.fa-comment-alt:before{content:"\f27a"}.fa-comment-dollar:before{content:"\f651"}.fa-comment-dots:before{content:"\f4ad"}.fa-comment-medical:before{content:"\f7f5"}.fa-comment-slash:before{content:"\f4b3"}.fa-comments:before{content:"\f086"}.fa-comments-dollar:before{content:"\f653"}.fa-compact-disc:before{content:"\f51f"}.fa-compass:before{content:"\f14e"}.fa-compress:before{content:"\f066"}.fa-compress-alt:before{content:"\f422"}.fa-compress-arrows-alt:before{content:"\f78c"}.fa-concierge-bell:before{content:"\f562"}.fa-confluence:before{content:"\f78d"}.fa-connectdevelop:before{content:"\f20e"}.fa-contao:before{content:"\f26d"}.fa-cookie:before{content:"\f563"}.fa-cookie-bite:before{content:"\f564"}.fa-copy:before{content:"\f0c5"}.fa-copyright:before{content:"\f1f9"}.fa-cotton-bureau:before{content:"\f89e"}.fa-couch:before{content:"\f4b8"}.fa-cpanel:before{content:"\f388"}.fa-creative-commons:before{content:"\f25e"}.fa-creative-commons-by:before{content:"\f4e7"}.fa-creative-commons-nc:before{content:"\f4e8"}.fa-creative-commons-nc-eu:before{content:"\f4e9"}.fa-creative-commons-nc-jp:before{content:"\f4ea"}.fa-creative-commons-nd:before{content:"\f4eb"}.fa-creative-commons-pd:before{content:"\f4ec"}.fa-creative-commons-pd-alt:before{content:"\f4ed"}.fa-creative-commons-remix:before{content:"\f4ee"}.fa-creative-commons-sa:before{content:"\f4ef"}.fa-creative-commons-sampling:before{content:"\f4f0"}.fa-creative-commons-sampling-plus:before{content:"\f4f1"}.fa-creative-commons-share:before{content:"\f4f2"}.fa-creative-commons-zero:before{content:"\f4f3"}.fa-credit-card:before{content:"\f09d"}.fa-critical-role:before{content:"\f6c9"}.fa-crop:before{content:"\f125"}.fa-crop-alt:before{content:"\f565"}.fa-cross:before{content:"\f654"}.fa-crosshairs:before{content:"\f05b"}.fa-crow:before{content:"\f520"}.fa-crown:before{content:"\f521"}.fa-crutch:before{content:"\f7f7"}.fa-css3:before{content:"\f13c"}.fa-css3-alt:before{content:"\f38b"}.fa-cube:before{content:"\f1b2"}.fa-cubes:before{content:"\f1b3"}.fa-cut:before{content:"\f0c4"}.fa-cuttlefish:before{content:"\f38c"}.fa-d-and-d:before{content:"\f38d"}.fa-d-and-d-beyond:before{content:"\f6ca"}.fa-dailymotion:before{content:"\e052"}.fa-dashcube:before{content:"\f210"}.fa-database:before{content:"\f1c0"}.fa-deaf:before{content:"\f2a4"}.fa-deezer:before{content:"\e077"}.fa-delicious:before{content:"\f1a5"}.fa-democrat:before{content:"\f747"}.fa-deploydog:before{content:"\f38e"}.fa-deskpro:before{content:"\f38f"}.fa-desktop:before{content:"\f108"}.fa-dev:before{content:"\f6cc"}.fa-deviantart:before{content:"\f1bd"}.fa-dharmachakra:before{content:"\f655"}.fa-dhl:before{content:"\f790"}.fa-diagnoses:before{content:"\f470"}.fa-diaspora:before{content:"\f791"}.fa-dice:before{content:"\f522"}.fa-dice-d20:before{content:"\f6cf"}.fa-dice-d6:before{content:"\f6d1"}.fa-dice-five:before{content:"\f523"}.fa-dice-four:before{content:"\f524"}.fa-dice-one:before{content:"\f525"}.fa-dice-six:before{content:"\f526"}.fa-dice-three:before{content:"\f527"}.fa-dice-two:before{content:"\f528"}.fa-digg:before{content:"\f1a6"}.fa-digital-ocean:before{content:"\f391"}.fa-digital-tachograph:before{content:"\f566"}.fa-directions:before{content:"\f5eb"}.fa-discord:before{content:"\f392"}.fa-discourse:before{content:"\f393"}.fa-disease:before{content:"\f7fa"}.fa-divide:before{content:"\f529"}.fa-dizzy:before{content:"\f567"}.fa-dna:before{content:"\f471"}.fa-dochub:before{content:"\f394"}.fa-docker:before{content:"\f395"}.fa-dog:before{content:"\f6d3"}.fa-dollar-sign:before{content:"\f155"}.fa-dolly:before{content:"\f472"}.fa-dolly-flatbed:before{content:"\f474"}.fa-donate:before{content:"\f4b9"}.fa-door-closed:before{content:"\f52a"}.fa-door-open:before{content:"\f52b"}.fa-dot-circle:before{content:"\f192"}.fa-dove:before{content:"\f4ba"}.fa-download:before{content:"\f019"}.fa-draft2digital:before{content:"\f396"}.fa-drafting-compass:before{content:"\f568"}.fa-dragon:before{content:"\f6d5"}.fa-draw-polygon:before{content:"\f5ee"}.fa-dribbble:before{content:"\f17d"}.fa-dribbble-square:before{content:"\f397"}.fa-dropbox:before{content:"\f16b"}.fa-drum:before{content:"\f569"}.fa-drum-steelpan:before{content:"\f56a"}.fa-drumstick-bite:before{content:"\f6d7"}.fa-drupal:before{content:"\f1a9"}.fa-dumbbell:before{content:"\f44b"}.fa-dumpster:before{content:"\f793"}.fa-dumpster-fire:before{content:"\f794"}.fa-dungeon:before{content:"\f6d9"}.fa-dyalog:before{content:"\f399"}.fa-earlybirds:before{content:"\f39a"}.fa-ebay:before{content:"\f4f4"}.fa-edge:before{content:"\f282"}.fa-edge-legacy:before{content:"\e078"}.fa-edit:before{content:"\f044"}.fa-egg:before{content:"\f7fb"}.fa-eject:before{content:"\f052"}.fa-elementor:before{content:"\f430"}.fa-ellipsis-h:before{content:"\f141"}.fa-ellipsis-v:before{content:"\f142"}.fa-ello:before{content:"\f5f1"}.fa-ember:before{content:"\f423"}.fa-empire:before{content:"\f1d1"}.fa-envelope:before{content:"\f0e0"}.fa-envelope-open:before{content:"\f2b6"}.fa-envelope-open-text:before{content:"\f658"}.fa-envelope-square:before{content:"\f199"}.fa-envira:before{content:"\f299"}.fa-equals:before{content:"\f52c"}.fa-eraser:before{content:"\f12d"}.fa-erlang:before{content:"\f39d"}.fa-ethereum:before{content:"\f42e"}.fa-ethernet:before{content:"\f796"}.fa-etsy:before{content:"\f2d7"}.fa-euro-sign:before{content:"\f153"}.fa-evernote:before{content:"\f839"}.fa-exchange-alt:before{content:"\f362"}.fa-exclamation:before{content:"\f12a"}.fa-exclamation-circle:before{content:"\f06a"}.fa-exclamation-triangle:before{content:"\f071"}.fa-expand:before{content:"\f065"}.fa-expand-alt:before{content:"\f424"}.fa-expand-arrows-alt:before{content:"\f31e"}.fa-expeditedssl:before{content:"\f23e"}.fa-external-link-alt:before{content:"\f35d"}.fa-external-link-square-alt:before{content:"\f360"}.fa-eye:before{content:"\f06e"}.fa-eye-dropper:before{content:"\f1fb"}.fa-eye-slash:before{content:"\f070"}.fa-facebook:before{content:"\f09a"}.fa-facebook-f:before{content:"\f39e"}.fa-facebook-messenger:before{content:"\f39f"}.fa-facebook-square:before{content:"\f082"}.fa-fan:before{content:"\f863"}.fa-fantasy-flight-games:before{content:"\f6dc"}.fa-fast-backward:before{content:"\f049"}.fa-fast-forward:before{content:"\f050"}.fa-faucet:before{content:"\e005"}.fa-fax:before{content:"\f1ac"}.fa-feather:before{content:"\f52d"}.fa-feather-alt:before{content:"\f56b"}.fa-fedex:before{content:"\f797"}.fa-fedora:before{content:"\f798"}.fa-female:before{content:"\f182"}.fa-fighter-jet:before{content:"\f0fb"}.fa-figma:before{content:"\f799"}.fa-file:before{content:"\f15b"}.fa-file-alt:before{content:"\f15c"}.fa-file-archive:before{content:"\f1c6"}.fa-file-audio:before{content:"\f1c7"}.fa-file-code:before{content:"\f1c9"}.fa-file-contract:before{content:"\f56c"}.fa-file-csv:before{content:"\f6dd"}.fa-file-download:before{content:"\f56d"}.fa-file-excel:before{content:"\f1c3"}.fa-file-export:before{content:"\f56e"}.fa-file-image:before{content:"\f1c5"}.fa-file-import:before{content:"\f56f"}.fa-file-invoice:before{content:"\f570"}.fa-file-invoice-dollar:before{content:"\f571"}.fa-file-medical:before{content:"\f477"}.fa-file-medical-alt:before{content:"\f478"}.fa-file-pdf:before{content:"\f1c1"}.fa-file-powerpoint:before{content:"\f1c4"}.fa-file-prescription:before{content:"\f572"}.fa-file-signature:before{content:"\f573"}.fa-file-upload:before{content:"\f574"}.fa-file-video:before{content:"\f1c8"}.fa-file-word:before{content:"\f1c2"}.fa-fill:before{content:"\f575"}.fa-fill-drip:before{content:"\f576"}.fa-film:before{content:"\f008"}.fa-filter:before{content:"\f0b0"}.fa-fingerprint:before{content:"\f577"}.fa-fire:before{content:"\f06d"}.fa-fire-alt:before{content:"\f7e4"}.fa-fire-extinguisher:before{content:"\f134"}.fa-firefox:before{content:"\f269"}.fa-firefox-browser:before{content:"\e007"}.fa-first-aid:before{content:"\f479"}.fa-first-order:before{content:"\f2b0"}.fa-first-order-alt:before{content:"\f50a"}.fa-firstdraft:before{content:"\f3a1"}.fa-fish:before{content:"\f578"}.fa-fist-raised:before{content:"\f6de"}.fa-flag:before{content:"\f024"}.fa-flag-checkered:before{content:"\f11e"}.fa-flag-usa:before{content:"\f74d"}.fa-flask:before{content:"\f0c3"}.fa-flickr:before{content:"\f16e"}.fa-flipboard:before{content:"\f44d"}.fa-flushed:before{content:"\f579"}.fa-fly:before{content:"\f417"}.fa-folder:before{content:"\f07b"}.fa-folder-minus:before{content:"\f65d"}.fa-folder-open:before{content:"\f07c"}.fa-folder-plus:before{content:"\f65e"}.fa-font:before{content:"\f031"}.fa-font-awesome:before{content:"\f2b4"}.fa-font-awesome-alt:before{content:"\f35c"}.fa-font-awesome-flag:before{content:"\f425"}.fa-font-awesome-logo-full:before{content:"\f4e6"}.fa-fonticons:before{content:"\f280"}.fa-fonticons-fi:before{content:"\f3a2"}.fa-football-ball:before{content:"\f44e"}.fa-fort-awesome:before{content:"\f286"}.fa-fort-awesome-alt:before{content:"\f3a3"}.fa-forumbee:before{content:"\f211"}.fa-forward:before{content:"\f04e"}.fa-foursquare:before{content:"\f180"}.fa-free-code-camp:before{content:"\f2c5"}.fa-freebsd:before{content:"\f3a4"}.fa-frog:before{content:"\f52e"}.fa-frown:before{content:"\f119"}.fa-frown-open:before{content:"\f57a"}.fa-fulcrum:before{content:"\f50b"}.fa-funnel-dollar:before{content:"\f662"}.fa-futbol:before{content:"\f1e3"}.fa-galactic-republic:before{content:"\f50c"}.fa-galactic-senate:before{content:"\f50d"}.fa-gamepad:before{content:"\f11b"}.fa-gas-pump:before{content:"\f52f"}.fa-gavel:before{content:"\f0e3"}.fa-gem:before{content:"\f3a5"}.fa-genderless:before{content:"\f22d"}.fa-get-pocket:before{content:"\f265"}.fa-gg:before{content:"\f260"}.fa-gg-circle:before{content:"\f261"}.fa-ghost:before{content:"\f6e2"}.fa-gift:before{content:"\f06b"}.fa-gifts:before{content:"\f79c"}.fa-git:before{content:"\f1d3"}.fa-git-alt:before{content:"\f841"}.fa-git-square:before{content:"\f1d2"}.fa-github:before{content:"\f09b"}.fa-github-alt:before{content:"\f113"}.fa-github-square:before{content:"\f092"}.fa-gitkraken:before{content:"\f3a6"}.fa-gitlab:before{content:"\f296"}.fa-gitter:before{content:"\f426"}.fa-glass-cheers:before{content:"\f79f"}.fa-glass-martini:before{content:"\f000"}.fa-glass-martini-alt:before{content:"\f57b"}.fa-glass-whiskey:before{content:"\f7a0"}.fa-glasses:before{content:"\f530"}.fa-glide:before{content:"\f2a5"}.fa-glide-g:before{content:"\f2a6"}.fa-globe:before{content:"\f0ac"}.fa-globe-africa:before{content:"\f57c"}.fa-globe-americas:before{content:"\f57d"}.fa-globe-asia:before{content:"\f57e"}.fa-globe-europe:before{content:"\f7a2"}.fa-gofore:before{content:"\f3a7"}.fa-golf-ball:before{content:"\f450"}.fa-goodreads:before{content:"\f3a8"}.fa-goodreads-g:before{content:"\f3a9"}.fa-google:before{content:"\f1a0"}.fa-google-drive:before{content:"\f3aa"}.fa-google-pay:before{content:"\e079"}.fa-google-play:before{content:"\f3ab"}.fa-google-plus:before{content:"\f2b3"}.fa-google-plus-g:before{content:"\f0d5"}.fa-google-plus-square:before{content:"\f0d4"}.fa-google-wallet:before{content:"\f1ee"}.fa-gopuram:before{content:"\f664"}.fa-graduation-cap:before{content:"\f19d"}.fa-gratipay:before{content:"\f184"}.fa-grav:before{content:"\f2d6"}.fa-greater-than:before{content:"\f531"}.fa-greater-than-equal:before{content:"\f532"}.fa-grimace:before{content:"\f57f"}.fa-grin:before{content:"\f580"}.fa-grin-alt:before{content:"\f581"}.fa-grin-beam:before{content:"\f582"}.fa-grin-beam-sweat:before{content:"\f583"}.fa-grin-hearts:before{content:"\f584"}.fa-grin-squint:before{content:"\f585"}.fa-grin-squint-tears:before{content:"\f586"}.fa-grin-stars:before{content:"\f587"}.fa-grin-tears:before{content:"\f588"}.fa-grin-tongue:before{content:"\f589"}.fa-grin-tongue-squint:before{content:"\f58a"}.fa-grin-tongue-wink:before{content:"\f58b"}.fa-grin-wink:before{content:"\f58c"}.fa-grip-horizontal:before{content:"\f58d"}.fa-grip-lines:before{content:"\f7a4"}.fa-grip-lines-vertical:before{content:"\f7a5"}.fa-grip-vertical:before{content:"\f58e"}.fa-gripfire:before{content:"\f3ac"}.fa-grunt:before{content:"\f3ad"}.fa-guilded:before{content:"\e07e"}.fa-guitar:before{content:"\f7a6"}.fa-gulp:before{content:"\f3ae"}.fa-h-square:before{content:"\f0fd"}.fa-hacker-news:before{content:"\f1d4"}.fa-hacker-news-square:before{content:"\f3af"}.fa-hackerrank:before{content:"\f5f7"}.fa-hamburger:before{content:"\f805"}.fa-hammer:before{content:"\f6e3"}.fa-hamsa:before{content:"\f665"}.fa-hand-holding:before{content:"\f4bd"}.fa-hand-holding-heart:before{content:"\f4be"}.fa-hand-holding-medical:before{content:"\e05c"}.fa-hand-holding-usd:before{content:"\f4c0"}.fa-hand-holding-water:before{content:"\f4c1"}.fa-hand-lizard:before{content:"\f258"}.fa-hand-middle-finger:before{content:"\f806"}.fa-hand-paper:before{content:"\f256"}.fa-hand-peace:before{content:"\f25b"}.fa-hand-point-down:before{content:"\f0a7"}.fa-hand-point-left:before{content:"\f0a5"}.fa-hand-point-right:before{content:"\f0a4"}.fa-hand-point-up:before{content:"\f0a6"}.fa-hand-pointer:before{content:"\f25a"}.fa-hand-rock:before{content:"\f255"}.fa-hand-scissors:before{content:"\f257"}.fa-hand-sparkles:before{content:"\e05d"}.fa-hand-spock:before{content:"\f259"}.fa-hands:before{content:"\f4c2"}.fa-hands-helping:before{content:"\f4c4"}.fa-hands-wash:before{content:"\e05e"}.fa-handshake:before{content:"\f2b5"}.fa-handshake-alt-slash:before{content:"\e05f"}.fa-handshake-slash:before{content:"\e060"}.fa-hanukiah:before{content:"\f6e6"}.fa-hard-hat:before{content:"\f807"}.fa-hashtag:before{content:"\f292"}.fa-hat-cowboy:before{content:"\f8c0"}.fa-hat-cowboy-side:before{content:"\f8c1"}.fa-hat-wizard:before{content:"\f6e8"}.fa-hdd:before{content:"\f0a0"}.fa-head-side-cough:before{content:"\e061"}.fa-head-side-cough-slash:before{content:"\e062"}.fa-head-side-mask:before{content:"\e063"}.fa-head-side-virus:before{content:"\e064"}.fa-heading:before{content:"\f1dc"}.fa-headphones:before{content:"\f025"}.fa-headphones-alt:before{content:"\f58f"}.fa-headset:before{content:"\f590"}.fa-heart:before{content:"\f004"}.fa-heart-broken:before{content:"\f7a9"}.fa-heartbeat:before{content:"\f21e"}.fa-helicopter:before{content:"\f533"}.fa-highlighter:before{content:"\f591"}.fa-hiking:before{content:"\f6ec"}.fa-hippo:before{content:"\f6ed"}.fa-hips:before{content:"\f452"}.fa-hire-a-helper:before{content:"\f3b0"}.fa-history:before{content:"\f1da"}.fa-hive:before{content:"\e07f"}.fa-hockey-puck:before{content:"\f453"}.fa-holly-berry:before{content:"\f7aa"}.fa-home:before{content:"\f015"}.fa-hooli:before{content:"\f427"}.fa-hornbill:before{content:"\f592"}.fa-horse:before{content:"\f6f0"}.fa-horse-head:before{content:"\f7ab"}.fa-hospital:before{content:"\f0f8"}.fa-hospital-alt:before{content:"\f47d"}.fa-hospital-symbol:before{content:"\f47e"}.fa-hospital-user:before{content:"\f80d"}.fa-hot-tub:before{content:"\f593"}.fa-hotdog:before{content:"\f80f"}.fa-hotel:before{content:"\f594"}.fa-hotjar:before{content:"\f3b1"}.fa-hourglass:before{content:"\f254"}.fa-hourglass-end:before{content:"\f253"}.fa-hourglass-half:before{content:"\f252"}.fa-hourglass-start:before{content:"\f251"}.fa-house-damage:before{content:"\f6f1"}.fa-house-user:before{content:"\e065"}.fa-houzz:before{content:"\f27c"}.fa-hryvnia:before{content:"\f6f2"}.fa-html5:before{content:"\f13b"}.fa-hubspot:before{content:"\f3b2"}.fa-i-cursor:before{content:"\f246"}.fa-ice-cream:before{content:"\f810"}.fa-icicles:before{content:"\f7ad"}.fa-icons:before{content:"\f86d"}.fa-id-badge:before{content:"\f2c1"}.fa-id-card:before{content:"\f2c2"}.fa-id-card-alt:before{content:"\f47f"}.fa-ideal:before{content:"\e013"}.fa-igloo:before{content:"\f7ae"}.fa-image:before{content:"\f03e"}.fa-images:before{content:"\f302"}.fa-imdb:before{content:"\f2d8"}.fa-inbox:before{content:"\f01c"}.fa-indent:before{content:"\f03c"}.fa-industry:before{content:"\f275"}.fa-infinity:before{content:"\f534"}.fa-info:before{content:"\f129"}.fa-info-circle:before{content:"\f05a"}.fa-innosoft:before{content:"\e080"}.fa-instagram:before{content:"\f16d"}.fa-instagram-square:before{content:"\e055"}.fa-instalod:before{content:"\e081"}.fa-intercom:before{content:"\f7af"}.fa-internet-explorer:before{content:"\f26b"}.fa-invision:before{content:"\f7b0"}.fa-ioxhost:before{content:"\f208"}.fa-italic:before{content:"\f033"}.fa-itch-io:before{content:"\f83a"}.fa-itunes:before{content:"\f3b4"}.fa-itunes-note:before{content:"\f3b5"}.fa-java:before{content:"\f4e4"}.fa-jedi:before{content:"\f669"}.fa-jedi-order:before{content:"\f50e"}.fa-jenkins:before{content:"\f3b6"}.fa-jira:before{content:"\f7b1"}.fa-joget:before{content:"\f3b7"}.fa-joint:before{content:"\f595"}.fa-joomla:before{content:"\f1aa"}.fa-journal-whills:before{content:"\f66a"}.fa-js:before{content:"\f3b8"}.fa-js-square:before{content:"\f3b9"}.fa-jsfiddle:before{content:"\f1cc"}.fa-kaaba:before{content:"\f66b"}.fa-kaggle:before{content:"\f5fa"}.fa-key:before{content:"\f084"}.fa-keybase:before{content:"\f4f5"}.fa-keyboard:before{content:"\f11c"}.fa-keycdn:before{content:"\f3ba"}.fa-khanda:before{content:"\f66d"}.fa-kickstarter:before{content:"\f3bb"}.fa-kickstarter-k:before{content:"\f3bc"}.fa-kiss:before{content:"\f596"}.fa-kiss-beam:before{content:"\f597"}.fa-kiss-wink-heart:before{content:"\f598"}.fa-kiwi-bird:before{content:"\f535"}.fa-korvue:before{content:"\f42f"}.fa-landmark:before{content:"\f66f"}.fa-language:before{content:"\f1ab"}.fa-laptop:before{content:"\f109"}.fa-laptop-code:before{content:"\f5fc"}.fa-laptop-house:before{content:"\e066"}.fa-laptop-medical:before{content:"\f812"}.fa-laravel:before{content:"\f3bd"}.fa-lastfm:before{content:"\f202"}.fa-lastfm-square:before{content:"\f203"}.fa-laugh:before{content:"\f599"}.fa-laugh-beam:before{content:"\f59a"}.fa-laugh-squint:before{content:"\f59b"}.fa-laugh-wink:before{content:"\f59c"}.fa-layer-group:before{content:"\f5fd"}.fa-leaf:before{content:"\f06c"}.fa-leanpub:before{content:"\f212"}.fa-lemon:before{content:"\f094"}.fa-less:before{content:"\f41d"}.fa-less-than:before{content:"\f536"}.fa-less-than-equal:before{content:"\f537"}.fa-level-down-alt:before{content:"\f3be"}.fa-level-up-alt:before{content:"\f3bf"}.fa-life-ring:before{content:"\f1cd"}.fa-lightbulb:before{content:"\f0eb"}.fa-line:before{content:"\f3c0"}.fa-link:before{content:"\f0c1"}.fa-linkedin:before{content:"\f08c"}.fa-linkedin-in:before{content:"\f0e1"}.fa-linode:before{content:"\f2b8"}.fa-linux:before{content:"\f17c"}.fa-lira-sign:before{content:"\f195"}.fa-list:before{content:"\f03a"}.fa-list-alt:before{content:"\f022"}.fa-list-ol:before{content:"\f0cb"}.fa-list-ul:before{content:"\f0ca"}.fa-location-arrow:before{content:"\f124"}.fa-lock:before{content:"\f023"}.fa-lock-open:before{content:"\f3c1"}.fa-long-arrow-alt-down:before{content:"\f309"}.fa-long-arrow-alt-left:before{content:"\f30a"}.fa-long-arrow-alt-right:before{content:"\f30b"}.fa-long-arrow-alt-up:before{content:"\f30c"}.fa-low-vision:before{content:"\f2a8"}.fa-luggage-cart:before{content:"\f59d"}.fa-lungs:before{content:"\f604"}.fa-lungs-virus:before{content:"\e067"}.fa-lyft:before{content:"\f3c3"}.fa-magento:before{content:"\f3c4"}.fa-magic:before{content:"\f0d0"}.fa-magnet:before{content:"\f076"}.fa-mail-bulk:before{content:"\f674"}.fa-mailchimp:before{content:"\f59e"}.fa-male:before{content:"\f183"}.fa-mandalorian:before{content:"\f50f"}.fa-map:before{content:"\f279"}.fa-map-marked:before{content:"\f59f"}.fa-map-marked-alt:before{content:"\f5a0"}.fa-map-marker:before{content:"\f041"}.fa-map-marker-alt:before{content:"\f3c5"}.fa-map-pin:before{content:"\f276"}.fa-map-signs:before{content:"\f277"}.fa-markdown:before{content:"\f60f"}.fa-marker:before{content:"\f5a1"}.fa-mars:before{content:"\f222"}.fa-mars-double:before{content:"\f227"}.fa-mars-stroke:before{content:"\f229"}.fa-mars-stroke-h:before{content:"\f22b"}.fa-mars-stroke-v:before{content:"\f22a"}.fa-mask:before{content:"\f6fa"}.fa-mastodon:before{content:"\f4f6"}.fa-maxcdn:before{content:"\f136"}.fa-mdb:before{content:"\f8ca"}.fa-medal:before{content:"\f5a2"}.fa-medapps:before{content:"\f3c6"}.fa-medium:before{content:"\f23a"}.fa-medium-m:before{content:"\f3c7"}.fa-medkit:before{content:"\f0fa"}.fa-medrt:before{content:"\f3c8"}.fa-meetup:before{content:"\f2e0"}.fa-megaport:before{content:"\f5a3"}.fa-meh:before{content:"\f11a"}.fa-meh-blank:before{content:"\f5a4"}.fa-meh-rolling-eyes:before{content:"\f5a5"}.fa-memory:before{content:"\f538"}.fa-mendeley:before{content:"\f7b3"}.fa-menorah:before{content:"\f676"}.fa-mercury:before{content:"\f223"}.fa-meteor:before{content:"\f753"}.fa-microblog:before{content:"\e01a"}.fa-microchip:before{content:"\f2db"}.fa-microphone:before{content:"\f130"}.fa-microphone-alt:before{content:"\f3c9"}.fa-microphone-alt-slash:before{content:"\f539"}.fa-microphone-slash:before{content:"\f131"}.fa-microscope:before{content:"\f610"}.fa-microsoft:before{content:"\f3ca"}.fa-minus:before{content:"\f068"}.fa-minus-circle:before{content:"\f056"}.fa-minus-square:before{content:"\f146"}.fa-mitten:before{content:"\f7b5"}.fa-mix:before{content:"\f3cb"}.fa-mixcloud:before{content:"\f289"}.fa-mixer:before{content:"\e056"}.fa-mizuni:before{content:"\f3cc"}.fa-mobile:before{content:"\f10b"}.fa-mobile-alt:before{content:"\f3cd"}.fa-modx:before{content:"\f285"}.fa-monero:before{content:"\f3d0"}.fa-money-bill:before{content:"\f0d6"}.fa-money-bill-alt:before{content:"\f3d1"}.fa-money-bill-wave:before{content:"\f53a"}.fa-money-bill-wave-alt:before{content:"\f53b"}.fa-money-check:before{content:"\f53c"}.fa-money-check-alt:before{content:"\f53d"}.fa-monument:before{content:"\f5a6"}.fa-moon:before{content:"\f186"}.fa-mortar-pestle:before{content:"\f5a7"}.fa-mosque:before{content:"\f678"}.fa-motorcycle:before{content:"\f21c"}.fa-mountain:before{content:"\f6fc"}.fa-mouse:before{content:"\f8cc"}.fa-mouse-pointer:before{content:"\f245"}.fa-mug-hot:before{content:"\f7b6"}.fa-music:before{content:"\f001"}.fa-napster:before{content:"\f3d2"}.fa-neos:before{content:"\f612"}.fa-network-wired:before{content:"\f6ff"}.fa-neuter:before{content:"\f22c"}.fa-newspaper:before{content:"\f1ea"}.fa-nimblr:before{content:"\f5a8"}.fa-node:before{content:"\f419"}.fa-node-js:before{content:"\f3d3"}.fa-not-equal:before{content:"\f53e"}.fa-notes-medical:before{content:"\f481"}.fa-npm:before{content:"\f3d4"}.fa-ns8:before{content:"\f3d5"}.fa-nutritionix:before{content:"\f3d6"}.fa-object-group:before{content:"\f247"}.fa-object-ungroup:before{content:"\f248"}.fa-octopus-deploy:before{content:"\e082"}.fa-odnoklassniki:before{content:"\f263"}.fa-odnoklassniki-square:before{content:"\f264"}.fa-oil-can:before{content:"\f613"}.fa-old-republic:before{content:"\f510"}.fa-om:before{content:"\f679"}.fa-opencart:before{content:"\f23d"}.fa-openid:before{content:"\f19b"}.fa-opera:before{content:"\f26a"}.fa-optin-monster:before{content:"\f23c"}.fa-orcid:before{content:"\f8d2"}.fa-osi:before{content:"\f41a"}.fa-otter:before{content:"\f700"}.fa-outdent:before{content:"\f03b"}.fa-page4:before{content:"\f3d7"}.fa-pagelines:before{content:"\f18c"}.fa-pager:before{content:"\f815"}.fa-paint-brush:before{content:"\f1fc"}.fa-paint-roller:before{content:"\f5aa"}.fa-palette:before{content:"\f53f"}.fa-palfed:before{content:"\f3d8"}.fa-pallet:before{content:"\f482"}.fa-paper-plane:before{content:"\f1d8"}.fa-paperclip:before{content:"\f0c6"}.fa-parachute-box:before{content:"\f4cd"}.fa-paragraph:before{content:"\f1dd"}.fa-parking:before{content:"\f540"}.fa-passport:before{content:"\f5ab"}.fa-pastafarianism:before{content:"\f67b"}.fa-paste:before{content:"\f0ea"}.fa-patreon:before{content:"\f3d9"}.fa-pause:before{content:"\f04c"}.fa-pause-circle:before{content:"\f28b"}.fa-paw:before{content:"\f1b0"}.fa-paypal:before{content:"\f1ed"}.fa-peace:before{content:"\f67c"}.fa-pen:before{content:"\f304"}.fa-pen-alt:before{content:"\f305"}.fa-pen-fancy:before{content:"\f5ac"}.fa-pen-nib:before{content:"\f5ad"}.fa-pen-square:before{content:"\f14b"}.fa-pencil-alt:before{content:"\f303"}.fa-pencil-ruler:before{content:"\f5ae"}.fa-penny-arcade:before{content:"\f704"}.fa-people-arrows:before{content:"\e068"}.fa-people-carry:before{content:"\f4ce"}.fa-pepper-hot:before{content:"\f816"}.fa-perbyte:before{content:"\e083"}.fa-percent:before{content:"\f295"}.fa-percentage:before{content:"\f541"}.fa-periscope:before{content:"\f3da"}.fa-person-booth:before{content:"\f756"}.fa-phabricator:before{content:"\f3db"}.fa-phoenix-framework:before{content:"\f3dc"}.fa-phoenix-squadron:before{content:"\f511"}.fa-phone:before{content:"\f095"}.fa-phone-alt:before{content:"\f879"}.fa-phone-slash:before{content:"\f3dd"}.fa-phone-square:before{content:"\f098"}.fa-phone-square-alt:before{content:"\f87b"}.fa-phone-volume:before{content:"\f2a0"}.fa-photo-video:before{content:"\f87c"}.fa-php:before{content:"\f457"}.fa-pied-piper:before{content:"\f2ae"}.fa-pied-piper-alt:before{content:"\f1a8"}.fa-pied-piper-hat:before{content:"\f4e5"}.fa-pied-piper-pp:before{content:"\f1a7"}.fa-pied-piper-square:before{content:"\e01e"}.fa-piggy-bank:before{content:"\f4d3"}.fa-pills:before{content:"\f484"}.fa-pinterest:before{content:"\f0d2"}.fa-pinterest-p:before{content:"\f231"}.fa-pinterest-square:before{content:"\f0d3"}.fa-pizza-slice:before{content:"\f818"}.fa-place-of-worship:before{content:"\f67f"}.fa-plane:before{content:"\f072"}.fa-plane-arrival:before{content:"\f5af"}.fa-plane-departure:before{content:"\f5b0"}.fa-plane-slash:before{content:"\e069"}.fa-play:before{content:"\f04b"}.fa-play-circle:before{content:"\f144"}.fa-playstation:before{content:"\f3df"}.fa-plug:before{content:"\f1e6"}.fa-plus:before{content:"\f067"}.fa-plus-circle:before{content:"\f055"}.fa-plus-square:before{content:"\f0fe"}.fa-podcast:before{content:"\f2ce"}.fa-poll:before{content:"\f681"}.fa-poll-h:before{content:"\f682"}.fa-poo:before{content:"\f2fe"}.fa-poo-storm:before{content:"\f75a"}.fa-poop:before{content:"\f619"}.fa-portrait:before{content:"\f3e0"}.fa-pound-sign:before{content:"\f154"}.fa-power-off:before{content:"\f011"}.fa-pray:before{content:"\f683"}.fa-praying-hands:before{content:"\f684"}.fa-prescription:before{content:"\f5b1"}.fa-prescription-bottle:before{content:"\f485"}.fa-prescription-bottle-alt:before{content:"\f486"}.fa-print:before{content:"\f02f"}.fa-procedures:before{content:"\f487"}.fa-product-hunt:before{content:"\f288"}.fa-project-diagram:before{content:"\f542"}.fa-pump-medical:before{content:"\e06a"}.fa-pump-soap:before{content:"\e06b"}.fa-pushed:before{content:"\f3e1"}.fa-puzzle-piece:before{content:"\f12e"}.fa-python:before{content:"\f3e2"}.fa-qq:before{content:"\f1d6"}.fa-qrcode:before{content:"\f029"}.fa-question:before{content:"\f128"}.fa-question-circle:before{content:"\f059"}.fa-quidditch:before{content:"\f458"}.fa-quinscape:before{content:"\f459"}.fa-quora:before{content:"\f2c4"}.fa-quote-left:before{content:"\f10d"}.fa-quote-right:before{content:"\f10e"}.fa-quran:before{content:"\f687"}.fa-r-project:before{content:"\f4f7"}.fa-radiation:before{content:"\f7b9"}.fa-radiation-alt:before{content:"\f7ba"}.fa-rainbow:before{content:"\f75b"}.fa-random:before{content:"\f074"}.fa-raspberry-pi:before{content:"\f7bb"}.fa-ravelry:before{content:"\f2d9"}.fa-react:before{content:"\f41b"}.fa-reacteurope:before{content:"\f75d"}.fa-readme:before{content:"\f4d5"}.fa-rebel:before{content:"\f1d0"}.fa-receipt:before{content:"\f543"}.fa-record-vinyl:before{content:"\f8d9"}.fa-recycle:before{content:"\f1b8"}.fa-red-river:before{content:"\f3e3"}.fa-reddit:before{content:"\f1a1"}.fa-reddit-alien:before{content:"\f281"}.fa-reddit-square:before{content:"\f1a2"}.fa-redhat:before{content:"\f7bc"}.fa-redo:before{content:"\f01e"}.fa-redo-alt:before{content:"\f2f9"}.fa-registered:before{content:"\f25d"}.fa-remove-format:before{content:"\f87d"}.fa-renren:before{content:"\f18b"}.fa-reply:before{content:"\f3e5"}.fa-reply-all:before{content:"\f122"}.fa-replyd:before{content:"\f3e6"}.fa-republican:before{content:"\f75e"}.fa-researchgate:before{content:"\f4f8"}.fa-resolving:before{content:"\f3e7"}.fa-restroom:before{content:"\f7bd"}.fa-retweet:before{content:"\f079"}.fa-rev:before{content:"\f5b2"}.fa-ribbon:before{content:"\f4d6"}.fa-ring:before{content:"\f70b"}.fa-road:before{content:"\f018"}.fa-robot:before{content:"\f544"}.fa-rocket:before{content:"\f135"}.fa-rocketchat:before{content:"\f3e8"}.fa-rockrms:before{content:"\f3e9"}.fa-route:before{content:"\f4d7"}.fa-rss:before{content:"\f09e"}.fa-rss-square:before{content:"\f143"}.fa-ruble-sign:before{content:"\f158"}.fa-ruler:before{content:"\f545"}.fa-ruler-combined:before{content:"\f546"}.fa-ruler-horizontal:before{content:"\f547"}.fa-ruler-vertical:before{content:"\f548"}.fa-running:before{content:"\f70c"}.fa-rupee-sign:before{content:"\f156"}.fa-rust:before{content:"\e07a"}.fa-sad-cry:before{content:"\f5b3"}.fa-sad-tear:before{content:"\f5b4"}.fa-safari:before{content:"\f267"}.fa-salesforce:before{content:"\f83b"}.fa-sass:before{content:"\f41e"}.fa-satellite:before{content:"\f7bf"}.fa-satellite-dish:before{content:"\f7c0"}.fa-save:before{content:"\f0c7"}.fa-schlix:before{content:"\f3ea"}.fa-school:before{content:"\f549"}.fa-screwdriver:before{content:"\f54a"}.fa-scribd:before{content:"\f28a"}.fa-scroll:before{content:"\f70e"}.fa-sd-card:before{content:"\f7c2"}.fa-search:before{content:"\f002"}.fa-search-dollar:before{content:"\f688"}.fa-search-location:before{content:"\f689"}.fa-search-minus:before{content:"\f010"}.fa-search-plus:before{content:"\f00e"}.fa-searchengin:before{content:"\f3eb"}.fa-seedling:before{content:"\f4d8"}.fa-sellcast:before{content:"\f2da"}.fa-sellsy:before{content:"\f213"}.fa-server:before{content:"\f233"}.fa-servicestack:before{content:"\f3ec"}.fa-shapes:before{content:"\f61f"}.fa-share:before{content:"\f064"}.fa-share-alt:before{content:"\f1e0"}.fa-share-alt-square:before{content:"\f1e1"}.fa-share-square:before{content:"\f14d"}.fa-shekel-sign:before{content:"\f20b"}.fa-shield-alt:before{content:"\f3ed"}.fa-shield-virus:before{content:"\e06c"}.fa-ship:before{content:"\f21a"}.fa-shipping-fast:before{content:"\f48b"}.fa-shirtsinbulk:before{content:"\f214"}.fa-shoe-prints:before{content:"\f54b"}.fa-shopify:before{content:"\e057"}.fa-shopping-bag:before{content:"\f290"}.fa-shopping-basket:before{content:"\f291"}.fa-shopping-cart:before{content:"\f07a"}.fa-shopware:before{content:"\f5b5"}.fa-shower:before{content:"\f2cc"}.fa-shuttle-van:before{content:"\f5b6"}.fa-sign:before{content:"\f4d9"}.fa-sign-in-alt:before{content:"\f2f6"}.fa-sign-language:before{content:"\f2a7"}.fa-sign-out-alt:before{content:"\f2f5"}.fa-signal:before{content:"\f012"}.fa-signature:before{content:"\f5b7"}.fa-sim-card:before{content:"\f7c4"}.fa-simplybuilt:before{content:"\f215"}.fa-sink:before{content:"\e06d"}.fa-sistrix:before{content:"\f3ee"}.fa-sitemap:before{content:"\f0e8"}.fa-sith:before{content:"\f512"}.fa-skating:before{content:"\f7c5"}.fa-sketch:before{content:"\f7c6"}.fa-skiing:before{content:"\f7c9"}.fa-skiing-nordic:before{content:"\f7ca"}.fa-skull:before{content:"\f54c"}.fa-skull-crossbones:before{content:"\f714"}.fa-skyatlas:before{content:"\f216"}.fa-skype:before{content:"\f17e"}.fa-slack:before{content:"\f198"}.fa-slack-hash:before{content:"\f3ef"}.fa-slash:before{content:"\f715"}.fa-sleigh:before{content:"\f7cc"}.fa-sliders-h:before{content:"\f1de"}.fa-slideshare:before{content:"\f1e7"}.fa-smile:before{content:"\f118"}.fa-smile-beam:before{content:"\f5b8"}.fa-smile-wink:before{content:"\f4da"}.fa-smog:before{content:"\f75f"}.fa-smoking:before{content:"\f48d"}.fa-smoking-ban:before{content:"\f54d"}.fa-sms:before{content:"\f7cd"}.fa-snapchat:before{content:"\f2ab"}.fa-snapchat-ghost:before{content:"\f2ac"}.fa-snapchat-square:before{content:"\f2ad"}.fa-snowboarding:before{content:"\f7ce"}.fa-snowflake:before{content:"\f2dc"}.fa-snowman:before{content:"\f7d0"}.fa-snowplow:before{content:"\f7d2"}.fa-soap:before{content:"\e06e"}.fa-socks:before{content:"\f696"}.fa-solar-panel:before{content:"\f5ba"}.fa-sort:before{content:"\f0dc"}.fa-sort-alpha-down:before{content:"\f15d"}.fa-sort-alpha-down-alt:before{content:"\f881"}.fa-sort-alpha-up:before{content:"\f15e"}.fa-sort-alpha-up-alt:before{content:"\f882"}.fa-sort-amount-down:before{content:"\f160"}.fa-sort-amount-down-alt:before{content:"\f884"}.fa-sort-amount-up:before{content:"\f161"}.fa-sort-amount-up-alt:before{content:"\f885"}.fa-sort-down:before{content:"\f0dd"}.fa-sort-numeric-down:before{content:"\f162"}.fa-sort-numeric-down-alt:before{content:"\f886"}.fa-sort-numeric-up:before{content:"\f163"}.fa-sort-numeric-up-alt:before{content:"\f887"}.fa-sort-up:before{content:"\f0de"}.fa-soundcloud:before{content:"\f1be"}.fa-sourcetree:before{content:"\f7d3"}.fa-spa:before{content:"\f5bb"}.fa-space-shuttle:before{content:"\f197"}.fa-speakap:before{content:"\f3f3"}.fa-speaker-deck:before{content:"\f83c"}.fa-spell-check:before{content:"\f891"}.fa-spider:before{content:"\f717"}.fa-spinner:before{content:"\f110"}.fa-splotch:before{content:"\f5bc"}.fa-spotify:before{content:"\f1bc"}.fa-spray-can:before{content:"\f5bd"}.fa-square:before{content:"\f0c8"}.fa-square-full:before{content:"\f45c"}.fa-square-root-alt:before{content:"\f698"}.fa-squarespace:before{content:"\f5be"}.fa-stack-exchange:before{content:"\f18d"}.fa-stack-overflow:before{content:"\f16c"}.fa-stackpath:before{content:"\f842"}.fa-stamp:before{content:"\f5bf"}.fa-star:before{content:"\f005"}.fa-star-and-crescent:before{content:"\f699"}.fa-star-half:before{content:"\f089"}.fa-star-half-alt:before{content:"\f5c0"}.fa-star-of-david:before{content:"\f69a"}.fa-star-of-life:before{content:"\f621"}.fa-staylinked:before{content:"\f3f5"}.fa-steam:before{content:"\f1b6"}.fa-steam-square:before{content:"\f1b7"}.fa-steam-symbol:before{content:"\f3f6"}.fa-step-backward:before{content:"\f048"}.fa-step-forward:before{content:"\f051"}.fa-stethoscope:before{content:"\f0f1"}.fa-sticker-mule:before{content:"\f3f7"}.fa-sticky-note:before{content:"\f249"}.fa-stop:before{content:"\f04d"}.fa-stop-circle:before{content:"\f28d"}.fa-stopwatch:before{content:"\f2f2"}.fa-stopwatch-20:before{content:"\e06f"}.fa-store:before{content:"\f54e"}.fa-store-alt:before{content:"\f54f"}.fa-store-alt-slash:before{content:"\e070"}.fa-store-slash:before{content:"\e071"}.fa-strava:before{content:"\f428"}.fa-stream:before{content:"\f550"}.fa-street-view:before{content:"\f21d"}.fa-strikethrough:before{content:"\f0cc"}.fa-stripe:before{content:"\f429"}.fa-stripe-s:before{content:"\f42a"}.fa-stroopwafel:before{content:"\f551"}.fa-studiovinari:before{content:"\f3f8"}.fa-stumbleupon:before{content:"\f1a4"}.fa-stumbleupon-circle:before{content:"\f1a3"}.fa-subscript:before{content:"\f12c"}.fa-subway:before{content:"\f239"}.fa-suitcase:before{content:"\f0f2"}.fa-suitcase-rolling:before{content:"\f5c1"}.fa-sun:before{content:"\f185"}.fa-superpowers:before{content:"\f2dd"}.fa-superscript:before{content:"\f12b"}.fa-supple:before{content:"\f3f9"}.fa-surprise:before{content:"\f5c2"}.fa-suse:before{content:"\f7d6"}.fa-swatchbook:before{content:"\f5c3"}.fa-swift:before{content:"\f8e1"}.fa-swimmer:before{content:"\f5c4"}.fa-swimming-pool:before{content:"\f5c5"}.fa-symfony:before{content:"\f83d"}.fa-synagogue:before{content:"\f69b"}.fa-sync:before{content:"\f021"}.fa-sync-alt:before{content:"\f2f1"}.fa-syringe:before{content:"\f48e"}.fa-table:before{content:"\f0ce"}.fa-table-tennis:before{content:"\f45d"}.fa-tablet:before{content:"\f10a"}.fa-tablet-alt:before{content:"\f3fa"}.fa-tablets:before{content:"\f490"}.fa-tachometer-alt:before{content:"\f3fd"}.fa-tag:before{content:"\f02b"}.fa-tags:before{content:"\f02c"}.fa-tape:before{content:"\f4db"}.fa-tasks:before{content:"\f0ae"}.fa-taxi:before{content:"\f1ba"}.fa-teamspeak:before{content:"\f4f9"}.fa-teeth:before{content:"\f62e"}.fa-teeth-open:before{content:"\f62f"}.fa-telegram:before{content:"\f2c6"}.fa-telegram-plane:before{content:"\f3fe"}.fa-temperature-high:before{content:"\f769"}.fa-temperature-low:before{content:"\f76b"}.fa-tencent-weibo:before{content:"\f1d5"}.fa-tenge:before{content:"\f7d7"}.fa-terminal:before{content:"\f120"}.fa-text-height:before{content:"\f034"}.fa-text-width:before{content:"\f035"}.fa-th:before{content:"\f00a"}.fa-th-large:before{content:"\f009"}.fa-th-list:before{content:"\f00b"}.fa-the-red-yeti:before{content:"\f69d"}.fa-theater-masks:before{content:"\f630"}.fa-themeco:before{content:"\f5c6"}.fa-themeisle:before{content:"\f2b2"}.fa-thermometer:before{content:"\f491"}.fa-thermometer-empty:before{content:"\f2cb"}.fa-thermometer-full:before{content:"\f2c7"}.fa-thermometer-half:before{content:"\f2c9"}.fa-thermometer-quarter:before{content:"\f2ca"}.fa-thermometer-three-quarters:before{content:"\f2c8"}.fa-think-peaks:before{content:"\f731"}.fa-thumbs-down:before{content:"\f165"}.fa-thumbs-up:before{content:"\f164"}.fa-thumbtack:before{content:"\f08d"}.fa-ticket-alt:before{content:"\f3ff"}.fa-tiktok:before{content:"\e07b"}.fa-times:before{content:"\f00d"}.fa-times-circle:before{content:"\f057"}.fa-tint:before{content:"\f043"}.fa-tint-slash:before{content:"\f5c7"}.fa-tired:before{content:"\f5c8"}.fa-toggle-off:before{content:"\f204"}.fa-toggle-on:before{content:"\f205"}.fa-toilet:before{content:"\f7d8"}.fa-toilet-paper:before{content:"\f71e"}.fa-toilet-paper-slash:before{content:"\e072"}.fa-toolbox:before{content:"\f552"}.fa-tools:before{content:"\f7d9"}.fa-tooth:before{content:"\f5c9"}.fa-torah:before{content:"\f6a0"}.fa-torii-gate:before{content:"\f6a1"}.fa-tractor:before{content:"\f722"}.fa-trade-federation:before{content:"\f513"}.fa-trademark:before{content:"\f25c"}.fa-traffic-light:before{content:"\f637"}.fa-trailer:before{content:"\e041"}.fa-train:before{content:"\f238"}.fa-tram:before{content:"\f7da"}.fa-transgender:before{content:"\f224"}.fa-transgender-alt:before{content:"\f225"}.fa-trash:before{content:"\f1f8"}.fa-trash-alt:before{content:"\f2ed"}.fa-trash-restore:before{content:"\f829"}.fa-trash-restore-alt:before{content:"\f82a"}.fa-tree:before{content:"\f1bb"}.fa-trello:before{content:"\f181"}.fa-trophy:before{content:"\f091"}.fa-truck:before{content:"\f0d1"}.fa-truck-loading:before{content:"\f4de"}.fa-truck-monster:before{content:"\f63b"}.fa-truck-moving:before{content:"\f4df"}.fa-truck-pickup:before{content:"\f63c"}.fa-tshirt:before{content:"\f553"}.fa-tty:before{content:"\f1e4"}.fa-tumblr:before{content:"\f173"}.fa-tumblr-square:before{content:"\f174"}.fa-tv:before{content:"\f26c"}.fa-twitch:before{content:"\f1e8"}.fa-twitter:before{content:"\f099"}.fa-twitter-square:before{content:"\f081"}.fa-typo3:before{content:"\f42b"}.fa-uber:before{content:"\f402"}.fa-ubuntu:before{content:"\f7df"}.fa-uikit:before{content:"\f403"}.fa-umbraco:before{content:"\f8e8"}.fa-umbrella:before{content:"\f0e9"}.fa-umbrella-beach:before{content:"\f5ca"}.fa-uncharted:before{content:"\e084"}.fa-underline:before{content:"\f0cd"}.fa-undo:before{content:"\f0e2"}.fa-undo-alt:before{content:"\f2ea"}.fa-uniregistry:before{content:"\f404"}.fa-unity:before{content:"\e049"}.fa-universal-access:before{content:"\f29a"}.fa-university:before{content:"\f19c"}.fa-unlink:before{content:"\f127"}.fa-unlock:before{content:"\f09c"}.fa-unlock-alt:before{content:"\f13e"}.fa-unsplash:before{content:"\e07c"}.fa-untappd:before{content:"\f405"}.fa-upload:before{content:"\f093"}.fa-ups:before{content:"\f7e0"}.fa-usb:before{content:"\f287"}.fa-user:before{content:"\f007"}.fa-user-alt:before{content:"\f406"}.fa-user-alt-slash:before{content:"\f4fa"}.fa-user-astronaut:before{content:"\f4fb"}.fa-user-check:before{content:"\f4fc"}.fa-user-circle:before{content:"\f2bd"}.fa-user-clock:before{content:"\f4fd"}.fa-user-cog:before{content:"\f4fe"}.fa-user-edit:before{content:"\f4ff"}.fa-user-friends:before{content:"\f500"}.fa-user-graduate:before{content:"\f501"}.fa-user-injured:before{content:"\f728"}.fa-user-lock:before{content:"\f502"}.fa-user-md:before{content:"\f0f0"}.fa-user-minus:before{content:"\f503"}.fa-user-ninja:before{content:"\f504"}.fa-user-nurse:before{content:"\f82f"}.fa-user-plus:before{content:"\f234"}.fa-user-secret:before{content:"\f21b"}.fa-user-shield:before{content:"\f505"}.fa-user-slash:before{content:"\f506"}.fa-user-tag:before{content:"\f507"}.fa-user-tie:before{content:"\f508"}.fa-user-times:before{content:"\f235"}.fa-users:before{content:"\f0c0"}.fa-users-cog:before{content:"\f509"}.fa-users-slash:before{content:"\e073"}.fa-usps:before{content:"\f7e1"}.fa-ussunnah:before{content:"\f407"}.fa-utensil-spoon:before{content:"\f2e5"}.fa-utensils:before{content:"\f2e7"}.fa-vaadin:before{content:"\f408"}.fa-vector-square:before{content:"\f5cb"}.fa-venus:before{content:"\f221"}.fa-venus-double:before{content:"\f226"}.fa-venus-mars:before{content:"\f228"}.fa-vest:before{content:"\e085"}.fa-vest-patches:before{content:"\e086"}.fa-viacoin:before{content:"\f237"}.fa-viadeo:before{content:"\f2a9"}.fa-viadeo-square:before{content:"\f2aa"}.fa-vial:before{content:"\f492"}.fa-vials:before{content:"\f493"}.fa-viber:before{content:"\f409"}.fa-video:before{content:"\f03d"}.fa-video-slash:before{content:"\f4e2"}.fa-vihara:before{content:"\f6a7"}.fa-vimeo:before{content:"\f40a"}.fa-vimeo-square:before{content:"\f194"}.fa-vimeo-v:before{content:"\f27d"}.fa-vine:before{content:"\f1ca"}.fa-virus:before{content:"\e074"}.fa-virus-slash:before{content:"\e075"}.fa-viruses:before{content:"\e076"}.fa-vk:before{content:"\f189"}.fa-vnv:before{content:"\f40b"}.fa-voicemail:before{content:"\f897"}.fa-volleyball-ball:before{content:"\f45f"}.fa-volume-down:before{content:"\f027"}.fa-volume-mute:before{content:"\f6a9"}.fa-volume-off:before{content:"\f026"}.fa-volume-up:before{content:"\f028"}.fa-vote-yea:before{content:"\f772"}.fa-vr-cardboard:before{content:"\f729"}.fa-vuejs:before{content:"\f41f"}.fa-walking:before{content:"\f554"}.fa-wallet:before{content:"\f555"}.fa-warehouse:before{content:"\f494"}.fa-watchman-monitoring:before{content:"\e087"}.fa-water:before{content:"\f773"}.fa-wave-square:before{content:"\f83e"}.fa-waze:before{content:"\f83f"}.fa-weebly:before{content:"\f5cc"}.fa-weibo:before{content:"\f18a"}.fa-weight:before{content:"\f496"}.fa-weight-hanging:before{content:"\f5cd"}.fa-weixin:before{content:"\f1d7"}.fa-whatsapp:before{content:"\f232"}.fa-whatsapp-square:before{content:"\f40c"}.fa-wheelchair:before{content:"\f193"}.fa-whmcs:before{content:"\f40d"}.fa-wifi:before{content:"\f1eb"}.fa-wikipedia-w:before{content:"\f266"}.fa-wind:before{content:"\f72e"}.fa-window-close:before{content:"\f410"}.fa-window-maximize:before{content:"\f2d0"}.fa-window-minimize:before{content:"\f2d1"}.fa-window-restore:before{content:"\f2d2"}.fa-windows:before{content:"\f17a"}.fa-wine-bottle:before{content:"\f72f"}.fa-wine-glass:before{content:"\f4e3"}.fa-wine-glass-alt:before{content:"\f5ce"}.fa-wix:before{content:"\f5cf"}.fa-wizards-of-the-coast:before{content:"\f730"}.fa-wodu:before{content:"\e088"}.fa-wolf-pack-battalion:before{content:"\f514"}.fa-won-sign:before{content:"\f159"}.fa-wordpress:before{content:"\f19a"}.fa-wordpress-simple:before{content:"\f411"}.fa-wpbeginner:before{content:"\f297"}.fa-wpexplorer:before{content:"\f2de"}.fa-wpforms:before{content:"\f298"}.fa-wpressr:before{content:"\f3e4"}.fa-wrench:before{content:"\f0ad"}.fa-x-ray:before{content:"\f497"}.fa-xbox:before{content:"\f412"}.fa-xing:before{content:"\f168"}.fa-xing-square:before{content:"\f169"}.fa-y-combinator:before{content:"\f23b"}.fa-yahoo:before{content:"\f19e"}.fa-yammer:before{content:"\f840"}.fa-yandex:before{content:"\f413"}.fa-yandex-international:before{content:"\f414"}.fa-yarn:before{content:"\f7e3"}.fa-yelp:before{content:"\f1e9"}.fa-yen-sign:before{content:"\f157"}.fa-yin-yang:before{content:"\f6ad"}.fa-yoast:before{content:"\f2b1"}.fa-youtube:before{content:"\f167"}.fa-youtube-square:before{content:"\f431"}.fa-zhihu:before{content:"\f63f"}.sr-only{border:0;clip:rect(0,0,0,0);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;width:1px}.sr-only-focusable:active,.sr-only-focusable:focus{clip:auto;height:auto;margin:0;overflow:visible;position:static;width:auto}.fab{font-family:"Font Awesome 5 Brands"}.fab,.far{font-weight:400}.fa,.far,.fas{font-family:"Font Awesome 5 Free"}.fa,.fas{font-weight:900}</style><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="bfa-font-awesome-v4-shim-css" href="https://use.fontawesome.com/releases/v5.15.4/css/v4-shims.css?ver=2.0.3" type="text/css" media="all"><style class="darkreader darkreader--cors" media="screen">.fa.fa-glass:before{content:"\f000"}.fa.fa-meetup{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-star-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-star-o:before{content:"\f005"}.fa.fa-close:before,.fa.fa-remove:before{content:"\f00d"}.fa.fa-gear:before{content:"\f013"}.fa.fa-trash-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-trash-o:before{content:"\f2ed"}.fa.fa-file-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-o:before{content:"\f15b"}.fa.fa-clock-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-clock-o:before{content:"\f017"}.fa.fa-arrow-circle-o-down{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-arrow-circle-o-down:before{content:"\f358"}.fa.fa-arrow-circle-o-up{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-arrow-circle-o-up:before{content:"\f35b"}.fa.fa-play-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-play-circle-o:before{content:"\f144"}.fa.fa-repeat:before,.fa.fa-rotate-right:before{content:"\f01e"}.fa.fa-refresh:before{content:"\f021"}.fa.fa-list-alt{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-dedent:before{content:"\f03b"}.fa.fa-video-camera:before{content:"\f03d"}.fa.fa-picture-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-picture-o:before{content:"\f03e"}.fa.fa-photo{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-photo:before{content:"\f03e"}.fa.fa-image{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-image:before{content:"\f03e"}.fa.fa-pencil:before{content:"\f303"}.fa.fa-map-marker:before{content:"\f3c5"}.fa.fa-pencil-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-pencil-square-o:before{content:"\f044"}.fa.fa-share-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-share-square-o:before{content:"\f14d"}.fa.fa-check-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-check-square-o:before{content:"\f14a"}.fa.fa-arrows:before{content:"\f0b2"}.fa.fa-times-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-times-circle-o:before{content:"\f057"}.fa.fa-check-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-check-circle-o:before{content:"\f058"}.fa.fa-mail-forward:before{content:"\f064"}.fa.fa-expand:before{content:"\f424"}.fa.fa-compress:before{content:"\f422"}.fa.fa-eye,.fa.fa-eye-slash{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-warning:before{content:"\f071"}.fa.fa-calendar:before{content:"\f073"}.fa.fa-arrows-v:before{content:"\f338"}.fa.fa-arrows-h:before{content:"\f337"}.fa.fa-bar-chart{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-bar-chart:before{content:"\f080"}.fa.fa-bar-chart-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-bar-chart-o:before{content:"\f080"}.fa.fa-facebook-square,.fa.fa-twitter-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-gears:before{content:"\f085"}.fa.fa-thumbs-o-up{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-thumbs-o-up:before{content:"\f164"}.fa.fa-thumbs-o-down{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-thumbs-o-down:before{content:"\f165"}.fa.fa-heart-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-heart-o:before{content:"\f004"}.fa.fa-sign-out:before{content:"\f2f5"}.fa.fa-linkedin-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-linkedin-square:before{content:"\f08c"}.fa.fa-thumb-tack:before{content:"\f08d"}.fa.fa-external-link:before{content:"\f35d"}.fa.fa-sign-in:before{content:"\f2f6"}.fa.fa-github-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-lemon-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-lemon-o:before{content:"\f094"}.fa.fa-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-square-o:before{content:"\f0c8"}.fa.fa-bookmark-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-bookmark-o:before{content:"\f02e"}.fa.fa-facebook,.fa.fa-twitter{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-facebook:before{content:"\f39e"}.fa.fa-facebook-f{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-facebook-f:before{content:"\f39e"}.fa.fa-github{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-credit-card{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-feed:before{content:"\f09e"}.fa.fa-hdd-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hdd-o:before{content:"\f0a0"}.fa.fa-hand-o-right{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-o-right:before{content:"\f0a4"}.fa.fa-hand-o-left{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-o-left:before{content:"\f0a5"}.fa.fa-hand-o-up{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-o-up:before{content:"\f0a6"}.fa.fa-hand-o-down{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-o-down:before{content:"\f0a7"}.fa.fa-arrows-alt:before{content:"\f31e"}.fa.fa-group:before{content:"\f0c0"}.fa.fa-chain:before{content:"\f0c1"}.fa.fa-scissors:before{content:"\f0c4"}.fa.fa-files-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-files-o:before{content:"\f0c5"}.fa.fa-floppy-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-floppy-o:before{content:"\f0c7"}.fa.fa-navicon:before,.fa.fa-reorder:before{content:"\f0c9"}.fa.fa-google-plus,.fa.fa-google-plus-square,.fa.fa-pinterest,.fa.fa-pinterest-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-google-plus:before{content:"\f0d5"}.fa.fa-money{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-money:before{content:"\f3d1"}.fa.fa-unsorted:before{content:"\f0dc"}.fa.fa-sort-desc:before{content:"\f0dd"}.fa.fa-sort-asc:before{content:"\f0de"}.fa.fa-linkedin{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-linkedin:before{content:"\f0e1"}.fa.fa-rotate-left:before{content:"\f0e2"}.fa.fa-legal:before{content:"\f0e3"}.fa.fa-dashboard:before,.fa.fa-tachometer:before{content:"\f3fd"}.fa.fa-comment-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-comment-o:before{content:"\f075"}.fa.fa-comments-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-comments-o:before{content:"\f086"}.fa.fa-flash:before{content:"\f0e7"}.fa.fa-clipboard,.fa.fa-paste{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-paste:before{content:"\f328"}.fa.fa-lightbulb-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-lightbulb-o:before{content:"\f0eb"}.fa.fa-exchange:before{content:"\f362"}.fa.fa-cloud-download:before{content:"\f381"}.fa.fa-cloud-upload:before{content:"\f382"}.fa.fa-bell-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-bell-o:before{content:"\f0f3"}.fa.fa-cutlery:before{content:"\f2e7"}.fa.fa-file-text-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-text-o:before{content:"\f15c"}.fa.fa-building-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-building-o:before{content:"\f1ad"}.fa.fa-hospital-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hospital-o:before{content:"\f0f8"}.fa.fa-tablet:before{content:"\f3fa"}.fa.fa-mobile-phone:before,.fa.fa-mobile:before{content:"\f3cd"}.fa.fa-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-circle-o:before{content:"\f111"}.fa.fa-mail-reply:before{content:"\f3e5"}.fa.fa-github-alt{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-folder-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-folder-o:before{content:"\f07b"}.fa.fa-folder-open-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-folder-open-o:before{content:"\f07c"}.fa.fa-smile-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-smile-o:before{content:"\f118"}.fa.fa-frown-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-frown-o:before{content:"\f119"}.fa.fa-meh-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-meh-o:before{content:"\f11a"}.fa.fa-keyboard-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-keyboard-o:before{content:"\f11c"}.fa.fa-flag-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-flag-o:before{content:"\f024"}.fa.fa-mail-reply-all:before{content:"\f122"}.fa.fa-star-half-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-star-half-o:before{content:"\f089"}.fa.fa-star-half-empty{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-star-half-empty:before{content:"\f089"}.fa.fa-star-half-full{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-star-half-full:before{content:"\f089"}.fa.fa-code-fork:before{content:"\f126"}.fa.fa-chain-broken:before{content:"\f127"}.fa.fa-shield:before{content:"\f3ed"}.fa.fa-calendar-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-calendar-o:before{content:"\f133"}.fa.fa-css3,.fa.fa-html5,.fa.fa-maxcdn{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-ticket:before{content:"\f3ff"}.fa.fa-minus-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-minus-square-o:before{content:"\f146"}.fa.fa-level-up:before{content:"\f3bf"}.fa.fa-level-down:before{content:"\f3be"}.fa.fa-pencil-square:before{content:"\f14b"}.fa.fa-external-link-square:before{content:"\f360"}.fa.fa-compass{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-caret-square-o-down{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-caret-square-o-down:before{content:"\f150"}.fa.fa-toggle-down{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-toggle-down:before{content:"\f150"}.fa.fa-caret-square-o-up{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-caret-square-o-up:before{content:"\f151"}.fa.fa-toggle-up{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-toggle-up:before{content:"\f151"}.fa.fa-caret-square-o-right{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-caret-square-o-right:before{content:"\f152"}.fa.fa-toggle-right{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-toggle-right:before{content:"\f152"}.fa.fa-eur:before,.fa.fa-euro:before{content:"\f153"}.fa.fa-gbp:before{content:"\f154"}.fa.fa-dollar:before,.fa.fa-usd:before{content:"\f155"}.fa.fa-inr:before,.fa.fa-rupee:before{content:"\f156"}.fa.fa-cny:before,.fa.fa-jpy:before,.fa.fa-rmb:before,.fa.fa-yen:before{content:"\f157"}.fa.fa-rouble:before,.fa.fa-rub:before,.fa.fa-ruble:before{content:"\f158"}.fa.fa-krw:before,.fa.fa-won:before{content:"\f159"}.fa.fa-bitcoin,.fa.fa-btc{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-bitcoin:before{content:"\f15a"}.fa.fa-file-text:before{content:"\f15c"}.fa.fa-sort-alpha-asc:before{content:"\f15d"}.fa.fa-sort-alpha-desc:before{content:"\f881"}.fa.fa-sort-amount-asc:before{content:"\f160"}.fa.fa-sort-amount-desc:before{content:"\f884"}.fa.fa-sort-numeric-asc:before{content:"\f162"}.fa.fa-sort-numeric-desc:before{content:"\f886"}.fa.fa-xing,.fa.fa-xing-square,.fa.fa-youtube,.fa.fa-youtube-play,.fa.fa-youtube-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-youtube-play:before{content:"\f167"}.fa.fa-adn,.fa.fa-bitbucket,.fa.fa-bitbucket-square,.fa.fa-dropbox,.fa.fa-flickr,.fa.fa-instagram,.fa.fa-stack-overflow{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-bitbucket-square:before{content:"\f171"}.fa.fa-tumblr,.fa.fa-tumblr-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-long-arrow-down:before{content:"\f309"}.fa.fa-long-arrow-up:before{content:"\f30c"}.fa.fa-long-arrow-left:before{content:"\f30a"}.fa.fa-long-arrow-right:before{content:"\f30b"}.fa.fa-android,.fa.fa-apple,.fa.fa-dribbble,.fa.fa-foursquare,.fa.fa-gittip,.fa.fa-gratipay,.fa.fa-linux,.fa.fa-skype,.fa.fa-trello,.fa.fa-windows{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-gittip:before{content:"\f184"}.fa.fa-sun-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-sun-o:before{content:"\f185"}.fa.fa-moon-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-moon-o:before{content:"\f186"}.fa.fa-pagelines,.fa.fa-renren,.fa.fa-stack-exchange,.fa.fa-vk,.fa.fa-weibo{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-arrow-circle-o-right{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-arrow-circle-o-right:before{content:"\f35a"}.fa.fa-arrow-circle-o-left{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-arrow-circle-o-left:before{content:"\f359"}.fa.fa-caret-square-o-left{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-caret-square-o-left:before{content:"\f191"}.fa.fa-toggle-left{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-toggle-left:before{content:"\f191"}.fa.fa-dot-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-dot-circle-o:before{content:"\f192"}.fa.fa-vimeo-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-try:before,.fa.fa-turkish-lira:before{content:"\f195"}.fa.fa-plus-square-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-plus-square-o:before{content:"\f0fe"}.fa.fa-openid,.fa.fa-slack,.fa.fa-wordpress{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-bank:before,.fa.fa-institution:before{content:"\f19c"}.fa.fa-mortar-board:before{content:"\f19d"}.fa.fa-delicious,.fa.fa-digg,.fa.fa-drupal,.fa.fa-google,.fa.fa-joomla,.fa.fa-pied-piper-alt,.fa.fa-pied-piper-pp,.fa.fa-reddit,.fa.fa-reddit-square,.fa.fa-stumbleupon,.fa.fa-stumbleupon-circle,.fa.fa-yahoo{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-spoon:before{content:"\f2e5"}.fa.fa-behance,.fa.fa-behance-square,.fa.fa-steam,.fa.fa-steam-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-automobile:before{content:"\f1b9"}.fa.fa-envelope-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-envelope-o:before{content:"\f0e0"}.fa.fa-deviantart,.fa.fa-soundcloud,.fa.fa-spotify{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-file-pdf-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-pdf-o:before{content:"\f1c1"}.fa.fa-file-word-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-word-o:before{content:"\f1c2"}.fa.fa-file-excel-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-excel-o:before{content:"\f1c3"}.fa.fa-file-powerpoint-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-powerpoint-o:before{content:"\f1c4"}.fa.fa-file-image-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-image-o:before{content:"\f1c5"}.fa.fa-file-photo-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-photo-o:before{content:"\f1c5"}.fa.fa-file-picture-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-picture-o:before{content:"\f1c5"}.fa.fa-file-archive-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-archive-o:before{content:"\f1c6"}.fa.fa-file-zip-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-zip-o:before{content:"\f1c6"}.fa.fa-file-audio-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-audio-o:before{content:"\f1c7"}.fa.fa-file-sound-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-sound-o:before{content:"\f1c7"}.fa.fa-file-video-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-video-o:before{content:"\f1c8"}.fa.fa-file-movie-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-movie-o:before{content:"\f1c8"}.fa.fa-file-code-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-file-code-o:before{content:"\f1c9"}.fa.fa-codepen,.fa.fa-jsfiddle,.fa.fa-vine{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-life-bouy,.fa.fa-life-ring{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-life-bouy:before{content:"\f1cd"}.fa.fa-life-buoy{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-life-buoy:before{content:"\f1cd"}.fa.fa-life-saver{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-life-saver:before{content:"\f1cd"}.fa.fa-support{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-support:before{content:"\f1cd"}.fa.fa-circle-o-notch:before{content:"\f1ce"}.fa.fa-ra,.fa.fa-rebel{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-ra:before{content:"\f1d0"}.fa.fa-resistance{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-resistance:before{content:"\f1d0"}.fa.fa-empire,.fa.fa-ge{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-ge:before{content:"\f1d1"}.fa.fa-git,.fa.fa-git-square,.fa.fa-hacker-news,.fa.fa-y-combinator-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-y-combinator-square:before{content:"\f1d4"}.fa.fa-yc-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-yc-square:before{content:"\f1d4"}.fa.fa-qq,.fa.fa-tencent-weibo,.fa.fa-wechat,.fa.fa-weixin{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-wechat:before{content:"\f1d7"}.fa.fa-send:before{content:"\f1d8"}.fa.fa-paper-plane-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-paper-plane-o:before{content:"\f1d8"}.fa.fa-send-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-send-o:before{content:"\f1d8"}.fa.fa-circle-thin{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-circle-thin:before{content:"\f111"}.fa.fa-header:before{content:"\f1dc"}.fa.fa-sliders:before{content:"\f1de"}.fa.fa-futbol-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-futbol-o:before{content:"\f1e3"}.fa.fa-soccer-ball-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-soccer-ball-o:before{content:"\f1e3"}.fa.fa-slideshare,.fa.fa-twitch,.fa.fa-yelp{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-newspaper-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-newspaper-o:before{content:"\f1ea"}.fa.fa-cc-amex,.fa.fa-cc-discover,.fa.fa-cc-mastercard,.fa.fa-cc-paypal,.fa.fa-cc-stripe,.fa.fa-cc-visa,.fa.fa-google-wallet,.fa.fa-paypal{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-bell-slash-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-bell-slash-o:before{content:"\f1f6"}.fa.fa-trash:before{content:"\f2ed"}.fa.fa-copyright{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-eyedropper:before{content:"\f1fb"}.fa.fa-area-chart:before{content:"\f1fe"}.fa.fa-pie-chart:before{content:"\f200"}.fa.fa-line-chart:before{content:"\f201"}.fa.fa-angellist,.fa.fa-ioxhost,.fa.fa-lastfm,.fa.fa-lastfm-square{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-cc{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-cc:before{content:"\f20a"}.fa.fa-ils:before,.fa.fa-shekel:before,.fa.fa-sheqel:before{content:"\f20b"}.fa.fa-meanpath{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-meanpath:before{content:"\f2b4"}.fa.fa-buysellads,.fa.fa-connectdevelop,.fa.fa-dashcube,.fa.fa-forumbee,.fa.fa-leanpub,.fa.fa-sellsy,.fa.fa-shirtsinbulk,.fa.fa-simplybuilt,.fa.fa-skyatlas{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-diamond{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-diamond:before{content:"\f3a5"}.fa.fa-intersex:before{content:"\f224"}.fa.fa-facebook-official{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-facebook-official:before{content:"\f09a"}.fa.fa-pinterest-p,.fa.fa-whatsapp{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-hotel:before{content:"\f236"}.fa.fa-medium,.fa.fa-viacoin,.fa.fa-y-combinator,.fa.fa-yc{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-yc:before{content:"\f23b"}.fa.fa-expeditedssl,.fa.fa-opencart,.fa.fa-optin-monster{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-battery-4:before,.fa.fa-battery:before{content:"\f240"}.fa.fa-battery-3:before{content:"\f241"}.fa.fa-battery-2:before{content:"\f242"}.fa.fa-battery-1:before{content:"\f243"}.fa.fa-battery-0:before{content:"\f244"}.fa.fa-object-group,.fa.fa-object-ungroup,.fa.fa-sticky-note-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-sticky-note-o:before{content:"\f249"}.fa.fa-cc-diners-club,.fa.fa-cc-jcb{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-clone,.fa.fa-hourglass-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hourglass-o:before{content:"\f254"}.fa.fa-hourglass-1:before{content:"\f251"}.fa.fa-hourglass-2:before{content:"\f252"}.fa.fa-hourglass-3:before{content:"\f253"}.fa.fa-hand-rock-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-rock-o:before{content:"\f255"}.fa.fa-hand-grab-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-grab-o:before{content:"\f255"}.fa.fa-hand-paper-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-paper-o:before{content:"\f256"}.fa.fa-hand-stop-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-stop-o:before{content:"\f256"}.fa.fa-hand-scissors-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-scissors-o:before{content:"\f257"}.fa.fa-hand-lizard-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-lizard-o:before{content:"\f258"}.fa.fa-hand-spock-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-spock-o:before{content:"\f259"}.fa.fa-hand-pointer-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-pointer-o:before{content:"\f25a"}.fa.fa-hand-peace-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-hand-peace-o:before{content:"\f25b"}.fa.fa-registered{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-chrome,.fa.fa-creative-commons,.fa.fa-firefox,.fa.fa-get-pocket,.fa.fa-gg,.fa.fa-gg-circle,.fa.fa-internet-explorer,.fa.fa-odnoklassniki,.fa.fa-odnoklassniki-square,.fa.fa-opera,.fa.fa-safari,.fa.fa-tripadvisor,.fa.fa-wikipedia-w{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-television:before{content:"\f26c"}.fa.fa-500px,.fa.fa-amazon,.fa.fa-contao{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-calendar-plus-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-calendar-plus-o:before{content:"\f271"}.fa.fa-calendar-minus-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-calendar-minus-o:before{content:"\f272"}.fa.fa-calendar-times-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-calendar-times-o:before{content:"\f273"}.fa.fa-calendar-check-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-calendar-check-o:before{content:"\f274"}.fa.fa-map-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-map-o:before{content:"\f279"}.fa.fa-commenting:before{content:"\f4ad"}.fa.fa-commenting-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-commenting-o:before{content:"\f4ad"}.fa.fa-houzz,.fa.fa-vimeo{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-vimeo:before{content:"\f27d"}.fa.fa-black-tie,.fa.fa-edge,.fa.fa-fonticons,.fa.fa-reddit-alien{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-credit-card-alt:before{content:"\f09d"}.fa.fa-codiepie,.fa.fa-fort-awesome,.fa.fa-mixcloud,.fa.fa-modx,.fa.fa-product-hunt,.fa.fa-scribd,.fa.fa-usb{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-pause-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-pause-circle-o:before{content:"\f28b"}.fa.fa-stop-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-stop-circle-o:before{content:"\f28d"}.fa.fa-bluetooth,.fa.fa-bluetooth-b,.fa.fa-envira,.fa.fa-gitlab,.fa.fa-wheelchair-alt,.fa.fa-wpbeginner,.fa.fa-wpforms{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-wheelchair-alt:before{content:"\f368"}.fa.fa-question-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-question-circle-o:before{content:"\f059"}.fa.fa-volume-control-phone:before{content:"\f2a0"}.fa.fa-asl-interpreting:before{content:"\f2a3"}.fa.fa-deafness:before,.fa.fa-hard-of-hearing:before{content:"\f2a4"}.fa.fa-glide,.fa.fa-glide-g{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-signing:before{content:"\f2a7"}.fa.fa-first-order,.fa.fa-google-plus-official,.fa.fa-pied-piper,.fa.fa-snapchat,.fa.fa-snapchat-ghost,.fa.fa-snapchat-square,.fa.fa-themeisle,.fa.fa-viadeo,.fa.fa-viadeo-square,.fa.fa-yoast{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-google-plus-official:before{content:"\f2b3"}.fa.fa-google-plus-circle{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-google-plus-circle:before{content:"\f2b3"}.fa.fa-fa,.fa.fa-font-awesome{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-fa:before{content:"\f2b4"}.fa.fa-handshake-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-handshake-o:before{content:"\f2b5"}.fa.fa-envelope-open-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-envelope-open-o:before{content:"\f2b6"}.fa.fa-linode{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-address-book-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-address-book-o:before{content:"\f2b9"}.fa.fa-vcard:before{content:"\f2bb"}.fa.fa-address-card-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-address-card-o:before{content:"\f2bb"}.fa.fa-vcard-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-vcard-o:before{content:"\f2bb"}.fa.fa-user-circle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-user-circle-o:before{content:"\f2bd"}.fa.fa-user-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-user-o:before{content:"\f007"}.fa.fa-id-badge{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-drivers-license:before{content:"\f2c2"}.fa.fa-id-card-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-id-card-o:before{content:"\f2c2"}.fa.fa-drivers-license-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-drivers-license-o:before{content:"\f2c2"}.fa.fa-free-code-camp,.fa.fa-quora,.fa.fa-telegram{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-thermometer-4:before,.fa.fa-thermometer:before{content:"\f2c7"}.fa.fa-thermometer-3:before{content:"\f2c8"}.fa.fa-thermometer-2:before{content:"\f2c9"}.fa.fa-thermometer-1:before{content:"\f2ca"}.fa.fa-thermometer-0:before{content:"\f2cb"}.fa.fa-bathtub:before,.fa.fa-s15:before{content:"\f2cd"}.fa.fa-window-maximize,.fa.fa-window-restore{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-times-rectangle:before{content:"\f410"}.fa.fa-window-close-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-window-close-o:before{content:"\f410"}.fa.fa-times-rectangle-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-times-rectangle-o:before{content:"\f410"}.fa.fa-bandcamp,.fa.fa-eercast,.fa.fa-etsy,.fa.fa-grav,.fa.fa-imdb,.fa.fa-ravelry{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-eercast:before{content:"\f2da"}.fa.fa-snowflake-o{font-family:"Font Awesome 5 Free";font-weight:400}.fa.fa-snowflake-o:before{content:"\f2dc"}.fa.fa-superpowers,.fa.fa-wpexplorer{font-family:"Font Awesome 5 Brands";font-weight:400}.fa.fa-cab:before{content:"\f1ba"}</style><style class="darkreader darkreader--sync" media="screen"></style>
        <style id="bfa-font-awesome-v4-shim-inline-css" type="text/css">

    @font-face {
        font-family: 'FontAwesome';
        src: url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.eot'),
                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.eot?#iefix') format('embedded-opentype'),
                     url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.woff2') format('woff2'),
                         url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.woff') format('woff'),
                             url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.ttf') format('truetype'),
                                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-brands-400.svg#fontawesome') format('svg');
    }

    @font-face {
        font-family: 'FontAwesome';
        src: url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.eot'),
                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.eot?#iefix') format('embedded-opentype'),
                     url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.woff2') format('woff2'),
                         url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.woff') format('woff'),
                             url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.ttf') format('truetype'),
                                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-solid-900.svg#fontawesome') format('svg');
    }

    @font-face {
        font-family: 'FontAwesome';
        src: url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.eot'),
                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.eot?#iefix') format('embedded-opentype'),
                     url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.woff2') format('woff2'),
                         url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.woff') format('woff'),
                             url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.ttf') format('truetype'),
                                 url('https://use.fontawesome.com/releases/v5.15.4/webfonts/fa-regular-400.svg#fontawesome') format('svg');
                                     unicode-range: U+F004-F005,U+F007,U+F017,U+F022,U+F024,U+F02E,U+F03E,U+F044,U+F057-F059,U+F06E,U+F070,U+F075,U+F07B-F07C,U+F080,U+F086,U+F089,U+F094,U+F09D,U+F0A0,U+F0A4-F0A7,U+F0C5,U+F0C7-F0C8,U+F0E0,U+F0EB,U+F0F3,U+F0F8,U+F0FE,U+F111,U+F118-F11A,U+F11C,U+F133,U+F144,U+F146,U+F14A,U+F14D-F14E,U+F150-F152,U+F15B-F15C,U+F164-F165,U+F185-F186,U+F191-F192,U+F1AD,U+F1C1-F1C9,U+F1CD,U+F1D8,U+F1E3,U+F1EA,U+F1F6,U+F1F9,U+F20A,U+F247-F249,U+F24D,U+F254-F25B,U+F25D,U+F267,U+F271-F274,U+F279,U+F28B,U+F28D,U+F2B5-F2B6,U+F2B9,U+F2BB,U+F2BD,U+F2C1-F2C2,U+F2D0,U+F2D2,U+F2DC,U+F2ED,U+F328,U+F358-F35B,U+F3A5,U+F3D1,U+F410,U+F4AD;
    }

        </style><style class="darkreader darkreader--sync" media="screen"></style>
        <link rel="stylesheet" id="jupiter-donut-shortcodes-css" href="https://www.misesde.org/wp-content/plugins/jupiter-donut/assets/css/shortcodes-styles.min.css?ver=1.4.3" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
        <script type="text/javascript" data-noptimize="" data-no-minify="" src="https://www.misesde.org/wp-content/themes/jupiter/assets/js/plugins/wp-enqueue/min/webfontloader.js?ver=6.3.1" id="mk-webfontloader-js"></script>
        <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Open+Sans:100italic,200italic,300italic,400italic,500italic,600italic,700italic,800italic,900italic,100,200,300,400,500,600,700,800,900" media="all"><script id="mk-webfontloader-js-after" type="text/javascript">
        WebFontConfig = {
        timeout: 2000
    }

    if ( mk_typekit_id.length > 0 ) {
        WebFontConfig.typekit = {
            id: mk_typekit_id
        }
    }

    if ( mk_google_fonts.length > 0 ) {
        WebFontConfig.google = {
            families:  mk_google_fonts
        }
    }

    if ( (mk_google_fonts.length > 0 || mk_typekit_id.length > 0) && navigator.userAgent.indexOf("Speed Insights") == -1) {
        WebFont.load( WebFontConfig );
    }

    </script>
        <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/jquery/jquery.min.js?ver=3.7.0" id="jquery-clients-js"></script>
        <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/jquery/jquery-migrate.min.js?ver=3.4.1" id="jquery-migrate-js"></script>
        <script type="text/javascript" id="cookie-law-info-js-extra">
        /* <![CDATA[ */
        var Cli_Data = {"nn_cookie_ids":[],"cookielist":[],"non_necessary_cookies":[],"ccpaEnabled":"","ccpaRegionBased":"","ccpaBarEnabled":"","strictlyEnabled":["necessary","obligatoire"],"ccpaType":"gdpr","js_blocking":"","custom_integration":"","triggerDomRefresh":"","secure_cookies":""};
    var cli_cookiebar_settings = {"animate_speed_hide":"500","animate_speed_show":"500","background":#3f3f3f","border":#303030","border_on":"","button_1_button_colour":#000","button_1_button_hover":#000000","button_1_link_colour":#fff","button_1_as_button":"1","button_1_new_win":"","button_2_button_colour":#333","button_2_button_hover":#292929","button_2_link_colour":#444","button_2_as_button":"","button_2_hidebar":"","button_3_button_colour":#000","button_3_button_hover":#000000","button_3_link_colour":#fff","button_3_as_button":"1","button_3_new_win":"","button_4_button_colour":#000","button_4_button_hover":#000000","button_4_link_colour":#fff","button_4_as_button":"1","button_7_button_colour":#61a229","button_7_button_hover":#4e8221","button_7_link_colour":#fff","button_7_as_button":"1","button_7_new_win":"","font_family":"inherit","header_fix":"","notify_animate_hide":"1","notify_animate_show":"","notify_div_id":#cookie-law-info-bar","notify_position_horizontal":"right","notify_position_vertical":"bottom","scroll_close":"","scroll_close_reload":"","accept_close_reload":"","reject_close_reload":"","showagain_tab":"1","showagain_background":#fff","showagain_border":#000","showagain_div_id":#cookie-law-info-again","showagain_x_position":"100px","text":#efefef","show_once_yn":"","show_once":"10000","logging_on":"","as_popup":"","popup_overlay":"1","bar_heading_text":"","cookie_bar_as":"banner","popup_showagain_position":"bottom-right","widget_position":"left"};
    var log_object = {"ajax_url":"https:\/\/www.misesde.org\/wp-admin\/admin-ajax.php"};
    /* ]]> */
    </script>
        <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/cookie-law-info/legacy/public/js/cookie-law-info-public.js?ver=3.1.4" id="cookie-law-info-js"></script>
        <script type="text/javascript" id="wp-statistics-tracker-js-extra">
        /* <![CDATA[ */
        var WP_Statistics_Tracker_Object = {"hitRequestUrl":"https:\/\/www.misesde.org\/wp-json\/wp-statistics\/v2\/hit?wp_statistics_hit_rest=yes&track_all=1&current_page_type=home&current_page_id=18800&search_query&page_uri=Lw=","keepOnlineRequestUrl":"https:\/\/www.misesde.org\/wp-json\/wp-statistics\/v2\/online?wp_statistics_hit_rest=yes&track_all=1&current_page_type=home&current_page_id=18800&search_query&page_uri=Lw=","option":{"dntEnabled":false,"cacheCompatibility":"1"}};
    /* ]]> */
    </script>
        <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/wp-statistics/assets/js/tracker.js?ver=6.3.1" id="wp-statistics-tracker-js"></script>
        <script></script><link rel="https://api.w.org/" href="https://www.misesde.org/wp-json/"><link rel="alternate" type="application/json" href="https://www.misesde.org/wp-json/wp/v2/pages/18800"><link rel="EditURI" type="application/rsd+xml" title="RSD" href="https://www.misesde.org/xmlrpc.php?rsd">
        <meta name="generator" content="WordPress 6.3.1">
        <meta name="generator" content="Seriously Simple Podcasting 2.23.0">
        <link rel="canonical" href="https://www.misesde.org/">
        <link rel="shortlink" href="https://www.misesde.org/">
        <link rel="alternate" type="application/json+oembed" href="https://www.misesde.org/wp-json/oembed/1.0/embed?url=https%3A%2F%2Fwww.misesde.org%2F">
        <link rel="alternate" type="text/xml+oembed" href="https://www.misesde.org/wp-json/oembed/1.0/embed?url=https%3A%2F%2Fwww.misesde.org%2F&amp;format=xml">
        <script>var ms_grabbing_curosr='https://www.misesde.org/wp-content/plugins/masterslider/public/assets/css/common/grabbing.cur',ms_grab_curosr='https://www.misesde.org/wp-content/plugins/masterslider/public/assets/css/common/grab.cur';</script>
    <meta name="generator" content="MasterSlider 3.6.5 - Responsive Touch Image Slider">

        <link rel="alternate" type="application/rss+xml" title="Podcast RSS-Feed" href="https://www.misesde.org/feed/podcast">

        <script>
        window.addEventListener("sfsi_plus_functions_loaded", function() {
            var body = document.getElementsByTagName('body')[0];
            // console.log(body);
            body.classList.add("sfsi_plus_3.53");
        })
    // window.addEventListener('sfsi_plus_functions_loaded',function(e) {
    // 	jQuery("body").addClass("sfsi_plus_3.53")
    // });
    jQuery(document).ready(function(e) {
        jQuery("body").addClass("sfsi_plus_3.53")
    });

    function sfsi_plus_processfurther(ref) {
        var feed_id = 'ZmZ0MVc4dEE4eFdERXhqdWpobkdSc01jSTFlTFVqV3JaZURjRDdyR1Fvb3B3SWF3OTBoWWUyTGhxN0p3U2ppdnc2R1NGT1RzQk9oRVkvWmxoSXNPT00zMEpvc1V4aVhwMFZQcjcyQVZwcXFsWHhOZlFyRW9WZm1NaXNmeVBobjV8MDgwQ0J5SEN5YjZ5emNlYnE1NGRLNWJpVDZ3UHlNSWI2T0ZQMlhMZDRobz0=';
        var feedtype = 8;
        var email = jQuery(ref).find('input[name="email"]').val();
        var filter = /^([a-zA-Z0-9_\.\-])+\@(([a-zA-Z0-9\-])+\.)+([a-zA-Z0-9]{2,4})+$/;
    if ((email != "Enter your email") && (filter.test(email))) {
    if (feedtype == "8") {
    var url = "https://api.follow.it/subscription-form/" + feed_id + "/" + feedtype;
    window.open(url, "popupwindow", "scrollbars=yes,width=1080,height=760");
    return true;
    }
    } else {
    alert("Please enter email address");
    jQuery(ref).find('input[name="email"]').focus();
    return false;
    }
    }
    </script>
    <style>
    .sfsi_plus_subscribe_Popinner {
    width: 100% !important;
    height: auto !important;
    border: 1px solid #b5b5b5 !important;
    padding: 18px 0px !important;
    background-color: #ffffff !important;
    }

    .sfsi_plus_subscribe_Popinner form {
    margin: 0 20px !important;
    }

    .sfsi_plus_subscribe_Popinner h5 {
    font-family: Helvetica,Arial,sans-serif !important;

    font-weight: bold !important;
    color: #000000 !important;
    font-size: 16px !important;
    text-align: center !important;
    margin: 0 0 10px !important;
    padding: 0 !important;
    }

    .sfsi_plus_subscription_form_field {
    margin: 5px 0 !important;
    width: 100% !important;
    display: inline-flex;
    display: -webkit-inline-flex;
    }

    .sfsi_plus_subscription_form_field input {
    width: 100% !important;
    padding: 10px 0px !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=email] {
    font-family: Helvetica,Arial,sans-serif !important;

    font-style: normal !important;
    color:  !important;
    font-size: 14px !important;
    text-align: center !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=email]::-webkit-input-placeholder {
    font-family: Helvetica,Arial,sans-serif !important;

    font-style: normal !important;
    color:  !important;
    font-size: 14px !important;
    text-align: center !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=email]:-moz-placeholder {
    /* Firefox 18- */
    font-family: Helvetica,Arial,sans-serif !important;

    font-style: normal !important;
    color:  !important;
    font-size: 14px !important;
    text-align: center !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=email]::-moz-placeholder {
    /* Firefox 19+ */
    font-family: Helvetica,Arial,sans-serif !important;

    font-style: normal !important;
    color:  !important;
    font-size: 14px !important;
    text-align: center !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=email]:-ms-input-placeholder {
    font-family: Helvetica,Arial,sans-serif !important;

    font-style: normal !important;
    color:  !important;
    font-size: 14px !important;
    text-align: center !important;
    }

    .sfsi_plus_subscribe_Popinner input[type=submit] {
    font-family: Helvetica,Arial,sans-serif !important;

    font-weight: bold !important;
    color: #000000 !important;
    font-size: 16px !important;
    text-align: center !important;
    background-color: #dedede !important;
    }
    </style><style class="darkreader darkreader--sync" media="screen"></style>
    <meta name="follow.it-verification-code-ZmZ0MVc4dEE4eFdERXhqdWpobkdSc01jSTFlTFVqV3JaZURjRDdyR1Fvb3B3SWF3OTBoWWUyTGhxN0p3U2ppdnc2R1NGT1RzQk9oRVkvWmxoSXNPT00zMEpvc1V4aVhwMFZQcjcyQVZwcXFsWHhOZlFyRW9WZm1NaXNmeVBobjV8MDgwQ0J5SEN5YjZ5emNlYnE1NGRLNWJpVDZ3UHlNSWI2T0ZQMlhMZDRobz0=" content="QLZNgpHBlzrlDWvthuSm"> <meta name="viewport" content="width=device-width, initial-scale=1"><meta itemprop="author" content="LvMID"><meta itemprop="datePublished" content="19. Oktober 2017"><meta itemprop="dateModified" content="13. Juni 2022"><meta itemprop="publisher" content="Ludwig von Mises Institut Deutschland">        <style>
    .molongui-disabled-link
    {
    border-bottom: none !important;
    text-decoration: none !important;
    color: inherit !important;
    cursor: inherit !important;
    }
    .molongui-disabled-link:hover,
    .molongui-disabled-link:hover span
    {
    border-bottom: none !important;
    text-decoration: none !important;
    color: inherit !important;
    cursor: inherit !important;
    }
    </style><style class="darkreader darkreader--sync" media="screen"></style>
    <!-- Analytics by WP Statistics v14.2 - https://wp-statistics.com/ -->
    <script> var isTest = false; </script><meta name="generator" content="Powered by WPBakery Page Builder - drag and drop page builder for WordPress.">
    <meta name="generator" content="Jupiter 6.10.2"><style type="text/css" data-type="vc_shortcodes-custom-css">.vc_custom_1528725039210{margin-bottom: 0px !important;}</style><style class="darkreader darkreader--sync" media="screen"></style><noscript><style> .wpb_animate_when_almost_visible { opacity: 1; }</style></noscript>	<style type="text/css">@keyframes resizeanim { from { opacity: 0; } to { opacity: 0; } } .resize-triggers { animation: 1ms resizeanim; visibility: hidden; opacity: 0; } .resize-triggers, .resize-triggers > div, .contract-trigger:before { content: " "; display: block; position: absolute; top: 0; left: 0; height: 100%; width: 100%; overflow: hidden; } .resize-triggers > div { background: #eee; overflow: auto; } .contract-trigger:before { width: 200%; height: 200%; }</style><style class="darkreader darkreader--sync" media="screen"></style><style type="text/css">[responsive-image] > img, [data-responsive-image] {overflow: hidden; padding: 0; } [responsive-image] > img, [data-responsive-image] > img {width: 100%;}
    @keyframes element-queries { 0% { visibility: inherit; } }
    .mk-process-steps {animation: 0.1s element-queries;}
    .mk-process-steps > .resize-sensor {min-width: 0px;}
    .mk-event-countdown-ul {animation: 0.1s element-queries;}
    .mk-event-countdown-ul > .resize-sensor {min-width: 1px;}</style><style class="darkreader darkreader--sync" media="screen"></style><meta http-equiv="Cache-Control" content="no-cache, no-store, must-revalidate"><meta http-equiv="Pragma" content="no-cache"><meta http-equiv="Expires" content="0"></head>

    <body class="home page-template-default page page-id-18800 _masterslider _msp_version_3.6.5 sfsi_plus_actvite_theme_flat wpb-js-composer js-comp-ver-7.0 vc_responsive sfsi_plus_3.53" itemscope="itemscope" itemtype="https://schema.org/WebPage" data-adminbar="">

    <!-- Target for scroll anchors to achieve native browser bahaviour + possible enhancements like smooth scrolling -->
    <div id="top-of-page"></div>

    <div id="mk-boxed-layout">

    <div id="mk-theme-container">


    <header data-height="103" data-sticky-height="55" data-responsive-height="90" data-transparent-skin="" data-header-style="2" data-sticky-style="fixed" data-sticky-offset="header" id="mk-header-1" class="mk-header header-style-2 header-align-left toolbar-false menu-hover-5 sticky-style-fixed mk-background-stretch boxed-header a-sticky" role="banner" itemscope="itemscope" itemtype="https://schema.org/WPHeader">
    <div class="mk-header-holder">
    <div class="mk-header-inner">

    <div class="mk-header-bg mk-background-stretch"></div>


    <div class="mk-grid header-grid">
    <div class="add-header-height">

    <div class="mk-nav-responsive-link">
    <div class="mk-css-icon-menu">
    <div class="mk-css-icon-menu-line-1"></div>
    <div class="mk-css-icon-menu-line-2"></div>
    <div class="mk-css-icon-menu-line-3"></div>
    </div>
    </div>	<div class=" header-logo fit-logo-img add-header-height  logo-has-sticky">
    <a href="https://www.misesde.org/" title="Ludwig von Mises Institut Deutschland">

    <img class="mk-desktop-logo dark-logo " title="" alt="" src="https://www.misesde.org/wp-content/uploads/2018/06/mises_logo_header_neu_plain.png">



    <img class="mk-sticky-logo " title="" alt="" src="https://www.misesde.org/wp-content/uploads/2018/06/cropped-msthd_shield_cut.png">
    </a>
    </div>
    </div>

    </div>

    <div class="clearboth"></div>

    <div class="mk-header-nav-container menu-hover-style-5" role="navigation" itemscope="itemscope" itemtype="https://schema.org/SiteNavigationElement">
    <div class="mk-classic-nav-bg"></div>
    <div class="mk-classic-menu-wrapper">
    <nav class="mk-main-navigation js-main-nav"><ul id="menu-hauptmenue" class="main-navigation-ul dropdownJavascript"><li id="menu-item-18813" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-home current-menu-item page_item page-item-18800 current_page_item no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/">Home</a></li>
    <li id="menu-item-23145" class="menu-item menu-item-type-taxonomy menu-item-object-category no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/veranstaltungen/">Veranstaltungen</a></li>
    <li id="menu-item-3147" class="menu-item menu-item-type-post_type menu-item-object-page no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/autoren/">Autoren</a></li>
    <li id="menu-item-22521" class="menu-item menu-item-type-custom menu-item-object-custom menu-item-has-children no-mega-menu"><a class="menu-item-link js-smooth-scroll" href=# aria-haspopup="true">Medien</a>
    <ul style="" class="sub-menu ">
    <li id="menu-item-3140" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/videos/"><svg class="mk-svg-icon" data-name="mk-li-video" data-cacheid="icon-65250e97821a5" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M83.383 224.521c43.345 0 78.476-35.138 78.476-78.476s-35.131-78.468-78.476-78.468c-43.346 0-78.476 35.131-78.476 78.468 0 43.338 35.13 78.476 78.476 78.476zm0-125.554c25.965 0 47.086 21.121 47.086 47.078 0 25.958-21.121 47.086-47.086 47.086s-47.086-21.128-47.086-47.086c.001-25.957 21.122-47.078 47.086-47.078zm219.703 125.631c60.673 0 109.866-49.186 109.866-109.859 0-60.674-49.194-109.866-109.866-109.866-60.682 0-109.866 49.193-109.866 109.866-.001 60.673 49.184 109.859 109.866 109.859zm0-188.335c43.268 0 78.476 35.207 78.476 78.476s-35.208 78.468-78.476 78.468c-43.269 0-78.476-35.199-78.476-78.468s35.206-78.476 78.476-78.476zm172.647 251.131c-6.905 0-13.236 2.292-18.416 6.077v-.108l-60.06 44.373v-34.646c0-26.003-21.083-47.086-47.086-47.086h-298.209c-26.003 0-47.086 21.083-47.086 47.086v156.952c0 26.002 21.082 47.086 47.086 47.086h298.209c26.002 0 47.086-21.083 47.086-47.086v-35.23l60.819 45.476c5.027 3.433 11.104 5.449 17.657 5.449 17.335 0 31.39-14.048 31.39-31.39v-125.562c0-17.343-14.055-31.391-31.39-31.391zm-109.867 50.343v122.304c0 8.652-7.043 15.695-15.695 15.695h-298.209c-8.653 0-15.695-7.043-15.695-15.695v-156.951c0-8.653 7.042-15.695 15.695-15.695h298.209c8.652 0 15.695 7.042 15.695 15.695v34.647zm50.19 61.938l-24.754-18.5 84.431-62.383.045 125.531-59.722-44.648z"></path></svg>Videos</a></li>
    <li id="menu-item-25502" class="menu-item menu-item-type-post_type_archive menu-item-object-podcast"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/podcast/"><svg class="mk-svg-icon" data-name="mk-li-volume" data-cacheid="icon-65250e978227a" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M445.097 106.188c-6.131-6.127-16.062-6.127-22.194 0-6.131 6.127-6.131 16.067 0 22.195 70.376 70.383 70.376 184.893 0 255.261-6.131 6.131-6.131 16.071 0 22.194 3.065 3.066 7.081 4.599 11.097 4.599 4.016 0 8.031-1.533 11.096-4.599 82.616-82.606 82.616-217.035.001-299.65zm-44.395 44.392c-6.131-6.131-16.063-6.131-22.194 0-6.131 6.127-6.131 16.066 0 22.19 45.898 45.905 45.898 120.588.008 166.486-6.131 6.131-6.131 16.062 0 22.194 3.065 3.065 7.081 4.598 11.096 4.598 4.016 0 8.032-1.533 11.097-4.598 58.137-58.136 58.129-152.737-.007-210.87zm-44.396 44.392c-6.131-6.123-16.063-6.123-22.195.008-6.131 6.131-6.131 16.063 0 22.194 21.42 21.412 21.42 56.267.008 77.694-6.131 6.131-6.131 16.063 0 22.194 3.066 3.066 7.082 4.599 11.097 4.599 4.016 0 8.031-1.533 11.097-4.599 33.659-33.659 33.652-88.431-.007-122.09zm-54.956-188.871c-1.954-.82-4.016-1.222-6.061-1.222-4.031 0-8.008 1.551-11.005 4.49l-139.419 136.767h-92.838c-26.002 0-47.086 21.083-47.086 47.082v125.562c0 26.01 21.083 47.086 47.086 47.086h95.673l136.467 136.659c3.004 3.004 7.02 4.598 11.104 4.598 2.023 0 4.062-.383 6.001-1.188 5.87-2.429 9.694-8.154 9.694-14.507v-470.854c.002-6.323-3.792-12.024-9.616-14.473zm-186.542 171.421v156.952h-31.39v-156.952h31.39zm-78.476 141.257v-125.562c0-8.645 7.043-15.695 15.695-15.695h15.695v156.952h-15.695c-8.652 0-15.695-7.035-15.695-15.695zm243.245 134.727l-109.667-109.82c-5.886-5.901-13.878-9.212-22.209-9.212h-17.197v-156.952h14.362c8.223 0 16.117-3.219 21.98-8.982l112.731-110.582v395.548z"></path></svg>Podcasts</a></li>
    </ul>
    </li>
    <li id="menu-item-3137" class="menu-item menu-item-type-taxonomy menu-item-object-category menu-item-has-children no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/buecher/" aria-haspopup="true">Bücher</a>
    <ul style="" class="sub-menu ">
    <li id="menu-item-3139" class="menu-item menu-item-type-taxonomy menu-item-object-category"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/interviews/">Interviews</a></li>
    </ul>
    </li>
    <li id="menu-item-18839" class="menu-item menu-item-type-post_type menu-item-object-page no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/charts-neu/">Charts / Studien</a></li>
    <li id="menu-item-3156" class="menu-item menu-item-type-post_type menu-item-object-page no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/unterstutzen/">Unterstützen</a></li>
    <li id="menu-item-18803" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-has-children no-mega-menu"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/uber-den-autor/" aria-haspopup="true">Über uns</a>
    <ul style="" class="sub-menu ">
    <li id="menu-item-3141" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/beispiel-seite/">Kontakt</a></li>
    </ul>
    </li>
    </ul></nav>
    <div class="main-nav-side-search">
    <a class="mk-search-trigger  mk-fullscreen-trigger" href=#><i class="mk-svg-icon-wrapper"><svg class="mk-svg-icon" data-name="mk-icon-search" data-cacheid="icon-65250e978253e" style=" height:16px; width: 14.857142857143px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1664 1792"><path d="M1152 832q0-185-131.5-316.5t-316.5-131.5-316.5 131.5-131.5 316.5 131.5 316.5 316.5 131.5 316.5-131.5 131.5-316.5zm512 832q0 52-38 90t-90 38q-54 0-90-38l-343-342q-179 124-399 124-143 0-273.5-55.5t-225-150-150-225-55.5-273.5 55.5-273.5 150-225 225-150 273.5-55.5 273.5 55.5 225 150 150 225 55.5 273.5q0 220-124 399l343 343q37 37 37 90z"></path></svg></i></a>
    </div>

    </div>
    </div>


    <div class="mk-header-right">
    <div class="mk-header-social header-section show"><ul><li><a class="mk-square-rounded twitter-hover small" target="_blank" rel="noreferrer noopener" href="https://twitter.com/MisesInstitut"><svg class="mk-svg-icon" data-name="mk-jupiter-icon-simple-twitter" data-cacheid="icon-65250e9782663" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M454.058 213.822c28.724-2.382 48.193-15.423 55.683-33.132-10.365 6.373-42.524 13.301-60.269 6.681-.877-4.162-1.835-8.132-2.792-11.706-13.527-49.679-59.846-89.698-108.382-84.865 3.916-1.589 7.914-3.053 11.885-4.388 5.325-1.923 36.678-7.003 31.749-18.079-4.176-9.728-42.471 7.352-49.672 9.597 9.501-3.581 25.26-9.735 26.93-20.667-14.569 1.991-28.901 8.885-39.937 18.908 3.998-4.293 7.01-9.536 7.666-15.171-38.91 24.85-61.624 74.932-80.025 123.523-14.438-13.972-27.239-25.008-38.712-31.114-32.209-17.285-70.722-35.303-131.156-57.736-1.862 19.996 9.899 46.591 43.723 64.273-7.325-.986-20.736 1.219-31.462 3.773 4.382 22.912 18.627 41.805 57.251 50.918-17.642 1.163-26.767 5.182-35.036 13.841 8.043 15.923 27.656 34.709 62.931 30.82-39.225 16.935-15.998 48.234 15.93 43.565-54.444 56.244-140.294 52.123-189.596 5.08 128.712 175.385 408.493 103.724 450.21-65.225 31.23.261 49.605-10.823 60.994-23.05-17.99 3.053-44.072-.095-57.914-5.846z"></path></svg></a></li><li><a class="mk-square-rounded facebook-hover small" target="_blank" rel="noreferrer noopener" href="https://www.facebook.com/ludwigvonmisesinstitut"><svg class="mk-svg-icon" data-name="mk-jupiter-icon-simple-facebook" data-cacheid="icon-65250e97826d1" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M192.191 92.743v60.485h-63.638v96.181h63.637v256.135h97.069v-256.135h84.168s6.674-51.322 9.885-96.508h-93.666v-42.921c0-8.807 11.565-20.661 23.01-20.661h71.791v-95.719h-83.57c-111.317 0-108.686 86.262-108.686 99.142z"></path></svg></a></li><li><a class="mk-square-rounded youtube-hover small" target="_blank" rel="noreferrer noopener" href="https://www.youtube.com/user/misesde/feed"><svg class="mk-svg-icon" data-name="mk-jupiter-icon-simple-youtube" data-cacheid="icon-65250e9782738" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M146.112 194.063h31.18l.036-107.855 36.879-92.4h-34.136l-19.588 68.63-19.881-68.82h-33.762l39.219 92.627zm257.78 157.717c0-7.255-5.968-13.18-13.282-13.18h-1.769c-7.285 0-13.253 5.925-13.253 13.18l-.118 16.326h28.103l.32-16.326zm-192.18-214.16c0 12.324.594 21.577 1.851 27.736 1.236 6.151 3.284 11.439 6.202 15.755 2.897 4.323 6.948 7.599 12.2 9.75 5.237 2.187 11.578 3.218 19.119 3.218 6.744 0 12.727-1.236 17.95-3.76 5.164-2.508 9.42-6.443 12.726-11.695 3.335-5.325 5.514-10.986 6.51-17.094 1.009-6.093 1.536-15.688 1.536-28.738v-35.562c0-10.306-.557-17.956-1.654-23.025-1.082-5.002-3.115-9.889-6.113-14.643-2.956-4.74-7.198-8.587-12.698-11.534-5.471-2.948-12.04-4.448-19.682-4.448-9.099 0-16.574 2.312-22.418 6.92-5.865 4.587-9.918 10.679-12.156 18.25-2.231 7.599-3.373 18.138-3.373 31.64v37.23zm25.9-56.232c0-7.951 5.932-14.453 13.151-14.453 7.227 0 13.107 6.502 13.107 14.453v74.861c0 7.965-5.88 14.475-13.107 14.475-7.219 0-13.151-6.51-13.151-14.475v-74.861zm60.562 251.726c-7.139 0-12.976 4.798-12.976 10.664v79.374c0 5.866 5.836 10.635 12.976 10.635 7.137 0 12.99-4.769 12.99-10.635v-79.374c0-5.866-5.851-10.664-12.99-10.664zm13.75-153.306c1.536 3.73 3.921 6.743 7.139 9.018 3.188 2.238 7.269 3.372 12.142 3.372 4.286 0 8.06-1.156 11.366-3.54 3.291-2.377 6.072-5.917 8.323-10.649l-.557 11.644h33.06v-140.623h-26.039v109.443c0 5.931-4.871 10.773-10.839 10.773-5.94 0-10.825-4.842-10.825-10.773v-109.443h-27.193v94.844c0 12.083.219 20.135.584 24.224.381 4.053 1.317 7.951 2.838 11.711zm87.595 43.066h-287.031c-38.406 0-69.814 29.652-69.814 65.857v150.994c0 36.221 31.407 65.858 69.814 65.858h287.031c38.385 0 69.808-29.637 69.808-65.858v-150.994c0-36.205-31.422-65.857-69.808-65.857zm-297.577 233.236v-159.494l-29.609-.087v-23.172l94.857.161v23.551h-35.591l.023 159.041h-29.68zm136.35-.029l-23.829-.031.066-17.553c-6.407 13.751-31.977 24.824-45.333 15.185-7.154-5.135-6.898-14.13-7.63-21.856-.387-4.373-.065-13.999-.101-26.902l-.088-84.17h29.512l.117 85.531c0 11.659-.629 18.461.081 20.714 4.243 12.858 15.09 5.881 17.496-.717.775-2.164.029-8.308.029-20.596v-84.932h29.681v135.327zm44.215-12.801l-2.223 11.294-24.372.365.147-181.406 29.636-.06-.103 52.575c27.356-21.81 47.512-5.661 47.542 21.269l.06 70.714c.043 34.244-19.544 53.817-50.688 25.248zm68.578-34.537v-42.129c0-12.656 1.242-22.617 3.774-29.901 2.5-7.285 6.817-12.713 12.447-16.764 17.978-12.96 53.526-8.938 57.169 16.399 1.156 8.017 1.536 22.015 1.536 36.031v19.163h-50.952v32.635c0 6.656 5.486 12.053 12.173 12.053h4.358c6.657 0 12.144-5.397 12.144-12.053v-12.404c.014-1.098.043-2.106.058-2.999l22.25-.117c10.151 60.269-74.956 70.173-74.956.088z"></path></svg></a></li></ul><div class="clearboth"></div></div>                    </div>

    <div class="mk-responsive-wrap">

    <nav class="menu-hauptmenue-container"><ul id="menu-hauptmenue-1" class="mk-responsive-nav"><li id="responsive-menu-item-18813" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-home current-menu-item page_item page-item-18800 current_page_item"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/">Home</a></li>
    <li id="responsive-menu-item-23145" class="menu-item menu-item-type-taxonomy menu-item-object-category"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/veranstaltungen/">Veranstaltungen</a></li>
    <li id="responsive-menu-item-3147" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/autoren/">Autoren</a></li>
    <li id="responsive-menu-item-22521" class="menu-item menu-item-type-custom menu-item-object-custom menu-item-has-children"><a class="menu-item-link js-smooth-scroll" href=#>Medien</a><span class="mk-nav-arrow mk-nav-sub-closed"><svg class="mk-svg-icon" data-name="mk-moon-arrow-down" data-cacheid="icon-65250e9783276" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 192l-96-96-160 160-160-160-96 96 256 255.999z"></path></svg></span>
    <ul class="sub-menu ">
    <li id="responsive-menu-item-3140" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/videos/"><svg class="mk-svg-icon" data-name="mk-li-video" data-cacheid="icon-65250e97832fd" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M83.383 224.521c43.345 0 78.476-35.138 78.476-78.476s-35.131-78.468-78.476-78.468c-43.346 0-78.476 35.131-78.476 78.468 0 43.338 35.13 78.476 78.476 78.476zm0-125.554c25.965 0 47.086 21.121 47.086 47.078 0 25.958-21.121 47.086-47.086 47.086s-47.086-21.128-47.086-47.086c.001-25.957 21.122-47.078 47.086-47.078zm219.703 125.631c60.673 0 109.866-49.186 109.866-109.859 0-60.674-49.194-109.866-109.866-109.866-60.682 0-109.866 49.193-109.866 109.866-.001 60.673 49.184 109.859 109.866 109.859zm0-188.335c43.268 0 78.476 35.207 78.476 78.476s-35.208 78.468-78.476 78.468c-43.269 0-78.476-35.199-78.476-78.468s35.206-78.476 78.476-78.476zm172.647 251.131c-6.905 0-13.236 2.292-18.416 6.077v-.108l-60.06 44.373v-34.646c0-26.003-21.083-47.086-47.086-47.086h-298.209c-26.003 0-47.086 21.083-47.086 47.086v156.952c0 26.002 21.082 47.086 47.086 47.086h298.209c26.002 0 47.086-21.083 47.086-47.086v-35.23l60.819 45.476c5.027 3.433 11.104 5.449 17.657 5.449 17.335 0 31.39-14.048 31.39-31.39v-125.562c0-17.343-14.055-31.391-31.39-31.391zm-109.867 50.343v122.304c0 8.652-7.043 15.695-15.695 15.695h-298.209c-8.653 0-15.695-7.043-15.695-15.695v-156.951c0-8.653 7.042-15.695 15.695-15.695h298.209c8.652 0 15.695 7.042 15.695 15.695v34.647zm50.19 61.938l-24.754-18.5 84.431-62.383.045 125.531-59.722-44.648z"></path></svg>Videos</a></li>
    <li id="responsive-menu-item-25502" class="menu-item menu-item-type-post_type_archive menu-item-object-podcast"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/podcast/"><svg class="mk-svg-icon" data-name="mk-li-volume" data-cacheid="icon-65250e97833ae" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M445.097 106.188c-6.131-6.127-16.062-6.127-22.194 0-6.131 6.127-6.131 16.067 0 22.195 70.376 70.383 70.376 184.893 0 255.261-6.131 6.131-6.131 16.071 0 22.194 3.065 3.066 7.081 4.599 11.097 4.599 4.016 0 8.031-1.533 11.096-4.599 82.616-82.606 82.616-217.035.001-299.65zm-44.395 44.392c-6.131-6.131-16.063-6.131-22.194 0-6.131 6.127-6.131 16.066 0 22.19 45.898 45.905 45.898 120.588.008 166.486-6.131 6.131-6.131 16.062 0 22.194 3.065 3.065 7.081 4.598 11.096 4.598 4.016 0 8.032-1.533 11.097-4.598 58.137-58.136 58.129-152.737-.007-210.87zm-44.396 44.392c-6.131-6.123-16.063-6.123-22.195.008-6.131 6.131-6.131 16.063 0 22.194 21.42 21.412 21.42 56.267.008 77.694-6.131 6.131-6.131 16.063 0 22.194 3.066 3.066 7.082 4.599 11.097 4.599 4.016 0 8.031-1.533 11.097-4.599 33.659-33.659 33.652-88.431-.007-122.09zm-54.956-188.871c-1.954-.82-4.016-1.222-6.061-1.222-4.031 0-8.008 1.551-11.005 4.49l-139.419 136.767h-92.838c-26.002 0-47.086 21.083-47.086 47.082v125.562c0 26.01 21.083 47.086 47.086 47.086h95.673l136.467 136.659c3.004 3.004 7.02 4.598 11.104 4.598 2.023 0 4.062-.383 6.001-1.188 5.87-2.429 9.694-8.154 9.694-14.507v-470.854c.002-6.323-3.792-12.024-9.616-14.473zm-186.542 171.421v156.952h-31.39v-156.952h31.39zm-78.476 141.257v-125.562c0-8.645 7.043-15.695 15.695-15.695h15.695v156.952h-15.695c-8.652 0-15.695-7.035-15.695-15.695zm243.245 134.727l-109.667-109.82c-5.886-5.901-13.878-9.212-22.209-9.212h-17.197v-156.952h14.362c8.223 0 16.117-3.219 21.98-8.982l112.731-110.582v395.548z"></path></svg>Podcasts</a></li>
    </ul>
    </li>
    <li id="responsive-menu-item-3137" class="menu-item menu-item-type-taxonomy menu-item-object-category menu-item-has-children"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/buecher/">Bücher</a><span class="mk-nav-arrow mk-nav-sub-closed"><svg class="mk-svg-icon" data-name="mk-moon-arrow-down" data-cacheid="icon-65250e9783597" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 192l-96-96-160 160-160-160-96 96 256 255.999z"></path></svg></span>
    <ul class="sub-menu ">
    <li id="responsive-menu-item-3139" class="menu-item menu-item-type-taxonomy menu-item-object-category"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/category/interviews/">Interviews</a></li>
    </ul>
    </li>
    <li id="responsive-menu-item-18839" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/charts-neu/">Charts / Studien</a></li>
    <li id="responsive-menu-item-3156" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/unterstutzen/">Unterstützen</a></li>
    <li id="responsive-menu-item-18803" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-has-children"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/uber-den-autor/">Über uns</a><span class="mk-nav-arrow mk-nav-sub-closed"><svg class="mk-svg-icon" data-name="mk-moon-arrow-down" data-cacheid="icon-65250e978380c" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 192l-96-96-160 160-160-160-96 96 256 255.999z"></path></svg></span>
    <ul class="sub-menu ">
    <li id="responsive-menu-item-3141" class="menu-item menu-item-type-post_type menu-item-object-page"><a class="menu-item-link js-smooth-scroll" href="https://www.misesde.org/beispiel-seite/">Kontakt</a></li>
    </ul>
    </li>
    </ul></nav>
    <form class="responsive-searchform" method="get" action="https://www.misesde.org/">
    <input type="text" class="text-input" value="" name="s" id="s" placeholder="Suche...">
    <i><input value="" type="submit"><svg class="mk-svg-icon" data-name="mk-icon-search" data-cacheid="icon-65250e978396b" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1664 1792"><path d="M1152 832q0-185-131.5-316.5t-316.5-131.5-316.5 131.5-131.5 316.5 131.5 316.5 316.5 131.5 316.5-131.5 131.5-316.5zm512 832q0 52-38 90t-90 38q-54 0-90-38l-343-342q-179 124-399 124-143 0-273.5-55.5t-225-150-150-225-55.5-273.5 55.5-273.5 150-225 225-150 273.5-55.5 273.5 55.5 225 150 150 225 55.5 273.5q0 220-124 399l343 343q37 37 37 90z"></path></svg></i>
    </form>


    </div>

    </div>
    </div>
    <div class="mk-header-padding-wrapper"></div>

    </header>

    <div id="theme-page" class="master-holder  clearfix" itemscope="itemscope" itemtype="https://schema.org/Blog">
    <div class="master-holder-bg-holder">
    <div id="theme-page-bg" class="master-holder-bg js-el"></div>
    </div>
    <div class="mk-main-wrapper-holder">
    <div id="mk-page-id-18800" class="theme-page-wrapper mk-main-wrapper mk-grid full-layout false">
    <div class="theme-content false" itemprop="mainEntityOfPage">
    <section class="wpb-content-wrapper">
    <div data-mk-stretch-content="true" class="wpb_row vc_row vc_row-fluid jupiter-donut- mk-fullwidth-false attched-false js-master-row mk-grid mk-in-viewport">

    <div class="vc_col-sm-12 wpb_column column_container  jupiter-donut- _ jupiter-donut-height-full">

    <div class="wpb_raw_code wpb_content_element wpb_raw_html">
    <div class="wpb_wrapper">
    <div id="div-slider-box">
    <div id="div-slider-left" class="class-slider-box">

    <!-- MasterSlider -->
    <div id="P_MS6524fd8999b8d" class="master-slider-parent ms-parent-id-17" style="position: relative;">


    <!-- MasterSlider Main -->
    <div id="MS6524fd8999b8d" class="master-slider ms-skin-default ms-moz ms-scroll-parallax" style="visibility: visible; opacity: 1; margin: 0px;"><div class="ms-container"><div class="ms-inner-controls-cont"><div class="ms-view ms-fade-basic-view ms-grab-cursor" style="width: 738px; height: 370px;"><div class="ms-slide-container" style="transform-style: preserve-3d; transform: translateX(0px) translateZ(0px);"><div class="ms-slide ms-slide-post-30992 ms-sl-selected" data-delay="10" data-fill-mode="fill" style="width: 738px; height: 370px; left: 0px; opacity: 1; z-index: 1;">

    <a href="https://www.misesde.org/2023/10/zu-ehren-ludwig-von-mises-50-todestag/" target="_self" class="ms-slide-link"></a>








    <div class="ms-slide-bgcont" style="height: 100%; top: 352px; opacity: 1; position: fixed;"><img src="https://www.misesde.org/wp-content/uploads/2023/10/20231008-mises-1024x576.jpg" alt="Zu Ehren Ludwig von Mises | 50. Todestag" title="Zu Ehren Ludwig von Mises | 50. Todestag" style="width: 738px; height: 415.125px; margin-top: -22.5px; margin-left: 0px;"></div><div class="ms-scroll-parallax-cont" style="opacity: 0; transform: translateY(211.2px) translateZ(0.4px);"><div class="ms-slide-layers" style="left: 0px; max-width: 800px;"><div class="ms-anim-layers" style="transition-duration: 500ms; transition-property: all; transition-timing-function: linear;"><div class="ms-layer msp-cn-12-16 ms-hover-active" style="width: 800px; height: 370px; top: 0px; left: 0px; transform-origin: 50% 50% 0px;">
    </div><div class="ms-layer msp-cn-17-14 ms-hover-active" style="bottom: 9.225px; left: 9.225px; margin: 0px; padding: 0px 0px 9.225px; font-size: 22.14px; transform-origin: 50% 50% 0px;">
    <span style="font-family: tahoma, arial, helvetica, sans-serif; font-size: 10pt; color: rgb(181, 181, 181); --darkreader-inline-color: #b9b3aa;" data-darkreader-inline-color="">10. Oktober 2023 | George Reisman</span></div><div class="ms-layer msp-cn-17-1 ms-hover-active" style="bottom: 18.45px; left: 9.225px; margin: 0px; padding: 0px; font-size: 36.9px; transform-origin: 50% 50% 0px;">
    <p><span style="color: rgb(255, 255, 255); font-family: tahoma, arial, helvetica, sans-serif; font-size: 18pt; --darkreader-inline-color: #e8e6e3;" data-darkreader-inline-color="">Zu Ehren Ludwig von Mises | 50. Todestag</span></p></div></div></div></div></div></div></div></div></div>



    </div>
    <!-- END MasterSlider Main -->


    </div>
    <!-- END MasterSlider -->

    <script>
    ( window.MSReady = window.MSReady || [] ).push( function( $ ) {

    "use strict";
    var masterslider_9b8d = new MasterSlider();

    // slider controls
    // slider setup
    masterslider_9b8d.setup("MS6524fd8999b8d", {
    width           : 800,
    height          : 370,
    minHeight       : 370,
    space           : 10,
    start           : 1,
    grabCursor      : true,
    swipe           : true,
    mouse           : true,
    keyboard        : false,
    layout          : "fillwidth",
    wheel           : false,
    autoplay        : false,
    instantStartLayers:true,
    mobileBGVideo:false,
    loop            : true,
    shuffle         : false,
    preload         : 0,
    heightLimit     : true,
    autoHeight      : false,
    smoothHeight    : true,
    endPause        : false,
    overPause       : true,
    fillMode        : "fill",
    centerControls  : false,
    startOnAppear   : false,
    layersMode      : "center",
    autofillTarget  : "",
    hideLayers      : false,
    fullscreenMargin: 0,
    speed           : 15,
    dir             : "h",
    responsive      : true,
    tabletWidth     : 768,
    tabletHeight    : null,
    phoneWidth      : 480,
    phoneHeight    : null,
    sizingReference : window,
    parallaxMode    : 'swipe',
    view            : "fadeBasic"
    });

    MSScrollParallax.setup( masterslider_9b8d, 30, 50, true );
    window.masterslider_instances = window.masterslider_instances || [];
    window.masterslider_instances.push( masterslider_9b8d );
    });
    </script>


    </div>
    <div id="div-slider-right">
    <div id="div-slider-right-top" class="class-slider-box">
    <!-- MasterSlider -->
    <div id="P_MS6524fd89abae3" class="master-slider-parent ms-tabs-template ms-parent-id-11" style="position: relative;">


    <!-- MasterSlider Main -->
    <div id="MS6524fd89abae3" class="master-slider ms-skin-default ms-moz ms-scroll-parallax" style="visibility: visible; opacity: 1; margin: 0px;"><div class="ms-container"><div class="ms-inner-controls-cont"><div class="ms-view ms-fade-basic-view ms-grab-cursor" style="width: 431px; height: 200px;"><div class="ms-slide-container" style="transform-style: preserve-3d; transform: translateX(0px) translateZ(0px);"><div class="ms-slide ms-slide-post-30983 ms-sl-selected" data-delay="10" data-fill-mode="fill" style="width: 431px; height: 200px; left: 0px; opacity: 1; z-index: 1;">

    <a href="https://www.misesde.org/2023/10/kuendigt-den-faustischen-fiatgeld-pakt-mises-momente-11/" target="_self" class="ms-slide-link"></a>








    <div class="ms-slide-bgcont" style="height: 100%; top: 352px; opacity: 1; position: fixed;"><img src="https://www.misesde.org/wp-content/uploads/2023/10/mm11_thumbnail_landscape-1024x576.png" alt="Kündigt den faustischen Fiatgeld Pakt! | MISES Momente #11" title="Kündigt den faustischen Fiatgeld Pakt! | MISES Momente #11" style="width: 431px; height: 242.438px; margin-top: -21px; margin-left: 0px;"></div><div class="ms-scroll-parallax-cont" style="opacity: 0; transform: translateY(211.2px) translateZ(0.4px);"><div class="ms-slide-layers" style="left: 0px; max-width: 800px;"><div class="ms-anim-layers" style="transition-duration: 500ms; transition-property: all; transition-timing-function: linear;"><div class="ms-layer msp-cn-11-16 ms-hover-active" style="top: 0px; left: 0px; margin: 0px; padding: 0px; font-size: 12.93px; transform-origin: 50% 50% 0px;">
    </div><div class="ms-layer msp-preset-18 ms-hover-active" style="bottom: 5.3875px; left: 0px; margin: 0px; padding: 0px 0px 5.3875px; font-size: 12.93px; transform-origin: 50% 50% 0px;">
    <span style="font-family: tahoma, arial, helvetica, sans-serif; font-size: 8pt; color: rgb(181, 181, 181); --darkreader-inline-color: #b9b3aa;" data-darkreader-inline-color="">4. Oktober 2023 | Johanna &amp; Manuel</span></div><div class="ms-layer msp-cn-10-1 ms-hover-active" style="bottom: 26.9375px; left: 5.3875px; margin: 0px; padding: 0px; font-size: 26.9375px; transform-origin: 50% 50% 0px;">
    <span style="color: rgb(255, 255, 255); font-family: tahoma, arial, helvetica, sans-serif; --darkreader-inline-color: #e8e6e3;" data-darkreader-inline-color="">Kündigt den faustischen Fiatgeld Pakt! | MISES Momente #11</span></div></div></div></div></div></div></div></div></div>



    </div>
    <!-- END MasterSlider Main -->


    </div>
    <!-- END MasterSlider -->

    <script>
    ( window.MSReady = window.MSReady || [] ).push( function( $ ) {

    "use strict";
    var masterslider_bae3 = new MasterSlider();

    // slider controls
    // slider setup
    masterslider_bae3.setup("MS6524fd89abae3", {
    width           : 800,
    height          : 230,
    minHeight       : 200,
    space           : 0,
    start           : 1,
    grabCursor      : true,
    swipe           : true,
    mouse           : true,
    keyboard        : false,
    layout          : "fillwidth",
    wheel           : false,
    autoplay        : false,
    instantStartLayers:true,
    mobileBGVideo:false,
    loop            : true,
    shuffle         : false,
    preload         : 0,
    heightLimit     : true,
    autoHeight      : false,
    smoothHeight    : true,
    endPause        : false,
    overPause       : true,
    fillMode        : "fill",
    centerControls  : false,
    startOnAppear   : false,
    layersMode      : "center",
    autofillTarget  : "",
    hideLayers      : false,
    fullscreenMargin: 0,
    speed           : 15,
    dir             : "h",
    responsive      : true,
    tabletWidth     : 768,
    tabletHeight    : null,
    phoneWidth      : 480,
    phoneHeight    : null,
    sizingReference : window,
    parallaxMode    : 'swipe',
    view            : "fadeBasic"
    });

    MSScrollParallax.setup( masterslider_bae3, 30, 50, true );
    window.masterslider_instances = window.masterslider_instances || [];
    window.masterslider_instances.push( masterslider_bae3 );
    });
    </script>

    </div>
    <div id="div-slider-right-b1" class="class-slider-box">
    <!-- MasterSlider -->
    <div id="P_MS6524fd89bb052" class="master-slider-parent ms-tabs-template ms-parent-id-10" style="position: relative;">


    <!-- MasterSlider Main -->
    <div id="MS6524fd89bb052" class="master-slider ms-skin-default ms-moz ms-scroll-parallax" style="visibility: visible; opacity: 1; margin: 0px;"><div class="ms-container"><div class="ms-inner-controls-cont"><div class="ms-view ms-fade-basic-view ms-grab-cursor" style="width: 207px; height: 160px;"><div class="ms-slide-container" style="transform-style: preserve-3d; transform: translateX(0px) translateZ(0px);"><div class="ms-slide ms-slide-post-30962 ms-sl-selected" data-delay="10" data-fill-mode="fill" style="width: 207px; height: 160px; left: 0px; opacity: 1; z-index: 1;">

    <a href="https://www.misesde.org/2023/10/mises-monatsmagazin-die-beitraege-im-september-2023/" target="_self" class="ms-slide-link"></a>








    <div class="ms-slide-bgcont" style="height: 100%; top: 245.55px; opacity: 1; position: fixed;"><img src="https://www.misesde.org/wp-content/uploads/2023/03/20230328-adobestock_56479643-800x800.jpeg" alt="MISES Monatsmagazin – Die Beiträge im September 2023" title="MISES Monatsmagazin – Die Beiträge im September 2023" style="width: 207px; height: 207px; margin-top: -23.5px; margin-left: 0px;"></div><div class="ms-scroll-parallax-cont" style="opacity: 0; transform: translateY(147.33px) translateZ(0.4px);"><div class="ms-slide-layers" style="left: 0px; max-width: 800px;"><div class="ms-anim-layers" style="transition-duration: 500ms; transition-property: all; transition-timing-function: linear;"><div class="ms-layer msp-cn-10-16 ms-hover-active" style="top: 0px; left: 0px; margin: 0px; padding: 0px; font-size: 6.21px; transform-origin: 50% 50% 0px;">
    </div><div class="ms-layer msp-cn-10-1 ms-hover-active" style="bottom: 15.525px; left: 2.5875px; margin: 0px; padding: 0px; font-size: 12.9375px; transform-origin: 50% 50% 0px;">
    <span style="color: rgb(255, 255, 255); --darkreader-inline-color: #e8e6e3;" data-darkreader-inline-color="">MISES Monatsmagazin - Die Beiträge im September 2023</span></div><div class="ms-layer msp-cn-10-17 ms-hover-active" style="bottom: 0px; left: 2.5875px; margin: 0px; padding: 0px 0px 2.5875px; font-size: 10.35px;">
    2. Oktober 2023</div></div></div></div></div></div></div></div></div>



    </div>
    <!-- END MasterSlider Main -->


    </div>
    <!-- END MasterSlider -->

    <script>
    ( window.MSReady = window.MSReady || [] ).push( function( $ ) {

    "use strict";
    var masterslider_b052 = new MasterSlider();

    // slider controls
    // slider setup
    masterslider_b052.setup("MS6524fd89bb052", {
    width           : 800,
    height          : 160,
    minHeight       : 160,
    space           : 0,
    start           : 1,
    grabCursor      : true,
    swipe           : true,
    mouse           : true,
    keyboard        : false,
    layout          : "fillwidth",
    wheel           : false,
    autoplay        : false,
    instantStartLayers:true,
    mobileBGVideo:false,
    loop            : true,
    shuffle         : false,
    preload         : 0,
    heightLimit     : true,
    autoHeight      : false,
    smoothHeight    : true,
    endPause        : false,
    overPause       : true,
    fillMode        : "fill",
    centerControls  : false,
    startOnAppear   : false,
    layersMode      : "center",
    autofillTarget  : "",
    hideLayers      : false,
    fullscreenMargin: 0,
    speed           : 15,
    dir             : "h",
    responsive      : true,
    tabletWidth     : 768,
    tabletHeight    : null,
    phoneWidth      : 480,
    phoneHeight    : null,
    sizingReference : window,
    parallaxMode    : 'swipe',
    view            : "fadeBasic"
    });

    MSScrollParallax.setup( masterslider_b052, 30, 50, true );
    window.masterslider_instances = window.masterslider_instances || [];
    window.masterslider_instances.push( masterslider_b052 );
    });
    </script>

    </div>
    <div id="div-slider-right-b2" class="class-slider-box">
    <!-- MasterSlider -->
    <div id="P_MS6524fd89ca3aa" class="master-slider-parent ms-tabs-template ms-parent-id-13" style="position: relative;">


    <!-- MasterSlider Main -->
    <div id="MS6524fd89ca3aa" class="master-slider ms-skin-default ms-moz ms-scroll-parallax" style="visibility: visible; opacity: 1; margin: 0px;"><div class="ms-container"><div class="ms-inner-controls-cont"><div class="ms-view ms-fade-basic-view ms-grab-cursor" style="width: 207px; height: 160px;"><div class="ms-slide-container" style="transform-style: preserve-3d; transform: translateX(0px) translateZ(0px);"><div class="ms-slide ms-slide-post-30944 ms-sl-selected" data-delay="10" data-fill-mode="fill" style="width: 207px; height: 160px; left: 0px; opacity: 1; z-index: 1;">

    <a href="https://www.misesde.org/2023/09/mises-spezial-142-geburtstag-ludwig-von-mises/" target="_self" class="ms-slide-link"></a>








    <div class="ms-slide-bgcont" style="height: 100%; top: 245.55px; opacity: 1; position: fixed;"><img src="https://www.misesde.org/wp-content/uploads/2023/09/20230920-thumbnail_youtube-800x800.png" alt="MISES Spezial | 142. Geburtstag Ludwig von Mises" title="MISES Spezial | 142. Geburtstag Ludwig von Mises" style="width: 207px; height: 207px; margin-top: -23.5px; margin-left: 0px;"></div><div class="ms-scroll-parallax-cont" style="opacity: 0; transform: translateY(147.33px) translateZ(0.4px);"><div class="ms-slide-layers" style="left: 0px; max-width: 800px;"><div class="ms-anim-layers" style="transition-duration: 500ms; transition-property: all; transition-timing-function: linear;"><div class="ms-layer msp-cn-10-16 ms-hover-active" style="top: 0px; left: 0px; margin: 0px; padding: 0px; font-size: 6.21px; transform-origin: 50% 50% 0px;">
    </div><div class="ms-layer msp-preset-17 ms-hover-active" style="bottom: 15.525px; left: 2.5875px; margin: 0px; padding: 0px; font-size: 12.9375px; transform-origin: 50% 50% 0px;">
    <span style="color: rgb(255, 255, 255); --darkreader-inline-color: #e8e6e3;" data-darkreader-inline-color="">MISES Spezial | 142. Geburtstag Ludwig von Mises</span></div><div class="ms-layer msp-preset-19 ms-hover-active" style="bottom: 0px; left: 2.5875px; margin: 0px; padding: 0px 0px 2.5875px; font-size: 10.35px;">
    29. September 2023</div></div></div></div></div></div></div></div></div>



    </div>
    <!-- END MasterSlider Main -->


    </div>
    <!-- END MasterSlider -->

    <script>
    ( window.MSReady = window.MSReady || [] ).push( function( $ ) {

    "use strict";
    var masterslider_a3aa = new MasterSlider();

    // slider controls
    // slider setup
    masterslider_a3aa.setup("MS6524fd89ca3aa", {
    width           : 800,
    height          : 160,
    minHeight       : 160,
    space           : 0,
    start           : 1,
    grabCursor      : true,
    swipe           : true,
    mouse           : true,
    keyboard        : false,
    layout          : "fillwidth",
    wheel           : false,
    autoplay        : false,
    instantStartLayers:true,
    mobileBGVideo:false,
    loop            : true,
    shuffle         : false,
    preload         : 0,
    heightLimit     : true,
    autoHeight      : false,
    smoothHeight    : true,
    endPause        : false,
    overPause       : true,
    fillMode        : "fill",
    centerControls  : false,
    startOnAppear   : false,
    layersMode      : "center",
    autofillTarget  : "",
    hideLayers      : false,
    fullscreenMargin: 0,
    speed           : 15,
    dir             : "h",
    responsive      : true,
    tabletWidth     : 768,
    tabletHeight    : null,
    phoneWidth      : 480,
    phoneHeight    : null,
    sizingReference : window,
    parallaxMode    : 'swipe',
    view            : "fadeBasic"
    });

    MSScrollParallax.setup( masterslider_a3aa, 30, 50, true );
    window.masterslider_instances = window.masterslider_instances || [];
    window.masterslider_instances.push( masterslider_a3aa );
    });
    </script>

    </div>
    </div>
    </div>

    </div>
    </div>
    <div class="vc_empty_space" style="height: 12px"><span class="vc_empty_space_inner"></span></div></div>
    </div>

    <div data-mk-stretch-content="true" class="wpb_row vc_row vc_row-fluid jupiter-donut- mk-fullwidth-false attched-false js-master-row mk-grid mk-in-viewport">

    <div class="vc_col-sm-12 wpb_column column_container  jupiter-donut- _ jupiter-donut-height-full">
    <div class="vc_separator wpb_content_element vc_separator_align_center vc_sep_width_100 vc_sep_pos_align_center vc_sep_color_sky wpb_animate_when_almost_visible wpb_slideInRight slideInRight vc_separator-has-text wpb_start_animation animated"><span class="vc_sep_holder vc_sep_holder_l"><span class="vc_sep_line"></span></span><div class="vc_icon_element vc_icon_element-outer vc_icon_element-align-left"><div class="vc_icon_element-inner vc_icon_element-color-peacoc vc_icon_element-size-md vc_icon_element-style- vc_icon_element-background-color-grey"><span class="vc_icon_element-icon fa fa-book"></span></div></div><h4>Aktuelle Beiträge</h4><span class="vc_sep_holder vc_sep_holder_r"><span class="vc_sep_line"></span></span>
    </div></div>
    </div>

    <div data-mk-stretch-content="true" class="wpb_row vc_row vc_row-fluid jupiter-donut- mk-fullwidth-false attched-false js-master-row mk-grid mk-in-viewport">

    <div class="vc_col-sm-8 wpb_column column_container  jupiter-donut- _ jupiter-donut-height-full">
    <div class=" vc_custom_1528725039210">

    <div id="text-block-5" class="mk-text-block  jupiter-donut- ">


    <div class="pt-cv-wrapper"><div class="pt-cv-view pt-cv-grid pt-cv-colsys pt-cv-same-height pt-cv-post-border pt-cv-content-hover pt-cv-clickable pt-cv-force-mask pt-cv-overlay-bottom pt-cv-pgregular" id="pt-cv-view-dacd42bydp"><div data-id="pt-cv-page-1" class="pt-cv-page" data-cvc="2" data-cvct="2" data-cvcm="1"><div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30941"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 63.45px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2017/05/Puster.jpg" srcset="https://www.misesde.org/wp-content/uploads/2017/05/Puster.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Rolf W. Puster</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-27T07:54:54+02:00">27. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/podcast-ludwig-von-mises-hat-so-tief-wie-kaum-jemand-vor-ihm-ueber-die-natur-der-freiheit-nachgedacht/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230906-thumb-web1-520x360.png&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230906-thumb-web1-520x360.png" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: „Ludwig von Mises hat so tief wie kaum jemand vor ihm über die Natur der Freiheit nachgedacht“" decoding="async" fetchpriority="high" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/podcast-ludwig-von-mises-hat-so-tief-wie-kaum-jemand-vor-ihm-ueber-die-natur-der-freiheit-nachgedacht/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: „Ludwig von Mises hat so tief wie kaum jemand vor ihm über die Natur der Freiheit nachgedacht“</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30930"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 63.45px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2022/01/at2.jpg" srcset="https://www.misesde.org/wp-content/uploads/2022/01/at2.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Andreas Tögel</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-25T08:30:03+02:00">25. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/millionaerssteuer-und-gerechtigkeit/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230127-adobestock_19596834-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230127-adobestock_19596834-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Millionärssteuer und Gerechtigkeit" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/millionaerssteuer-und-gerechtigkeit/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Millionärssteuer und Gerechtigkeit</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30919"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 61.3833px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/11/sievert21-_-indischerseiltrick_ebook_2835px.jpeg" srcset="https://www.misesde.org/wp-content/uploads/2021/11/sievert21-_-indischerseiltrick_ebook_2835px.jpeg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Burkhard Sievert</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-22T09:18:14+02:00">22. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/wer-bestimmt-die-regeln/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230919-adobestock_98515510-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230919-adobestock_98515510-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Wer bestimmt die Regeln?" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/wer-bestimmt-die-regeln/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Wer bestimmt die Regeln?</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30914"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 61.3833px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/07/polleit_thorsten3.jpg" srcset="https://www.misesde.org/wp-content/uploads/2021/07/polleit_thorsten3.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Thorsten Polleit</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-20T07:52:48+02:00">20. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/podcast-die-falsche-priesterherrschaft-der-intellektuellen-und-die-macht-der-ideen/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230918-adobestock_622127990-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230918-adobestock_622127990-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: Die falsche Priesterherrschaft der Intellektuellen und die Macht der Ideen" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/podcast-die-falsche-priesterherrschaft-der-intellektuellen-und-die-macht-der-ideen/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: Die falsche Priesterherrschaft der Intellektuellen und die Macht der Ideen</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30903"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/09/fassnacht.png" srcset="https://www.misesde.org/wp-content/uploads/2021/09/fassnacht.png 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Rainer Fassnacht</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-18T07:54:12+02:00">18. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/was-hat-ein-belegtes-broetchen-mit-markt-und-freiheit-zu-tun/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230917-adobestock_54419654-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230917-adobestock_54419654-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Was hat ein belegtes Brötchen mit Markt und Freiheit zu tun?" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/was-hat-ein-belegtes-broetchen-mit-markt-und-freiheit-zu-tun/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Was hat ein belegtes Brötchen mit Markt und Freiheit zu tun?</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30874"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/12/kristoffer_hansen_2020.jpg" srcset="https://www.misesde.org/wp-content/uploads/2021/12/kristoffer_hansen_2020.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Kristoffer Mousten Hansen</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-15T09:47:15+02:00">15. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/will-die-fed-das-internationale-finanzsystem-retten-so-mag-es-vielleicht-aussehen/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230913-adobestock_611428443-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230913-adobestock_611428443-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Will die Fed das internationale Finanzsystem retten? So mag es vielleicht aussehen" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/will-die-fed-das-internationale-finanzsystem-retten-so-mag-es-vielleicht-aussehen/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Will die Fed das internationale Finanzsystem retten? So mag es vielleicht aussehen</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30856"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2023/09/20230911-mises_karma.png" srcset="https://www.misesde.org/wp-content/uploads/2023/09/20230911-mises_karma.png 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Mises Karma</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-13T10:30:04+02:00">13. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/podcast-erkenntnistheoretische-kriegsfuehrung-mises-karma-live-event/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230911-youtube_cover-520x360.jpg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230911-youtube_cover-520x360.jpg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: Erkenntnistheoretische Kriegsführung | Mises Karma Live Event" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/podcast-erkenntnistheoretische-kriegsfuehrung-mises-karma-live-event/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: Erkenntnistheoretische Kriegsführung | Mises Karma Live Event</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30848"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2022/03/20220316-david_duerr_1-scaled.jpg" srcset="https://www.misesde.org/wp-content/uploads/2022/03/20220316-david_duerr_1-scaled.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>David Dürr</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-11T08:31:04+02:00">11. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/175-jahre-schweizerische-eidgenossenschaft-warum-ludwig-von-mises-keine-freude-haette/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2019/10/alphorn_schweiz1-520x360.jpg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2019/10/alphorn_schweiz1-520x360.jpg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="175 Jahre Schweizerische Eidgenossenschaft. Warum Ludwig von Mises keine Freude hätte" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 79.2px;"><a href="https://www.misesde.org/2023/09/175-jahre-schweizerische-eidgenossenschaft-warum-ludwig-von-mises-keine-freude-haette/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">175 Jahre Schweizerische Eidgenossenschaft. Warum Ludwig von Mises keine Freude hätte</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30833"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2022/02/philipp_bagus.jpg" srcset="https://www.misesde.org/wp-content/uploads/2022/02/philipp_bagus.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Philipp Bagus</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-08T07:45:33+02:00">8. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/rechter-populismus-als-erfolgreiche-strategie-javier-milei/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230906-adobestock_101653989-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230906-adobestock_101653989-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Rechter Populismus als erfolgreiche Strategie. Javier Milei" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/rechter-populismus-als-erfolgreiche-strategie-javier-milei/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Rechter Populismus als erfolgreiche Strategie. Javier Milei</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30830"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2022/12/20221207-johanna_u_manuel.png" srcset="https://www.misesde.org/wp-content/uploads/2022/12/20221207-johanna_u_manuel.png 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Johanna &amp; Manuel</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-06T09:30:09+02:00">6. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/zwischen-schlager-und-autismus-mit-eigentuemlich-frei-mises-momente-10/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230905-mm10_thumbnail_landscape-520x360.png&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230905-mm10_thumbnail_landscape-520x360.png" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Zwischen Schlager und Autismus mit Eigentümlich Frei | Mises Momente #10" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/zwischen-schlager-und-autismus-mit-eigentuemlich-frei-mises-momente-10/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Zwischen Schlager und Autismus mit Eigentümlich Frei | Mises Momente #10</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30811"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2023/09/20230902-newman-jonathan_2022.jpg" srcset="https://www.misesde.org/wp-content/uploads/2023/09/20230902-newman-jonathan_2022.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Jonathan Newman</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-04T07:56:53+02:00">4. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/hat-das-brettspiel-monopoly-etwas-mit-wirklicher-marktwirtschaft-zu-tun/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/09/20230903-adobestock_78823905-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/09/20230903-adobestock_78823905-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Hat das Brettspiel Monopoly etwas mit wirklicher Marktwirtschaft zu tun?" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/hat-das-brettspiel-monopoly-etwas-mit-wirklicher-marktwirtschaft-zu-tun/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Hat das Brettspiel Monopoly etwas mit wirklicher Marktwirtschaft zu tun?</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30792"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2011/04/mises_wappen1.jpg" srcset="https://www.misesde.org/wp-content/uploads/2011/04/mises_wappen1.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>LvMID</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-09-01T07:57:02+02:00">1. September 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/09/mises-monatsmagazin-die-beitraege-im-august-2023/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/03/20230328-adobestock_56479643-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/03/20230328-adobestock_56479643-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="MISES Monatsmagazin - Die Beiträge im August 2023" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/09/mises-monatsmagazin-die-beitraege-im-august-2023/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">MISES Monatsmagazin – Die Beiträge im August 2023</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30789"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2019/11/murphy_2017.jpg" srcset="https://www.misesde.org/wp-content/uploads/2019/11/murphy_2017.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Robert P. Murphy</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-30T07:59:09+02:00">30. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/podcast-klimawandel-die-falsche-behauptung-vom-97-prozent-konsens/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230828-adobestock_597784174-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230828-adobestock_597784174-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: Klimawandel: Die falsche Behauptung vom „97-Prozent-Konsens“" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/podcast-klimawandel-die-falsche-behauptung-vom-97-prozent-konsens/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: Klimawandel: Die falsche Behauptung vom „97-Prozent-Konsens“</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30768"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/07/zt4.jpg" srcset="https://www.misesde.org/wp-content/uploads/2021/07/zt4.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Rainer Zitelmann</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-28T08:45:03+02:00">28. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/jung-rebellisch-kapitalistisch-kennen-sie-schon-die-anti-klima-kleber/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230823-image0-002-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230823-image0-002-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Jung, rebellisch, kapitalistisch: Kennen Sie schon die Anti-Klima-Kleber?" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/jung-rebellisch-kapitalistisch-kennen-sie-schon-die-anti-klima-kleber/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Jung, rebellisch, kapitalistisch: Kennen Sie schon die Anti-Klima-Kleber?</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30744"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2015/08/Barron1.jpg" srcset="https://www.misesde.org/wp-content/uploads/2015/08/Barron1.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Patrick Barron</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-25T13:51:40+02:00">25. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/der-geopolitische-wandel-und-das-resultierende-ende-der-dollar-vorherrschaft/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230825-adobestock_567213908-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230825-adobestock_567213908-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="Der Geopolitische Wandel und das resultierende Ende der Dollar-Vorherrschaft" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/der-geopolitische-wandel-und-das-resultierende-ende-der-dollar-vorherrschaft/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">Der Geopolitische Wandel und das resultierende Ende der Dollar-Vorherrschaft</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30738"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/07/polleit_thorsten3.jpg" srcset="https://www.misesde.org/wp-content/uploads/2021/07/polleit_thorsten3.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Thorsten Polleit</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-23T14:29:26+02:00">23. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/podcast-neue-brics-waehrung-das-kann-die-welt-aus-den-angeln-heben/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_594349786-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_594349786-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: Neue BRICS-Währung: „Das kann die Welt aus den Angeln heben“" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/podcast-neue-brics-waehrung-das-kann-die-welt-aus-den-angeln-heben/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: Neue BRICS-Währung: „Das kann die Welt aus den Angeln heben“</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30757"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2022/08/mudlack1-scaled.jpg" srcset="https://www.misesde.org/wp-content/uploads/2022/08/mudlack1-scaled.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Benjamin Mudlack</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-21T13:53:51+02:00">21. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/co2-ablasshandel-umverteilung-und-ressourcen-verschwendung-im-namen-eines-neuen-glaubens/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230818-adobestock_407316495-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230818-adobestock_407316495-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="CO2-Ablasshandel. Umverteilung und Ressourcen-Verschwendung im Namen eines neuen Glaubens" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/co2-ablasshandel-umverteilung-und-ressourcen-verschwendung-im-namen-eines-neuen-glaubens/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">CO2-Ablasshandel. Umverteilung und Ressourcen-Verschwendung im Namen eines neuen Glaubens</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30729"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2021/09/fassnacht.png" srcset="https://www.misesde.org/wp-content/uploads/2021/09/fassnacht.png 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Rainer Fassnacht</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-18T08:35:21+02:00">18. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/100-euro-fuer-den-liter-sprit-moderne-missionare-auf-dem-holzweg/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_604355774-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_604355774-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="100 Euro für den Liter Sprit. Moderne Missionare auf dem Holzweg" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/100-euro-fuer-den-liter-sprit-moderne-missionare-auf-dem-holzweg/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">100 Euro für den Liter Sprit. Moderne Missionare auf dem Holzweg</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30725"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2015/08/Barron1.jpg" srcset="https://www.misesde.org/wp-content/uploads/2015/08/Barron1.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Patrick Barron</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-16T07:48:00+02:00">16. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/podcast-weinen-sie-dem-untergang-des-fiat-dollars-als-reservewaehrung-nicht-hinterher/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_631267893-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230815-adobestock_631267893-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="PODCAST: Weinen Sie dem Untergang des Fiat-Dollars als Reservewährung nicht hinterher" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/podcast-weinen-sie-dem-untergang-des-fiat-dollars-als-reservewaehrung-nicht-hinterher/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">PODCAST: Weinen Sie dem Untergang des Fiat-Dollars als Reservewährung nicht hinterher</a></h4></div></div></div></div>
    <div class="col-md-6 col-sm-6 col-xs-12 pt-cv-content-item pt-cv-2-col" data-pid="30711"><div class="pt-cv-ifield"><div class="pt-cv-meta-fields" style="height: 40px;"><span class="author"><a rel="author" class="molongui-disabled-link"><img alt="" src="https://www.misesde.org/wp-content/uploads/2014/03/P_Bistoletti.jpg" srcset="https://www.misesde.org/wp-content/uploads/2014/03/P_Bistoletti.jpg 2x" class="avatar avatar-40 photo" height="40" width="40" loading="lazy" decoding="async"><span>Peter Bistoletti</span></a></span><span class="entry-date"><span class="glyphicon glyphicon-calendar"></span> <time datetime="2023-08-14T08:24:31+02:00">14. August 2023</time></span></div>
    <div class="pt-cv-hover-wrapper"><a href="https://www.misesde.org/2023/08/e-krona-schwedens-geplantes-digitales-zentralbankgeld-vs-bitcoin/" class="_self pt-cv-href-thumbnail pt-cv-thumb-left cvplbd cvp-responsive-image img-thumbnail" target="_self" data-iw="520" data-ih="360" style="background-image: url(&quot;https://www.misesde.org/wp-content/uploads/2023/08/20230811-adobestock_537932572-520x360.jpeg&quot;);"><img width="520" height="360" src="https://www.misesde.org/wp-content/uploads/2023/08/20230811-adobestock_537932572-520x360.jpeg" class="pt-cv-thumbnail img-thumbnail pull-left skip-lazy " alt="E-krona, Schwedens geplantes digitales Zentralbankgeld, vs. Bitcoin" decoding="async" itemprop="image"></a><div class="pt-cv-mask"><h4 class="pt-cv-animation-left pt-cv-title" style="height: 59.4px;"><a href="https://www.misesde.org/2023/08/e-krona-schwedens-geplantes-digitales-zentralbankgeld-vs-bitcoin/" class="_self cvplbd" target="_self" data-iw="520" data-ih="360">E-krona, Schwedens geplantes digitales Zentralbankgeld, vs. Bitcoin</a></h4></div></div></div></div></div></div>
    <div class="text-center pt-cv-pagination-wrapper"><ul class="pt-cv-pagination pt-cv-ajax pagination" data-totalpages="90" data-currentpage="1" data-sid="dacd42bydp" data-unid="" data-isblock="" data-postid=""><li class="cv-pageitem-first active"><a title="Gehe zur ersten Seite">«</a></li><li class="cv-pageitem-prev active"><a title="Gehe zur vorherigen Seite">‹</a></li><li class="cv-pageitem-number active"><a title="Aktuelle Seite ist 1">1</a></li><li class="cv-pageitem-number"><a title="Gehe zu Seite 2">2</a></li><li class="cv-pageitem-number"><a title="Gehe zu Seite 3">3</a></li><li class="cv-pageitem-number"><a title="Gehe zu Seite 4">4</a></li><li class="cv-pageitem-number"><a title="Gehe zu Seite 5">5</a></li><li class="cv-pageitem-next"><a title="Gehe zur nächsten Seite">›</a></li><li class="cv-pageitem-last"><a title="Gehe zur letzten Seite">»</a></li></ul><img width="15" height="15" class="pt-cv-spinner" alt="Wird geladen&nbsp;…" src="data:image/gif;base64,R0lGODlhDwAPALMPAMrKygwMDJOTkz09PZWVla+vr3p6euTk5M7OzuXl5TMzMwAAAJmZmWZmZszMzP///yH/C05FVFNDQVBFMi4wAwEAAAAh+QQFCgAPACwAAAAADwAPAAAEQvDJaZaZOIcV8iQK8VRX4iTYoAwZ4iCYoAjZ4RxejhVNoT+mRGP4cyF4Pp0N98sBGIBMEMOotl6YZ3S61Bmbkm4mAgAh+QQFCgAPACwAAAAADQANAAAENPDJSRSZeA418itN8QiK8BiLITVsFiyBBIoYqnoewAD4xPw9iY4XLGYSjkQR4UAUD45DLwIAIfkEBQoADwAsAAAAAA8ACQAABC/wyVlamTi3nSdgwFNdhEJgTJoNyoB9ISYoQmdjiZPcj7EYCAeCF1gEDo4Dz2eIAAAh+QQFCgAPACwCAAAADQANAAAEM/DJBxiYeLKdX3IJZT1FU0iIg2RNKx3OkZVnZ98ToRD4MyiDnkAh6BkNC0MvsAj0kMpHBAAh+QQFCgAPACwGAAAACQAPAAAEMDC59KpFDll73HkAA2wVY5KgiK5b0RRoI6MuzG6EQqCDMlSGheEhUAgqgUUAFRySIgAh+QQFCgAPACwCAAIADQANAAAEM/DJKZNLND/kkKaHc3xk+QAMYDKsiaqmZCxGVjSFFCxB1vwy2oOgIDxuucxAMTAJFAJNBAAh+QQFCgAPACwAAAYADwAJAAAEMNAs86q1yaWwwv2Ig0jUZx3OYa4XoRAfwADXoAwfo1+CIjyFRuEho60aSNYlOPxEAAAh+QQFCgAPACwAAAIADQANAAAENPA9s4y8+IUVcqaWJ4qEQozSoAzoIyhCK2NFU2SJk0hNnyEOhKR2AzAAj4Pj4GE4W0bkJQIAOw=="><div class="clear pt-cv-clear-pagination"></div></div></div>			<style type="text/css" id="pt-cv-inline-style-89c5569cnw">#pt-cv-view-dacd42bydp.pt-cv-post-border { margin: 0; border-top-style: dotted; border-left-style: dotted }
    #pt-cv-view-dacd42bydp.pt-cv-post-border { margin: 0; border-top-color: #efefef; border-left-color: #efefef }
    #pt-cv-view-dacd42bydp.pt-cv-post-border .pt-cv-content-item   { border-right-style: dotted; border-bottom-style: dotted; border-right-color: #efefef; border-bottom-color: #efefef; }
    #pt-cv-view-dacd42bydp .pt-cv-title a, #pt-cv-view-dacd42bydp  .panel-title { color: #ffffff !important; font-weight: 600 !important; }
    #pt-cv-view-dacd42bydp .pt-cv-content , #pt-cv-view-dacd42bydp  .pt-cv-content *:not(.pt-cv-readmore):not(style):not(script) { color: #cecece !important; }
    #pt-cv-view-dacd42bydp  .pt-cv-hover-wrapper::before   { background-color: rgba(0,0,0,0.17) !important; }
    #pt-cv-view-dacd42bydp  .pt-cv-content-item:hover .pt-cv-hover-wrapper::before   { background-color: rgba(1,83,137,0.37) !important; }
    #pt-cv-view-dacd42bydp .pt-cv-meta-fields * { line-height: 20px !important; }
    #pt-cv-view-dacd42bydp .pt-cv-readmore  { color: #ffffff !important; background-color: #015389 !important; }
    #pt-cv-view-dacd42bydp .pt-cv-readmore:hover  { color: #ffffff !important; background-color: #00aeef !important; }
    #pt-cv-view-dacd42bydp  + .pt-cv-pagination-wrapper .pt-cv-more , #pt-cv-view-dacd42bydp  + .pt-cv-pagination-wrapper .pagination .active a, .pt-cv-pagination[data-sid='dacd42bydp'] .active a { color: #ffffff !important; background-color: #015389 !important; }
    #pt-cv-view-dacd42bydp .cvp-responsive-image[style*="background-image"] { width: 520px; max-height: 360px; overflow: hidden; aspect-ratio: 1.4444444444444 }
    #pt-cv-view-dacd42bydp .cvp-responsive-image img { max-width: 520px; max-height: 360px; aspect-ratio: 1.4444444444444 }</style><style class="darkreader darkreader--sync" media="screen"></style>


    <div class="clearboth"></div>
    </div>

    </div>
    </div>

    <div class="vc_col-sm-4 wpb_column column_container  jupiter-donut- _ jupiter-donut-height-full">

    <div id="padding-7" class="mk-padding-divider jupiter-donut-  jupiter-donut-clearfix"></div>


    <aside id="mk-sidebar" class="mises-sidebar jupiter-donut-">
    <div class="sidebar-wrapper" style="padding:0;">
    <section id="block-14" class="widget widget_block mk-in-viewport">
    <div class="wp-block-ultimate-social-media-plus-sfsi-plus-share-block sfsi_plus_block_wrapper" style="text-align: center;">

    <div class="sfsi_plus_block" data-count="5" data-align="center" data-icon-type="round" style="width: 225px;"><div class="sfsi_plus_widget"><div class="sfsiplus_norm_row sfsi_plus_wDiv" style="width:225px;position:static;;text-align:left"><div style="width:40px; height:auto;margin-left:5px;margin-bottom:5px;" class="sfsi_plus_wicons shuffeldiv "><div class="sfsiplus_inerCnt"><a class=" sficn" data-effect="scale" target="_blank" href="https://www.facebook.com/ludwigvonmisesinstitut" style="width: 40px; height: 40px; opacity: 1; background: rgb(51, 102, 153); --darkreader-inline-bgcolor: #29527a; --darkreader-inline-bgimage: none;" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage=""><img alt="Facebook" title="Facebook" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/icons_theme/flat/flat_fb.png" width="40" height="40" style="" class="sfcm sfsi_wicon sfsiplusid_round_icon_facebook" data-effect="scale"></a><div class="sfsi_plus_tool_tip_2 sfsi_plus_fb_tool_bdr sfsi_plus_Tlleft sfsiplusid_facebook" style="display:block;width:62px;opacity:0;z-index:-1;"><span class="bot_arow bot_fb_arow"></span><div class="sfsi_plus_inside"><div class="icon1"><a href="https://www.facebook.com/ludwigvonmisesinstitut" target="_blank"><img class="sfsi_plus_wicon" alt="Facebook" title="Facebook" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/visit_icons/Visit_us_fb/icon_Visit_us_de_DE.png"></a></div><div class="icon2"><div class="fb-like" data-href="https://www.misesde.org/wp-json/ultimate-social-media-plus/v1/icons" data-width="180" data-show-faces="false" data-layout="button" data-action="like"></div></div><div class="icon3"><a target="_blank" href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwww.misesde.org%2Fwp-json%2Fultimate-social-media-plus%2Fv1%2Ficons" style="display:inline-block;"> <img class="sfsi_wicon" data-pin-nopin="true" width="auto" height="auto" alt="fb-share-icon" title="Facebook Share" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/share_icons/fb_icons/de_DE.svg" '=""></a></div></div></div></div></div><div style="width:40px; height:auto;margin-left:5px;margin-bottom:5px;" class="sfsi_plus_wicons shuffeldiv "><div class="sfsiplus_inerCnt"><a class=" sficn" data-effect="scale" target="_blank" href="https://twitter.com/MisesInstitut" style="width: 40px; height: 40px; opacity: 1; background: rgb(0, 172, 236); --darkreader-inline-bgcolor: #008abd; --darkreader-inline-bgimage: none;" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage=""><img alt="Twitter" title="Twitter" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/icons_theme/flat/flat_twitter.png" width="40" height="40" style="" class="sfcm sfsi_wicon sfsiplusid_round_icon_twitter" data-effect="scale"></a><div class="sfsi_plus_tool_tip_2 sfsi_plus_twt_tool_bdr sfsi_plus_Tlleft sfsiplusid_twitter" style="display:block;width:59px;opacity:0;z-index:-1;"><span class="bot_arow bot_twt_arow"></span><div class="sfsi_plus_inside"><style>#sfsi_plus_floater .sfsi_plus_twt_tool_bdr .sfsi_plus_inside{margin-top: -18px;}</style><style class="darkreader darkreader--sync" media="screen"></style><div class="cstmicon1"><a href="https://twitter.com/MisesInstitut" target="_blank"><img class="sfsi_plus_wicon" alt="Visit Us" title="Visit Us" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/visit_icons/Visit_us_twitter/icon_Visit_us_de_DE.png"></a></div><div class="icon1"><a target="_blank" href="https://twitter.com/intent/user?screen_name=@MisesInstitut"><img nopin="nopin" width="auto" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/share_icons/Twitter_Follow/de_DE_Follow.svg" class="sfsi_premium_wicon" alt="Follow Me" title="Follow Me" style="opacity: 1;" '=""></a></div></div></div></div></div><div style="width:40px; height:auto;margin-left:5px;margin-bottom:5px;" class="sfsi_plus_wicons shuffeldiv "><div class="sfsiplus_inerCnt"><a class=" sficn" data-effect="scale" target="_blank" href="https://www.youtube.com/user/misesde" style="width: 40px; height: 40px; opacity: 1; background: linear-gradient(141.52deg, rgb(224, 47, 47) 14.26%, rgb(224, 47, 47) 48.98%, rgb(201, 42, 42) 49.12%, rgb(201, 42, 42) 85.18%); --darkreader-inline-bgcolor: rgba(0, 0, 0, 0); --darkreader-inline-bgimage: linear-gradient(141.52deg, #a91919 14.26%, #a91919 48.98%, #a12222 49.12%, #a12222 85.18%);" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage=""><img alt="YOUTUBE" title="YOUTUBE" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/images/icons_theme/flat/flat_youtube.png" width="40" height="40" style="" class="sfcm sfsi_wicon sfsiplusid_round_icon_youtube" data-effect="scale"></a></div></div></div><div id="sfsi_holder" class="sfsi_plus_holders" style="position: relative; float: left;width:100%;z-index:-1;"></div><script></script><div style="clear: both;"></div></div></div>
    </div>
    </section><section id="block-4" class="widget widget_block mk-in-viewport"><div class="mises-button" style="display: block;"><a href="https://www.misesde.org/2023/03/ludwig-von-mises-institut-deutschland-konferenz-2023/" target="_blank">Jetzt anmelden:<br>
    Ludwig von Mises Institut Deutschland<br>
    Konferenz 2023<br>
    Alle Informationen hier!<br>
    </a></div></section><section id="block-2" class="widget widget_block mk-in-viewport"><div class="mises-button" style="display: block;"><a href="https://www.misesde.org/unterstutzen/" target="_blank">Bitte unterstützen<br>
    Sie das Ludwig von Mises<br>Institut Deutschland.<br>
    </a></div></section><section id="block-31" class="widget widget_block"><p align="left">
    Spenden über PayPal:
    </p>
    <form action="https://www.paypal.com/donate" method="post" target="_top">
    <input type="hidden" name="hosted_button_id" value="K9TV5XKQ54D4Y">
    <input type="image" src="https://www.paypalobjects.com/de_DE/DE/i/btn/btn_donateCC_LG.gif" name="submit" title="PayPal - The safer, easier way to pay online!" alt="Spenden mit dem PayPal-Button" border="0">
    <img decoding="async" loading="lazy" alt="" src="https://www.paypal.com/de_DE/i/scr/pixel.gif" width="1" height="1" border="0" tqbb2mz7w="">
    </form></section><section id="displaycategorieswidget-2" class="widget DisplayCategoriesWidget"><div class="widgettitle">Artikel nach Themen</div><style>.dcw_c1 {float:left; width:100%} .dcw_c2 {float:left; width:50%} .dcw_c3 {float:left; width:33%}</style><style class="darkreader darkreader--sync" media="screen"></style><ul class="dcw">	<li class="cat-item cat-item-33 dcw_c1"><a href="https://www.misesde.org/category/aktuelles/">Aktuelles</a> (1.797)
    </li>
    <li class="cat-item cat-item-1 dcw_c1"><a href="https://www.misesde.org/category/allgemein/">Allgemein</a> (3)
    </li>
    <li class="cat-item cat-item-34 dcw_c1"><a href="https://www.misesde.org/category/autoren/">Autoren</a> (65)
    </li>
    <li class="cat-item cat-item-36 dcw_c1"><a href="https://www.misesde.org/category/buecher/">Bücher</a> (135)
    </li>
    <li class="cat-item cat-item-37 dcw_c1"><a href="https://www.misesde.org/category/interviews/">Interviews</a> (169)
    </li>
    <li class="cat-item cat-item-40 dcw_c1"><a href="https://www.misesde.org/category/rubriken/">Rubriken</a> (1.423)
    <ul class="children">
    <li class="cat-item cat-item-42 dcw_c1"><a href="https://www.misesde.org/category/rubriken/geld/">Geld</a> (492)
    </li>
    <li class="cat-item cat-item-43 dcw_c1"><a href="https://www.misesde.org/category/rubriken/gesellschaft/">Gesellschaft</a> (857)
    </li>
    <li class="cat-item cat-item-44 dcw_c1"><a href="https://www.misesde.org/category/rubriken/politik/">Politik</a> (918)
    </li>
    <li class="cat-item cat-item-45 dcw_c1"><a href="https://www.misesde.org/category/rubriken/recht/">Recht</a> (146)
    </li>
    <li class="cat-item cat-item-46 dcw_c1"><a href="https://www.misesde.org/category/rubriken/staat/">Staat</a> (745)
    </li>
    <li class="cat-item cat-item-41 dcw_c1"><a href="https://www.misesde.org/category/rubriken/wirtschaft/">Wirtschaft</a> (847)
    </li>
    </ul>
    </li>
    <li class="cat-item cat-item-39 dcw_c1"><a href="https://www.misesde.org/category/veranstaltungen/">Veranstaltungen</a> (41)
    </li>
    <li class="cat-item cat-item-38 dcw_c1"><a href="https://www.misesde.org/category/zeitloses/">Zeitloses</a> (600)
    </li>
    </ul><script>jQuery('ul.dcw').find('li').addClass('dcw_c1');</script></section><section id="block-29" class="widget widget_block"><br>&nbsp;</section><section id="email-subscribers-form-4" class="widget widget_email-subscribers-form"><div class="widgettitle"> Anmeldung zum Newsletter </div><div class="emaillist" id="es_form_f1-p18800-n1"><form action="/#es_form_f1-p18800-n1" method="post" class="es_subscription_form es_shortcode_form  es_ajax_subscription_form" id="es_subscription_form_65250e9809e33" data-source="ig-es" data-form-id="1"><div class="es-field-wrap"><label>Email*<br><input class="es_required_field es_txt_email ig_es_form_field_email" type="email" name="esfpx_email" value="" placeholder="Email" required="required"></label></div><input type="hidden" name="esfpx_lists[]" value="d85384fa3f18"><input type="hidden" name="esfpx_lists[]" value="c5647c0e0f1b"><input type="hidden" name="esfpx_form_id" value="1"><input type="hidden" name="es" value="subscribe">
    <input type="hidden" name="esfpx_es_form_identifier" value="f1-p18800-n1">
    <input type="hidden" name="esfpx_es_email_page" value="18800">
    <input type="hidden" name="esfpx_es_email_page_url" value="https://www.misesde.org/">
    <input type="hidden" name="esfpx_status" value="Unconfirmed">
    <input type="hidden" name="esfpx_es-subscribe" id="es-subscribe-65250e9809e33" value="79e803ecb3">
    <label style="position:absolute;top:-99999px;left:-99999px;z-index:-99;" aria-hidden="true"><span hidden="">Please leave this field empty.</span><input type="email" name="esfpx_es_hp_email" class="es_required_field" tabindex="-1" autocomplete="-1" value=""></label><input type="submit" name="submit" class="es_subscription_form_submit es_submit_button es_textbox_button" id="es_subscription_form_submit_65250e9809e33" value="Anmelden"><span class="es_spinner_image" id="spinner-image"><img src="https://www.misesde.org/wp-content/plugins/email-subscribers/lite/public/images/spinner.gif" alt="Loading"></span></form><span class="es_subscription_message " id="es_subscription_message_65250e9809e33"></span></div></section><section id="block-12" class="widget widget_block">
    <div class="wp-block-columns is-layout-flex wp-container-2 wp-block-columns-is-layout-flex">
    <div class="wp-block-column is-layout-flow wp-block-column-is-layout-flow" style="flex-basis:100%"><div class="widget widget_easy_facebook_like_box"><div class="widget-text easy-facebook-like-box_box"><div id="fb-root"></div>
    <script>(function(d, s, id) {
    var js, fjs = d.getElementsByTagName(s)[0];
    if (d.getElementById(id)) return;
    js = d.createElement(s); js.id = id;
    js.src = "//connect.facebook.net/de_DE/sdk.js#xfbml=1&version=v2.10";
    fjs.parentNode.insertBefore(js, fjs);
    }(document, "script", "facebook-jssdk"));</script><div class="fb-page" data-href="https://www.facebook.com/ludwigvonmisesinstitut/" data-width="250" data-height="80" data-hide-cover="false" data-tabs="" data-small-header="false" data-hide-cta="true" data-adapt-container-width="true" data-show-facepile="false"><blockquote cite="https://www.facebook.com/facebook" class="fb-xfbml-parse-ignore"><a href="https://www.facebook.com/facebook">Facebook</a></blockquote></div></div></div></div>
    </div>
    </section><section id="block-26" class="widget widget_block widget_media_image">
    <figure class="wp-block-image size-full is-resized"><img decoding="async" src="https://www.misesde.org/wp-content/uploads/2022/02/amazon_de_logo_rgb.png" alt="" class="wp-image-27376" width="120" srcset="https://www.misesde.org/wp-content/uploads/2022/02/amazon_de_logo_rgb.png 1500w, https://www.misesde.org/wp-content/uploads/2022/02/amazon_de_logo_rgb-300x78.png 300w, https://www.misesde.org/wp-content/uploads/2022/02/amazon_de_logo_rgb-1024x268.png 1024w, https://www.misesde.org/wp-content/uploads/2022/02/amazon_de_logo_rgb-768x201.png 768w" sizes="(max-width: 1500px) 100vw, 1500px"></figure>
    </section><section id="block-25" class="widget widget_block"><p><a target="_blank" href="https://www.amazon.de/b?_encoding=UTF8&amp;tag=ludwvonmisein-21&amp;linkCode=ur2&amp;linkId=db39e05da12a0fc23853a4b8ea8d2972&amp;camp=1638&amp;creative=6742&amp;node=186606">Bücher bei Amazon kaufen</a></p></section><section id="nav_menu-3" class="widget widget_nav_menu"><div class="widgettitle">Service</div>
    <div class="menu-submenu-container"><ul id="menu-submenu" class="menu">
    <li id="menu-item-3143" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3143"><a href="https://www.misesde.org/uber-den-autor/"><svg class="mk-svg-icon" data-name="mk-icon-angle-right" data-cacheid="icon-65250e980de5c" style=" height:14px; width: 5px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 1792"><path d="M595 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z"></path></svg>Institut</a></li>
    <li id="menu-item-3142" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3142"><a href="https://www.misesde.org/impressum/"><svg class="mk-svg-icon" data-name="mk-icon-angle-right" data-cacheid="icon-65250e980de5c" style=" height:14px; width: 5px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 1792"><path d="M595 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z"></path></svg>Impressum</a></li>
    <li id="menu-item-18613" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-privacy-policy menu-item-18613"><a rel="privacy-policy" href="https://www.misesde.org/datenschutz/"><svg class="mk-svg-icon" data-name="mk-icon-angle-right" data-cacheid="icon-65250e980de5c" style=" height:14px; width: 5px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 1792"><path d="M595 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z"></path></svg>Datenschutz</a></li>
    <li id="menu-item-3144" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3144"><a href="https://www.misesde.org/disclaimer/"><svg class="mk-svg-icon" data-name="mk-icon-angle-right" data-cacheid="icon-65250e980de5c" style=" height:14px; width: 5px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 1792"><path d="M595 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z"></path></svg>Disclaimer</a></li>
    </ul></div></section>	</div>
    </aside>

    </div>
    </div>

    </section>		<div class="clearboth"></div>
    <div class="clearboth"></div>
    </div>
    <div class="clearboth"></div>
    </div>
    </div>
    </div>


    <section id="mk-footer-unfold-spacer"></section>

    <section id="mk-footer" class="" role="contentinfo" itemscope="itemscope" itemtype="https://schema.org/WPFooter">
    <div class="footer-wrapper mk-grid">
    <div class="mk-padding-wrapper">
    <div class="mk-col-1-3"><section id="contact_info-2" class="widget widget_contact_info"><div class="widgettitle">Ludwig von Mises Institut Deutschland</div>			<ul itemscope="itemscope" itemtype="https://schema.org/Person">

    <li><svg class="mk-svg-icon" data-name="mk-icon-envelope" data-cacheid="icon-65250e980effd" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1792 1792"><path d="M1792 710v794q0 66-47 113t-113 47h-1472q-66 0-113-47t-47-113v-794q44 49 101 87 362 246 497 345 57 42 92.5 65.5t94.5 48 110 24.5h2q51 0 110-24.5t94.5-48 92.5-65.5q170-123 498-345 57-39 100-87zm0-294q0 79-49 151t-122 123q-376 261-468 325-10 7-42.5 30.5t-54 38-52 32.5-57.5 27-50 9h-2q-23 0-50-9t-57.5-27-52-32.5-54-38-42.5-30.5q-91-64-262-182.5t-205-142.5q-62-42-117-115.5t-55-136.5q0-78 41.5-130t118.5-52h1472q65 0 112.5 47t47.5 113z"></path></svg><span>
    <a itemprop="email" href="mailto:kontakt@misesde.org">kontakt@misesde.org</a></span></li>

    </ul>
    </section></div>
    <div class="mk-col-1-3"></div>
    <div class="mk-col-1-3"></div>
    <div class="clearboth"></div>
    </div>
    </div>

    <div id="sub-footer">
    <div class=" mk-grid">

    <span class="mk-footer-copyright">© Ludwig von Mises Institut Deutschland 2019</span>
    <nav id="mk-footer-navigation" class="footer_menu"><ul id="menu-submenu-1" class="menu"><li class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3143"><a href="https://www.misesde.org/uber-den-autor/">Institut</a></li>
    <li class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3142"><a href="https://www.misesde.org/impressum/">Impressum</a></li>
    <li class="menu-item menu-item-type-post_type menu-item-object-page menu-item-privacy-policy menu-item-18613"><a rel="privacy-policy" href="https://www.misesde.org/datenschutz/">Datenschutz</a></li>
    <li class="menu-item menu-item-type-post_type menu-item-object-page menu-item-3144"><a href="https://www.misesde.org/disclaimer/">Disclaimer</a></li>
    </ul></nav>	</div>
    <div class="clearboth"></div>
    </div>
    </section>
    <div class="resize-triggers"><div class="expand-trigger"><div style="width: 1513px; height: 4551px;"></div></div><div class="contract-trigger"></div></div></div>
    </div>

    <div class="bottom-corner-btns js-bottom-corner-btns">

    <a href=#top-of-page" class="mk-go-top js-smooth-scroll js-bottom-corner-btn js-bottom-corner-btn--back is-active">
    <svg class="mk-svg-icon" data-name="mk-icon-chevron-up" data-cacheid="icon-65250e980f9f2" style=" height:16px; width: 16px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1792 1792"><path d="M1683 1331l-166 165q-19 19-45 19t-45-19l-531-531-531 531q-19 19-45 19t-45-19l-166-165q-19-19-19-45.5t19-45.5l742-741q19-19 45-19t45 19l742 741q19 19 19 45.5t-19 45.5z"></path></svg></a>
    <div class="mk-quick-contact-wrapper js-bottom-corner-btn js-bottom-corner-btn--contact is-active">

    <a href=# class="mk-quick-contact-link"><svg class="mk-svg-icon" data-name="mk-icon-envelope" data-cacheid="icon-65250e980fb4f" style=" height:20px; width: 20px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1792 1792"><path d="M1792 710v794q0 66-47 113t-113 47h-1472q-66 0-113-47t-47-113v-794q44 49 101 87 362 246 497 345 57 42 92.5 65.5t94.5 48 110 24.5h2q51 0 110-24.5t94.5-48 92.5-65.5q170-123 498-345 57-39 100-87zm0-294q0 79-49 151t-122 123q-376 261-468 325-10 7-42.5 30.5t-54 38-52 32.5-57.5 27-50 9h-2q-23 0-50-9t-57.5-27-52-32.5-54-38-42.5-30.5q-91-64-262-182.5t-205-142.5q-62-42-117-115.5t-55-136.5q0-78 41.5-130t118.5-52h1472q65 0 112.5 47t47.5 113z"></path></svg></a>
    <div id="mk-quick-contact">
    <div class="mk-quick-contact-title">Kontaktieren Sie uns</div>
    <p>We're not around right now. But you can send us an email and we'll get back to you, asap.</p>
    <form class="mk-contact-form" method="post" novalidate="novalidate">
    <input type="text" placeholder="Name*" required="required" id="name" name="name" class="text-input" value="" tabindex="2320">
    <input type="email" data-type="email" required="required" placeholder="E-Mail*" id="email" name="email" class="text-input" value="" tabindex="2321">
    <textarea placeholder="Nachricht*" required="required" id="content" name="content" class="textarea" tabindex="2322"></textarea>
    <input placeholder="Captcha eingeben" type="text" data-type="captcha" name="captcha" class="captcha-form text-input full" required="required" autocomplete="off">
    <a href=# class="captcha-change-image">Nicht lesbar? Text ändern.</a>
    <span class="captcha-image-holder">
    <img src="https://www.misesde.org/wp-content/plugins/artbees-captcha/generate-captcha.php" class="captcha-image" alt="captcha txt">
    </span>
    <br>

    <div class="mk-quick-contact-gdpr-consent">
    <div>
    <input type="checkbox" name="contact_form_gdpr_check" id="gdpr_check_2323" class="mk-checkbox" required="required" value="" tabindex="2323"><label for="gdpr_check_2323">I consent to Ludwig von Mises Institut Deutschland collecting my details through this form.</label>
    </div>
    </div>

    <div class="btn-cont">
    <button tabindex="2324" class="mk-progress-button mk-contact-button accent-bg-color button" data-style="move-up">
    <span class="mk-progress-button-content">Senden</span>
    <span class="mk-progress">
    <span class="mk-progress-inner"></span>
    </span>
    <span class="state-success"><svg class="mk-svg-icon" data-name="mk-moon-checkmark" data-cacheid="icon-65250e980fd32" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M432 64l-240 240-112-112-80 80 192 192 320-320z"></path></svg></span>
    <span class="state-error"><svg class="mk-svg-icon" data-name="mk-moon-close" data-cacheid="icon-65250e980fe67" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M507.331 411.33l-.006-.005-155.322-155.325 155.322-155.325.006-.005c1.672-1.673 2.881-3.627 3.656-5.708 2.123-5.688.912-12.341-3.662-16.915l-73.373-73.373c-4.574-4.573-11.225-5.783-16.914-3.66-2.08.775-4.035 1.984-5.709 3.655l-.004.005-155.324 155.326-155.324-155.325-.005-.005c-1.673-1.671-3.627-2.88-5.707-3.655-5.69-2.124-12.341-.913-16.915 3.66l-73.374 73.374c-4.574 4.574-5.784 11.226-3.661 16.914.776 2.08 1.985 4.036 3.656 5.708l.005.005 155.325 155.324-155.325 155.326-.004.005c-1.671 1.673-2.88 3.627-3.657 5.707-2.124 5.688-.913 12.341 3.661 16.915l73.374 73.373c4.575 4.574 11.226 5.784 16.915 3.661 2.08-.776 4.035-1.985 5.708-3.656l.005-.005 155.324-155.325 155.324 155.325.006.004c1.674 1.672 3.627 2.881 5.707 3.657 5.689 2.123 12.342.913 16.914-3.661l73.373-73.374c4.574-4.574 5.785-11.227 3.662-16.915-.776-2.08-1.985-4.034-3.657-5.707z"></path></svg></span>
    </button>
    </div>
    <input type="hidden" id="security" name="security" value="e60b77d28d"><input type="hidden" name="_wp_http_referer" value="/">				<input type="hidden" id="sh_id" name="sh_id" value="15"><input type="hidden" id="p_id" name="p_id" value="2342">				<div class="contact-form-message clearfix"></div>
    </form>
    <div class="bottom-arrow"></div>
    </div>
    </div>
    </div>



    <div class="mk-fullscreen-search-overlay">
    <a href=# class="mk-fullscreen-close"><svg class="mk-svg-icon" data-name="mk-moon-close-2" data-cacheid="icon-65250e981002f" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M390.628 345.372l-45.256 45.256-89.372-89.373-89.373 89.372-45.255-45.255 89.373-89.372-89.372-89.373 45.254-45.254 89.373 89.372 89.372-89.373 45.256 45.255-89.373 89.373 89.373 89.372z"></path></svg></a>
    <div class="mk-fullscreen-search-wrapper">
    <p>Beginnen Sie mit der Eingabe und drücken Sie Enter, um zu suchen</p>
    <form method="get" id="mk-fullscreen-searchform" action="https://www.misesde.org/">
    <input type="text" value="" name="s" id="mk-fullscreen-search-input">
    <i class="fullscreen-search-icon"><svg class="mk-svg-icon" data-name="mk-icon-search" data-cacheid="icon-65250e98100ef" style=" height:25px; width: 23.214285714286px; " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1664 1792"><path d="M1152 832q0-185-131.5-316.5t-316.5-131.5-316.5 131.5-131.5 316.5 131.5 316.5 316.5 131.5 316.5-131.5 131.5-316.5zm512 832q0 52-38 90t-90 38q-54 0-90-38l-343-342q-179 124-399 124-143 0-273.5-55.5t-225-150-150-225-55.5-273.5 55.5-273.5 150-225 225-150 273.5-55.5 273.5 55.5 225 150 150 225 55.5 273.5q0 220-124 399l343 343q37 37 37 90z"></path></svg></i>
    </form>
    </div>
    </div>


    <style type="text/css"></style><style class="darkreader darkreader--sync" media="screen"></style><style id="mk-shortcode-static-styles" type="text/css">   #text-block-5 { margin-bottom:0px; text-align:left; }  #padding-7 { height:46px; }</style><style class="darkreader darkreader--sync" media="screen"></style><!--googleoff: all--><div id="cookie-law-info-bar" data-nosnippet="true" style="background-color: rgb(63, 63, 63); color: rgb(239, 239, 239); font-family: inherit; bottom: 0px; position: fixed; display: none; --darkreader-inline-bgcolor: #2f3335; --darkreader-inline-color: #dedbd7;" data-darkreader-inline-bgcolor="" data-darkreader-inline-color=""><span>Wir verwenden Cookies auf unserer Website, um Ihren Besuch effizienter zu machen und Ihnen mehr<br>
    Benutzerfreundlichkeit bieten zu können.<a role="button" data-cli_action="accept" id="cookie_action_close_header" class="medium cli-plugin-button cli-plugin-main-button cookie_action_close_header cli_action_button wt-cli-accept-btn" style="color: rgb(255, 255, 255); background-color: rgb(0, 0, 0); --darkreader-inline-color: #e8e6e3; --darkreader-inline-bgcolor: #000000;" data-darkreader-inline-color="" data-darkreader-inline-bgcolor="">Ok</a> <a href="http://www.misesde.org/wordpress" id="CONSTANT_OPEN_URL" target="_blank" class="cli-plugin-main-link" style="color: rgb(68, 68, 68); --darkreader-inline-color: #bdb7af;" data-darkreader-inline-color="">Read More</a></span></div><div id="cookie-law-info-again" data-nosnippet="true" style="background-color: rgb(63, 63, 63); color: rgb(239, 239, 239); position: fixed; font-family: inherit; width: auto; bottom: 0px; right: 100px; --darkreader-inline-bgcolor: #2f3335; --darkreader-inline-color: #dedbd7;" data-darkreader-inline-bgcolor="" data-darkreader-inline-color=""><span id="cookie_hdr_showagain">Privacy &amp; Cookies Policy</span></div><div class="cli-modal" data-nosnippet="true" id="cliSettingsPopup" tabindex="-1" role="dialog" aria-labelledby="cliSettingsPopup" aria-hidden="true">
    <div class="cli-modal-dialog" role="document">
    <div class="cli-modal-content cli-bar-popup">
    <button type="button" class="cli-modal-close" id="cliModalClose">
    <svg class="" viewBox="0 0 24 24"><path d="M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z"></path><path d="M0 0h24v24h-24z" fill="none"></path></svg>
    <span class="wt-cli-sr-only">Schließen</span>
    </button>
    <div class="cli-modal-body">
    <div class="cli-container-fluid cli-tab-container">
    <div class="cli-row">
    <div class="cli-col-12 cli-align-items-stretch cli-px-0">
    <div class="cli-privacy-overview">
    <h4>Privacy Overview</h4>				<div class="cli-privacy-content">
    <div class="cli-privacy-content-text">This website uses cookies to improve your experience while you navigate through the website. Out of these, the cookies that are categorized as necessary are stored on your browser as they are essential for the working of basic functionalities of the ...</div>
    </div>
    <a class="cli-privacy-readmore" aria-label="Mehr anzeigen" role="button" data-readmore-text="Mehr anzeigen" data-readless-text="Weniger anzeigen"></a>			</div>
    </div>
    <div class="cli-col-12 cli-align-items-stretch cli-px-0 cli-tab-section-container">
    <div class="cli-tab-section">
    <div class="cli-tab-header">
    <a role="button" tabindex="0" class="cli-nav-link cli-settings-mobile" data-target="necessary" data-toggle="cli-toggle-tab">
    Necessary							</a>
    <div class="wt-cli-necessary-checkbox">
    <input type="checkbox" class="cli-user-preference-checkbox" id="wt-cli-checkbox-necessary" data-id="checkbox-necessary" checked="checked">
    <label class="form-check-label" for="wt-cli-checkbox-necessary">Necessary</label>
    </div>
    <span class="cli-necessary-caption">immer aktiv</span>
    </div>
    <div class="cli-tab-content">
    <div class="cli-tab-pane cli-fade" data-id="necessary">
    <div class="wt-cli-cookie-description">
    Necessary cookies are absolutely essential for the website to function properly. This category only includes cookies that ensures basic functionalities and security features of the website. These cookies do not store any personal information.								</div>
    </div>
    </div>
    </div>
    <div class="cli-tab-section">
    <div class="cli-tab-header">
    <a role="button" tabindex="0" class="cli-nav-link cli-settings-mobile" data-target="non-necessary" data-toggle="cli-toggle-tab">
    Non-necessary							</a>
    <div class="cli-switch">
    <input type="checkbox" id="wt-cli-checkbox-non-necessary" class="cli-user-preference-checkbox" data-id="checkbox-non-necessary" checked="checked">
    <label for="wt-cli-checkbox-non-necessary" class="cli-slider" data-cli-enable="Aktiviert" data-cli-disable="Deaktiviert"><span class="wt-cli-sr-only">Non-necessary</span></label>
    </div>
    </div>
    <div class="cli-tab-content">
    <div class="cli-tab-pane cli-fade" data-id="non-necessary">
    <div class="wt-cli-cookie-description">
    Any cookies that may not be particularly necessary for the website to function and is used specifically to collect user personal data via analytics, ads, other embedded contents are termed as non-necessary cookies. It is mandatory to procure user consent prior to running these cookies on your website.								</div>
    </div>
    </div>
    </div>
    </div>
    </div>
    </div>
    </div>
    <div class="cli-modal-footer">
    <div class="wt-cli-element cli-container-fluid cli-tab-container">
    <div class="cli-row">
    <div class="cli-col-12 cli-align-items-stretch cli-px-0">
    <div class="cli-tab-footer wt-cli-privacy-overview-actions">

    <a id="wt-cli-privacy-save-btn" role="button" tabindex="0" data-cli-action="accept" class="wt-cli-privacy-btn cli_setting_save_button wt-cli-privacy-accept-btn cli-btn">SPEICHERN &amp; AKZEPTIEREN</a>
    </div>

    </div>
    </div>
    </div>
    </div>
    </div>
    </div>
    </div>
    <div class="cli-modal-backdrop cli-fade cli-settings-overlay"></div>
    <div class="cli-modal-backdrop cli-fade cli-popupbar-overlay"></div>
    <!--googleon: all-->		<!--facebook like and share js -->
    <div id="fb-root"></div>

    <script>
    (function(d, s, id) {
    var js, fjs = d.getElementsByTagName(s)[0];
    if (d.getElementById(id)) return;
    js = d.createElement(s);
    js.id = id;
    js.src = "//connect.facebook.net/de_DE/sdk.js#xfbml=1&version=v2.5";
    fjs.parentNode.insertBefore(js, fjs);
    }(document, 'script', 'facebook-jssdk'));
    </script>
    <script>
    window.addEventListener('sfsi_plus_functions_loaded', function() {
    if (typeof sfsi_plus_responsive_toggle == 'function') {
    sfsi_plus_responsive_toggle(0);
    // console.log('sfsi_plus_responsive_toggle');
    }
    })
    </script>

    <script type="text/javascript">
    php = {
    hasAdminbar: false,
    json: (null != null) ? null : "",
    jsPath: 'https://www.misesde.org/wp-content/themes/jupiter/assets/js'
    };
    </script><script type="text/html" id="wpb-modifications"> window.wpbCustomElement = 1; </script><link rel="stylesheet" id="vc_animate-css-css" href="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/lib/bower/animate-css/animate.min.css?ver=7.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
    <link rel="stylesheet" id="vc_font_awesome_5_shims-css" href="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/lib/bower/font-awesome/css/v4-shims.min.css?ver=7.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
    <link rel="stylesheet" id="vc_font_awesome_5-css" href="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/lib/bower/font-awesome/css/all.min.css?ver=7.0" type="text/css" media="all"><style class="darkreader darkreader--sync" media="screen"></style>
    <style id="clients-block-supports-inline-css" type="text/css">
    .wp-container-2.wp-container-2{flex-wrap:nowrap;}
    </style><style class="darkreader darkreader--sync" media="screen"></style>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/contact-form-7/includes/swv/js/index.js?ver=5.8.1" id="swv-js"></script>
    <script type="text/javascript" id="contact-form-7-js-extra">
    /* <![CDATA[ */
    var wpcf7 = {"api":{"root":"https:\/\/www.misesde.org\/wp-json\/","namespace":"contact-form-7\/v1"}};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/contact-form-7/includes/js/index.js?ver=5.8.1" id="contact-form-7-js"></script>
    <script type="text/javascript" id="pt-cv-content-views-script-js-extra">
    /* <![CDATA[ */
    var PT_CV_PUBLIC = {"_prefix":"pt-cv-","page_to_show":"5","_nonce":"1652d5d55e","is_admin":"","is_mobile":"","ajaxurl":"https:\/\/www.misesde.org\/wp-admin\/admin-ajax.php","lang":"","loading_image_src":"data:image\/gif;base64,R0lGODlhDwAPALMPAMrKygwMDJOTkz09PZWVla+vr3p6euTk5M7OzuXl5TMzMwAAAJmZmWZmZszMzP\/\/\/yH\/C05FVFNDQVBFMi4wAwEAAAAh+QQFCgAPACwAAAAADwAPAAAEQvDJaZaZOIcV8iQK8VRX4iTYoAwZ4iCYoAjZ4RxejhVNoT+mRGP4cyF4Pp0N98sBGIBMEMOotl6YZ3S61Bmbkm4mAgAh+QQFCgAPACwAAAAADQANAAAENPDJSRSZeA418itN8QiK8BiLITVsFiyBBIoYqnoewAD4xPw9iY4XLGYSjkQR4UAUD45DLwIAIfkEBQoADwAsAAAAAA8ACQAABC\/wyVlamTi3nSdgwFNdhEJgTJoNyoB9ISYoQmdjiZPcj7EYCAeCF1gEDo4Dz2eIAAAh+QQFCgAPACwCAAAADQANAAAEM\/DJBxiYeLKdX3IJZT1FU0iIg2RNKx3OkZVnZ98ToRD4MyiDnkAh6BkNC0MvsAj0kMpHBAAh+QQFCgAPACwGAAAACQAPAAAEMDC59KpFDll73HkAA2wVY5KgiK5b0RRoI6MuzG6EQqCDMlSGheEhUAgqgUUAFRySIgAh+QQFCgAPACwCAAIADQANAAAEM\/DJKZNLND\/kkKaHc3xk+QAMYDKsiaqmZCxGVjSFFCxB1vwy2oOgIDxuucxAMTAJFAJNBAAh+QQFCgAPACwAAAYADwAJAAAEMNAs86q1yaWwwv2Ig0jUZx3OYa4XoRAfwADXoAwfo1+CIjyFRuEho60aSNYlOPxEAAAh+QQFCgAPACwAAAIADQANAAAENPA9s4y8+IUVcqaWJ4qEQozSoAzoIyhCK2NFU2SJk0hNnyEOhKR2AzAAj4Pj4GE4W0bkJQIAOw==","is_mobile_tablet":"","sf_no_post_found":"Keine Beitr\u00e4ge gefunden.","lf__separator":","};
    var PT_CV_PAGINATION = {"first":"\u00ab","prev":"\u2039","next":"\u203a","last":"\u00bb","goto_first":"Gehe zur ersten Seite","goto_prev":"Gehe zur vorherigen Seite","goto_next":"Gehe zur n\u00e4chsten Seite","goto_last":"Gehe zur letzten Seite","current_page":"Aktuelle Seite ist","goto_page":"Gehe zu Seite"};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/content-views-query-and-display-post-page/public/assets/js/cv.js?ver=3.5.0" id="pt-cv-content-views-script-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/pt-content-views-pro/public/assets/js/cvpro.min.js?ver=5.9.2.2" id="pt-cv-public-pro-script-js"></script>
    <script type="text/javascript" id="email-subscribers-js-extra">
    /* <![CDATA[ */
    var es_data = {"messages":{"es_empty_email_notice":"Bitte gib eine E-Mail Adresse ein","es_rate_limit_notice":"You need to wait for some time before subscribing again","es_single_optin_success_message":"Erfolgreich angemeldet.","es_email_exists_notice":"Diese E-Mail Adresse ist bereits registriert!","es_unexpected_error_notice":"Entschuldigung! Ein unerwarteter Fehler ist aufgetreten.","es_invalid_email_notice":"Ung\u00fcltige E-Mail Adresse","es_try_later_notice":"Bitte versuche es in K\u00fcrze nochmal"},"es_ajax_url":"https:\/\/www.misesde.org\/wp-admin\/admin-ajax.php"};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/email-subscribers/lite/public/js/email-subscribers-public.js?ver=5.6.23" id="email-subscribers-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/jquery/ui/clients.min.js?ver=1.13.2" id="jquery-ui-clients-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/js/shuffle/modernizr.custom.min.js?ver=6.3.1" id="SFSIPLUSjqueryModernizr-js"></script>
    <script type="text/javascript" id="SFSIPLUSCustomJs-js-extra">
    /* <![CDATA[ */
    var sfsi_plus_ajax_object = {"ajax_url":"https:\/\/www.misesde.org\/wp-admin\/admin-ajax.php","plugin_url":"https:\/\/www.misesde.org\/wp-content\/plugins\/ultimate-social-media-plus\/","rest_url":"https:\/\/www.misesde.org\/wp-json\/"};
    var sfsi_plus_links = {"admin_url":"https:\/\/www.misesde.org\/wp-admin\/","plugin_dir_url":"https:\/\/www.misesde.org\/wp-content\/plugins\/ultimate-social-media-plus\/","rest_url":"https:\/\/www.misesde.org\/wp-json\/","pretty_perma":"yes"};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/ultimate-social-media-plus/js/custom.js?ver=3.6.0" id="SFSIPLUSCustomJs-js"></script>
    <script type="text/javascript" id="molongui-authorship-byline-js-extra">
    /* <![CDATA[ */
    var molongui_authorship_byline_params = {"byline_prefix":"","byline_suffix":"","byline_separator":", ","byline_last_separator":" and ","byline_link_title":"View all posts by","byline_link_class":"","byline_dom_tree":"","byline_dom_prepend":"","byline_dom_append":"","byline_decoder":"v3"};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/molongui-authorship/assets/js/byline.334a.min.js?ver=4.7.2" id="molongui-authorship-byline-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/themes/jupiter/assets/js/plugins/wp-enqueue/min/smoothscroll.js?ver=1656075786" id="smoothscroll-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/themes/jupiter/assets/js/min/full-scripts.6.10.2.js?ver=1656075786" id="theme-scripts-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/themes/jupiter/header-builder/includes/assets/js/mkhb-render.js?ver=6.10.2" id="mkhb-render-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/themes/jupiter/header-builder/includes/assets/js/mkhb-column.js?ver=6.10.2" id="mkhb-column-js"></script>
    <script type="text/javascript" id="jupiter-donut-shortcodes-js-extra">
    /* <![CDATA[ */
    var jupiterDonutVars = {"themeDir":"https:\/\/www.misesde.org\/wp-content\/themes\/jupiter","assetsUrl":"https:\/\/www.misesde.org\/wp-content\/plugins\/jupiter-donut\/assets","gridWidth":"1300","ajaxUrl":"https:\/\/www.misesde.org\/wp-admin\/admin-ajax.php","nonce":"524cc2400d"};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/jupiter-donut/assets/js/shortcodes-scripts.min.js?ver=1.4.3" id="jupiter-donut-shortcodes-js"></script>
    <script type="text/javascript" src="https://www.google.com/recaptcha/api.js?render=6LdRLpMcAAAAAK6kIxJdF336GpLu0Z7bjAQ8XTzD&amp;ver=3.0" id="google-recaptcha-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/dist/vendor/wp-polyfill-inert.min.js?ver=3.1.2" id="wp-polyfill-inert-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/dist/vendor/regenerator-runtime.min.js?ver=0.13.11" id="regenerator-runtime-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-includes/js/dist/vendor/wp-polyfill.min.js?ver=3.15.0" id="wp-polyfill-js"></script>
    <script type="text/javascript" id="wpcf7-recaptcha-js-extra">
    /* <![CDATA[ */
    var wpcf7_recaptcha = {"sitekey":"6LdRLpMcAAAAAK6kIxJdF336GpLu0Z7bjAQ8XTzD","actions":{"homepage":"homepage","contactform":"contactform"}};
    /* ]]> */
    </script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/contact-form-7/modules/recaptcha/index.js?ver=5.8.1" id="wpcf7-recaptcha-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/js/dist/js_composer_front.min.js?ver=7.0" id="wpb_composer_front_js-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/masterslider/public/assets/js/jquery.easing.min.js?ver=3.6.5" id="jquery-easing-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/masterslider/public/assets/js/masterslider.min.js?ver=3.6.5" id="masterslider-clients-js"></script>
    <script type="text/javascript" src="https://www.misesde.org/wp-content/plugins/js_composer_theme/assets/lib/vc_waypoints/vc-waypoints.min.js?ver=7.0" id="vc_waypoints-js"></script>
    <script id="pt-cv-append-scripts">if( PT_CV_PAGINATION ) { PT_CV_PAGINATION.links = {"page_1":"https:\/\/www.misesde.org\/","page_n":"https:\/\/www.misesde.org\/?_page=_CVNUMBER_"}; }
    </script><script></script>		<script type="text/javascript">
    jQuery(".mk-button--text").text(function () {
    return jQuery(this).text().replace("READ MORE", "mehr lesen...");
    });		</script>
    <script type="text/javascript">	window.get = {};	window.get.captcha = function(enteredCaptcha) {
    return jQuery.get(ajaxurl, { action : "mk_validate_captcha_input", captcha: enteredCaptcha });
    };</script><div data-m-brand="Molongui" data-m-id="Authorship" data-m-license="Lite" data-m-version="4.7.2" data-m-link="https://www.molongui.com/authorship/"></div>        <script type="text/javascript">
    function AI_responsive_widget() {
    jQuery('object.StefanoAI-youtube-responsive').each(function () {
    jQuery(this).parent('.fluid-width-video-wrapper').removeClass('fluid-width-video-wrapper').removeAttr('style').css('width', '100%').css('display', 'block');
    jQuery(this).children('.fluid-width-video-wrapper').removeClass('fluid-width-video-wrapper').removeAttr('style').css('width', '100%').css('display', 'block');
    var width = jQuery(this).parent().innerWidth();
    var maxwidth = jQuery(this).css('max-width').replace(/px/, '');
    var pl = parseInt(jQuery(this).parent().css('padding-left').replace(/px/, ''));
    var pr = parseInt(jQuery(this).parent().css('padding-right').replace(/px/, ''));
    width = width - pl - pr;
    if (maxwidth < width) {
    width = maxwidth;
    }
    var ratio = jQuery(this).attr('data-ratio');
    if (typeof ratio == 'undefined') {
    ratio = 16 / 9;
    }
    jQuery(this).css('width', width + "px");
    jQuery(this).css('height', width / ratio + "px");
    jQuery(this).find('iframe').css('width', width + "px");
    jQuery(this).find('iframe').css('height', width / ratio + "px");
    });
    }
    if (typeof jQuery !== 'undefined') {
    jQuery(document).ready(function () {
    setTimeout(function () {
    jQuery("div[data-iframe='StefanoAI-youtube-widget-responsive']").each(function () {
    var iframe = jQuery("<iframe></iframe>");
    jQuery.each(this.attributes, function () {
    if (this.name == 'data-iframe' || this.name == 'data-')
    return;
    iframe.attr(this.name.replace(/^data-/, ''), this.value);
    });
    jQuery(iframe).insertAfter(jQuery(this));
    jQuery(this).remove();
    });
    AI_responsive_widget();
    }, 50);
    });
    jQuery(window).resize(function () {
    AI_responsive_widget();
    });
    }
    </script>



    <div><div class="grecaptcha-badge" data-style="bottomright" style="width: 256px; height: 60px; display: block; transition: right 0.3s ease 0s; position: fixed; bottom: 14px; right: -186px; box-shadow: gray 0px 0px 5px; border-radius: 2px; overflow: hidden; --darkreader-inline-boxshadow: #60686c 0px 0px 5px;" data-darkreader-inline-boxshadow=""><div class="grecaptcha-logo"><iframe title="reCAPTCHA" width="256" height="60" role="presentation" name="a-5mm1ed46cmmo" frameborder="0" scrolling="no" sandbox="allow-forms allow-popups allow-same-origin allow-scripts allow-top-navigation allow-modals allow-popups-to-escape-sandbox allow-storage-access-by-user-activation" src="https://www.google.com/recaptcha/api2/anchor?ar=1&amp;k=6LdRLpMcAAAAAK6kIxJdF336GpLu0Z7bjAQ8XTzD&amp;co=aHR0cHM6Ly93d3cubWlzZXNkZS5vcmc6NDQz&amp;hl=de&amp;v=lLirU0na9roYU3wDDisGJEVT&amp;size=invisible&amp;cb=o6sovgkdm7d9"></iframe></div><div class="grecaptcha-error"></div><textarea id="g-recaptcha-response-100000" name="g-recaptcha-response" class="g-recaptcha-response" style="width: 250px; height: 40px; border: 1px solid rgb(193, 193, 193); margin: 10px 25px; padding: 0px; resize: none; display: none; --darkreader-inline-border-top: #42474a; --darkreader-inline-border-right: #42474a; --darkreader-inline-border-bottom: #42474a; --darkreader-inline-border-left: #42474a;" data-darkreader-inline-border-top="" data-darkreader-inline-border-right="" data-darkreader-inline-border-bottom="" data-darkreader-inline-border-left=""></textarea></div><iframe style="display: none;"></iframe></div><div id="cvpboxOverlay" style="display: none;"></div><div id="cvpcolorbox" class="" role="dialog" tabindex="-1" style="display: none;"><div id="cvpboxWrapper"><div><div id="cvpboxTopLeft" style="float: left;"></div><div id="cvpboxTopCenter" style="float: left;"></div><div id="cvpboxTopRight" style="float: left;"></div></div><div style="clear: left;"><div id="cvpboxMiddleLeft" style="float: left;"></div><div id="cvpboxContent" style="float: left;"><div id="cvpboxTitle" style="float: left;"></div><div id="cvpboxCurrent" style="float: left;"></div><button type="button" id="cvpboxPrevious">previous</button><button type="button" id="cvpboxNext">next</button><button type="button" id="cvpboxSlideshow">slideshow</button><div id="cvpboxLoadingOverlay" style="float: left;"></div><div id="cvpboxLoadingGraphic" style="float: left;"></div></div><div id="cvpboxMiddleRight" style="float: left;"></div></div><div style="clear: left;"><div id="cvpboxBottomLeft" style="float: left;"></div><div id="cvpboxBottomCenter" style="float: left;"></div><div id="cvpboxBottomRight" style="float: left;"></div></div></div><div style="position: absolute; width: 9999px; visibility: hidden; display: none; max-width: none;"></div></div></body></html>
    "#
}
