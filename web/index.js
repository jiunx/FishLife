import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const viewport = document.getElementById("viewport");
const viewportScale = window.devicePixelRatio || 1;

function resizeCanvas() {
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  // 根据devicePixelRatio调整canvas的实际尺寸
  viewport.width = viewportWidth * viewportScale;
  viewport.height = viewportHeight * viewportScale;

  // 使用原始尺寸设置canvas的CSS样式尺寸
  viewport.style.width = viewportWidth + "px";
  viewport.style.height = viewportHeight + "px";
}

// 初始化canvas大小并设置resize事件监听
resizeCanvas();
window.addEventListener("resize", resizeCanvas);

const ctxt = viewport.getContext("2d");
ctxt.scale(viewportScale, viewportScale);

// 设置线宽以及关闭图像平滑
ctxt.lineWidth = 2;
ctxt.imageSmoothingEnabled = false;

CanvasRenderingContext2D.prototype.drawFish = function (x, y, size, rotation) {
  this.save(); // 保存当前的上下文状态
  this.translate(x, y); // 移动坐标原点到 (x, y)
  this.rotate(rotation + Math.PI / 2); // 旋转坐标系

  // 鱼的身体
  this.beginPath();
  this.ellipse(0, 0, size, size * 0.6, 0, 0, 2 * Math.PI);

  // 鱼的尾巴
  this.moveTo(-size, 0); // 从鱼身结束处开始
  this.lineTo(-size * 1.7, -size * 0.6);
  this.lineTo(-size * 1.7, size * 0.6);
  this.closePath();

  // 上鳍
  this.moveTo(-size * 0.5, -size * 0.6);
  this.quadraticCurveTo(0, -size * 1.3, size * 0.4, -size * 0.5);

  // 下鳍
  this.moveTo(-size * 0.5, size * 0.4);
  this.quadraticCurveTo(0, size * 0.6, size * 0.4, size * 0.2);

  // 鱼的嘴巴
  this.moveTo(size * 0.1, 0);
  this.arc(0, 0, size * 0.1, 0, Math.PI, false);

  // 鱼的眼睛
  let eyeRadius = size * 0.1;
  this.moveTo(size * 0.4 + eyeRadius, -size * 0.4);
  this.arc(size * 0.4, -size * 0.4, eyeRadius, 0, 2 * Math.PI, false);

  this.fillStyle = "#4DB6AC";
  this.fill();
  this.strokeStyle = "#004D40";
  this.lineWidth = 2;
  this.stroke();

  this.restore();
};

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius, color) {
  this.beginPath();

  this.arc(x, y, radius, 0, 2.0 * Math.PI);

  this.fillStyle = color;
  this.fill();
};

function redraw() {
  ctxt.clearRect(
    0,
    0,
    viewport.width / viewportScale,
    viewport.height / viewportScale
  );

  simulation.step();

  const world = simulation.world();

  for (const food of world.foods) {
    ctxt.drawCircle(
      food.x * (viewport.width / viewportScale),
      food.y * (viewport.height / viewportScale),
      (0.01 / 2.0) * (viewport.width / viewportScale),
      food.color
    );
  }

  for (const animal of world.animals) {
    ctxt.drawFish(
      animal.x * (viewport.width / viewportScale),
      animal.y * (viewport.height / viewportScale),
      0.01 * (viewport.width / viewportScale),
      animal.rotation
    );
  }

  requestAnimationFrame(redraw);
}

redraw();
