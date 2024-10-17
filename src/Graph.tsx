import { useEffect, useState } from "react";

import { greet, evaluate_string } from "./pkg/wasm_graph_calc.js";

import * as d3 from "d3";
import { evaluate_graph } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function Graph({
	graphs,
	intercepts,

	minX,
	maxX,

	minY,
	maxY,
}: {
	graphs: any[],
	intercepts: number[][],

	minX: number,
	maxX: number,

	minY: number,
	maxY: number,
}) {

	let width = 700;
	let height = 550;

	// let gd = document.getElementById("graphDiv");

	// if (gd == null) {
	// 	width = 500;
	// 	height = 500;
	// } else {
	// 	width = gd.offsetWidth;
	// 	height = gd.offsetHeight;
	// }

	const margin_width = 10;
	const margin_height = 10;

	const inner_width = width - 2 * margin_width;
	const inner_height = height - 2 * margin_height;

	// const units_width = 10
	// const units_height = 10

	let lineGen = d3.line()
		.curve(d3.curveCardinal);
		// .curve(d3.curveLinear)
	let xScale = d3.scaleLinear()
		.domain([minX, maxX])
		.range([0, inner_width])

	let yScale = d3.scaleLinear()
		.domain([minY, maxY])
		.range([inner_height, 0])

	let points = [];

	for (let i = -10.0; i < 10.0; i += 0.1) {
		let x = i;
		let y = i;

		points.push([xScale(x), yScale(y)]);
	}

	// setup axes
	useEffect(() => {

		function handleZoom(e: any) {

			console.log(e.transform);

			d3.select('#svgMain')
				.attr('transform', e.transform);
		}
	
		let zoom = d3.zoom()
			.on('zoom', handleZoom)
	
		d3.select('svg')
			.call(zoom);

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

	}, [minX, maxX, minY, maxY]);


	// update lines when graphs change
	useEffect(() => {

		d3.selectAll(".plotted_line")
			.remove();

		graphs.forEach(g => {
			let adjusted_values: number[][] = [];

			g.forEach((point: number[]) => {
				let x = point[0];
				let y = point[1];

				let scaled_x = xScale(x);
				let scaled_y = yScale(y);

				let x_valid = x >= minX && x <= maxX;
				let y_valid = y >= minY && y <= maxY;

				if (x_valid && y_valid) {
					adjusted_values.push([scaled_x, scaled_y]);
				}
			});

			d3.select("#svgMain")
				.append("path")
					.attr('d', lineGen(adjusted_values))
					.attr('class', 'plotted_line')
					.attr('transform', `translate(${margin_width}, ${margin_height})`)
					.attr('fill', 'none')
					.attr("stroke", "black")
					.attr("stroke-width", 1.5);	
		});

	}, [graphs, minX, maxX, minY, maxY]);

	// update intercept points when intercepts change
	useEffect(() => {
		if (!intercepts) {
			return;
		}

		d3.select('#svgMain')
			.selectAll('circle')
			.data(intercepts)
			.join('circle')
			.attr('cx', function(i) {
				return xScale(i[0]) + margin_width;
			})
			.attr('cy', function(i) {
				return yScale(i[1]) + margin_height;
			})
			.attr('r', 5)
			.style('fill', 'orange');

	}, [intercepts, minX, maxX, minY, maxY])

	

	
	return (
		<>
			<svg id="topSvg" viewBox={`0 0 ${width} ${height}`}>
				<g id="svgMain">
					<g id="x-axis" transform={`translate(${margin_width}, ${height / 2})`}></g>
					<g id="y-axis" transform={`translate(${width / 2}, ${margin_height})`}></g>
				</g>
			</svg>
		</>
	);

}

export default Graph;