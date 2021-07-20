import { precacheAndRoute } from "workbox-precaching";

self.addEventListener("install", (event) => null);
self.addEventListener("fetch", (event) =>
  event.respondWith(fetch(event.request))
);
precacheAndRoute(self.__WB_MANIFEST);
