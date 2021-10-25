export const createDisplay = (
  $canvas,
  width,
  height,
  screenWidth,
  screenHeight
) => {

  const display = $canvas.getContext("2d");

  return {
    draw(vram) {
      const imageData = display.createImageData(width, height);

      for (let i = 0; i < vram.length; i++) {
        imageData.data[i * 4] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 1] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 2] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 3] = 255;
      }

      display.putImageData(imageData, 0, 0);
    },
  };
};
