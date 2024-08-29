import { useEffect, useState } from "react";

import { greet, evaluate_string } from "./pkg/wasm_graph_calc.js";


import * as d3 from "d3";
import { evaluate_graph } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function Graph({equations, setEquations, wasmLoaded}: {equations: string[], setEquations: any, wasmLoaded: boolean}) {

	var svg;

	useEffect(() => {

		if (!svg) {
			return
		}

		svg
			.attr('d', lineGen([]))

		if (!wasmLoaded) {
			return;
		}

		let data = evaluate_graph(equations[0]);

		if (!data) {
			return;
		}
		
		data = data.map(point => 
			[xScale(point[0]), yScale(point[1])]
		);

		console.log("data valid");

		d3.select("path")
			.attr('d', lineGen(data))
			.attr('fill', 'none')
			.attr("stroke", "black")
			.attr("stroke-width", 1.5);

	}, [equations, wasmLoaded]);


	let lineGen = d3.line()
		.curve(d3.curveCardinal);
		//.curve(d3.curveLinear)

	let xAxisScale = d3.scaleLinear()
		.domain([-10, 10])
		.range([0, 400]);

	let axis = d3.axisBottom(xAxisScale);
	
	let xScale = d3.scaleLinear()
		.domain([-10, 10])
		.range([0, 500])

	let yScale = d3.scaleLinear()
		.domain([-10, 10])
		.range([500, 0])

	let points = [];

	for (let i = -10.0; i < 10.0; i += 0.1) {

		let x = i;
		let y = 5 * Math.pow(x, 3) + 7 * Math.pow(x, 2) - 5 * x;

		points.push([xScale(x), yScale(y)]);
	}

	//console.log(xScale(-10));

	//console.log(points);

	let [graphMade, setGraphMade] = useState(
		false
	);

	useEffect(() => {
		// let pathData = lineGen(points);
		// d3.select("path")
		// 	.attr('d', pathData)
		// 	.attr('fill', 'none')
		// 	.attr("stroke", "black")
		// 	.attr("stroke-width", 1.5);

		if (graphMade) {
			return;
		}
		console.log("runnning make graph")

		setGraphMade(true);

		var width = 500;
		var height = 500;

		svg = d3.select("#graph")
			.append("svg")
				.attr("width", width)
				.attr("height", height)
			.append("g")
				//.attr("transform",
				//"translate(" + margin.left + "," + margin.top + ")");

		var x = d3.scaleLinear()
			.domain([-10, 10])
			.range([0, width]);
	
		svg.append("g")
			.call(d3.axisLeft(x));

		var y = d3.scaleLinear()
			.domain([-10, 10])
			.range([ height, 0 ]);

		svg.append("g")
			.call(d3.axisLeft(y));

		svg.append("path")
			.attr('d', lineGen(points))
			.attr('fill', 'none')
			.attr("stroke", "black")
			.attr("stroke-width", 1.5);	

	}, [graphMade]);
	
	return (

		<>
			<div id="graph"></div>
		</>
	);

}

export default Graph;