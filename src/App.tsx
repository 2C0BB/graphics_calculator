import { useEffect, useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';

import init, { Evaluator, EvaluatorResponse, initSync } from "./wasm-graph-calc/pkg/wasm_graph_calc.js"

function App() {
	const [equations, setEquations] = useState([
		"",
	]);

	const [wasmLoaded, setWasmLoaded] = useState(false);


	useEffect(() => {

		const asyncTask = async () => {
			await init();
			setWasmLoaded(true);

			// console.log("Finished Load");

			// let evaluator = new Evaluator();
			// console.log(evaluator.evaluate("a=1+2"));
			// console.log(evaluator.evaluate("a+5"));
		}

		asyncTask()
	});

	// load wasm
	// useEffect(() => {
	// 	init().then(() => {
	// 		setWasmLoaded(true)
	// 	})
	// })

  return (
	<>
		<div className="split left">
			<EquationInput 
				equations={equations}
				setEquations={setEquations}

				wasmLoaded={wasmLoaded}
			/>
		</div>
		<div className="split right">
			<Graph
				equations={equations}
				setEquations={setEquations}

				wasmLoaded={wasmLoaded}
			/>
		</div>
    </>
  )
}

export default App;