import('./pkg')
  .catch(console.error);

import { render } from './pkg';

const playButton = document.querySelector('#play');
const file = document.querySelector('#file');

let rom = null;

file.addEventListener('change', function readFile() {
  loadFile(this.files[0]);
});

playButton.addEventListener('click', function readFile() {
  if (rom) {
    playButton.style.visibility = 'hidden';
    render(rom);
  }
});

async function loadFile(file) {
  const { size, name } = file;
  playButton.removeAttribute('disabled');
  playButton.textContent = `Play ${name}`;
  const arrayBuffer = await file.arrayBuffer();
  const u8View = new Uint8Array(arrayBuffer);
  rom = u8View;
}
