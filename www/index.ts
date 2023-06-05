import initSync, { World } from "snake_game";

initSync().then((_) => {
	const CELL_SIZE = 10; // px
	const WORLD_WIDTH = 8;
	const SNAKE_SPAWN_IDX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

	const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

	const worldWitdh = world.width();
	const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");

	const ctx = canvas.getContext("2d");
	canvas.height = worldWitdh * CELL_SIZE;
	canvas.width = worldWitdh * CELL_SIZE;

	function drawWorld() {
		ctx.beginPath();

		for (let x = 0; x < worldWitdh + 1; x++) {
			ctx.moveTo(x * CELL_SIZE, 0);
			ctx.lineTo(x * CELL_SIZE, worldWitdh * CELL_SIZE);
		}

		for (let y = 0; y < worldWitdh + 1; y++) {
			ctx.moveTo(0, y * CELL_SIZE);
			ctx.lineTo(worldWitdh * CELL_SIZE, y * CELL_SIZE);
		}

		ctx.stroke();
	}

	function drawSnake() {
		const snakeIdx = world.snake_head();

		const col = snakeIdx % worldWitdh;
		const row = Math.floor(snakeIdx / worldWitdh);

		ctx.beginPath();

		ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

		ctx.stroke();
	}

	function paint() {
		drawWorld();
		drawSnake();
	}
	function update() {
		const fps = 10;
		setTimeout(() => {
			ctx.clearRect(0, 0, canvas.width, canvas.height);

			paint();

			world.update();
			requestAnimationFrame(update);
		}, 1000 / fps);
	}
	paint();
	update();
});
