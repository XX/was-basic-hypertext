use std::collections::HashMap;
use std::sync::LazyLock;

use hypertext::{Renderable, rsx};
use was_basic_hypertext::hypertext_elements;

#[derive(Debug, Clone, Copy)]
struct IconData {
    view: &'static str,
    d: &'static str,
}

impl Default for IconData {
    fn default() -> Self {
        // name = "ban"
        Self {
            view: "0 0 512 512",
            d: "M367.2 412.5L99.5 144.8c-22.4 31.4-35.5 69.8-35.5 111.2 0 106 86 192 192 192 41.5 0 79.9-13.1 111.2-35.5zm45.3-45.3c22.4-31.4 35.5-69.8 35.5-111.2 0-106-86-192-192-192-41.5 0-79.9 13.1-111.2 35.5L412.5 367.2zM0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0z",
        }
    }
}

static ICON_DATAS: LazyLock<HashMap<&'static str, IconData>> = LazyLock::new(|| {
    [
        (
            "ban",
            IconData::default(),
        ),
        (
            "bookmark",
            IconData {
                view: "0 0 384 512",
                d: "M0 64C0 28.7 28.7 0 64 0L320 0c35.3 0 64 28.7 64 64l0 417.1c0 25.6-28.5 40.8-49.8 26.6L192 412.8 49.8 507.7C28.5 521.9 0 506.6 0 481.1L0 64zM64 48c-8.8 0-16 7.2-16 16l0 387.2 117.4-78.2c16.1-10.7 37.1-10.7 53.2 0L336 451.2 336 64c0-8.8-7.2-16-16-16L64 48z",
            },
        ),
        (
            "bookmark-solid",
            IconData {
                view: "0 0 384 512",
                d: "M64 0C28.7 0 0 28.7 0 64L0 480c0 11.5 6.2 22.2 16.2 27.8s22.3 5.5 32.2-.4L192 421.3 335.5 507.4c9.9 5.9 22.2 6.1 32.2 .4S384 491.5 384 480l0-416c0-35.3-28.7-64-64-64L64 0z",
            }
        ),
        (
            "check",
            IconData {
                view: "0 0 448 512",
                d: "M434.8 70.1c14.3 10.4 17.5 30.4 7.1 44.7l-256 352c-5.5 7.6-14 12.3-23.4 13.1s-18.5-2.7-25.1-9.3l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l101.5 101.5 234-321.7c10.4-14.3 30.4-17.5 44.7-7.1z",
            }
        ),
        (
            "chevron-down",
            IconData {
                view: "0 0 448 512",
                d: "M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z",
            },
        ),
    ].into_iter().collect()
});

pub fn icon(name: &str) -> impl Renderable {
    let icon_data = ICON_DATAS.get(name).copied().unwrap_or_default();
    rsx! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox=(icon_data.view)>
            // ! Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
            // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
            <path fill="currentColor" d=(icon_data.d)></path>
        </svg>
    }
}
