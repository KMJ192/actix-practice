import wasm from './pkg/client.js';

(await wasm()).app();

const socket = new WebSocket(`ws://${window.location.host}/ws/`);

// setInterval(() => {
//   socket.send('test2');
// }, 2000);

socket.addEventListener('message', (msg) => {
  console.log(msg.data);
});
