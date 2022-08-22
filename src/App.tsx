import React from 'react';
import logo from './logo.svg';
import './App.css';
import { listen, emit } from '@tauri-apps/api/event';
// import { appWindow, WebviewWindow } from '@tauri-apps/api/window'; 

listen('event-name', (ev) => {
  alert(`${ev.event}-${JSON.stringify(ev.payload)}`);
})

setTimeout(() => {
  emit('event-name', { message: 'Tauri is awesome' })
}, 1000);

// setTimeout(() => {
//   emit('click', {
//     theMessage: 'Tauri is awesome!'
//   })
// }, 1000);

// only visible to the current window
// appWindow.emit('event', { message: 'Tauri is awesome!' });
// listen('event', (ev) => {
//   console.log(`${ev.event}-${JSON.stringify(ev.payload)}`);
// });

// const webView = new WebviewWindow('window');
// // only to the new window
// webView.emit('event');

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          My first tauri app
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
