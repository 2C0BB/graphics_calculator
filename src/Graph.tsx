import { useEffect, useState } from "react";

import { greet, evaluate_string } from "./pkg/wasm_graph_calc.js";

import * as d3 from "d3";
import { evaluate_graph } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function Graph({graphs, wasmLoaded}: {graphs: any[], wasmLoaded: boolean}) {

	const width = 885;
	const height = 580;

	const margin_width = 10;
	const margin_height = 10;

	const inner_width = width - 2 * margin_width;
	const inner_height = height - 2 * margin_height;

	const units_width = 10
	const units_height = 15

	let lineGen = d3.line()
		.curve(d3.curveCardinal);
		//.curve(d3.curveLinear)
	let xScale = d3.scaleLinear()
		.domain([-units_width, units_width])
		.range([0, inner_width])

	let yScale = d3.scaleLinear()
		.domain([-units_height, units_height])
		.range([inner_height, 0])

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

		graphs.forEach(g => {
			let adjusted_values: number[][] = [];

			g.forEach((point: number[]) => {
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
					.attr('transform', `translate(${margin_width}, ${margin_height})`)
					.attr('fill', 'none')
					.attr("stroke", "black")
					.attr("stroke-width", 1.5);	
		});

	}, [graphs, wasmLoaded]);
	
	return (
		<>
			<svg id="topSvg" width={width} height={height}>
				<g id="x-axis" transform={`translate(${margin_width}, ${height / 2})`}></g>
				<g id="y-axis" transform={`translate(${width / 2}, ${margin_height})`}></g>

				{/* <path className="plotted_line" transform="translate(10, 10)"></path> */}
			</svg>
		</>
	);

}

export default Graph;