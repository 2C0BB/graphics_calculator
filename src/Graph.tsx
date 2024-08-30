import { useEffect, useState } from "react";

import { greet, evaluate_string } from "./pkg/wasm_graph_calc.js";


import * as d3 from "d3";
import { evaluate_graph } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function Graph({equations, setEquations, wasmLoaded}: {equations: string[], setEquations: any, wasmLoaded: boolean}) {
	let lineGen = d3.line()
		.curve(d3.curveCardinal);
		//.curve(d3.curveLinear)
	
	let xScale = d3.scaleLinear()
		.domain([-10, 10])
		.range([0, 480])

	let yScale = d3.scaleLinear()
		.domain([-10, 10])
		.range([480, 0])

	let points = [];

	for (let i = -10.0; i < 10.0; i += 0.1) {

		let x = i;
		let y = i;

		points.push([xScale(x), yScale(y)]);
	}

	useEffect(() => {
		let x_axis = d3.axisTop(xScale);
		d3.select("#x-axis")
			.attr("class", "axisBlack")
			.call(x_axis);

		let y_axis = d3.axisLeft(yScale);
		d3.select("#y-axis")
			.attr("class", "axisBlack")
			.call(y_axis);

		d3.select("#path1")
			.attr('d', lineGen(points))
			.attr('fill', 'none')
			.attr("stroke", "black")
			.attr("stroke-width", 1.5);	

	}, []);

	useEffect(() => {

		d3.selectAll(".plotted_line")
			.remove();

		if (!wasmLoaded) {
			return;
		}

		equations.forEach(eq => {
			let values = evaluate_graph(eq);

			if (!values) {
				return;
			}

			let adjusted_values: number[][] = [];

			values.forEach((point: number[]) => {
				let x = point[0];
				let y = point[1];

				let scaled_x = xScale(x);
				let scaled_y = yScale(y);

				let x_valid = x >= -10 && x <= 10;
				let y_valid = y >= -10 && y <= 10;

				if (x_valid && y_valid) {
					adjusted_values.push([scaled_x, scaled_y]);
				}
			});

			console.log(adjusted_values)



			d3.select("#topSvg")
				.append("path")
					.attr('d', lineGen(adjusted_values))
					.attr('class', 'plotted_line')
					.attr('transform', 'translate(10, 10)')
					.attr('fill', 'none')
					.attr("stroke", "black")
					.attr("stroke-width", 1.5);	
		});

	}, [equations, wasmLoaded]);
	
	return (
		<>
			<svg id="topSvg" width="500" height="500">
				<g id="x-axis" transform="translate(10, 250)"></g>
				<g id="y-axis" transform="translate(250, 10)"></g>

				{/* <path className="plotted_line" transform="translate(10, 10)"></path> */}
			</svg>
		</>
	);

}

export default Graph;