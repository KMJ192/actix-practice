import wasm from './pkg/client.js';
import './index.css';

(await wasm()).app();
