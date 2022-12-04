import App from "./App.svelte";
import "./main.css";

function resize() {
    // NOTE - call this every time the content inside iframe changes..
    let height = document.getElementsByTagName("html")[0].scrollHeight;
    window.parent.postMessage(["setHeight", height], "*");
}

function setupResizer() {
    // dynamically update iframe height as content inside root element changes.
    // so that there's no vertical scrollbar.

    let rootNode = document.getElementById("root");
    const config = { childList: true, subtree: true };
    const callback = (m, o) => {
        resize();
    };
    const observer = new MutationObserver(callback);
    observer.observe(rootNode, config);
}

function createApp() {
    new App({
        target: document.getElementById("root"),
        props: {
            attrs: {
                host: import.meta.env.BASE_URL.slice(0, -1), // remove trailing slash.. vite expects trailing slash.. but then axios don't behave properly
                theme: "light",
            },
        },
    });
}

function init() {
    setupResizer();
    createApp();
}

init();
