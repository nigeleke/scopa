#![feature(coverage_attribute)]

#[coverage(off)]
fn main() {
    //     dioxus_logger::init(Level::DEBUG).expect("Cannot start logging");
    //     launch(app);
}

// fn app() -> Element {
//     let document_language = use_resource(move || async move {
//         let mut eval = document::eval("dioxus.send(navigator.language)");
//         let lang = Language::try_from(eval.recv::<String>().await.unwrap()).unwrap();
//         info!("use_resource::lang: {:?}", lang);
//         lang
//     });

//     #[allow(clippy::redundant_closure)]
//     let mut preferred_language =
//         use_storage::<LocalStorage, _>("lang".into(), || document_language());
//     let preferred_language_is_defined = use_signal(|| (*preferred_language.read()).is_some());

//     use_effect(move || {
//         if !*preferred_language_is_defined.read() {
//             preferred_language.set(document_language());
//         }
//     });

//     provide_context(preferred_language);

//     let mut i18n = use_init_i18n(|| i18n::config(langid!("en-GB")));
//     use_effect(move || {
//         if let Some(preferred_language) = preferred_language.read().as_ref() {
//             dioxus::logger::tracing::info!("Setting preferred language {:?}", preferred_language);
//             i18n.set_language(preferred_language.identifier());
//         }
//     });

//     rsx! { Home {} }
// }
