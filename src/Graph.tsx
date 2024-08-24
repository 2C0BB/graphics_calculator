import { useEffect } from "react";

import { greet, evaluate_string } from "./pkg/wasm_graph_calc.js";

function Graph({equations, setEquations, wasmLoaded}: {equations: string[], setEquations: any, wasmLoaded: boolean}) {

	useEffect(() => {

		// if (!wasmLoaded) {
		// 	console.log("wasm not loaded");
		// 	return;
		// } else {
		// 	//console.log("wasm loaded");
		// 	//greet("wasm loaded");
		// 	//alert(evaluate_string("5+2*3"));
		// 	//alert(evaluate_string("(5+2)*3"));
		// }

		const c = document.getElementById("graph")! as HTMLCanvasElement;
		const ctx = c.getContext("2d")!;

		ctx.beginPath();

		console.log(c.width, c.height);

		ctx.clearRect(0, 0, c.width, c.height);
		
		ctx.moveTo(0, 0);

		for (let i = 0; i < equations.length; i++) {
			let num: number = parseInt(equations[i]);
			console.log(num);
			ctx.lineTo(num, num);
		}

		ctx.stroke();


	}, [equations, wasmLoaded]);

	// useEffect(() => {
	// 	console.log(equations);
	// 	var c = document.getElementById("graph")! as HTMLCanvasElement;
	// 	var ctx = c.getContext("2d")!;

	// 	// ctx.clearRect(0, 0, c.width, c.height);

	// 	ctx.moveTo(0, 0);
	// 	ctx.lineWidth = 10.0;
	// 	ctx.scale(0.1, 0.1)

	// 	for (let i = 0; i < equations.length; i++) {
	// 		let num: number = parseInt(equations[i]);
	// 		console.log(num);
	// 		ctx.lineTo(num, num);
	// 	}

	// 	ctx.stroke();

	// }, [equations]);

	return (
		<canvas id="graph" width={4000} height={2000}></canvas>
	);
}

export default Graph;