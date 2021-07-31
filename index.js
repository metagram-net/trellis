import { run } from "./trellis_web/pkg";

import "./styles.css";

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    navigator.serviceWorker
      .register("/service_worker.js")
      .then((reg) => {
        console.log("ServiceWorker registered:", reg.scope);
      })
      .catch((err) => {
        console.error(err);
      });
  });
}

run();
