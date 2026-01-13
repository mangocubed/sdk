use dioxus::prelude::*;

#[component]
pub fn BackspaceMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M7.22 3.22A.75.75 0 0 1 7.75 3h9A2.25 2.25 0 0 1 19 5.25v9.5A2.25 2.25 0 0 1 16.75 17h-9a.75.75 0 0 1-.53-.22L.97 10.53a.75.75 0 0 1 0-1.06l6.25-6.25Zm3.06 4a.75.75 0 1 0-1.06 1.06L10.94 10l-1.72 1.72a.75.75 0 1 0 1.06 1.06L12 11.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L13.06 10l1.72-1.72a.75.75 0 0 0-1.06-1.06L12 8.94l-1.72-1.72Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn CheckMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M16.704 4.153a.75.75 0 0 1 .143 1.052l-8 10.5a.75.75 0 0 1-1.127.075l-4.5-4.5a.75.75 0 0 1 1.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 0 1 1.05-.143Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn ChevronDownMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn ChevronUpMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M9.47 6.47a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 1 1-1.06 1.06L10 8.06l-3.72 3.72a.75.75 0 0 1-1.06-1.06l4.25-4.25Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn DocumentTextMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M4.5 2A1.5 1.5 0 0 0 3 3.5v13A1.5 1.5 0 0 0 4.5 18h11a1.5 1.5 0 0 0 1.5-1.5V7.621a1.5 1.5 0 0 0-.44-1.06l-4.12-4.122A1.5 1.5 0 0 0 11.378 2H4.5Zm2.25 8.5a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5Zm0 3a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn EyeMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M10 12.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" }
            path {
                clip_rule: "evenodd",
                d: "M.664 10.59a1.651 1.651 0 0 1 0-1.186A10.004 10.004 0 0 1 10 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0 1 10 17c-4.257 0-7.893-2.66-9.336-6.41ZM14 10a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn EyeSlashMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M3.28 2.22a.75.75 0 0 0-1.06 1.06l14.5 14.5a.75.75 0 1 0 1.06-1.06l-1.745-1.745a10.029 10.029 0 0 0 3.3-4.38 1.651 1.651 0 0 0 0-1.185A10.004 10.004 0 0 0 9.999 3a9.956 9.956 0 0 0-4.744 1.194L3.28 2.22ZM7.752 6.69l1.092 1.092a2.5 2.5 0 0 1 3.374 3.373l1.091 1.092a4 4 0 0 0-5.557-5.557Z",
                fill_rule: "evenodd",
            }
            path { d: "m10.748 13.93 2.523 2.523a9.987 9.987 0 0 1-3.27.547c-4.258 0-7.894-2.66-9.337-6.41a1.651 1.651 0 0 1 0-1.186A10.007 10.007 0 0 1 2.839 6.02L6.07 9.252a4 4 0 0 0 4.678 4.678Z" }
        }
    }
}

#[component]
pub fn FilmMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M1 4.75C1 3.784 1.784 3 2.75 3h14.5c.966 0 1.75.784 1.75 1.75v10.515a1.75 1.75 0 0 1-1.75 1.75h-1.5c-.078 0-.155-.005-.23-.015H4.48c-.075.01-.152.015-.23.015h-1.5A1.75 1.75 0 0 1 1 15.265V4.75Zm16.5 7.385V11.01a.25.25 0 0 0-.25-.25h-1.5a.25.25 0 0 0-.25.25v1.125c0 .138.112.25.25.25h1.5a.25.25 0 0 0 .25-.25Zm0 2.005a.25.25 0 0 0-.25-.25h-1.5a.25.25 0 0 0-.25.25v1.125c0 .108.069.2.165.235h1.585a.25.25 0 0 0 .25-.25v-1.11Zm-15 1.11v-1.11a.25.25 0 0 1 .25-.25h1.5a.25.25 0 0 1 .25.25v1.125a.25.25 0 0 1-.164.235H2.75a.25.25 0 0 1-.25-.25Zm2-4.24v1.125a.25.25 0 0 1-.25.25h-1.5a.25.25 0 0 1-.25-.25V11.01a.25.25 0 0 1 .25-.25h1.5a.25.25 0 0 1 .25.25Zm13-2.005V7.88a.25.25 0 0 0-.25-.25h-1.5a.25.25 0 0 0-.25.25v1.125c0 .138.112.25.25.25h1.5a.25.25 0 0 0 .25-.25ZM4.25 7.63a.25.25 0 0 1 .25.25v1.125a.25.25 0 0 1-.25.25h-1.5a.25.25 0 0 1-.25-.25V7.88a.25.25 0 0 1 .25-.25h1.5Zm0-3.13a.25.25 0 0 1 .25.25v1.125a.25.25 0 0 1-.25.25h-1.5a.25.25 0 0 1-.25-.25V4.75a.25.25 0 0 1 .25-.25h1.5Zm11.5 1.625a.25.25 0 0 1-.25-.25V4.75a.25.25 0 0 1 .25-.25h1.5a.25.25 0 0 1 .25.25v1.125a.25.25 0 0 1-.25.25h-1.5Zm-9 3.125a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn GifMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M1 5.25A2.25 2.25 0 0 1 3.25 3h13.5A2.25 2.25 0 0 1 19 5.25v9.5A2.25 2.25 0 0 1 16.75 17H3.25A2.25 2.25 0 0 1 1 14.75v-9.5Zm4.026 2.879C5.356 7.65 5.72 7.5 6 7.5s.643.15.974.629a.75.75 0 0 0 1.234-.854C7.66 6.484 6.873 6 6 6c-.873 0-1.66.484-2.208 1.275C3.25 8.059 3 9.048 3 10c0 .952.25 1.941.792 2.725C4.34 13.516 5.127 14 6 14c.873 0 1.66-.484 2.208-1.275a.75.75 0 0 0 .133-.427V10a.75.75 0 0 0-.75-.75H6.25a.75.75 0 0 0 0 1.5h.591v1.295c-.293.342-.6.455-.841.455-.279 0-.643-.15-.974-.629C4.69 11.386 4.5 10.711 4.5 10c0-.711.19-1.386.526-1.871ZM10.75 6a.75.75 0 0 1 .75.75v6.5a.75.75 0 0 1-1.5 0v-6.5a.75.75 0 0 1 .75-.75Zm3 0h2.5a.75.75 0 0 1 0 1.5H14.5v1.75h.75a.75.75 0 0 1 0 1.5h-.75v2.5a.75.75 0 0 1-1.5 0v-6.5a.75.75 0 0 1 .75-.75Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn GlobeMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M16.555 5.412a8.028 8.028 0 0 0-3.503-2.81 14.899 14.899 0 0 1 1.663 4.472 8.547 8.547 0 0 0 1.84-1.662ZM13.326 7.825a13.43 13.43 0 0 0-2.413-5.773 8.087 8.087 0 0 0-1.826 0 13.43 13.43 0 0 0-2.413 5.773A8.473 8.473 0 0 0 10 8.5c1.18 0 2.304-.24 3.326-.675ZM6.514 9.376A9.98 9.98 0 0 0 10 10c1.226 0 2.4-.22 3.486-.624a13.54 13.54 0 0 1-.351 3.759A13.54 13.54 0 0 1 10 13.5c-1.079 0-2.128-.127-3.134-.366a13.538 13.538 0 0 1-.352-3.758ZM5.285 7.074a14.9 14.9 0 0 1 1.663-4.471 8.028 8.028 0 0 0-3.503 2.81c.529.638 1.149 1.199 1.84 1.66ZM17.334 6.798a7.973 7.973 0 0 1 .614 4.115 13.47 13.47 0 0 1-3.178 1.72 15.093 15.093 0 0 0 .174-3.939 10.043 10.043 0 0 0 2.39-1.896ZM2.666 6.798a10.042 10.042 0 0 0 2.39 1.896 15.196 15.196 0 0 0 .174 3.94 13.472 13.472 0 0 1-3.178-1.72 7.973 7.973 0 0 1 .615-4.115ZM10 15c.898 0 1.778-.079 2.633-.23a13.473 13.473 0 0 1-1.72 3.178 8.099 8.099 0 0 1-1.826 0 13.47 13.47 0 0 1-1.72-3.178c.855.151 1.735.23 2.633.23ZM14.357 14.357a14.912 14.912 0 0 1-1.305 3.04 8.027 8.027 0 0 0 4.345-4.345c-.953.542-1.971.981-3.04 1.305ZM6.948 17.397a8.027 8.027 0 0 1-4.345-4.345c.953.542 1.971.981 3.04 1.305a14.912 14.912 0 0 0 1.305 3.04Z",
            }
        }
    }
}

#[component]
pub fn MagnifyingGlassMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn PhotoMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M1 5.25A2.25 2.25 0 0 1 3.25 3h13.5A2.25 2.25 0 0 1 19 5.25v9.5A2.25 2.25 0 0 1 16.75 17H3.25A2.25 2.25 0 0 1 1 14.75v-9.5Zm1.5 5.81v3.69c0 .414.336.75.75.75h13.5a.75.75 0 0 0 .75-.75v-2.69l-2.22-2.219a.75.75 0 0 0-1.06 0l-1.91 1.909.47.47a.75.75 0 1 1-1.06 1.06L6.53 8.091a.75.75 0 0 0-1.06 0l-2.97 2.97ZM12 7a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn UsersMini(#[props(default = "size-5".to_owned())] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M7 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6ZM14.5 9a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5ZM1.615 16.428a1.224 1.224 0 0 1-.569-1.175 6.002 6.002 0 0 1 11.908 0c.058.467-.172.92-.57 1.174A9.953 9.953 0 0 1 7 18a9.953 9.953 0 0 1-5.385-1.572ZM14.5 16h-.106c.07-.297.088-.611.048-.933a7.47 7.47 0 0 0-1.588-3.755 4.502 4.502 0 0 1 5.874 2.636.818.818 0 0 1-.36.98A7.465 7.465 0 0 1 14.5 16Z" }
        }
    }
}
