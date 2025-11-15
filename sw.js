// This is a placeholder service worker.
// In a real app, you would add caching strategies here.

self.addEventListener('install', (event) => {
  console.log('Service Worker installing.');
});

self.addEventListener('fetch', (event) => {
  // This is a "pass-through" fetch handler.
  // It doesn't do anything special, just fetches from the network.
  event.respondWith(fetch(event.request));
});
