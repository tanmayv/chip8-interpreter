import Chip8, {EmulatorController} from "./chip8";

// Chip8.init(document, screen);

fetch("./breakout.ch8").then(async (response) => {
  const buffer = await response.arrayBuffer();
  const rom = new Uint8Array(buffer);
    
const controller = new EmulatorController(rom , {});
  // Chip8.run(rom);
});
