import * as wasm from "raytracer-wasm";

const CANVAS_WIDTH = 600;
const CANVAS_HEIGHT = 300;
let count = 0;

const world = wasm.create_world();
const camera = wasm.create_camera(CANVAS_WIDTH, CANVAS_HEIGHT);
const $canvas = document.getElementById("canvas");
$canvas.width = CANVAS_WIDTH;
$canvas.height = CANVAS_HEIGHT;

const $freezed = document.getElementById("freezed");
const $freezedText = document.getElementById("freezed-text");
$freezed.addEventListener("click", () => {
  count += 1;
  $freezedText.innerHTML = `No, you are not! (${count})`;
});

const $render = document.getElementById("render");
$render.addEventListener("click", async () => {
  console.log("rendering!");

  const ctx = $canvas.getContext("2d");

  for (let i = 0; i < canvas.width; i++) {
    await new Promise(resolve => {
      setTimeout(() => {
        for (let j = 0; j < canvas.height; j++) {
          const color = wasm.color_at(world, camera, i, j);
          let imageData = ctx.getImageData(i, j, 1, 1);
          imageData.data[0] = 255 * color.red;
          imageData.data[1] = 255 * color.green;
          imageData.data[2] = 255 * color.blue;
          imageData.data[3] = 255;
          ctx.putImageData(imageData, i, j);
        }
        resolve();
      }, 0);
    });
  }
});
