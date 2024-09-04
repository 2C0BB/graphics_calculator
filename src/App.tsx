import { useEffect, useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';

import init, { Evaluator, setup } from "./wasm-graph-calc/pkg/wasm_graph_calc.js"

function App() {

	const [wasmLoaded, setWasmLoaded] = useState(false);
	
	useEffect(() => {

		const wasmSetup = async () => {
			await init();
			setup(); // set panic hook
			setWasmLoaded(true);
		}

		wasmSetup()
	});

	const [equations, setEquations] = useState([
		"",
	]);

	const [answers, setAnswers] = useState<any[]>([
		null
	]);

	const [graphs, setGraphs] = useState<any[]>([]);

	// update answers and graphs when equations change
	useEffect(() => {

		if (!wasmLoaded) {
			setAnswers(
				equations.map(() => {
					return undefined;
				})
			);

			return;
		}

		let evaluator = new Evaluator();
		let data = [...equations];

		let new_answers: any[] = [];
		let new_graphs: any[] = [];

		data.forEach(eq => {
			let e = evaluator.evaluate(eq);

			if (!e) {
				new_answers.push(undefined);
			} else if (e.type == "Graph") {
				new_graphs.push(e.points);
				new_answers.push(undefined);
			} else {
				new_answers.push({value: e.value, var_name: e.var_name});
			}
		});

		console.log(new_answers);
		console.log(new_graphs);

		setAnswers(new_answers);
		setGraphs(new_graphs);

		// to avoid memory leaks as wasm does not automatically free structs
		evaluator.free();

	}, [equations, wasmLoaded]);

  return (
	<>
		<div className="split left">
			<EquationInput 
				equations={equations}
				setEquations={setEquations}

				answers={answers}

				wasmLoaded={wasmLoaded}
			/>
		</div>
		<div className="split right">
			<Graph

				graphs={graphs}

				wasmLoaded={wasmLoaded}
			/>
		</div>
    </>
  )
}

export default App;