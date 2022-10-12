const siteId = "aawaz";

document.addEventListener("DOMContentLoaded", (_) => {
    initEmbed();
});

window.addEventListener("message", (e) => {
    // ref - https://stackoverflow.com/a/23020025/7699859

    let [event_name, event_data] = e.data;

    let iframe = document.getElementById(siteId + "_iframe");

    switch (event_name) {
        case "setHeight":
            console.log("setheight event, data", event_data);
            iframe.height = event_data;
            break;

        default:
            break;
    }
});

function getSrcDoc() {
    // TODO - move this content to html file.. and refer to that file using relative path.
    // in worst case read the content of the html file and return it as string.
    return `<html>
  <head>
      <base target="_parent" />
  </head>
  <body onLoad="resize();">
    <div id="root"></div>
    <script src="http://localhost:8081/plugin.es.js"></script>
    <link rel="stylesheet" href="http://localhost:8081/style.css">
  </body>
</html>`;
}

function createIframe(siteId: string) {
    let iframe = document.createElement("iframe");
    iframe.id = siteId + "_iframe";
    iframe.srcdoc = getSrcDoc();
    iframe.width = "100%";
    iframe.frameBorder = "0";
    return iframe;
}

function initEmbed() {
    const node = document.getElementById(siteId);

    if (!node) {
        console.error("Aawaz: Can't find root node.");
        return;
    }

    let iframe = createIframe(siteId);
    node.appendChild(iframe);
}
