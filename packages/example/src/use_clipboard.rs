use anyhow::{Context, Result};
use gloo_utils::errors::JsError;
// pub async fn copy_to_clipboard(text: &str) -> Result<()> {
//     wasm_bindgen_futures::JsFuture::from(
//         web_sys::window()
//             .context("Unable to get the window object")?
//             .navigator()
//             .clipboard()
//             // .context("Unable to get the clipboard object")?
//             .write_text(text),
//     )
//     .await
//     .map_err(|err| JsError::try_from(err).unwrap())
//     .context("Unable to copy text into the clipboard")?;

//     Ok(())
// }
use std::cell::RefCell;

use dioxus::prelude::spawn;
use gloo::{timers::future::TimeoutFuture, utils::window};
use wasm_bindgen_futures::JsFuture;
// use web_sys::{AudioContext, OscillatorType, SpeechSynthesisUtterance};

// pub fn play_text(text: &str, mut onend: impl FnMut() + 'static) {
//     let synth = window().speech_synthesis().unwrap();
//     let utterance = SpeechSynthesisUtterance::new().unwrap();
//     utterance.set_text(text);
//     synth.speak(&utterance);

//     onend();
// }

pub fn copy_to_clipboard(text: &str) {
    let navigator = window().navigator();
    let clipboard = navigator.clipboard();

    let text = text.to_string();

    spawn(async move {
        JsFuture::from(clipboard.write_text(&text)).await.unwrap();
    });
}
