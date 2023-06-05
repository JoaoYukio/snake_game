import initSync, { World, Direction } from "snake_game";

initSync().then((wasm) => {
	const CELL_SIZE = 10; // px
	const WORLD_WIDTH = 8;
	const SNAKE_SPAWN_IDX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

	const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

	const worldWitdh = world.width();
	const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");

	const ctx = canvas.getContext("2d");
	canvas.height = worldWitdh * CELL_SIZE;
	canvas.width = worldWitdh * CELL_SIZE;

	const snakeCellPtr = world.snake_cells(); // É um numero, um endereco de memoria
	const skaneLen = world.snake_len();

	//? Isso é literalemnte, acessar um espaco de memoria, que esta representado pelos valores snakeCellPtr e skaneLen, onde o primeiro indica o inicio e o segundo o fim
	//? E a memoria fisica esta no wasm.memory.buffer
	const snakeCells = new Uint32Array(
		wasm.memory.buffer,
		snakeCellPtr,
		skaneLen
	);

	document.addEventListener("keydown", (e) => {
		switch (e.code) {
			case "ArrowUp":
				world.change_snake_dir(Direction.Up);
				break;
			case "ArrowRight":
				world.change_snake_dir(Direction.Right);
				break;
			case "ArrowDown":
				world.change_snake_dir(Direction.Down);
				break;
			case "ArrowLeft":
				world.change_snake_dir(Direction.Left);
				break;
			default:
				break;
		}
	});

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
		const snakeCells = new Uint32Array(
			wasm.memory.buffer,
			world.snake_cells(),
			world.snake_len()
		);

		snakeCells.forEach((cellIdx, idx) => {
			const col = cellIdx % worldWitdh;
			const row = Math.floor(cellIdx / worldWitdh);

			ctx.fillStyle = idx === 0 ? "#7878db" : "#000000";

			ctx.beginPath();

			ctx.fillRect(
				col * CELL_SIZE,
				row * CELL_SIZE,
				CELL_SIZE,
				CELL_SIZE
			);

			ctx.stroke();
		});
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
